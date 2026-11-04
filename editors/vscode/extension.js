'use strict';

const { execFile } = require('child_process');
const fs = require('fs');
const vscode = require('vscode');

const PARSE_ERROR_EXIT = 65;
const UNREADABLE_EXITS = [66, 74];

let notified;

function notifyOnce(neonPath, key, message) {
  if (notified?.path === neonPath && notified.code === key) {
    return;
  }
  notified = { path: neonPath, code: key };
  vscode.window.showErrorMessage(message);
}

function startFailureMessage(neonPath, error) {
  if (error.code === 'ENOENT') {
    return `Could not run "${neonPath}". Set the neon.path setting to the neon binary.`;
  }
  if (error.code === 'ERR_CHILD_PROCESS_STDIO_MAXBUFFER') {
    return `The output of "${neonPath}" was too large to use.`;
  }
  return `Could not run "${neonPath}": ${error.message}`;
}

function runNeon(neonPath, args, input, token) {
  return new Promise((resolve) => {
    let sub;
    let cancelled = false;
    const child = execFile(neonPath, args, { maxBuffer: 64 * 1024 * 1024 }, (error, stdout) => {
      sub?.dispose();
      if (error && typeof error.code !== 'number' && !error.signal) {
        resolve({ code: null, signal: null, cancelled, stdout: '', error });
      } else {
        resolve({ code: error ? error.code : 0, signal: error?.signal ?? null, cancelled, stdout, error: null });
      }
    });
    if (token) {
      sub = token.onCancellationRequested(() => {
        cancelled = true;
        child.kill();
      });
    }
    // A child that exits without reading all of stdin fails this write with EPIPE; its exit, reported above, decides the outcome.
    child.stdin.on('error', () => {});
    if (input === null || input === undefined) {
      child.stdin.end();
    } else {
      child.stdin.end(input);
    }
  });
}

async function provideDocumentFormattingEdits(document, options, token) {
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const text = document.getText();
  const { code, signal, cancelled, stdout, error } = await runNeon(neonPath, ['fmt', '-'], text, token);
  if (error) {
    notifyOnce(neonPath, error.code, startFailureMessage(neonPath, error));
    return [];
  }
  if (signal) {
    if (!cancelled) {
      notifyOnce(neonPath, signal, `"${neonPath}" was killed by ${signal} while formatting.`);
    }
    return [];
  }
  if (code !== 0 && code !== PARSE_ERROR_EXIT) {
    notifyOnce(
      neonPath,
      code,
      `"${neonPath}" exited with code ${code} while formatting. Check the neon.path setting and the binary.`,
    );
    return [];
  }
  notified = undefined;
  if (code !== 0 || stdout === text) {
    return [];
  }
  const end = document.positionAt(text.length);
  return [vscode.TextEdit.replace(new vscode.Range(new vscode.Position(0, 0), end), stdout)];
}

let diagnostics;
const owners = new Map();
const generations = new Map();

function isNeonFile(document) {
  return document.uri.scheme === 'file' && document.languageId === 'neon';
}

function nextGeneration(fsPath) {
  const generation = (generations.get(fsPath) ?? 0) + 1;
  generations.set(fsPath, generation);
  return generation;
}

function publish(fsPath) {
  const entry = owners.get(fsPath);
  if (entry.byReporter.size === 0) {
    owners.delete(fsPath);
    diagnostics.delete(entry.uri);
  } else {
    diagnostics.set(entry.uri, [...entry.byReporter.values()].flat());
  }
}

function report(reporter, byFile) {
  for (const [fsPath, entry] of owners) {
    if (!byFile.has(fsPath) && entry.byReporter.delete(reporter)) {
      publish(fsPath);
    }
  }
  for (const [fsPath, { uri, list }] of byFile) {
    if (!owners.has(fsPath)) {
      owners.set(fsPath, { uri, byReporter: new Map() });
    }
    owners.get(fsPath).byReporter.set(reporter, list);
    publish(fsPath);
  }
}

function clearReported(reporter) {
  report(reporter, new Map());
}

function realPathOf(fsPath) {
  try {
    return fs.realpathSync.native(fsPath);
  } catch {
    return fsPath;
  }
}

function lineOf(document, fsPath, row) {
  if (document) {
    return row < document.lineCount ? document.lineAt(row).text : undefined;
  }
  try {
    return fs.readFileSync(fsPath, 'utf8').split('\n')[row];
  } catch {
    return undefined;
  }
}

// neon counts columns in code points, VS Code in UTF-16 units.
function toPosition(document, fsPath, line, column) {
  const row = Math.max(0, line - 1);
  const col = Math.max(0, column - 1);
  const text = lineOf(document, fsPath, row);
  if (text === undefined) {
    return { start: new vscode.Position(row, col), end: new vscode.Position(row, col + 1) };
  }
  const points = Array.from(text);
  const start = points.slice(0, col).join('').length;
  const end = start + (points[col]?.length ?? 1);
  return { start: new vscode.Position(row, start), end: new vscode.Position(row, end) };
}

async function checkDocument(document) {
  const key = document.uri.fsPath;
  const generation = nextGeneration(key);
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const { code, signal, stdout, error } = await runNeon(neonPath, ['--check', '--json', key], null);
  if (generations.get(key) !== generation) {
    return;
  }
  if (error) {
    clearReported(key);
    notifyOnce(neonPath, error.code, startFailureMessage(neonPath, error));
    return;
  }
  if (signal) {
    clearReported(key);
    notifyOnce(neonPath, signal, `"${neonPath}" was killed by ${signal} while checking.`);
    return;
  }
  if (code === UNREADABLE_EXITS[0] || code === UNREADABLE_EXITS[1]) {
    clearReported(key);
    notifyOnce(neonPath, code, `"${neonPath}" could not read ${key} (exit ${code}).`);
    return;
  }
  if (code !== 0 && code !== PARSE_ERROR_EXIT) {
    clearReported(key);
    notifyOnce(
      neonPath,
      code,
      `"${neonPath}" exited with code ${code} while checking. Check the neon.path setting and the binary.`,
    );
    return;
  }
  let errors;
  try {
    errors = JSON.parse(stdout);
  } catch {
    clearReported(key);
    notifyOnce(neonPath, 'json', `"${neonPath}" printed output that is not valid JSON while checking.`);
    return;
  }
  notified = undefined;
  const realKey = realPathOf(key);
  const byFile = new Map([[key, { uri: document.uri, list: [] }]]);
  for (const err of errors) {
    const own = err.file === null || err.file === key || err.file === realKey;
    const uri = own ? document.uri : vscode.Uri.file(err.file);
    if (!byFile.has(uri.fsPath)) {
      byFile.set(uri.fsPath, { uri, list: [] });
    }
    const { start, end } = toPosition(own ? document : null, uri.fsPath, err.line, err.column);
    const diagnostic = new vscode.Diagnostic(new vscode.Range(start, end), err.message, vscode.DiagnosticSeverity.Error);
    diagnostic.source = 'neon';
    diagnostic.code = err.code;
    byFile.get(uri.fsPath).list.push(diagnostic);
  }
  report(key, byFile);
}

function checkIfNeon(document) {
  if (isNeonFile(document)) {
    return checkDocument(document);
  }
}

function closeDocument(document) {
  if (!isNeonFile(document)) {
    return;
  }
  const key = document.uri.fsPath;
  nextGeneration(key);
  owners.delete(key);
  diagnostics.delete(document.uri);
  clearReported(key);
}

function activate(context) {
  diagnostics = vscode.languages.createDiagnosticCollection('neon');
  owners.clear();
  generations.clear();
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider('neon', { provideDocumentFormattingEdits }),
    diagnostics,
    vscode.workspace.onDidOpenTextDocument(checkIfNeon),
    vscode.workspace.onDidSaveTextDocument(checkIfNeon),
    vscode.workspace.onDidCloseTextDocument(closeDocument),
  );
  vscode.workspace.textDocuments.forEach(checkIfNeon);
}

module.exports = { activate, provideDocumentFormattingEdits, checkDocument };
