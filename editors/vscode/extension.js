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

function publish(key) {
  const entry = owners.get(key);
  if (entry.byReporter.size === 0) {
    owners.delete(key);
    diagnostics.delete(entry.uri);
  } else {
    // A file's own check is authoritative for its URI; importers' entries only count while it has none.
    const own = entry.byReporter.get(key);
    diagnostics.set(entry.uri, own ?? [...entry.byReporter.values()].flat());
  }
}

function report(reporter, byFile) {
  for (const [key, entry] of owners) {
    if (!byFile.has(key) && entry.byReporter.delete(reporter)) {
      publish(key);
    }
  }
  for (const [key, { uri, list }] of byFile) {
    const entry = owners.get(key);
    if (!entry) {
      owners.set(key, { uri, byReporter: new Map([[reporter, list]]) });
    } else {
      // The open document's URI wins over the path an importer reported.
      if (key === reporter && entry.uri.fsPath !== uri.fsPath) {
        diagnostics.delete(entry.uri);
        entry.uri = uri;
      }
      entry.byReporter.set(reporter, list);
    }
    publish(key);
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

function stripVerbatimPrefix(file) {
  return file.replace(/^\\\\\?\\UNC\\/, '\\\\').replace(/^\\\\\?\\/, '');
}

function lineOf(document, fsPath, row, fileLines) {
  if (document) {
    return row < document.lineCount ? document.lineAt(row).text : undefined;
  }
  if (!fileLines.has(fsPath)) {
    try {
      fileLines.set(fsPath, fs.readFileSync(fsPath, 'utf8').split('\n'));
    } catch {
      fileLines.set(fsPath, undefined);
    }
  }
  return fileLines.get(fsPath)?.[row];
}

// neon prints canonical module paths; map them back to the URI of an open document, else to the path itself.
function openUrisByRealPath() {
  const map = new Map();
  for (const document of vscode.workspace.textDocuments) {
    if (isNeonFile(document)) {
      map.set(realPathOf(document.uri.fsPath), document.uri);
    }
  }
  return map;
}

// neon counts columns in code points, VS Code in UTF-16 units.
function utf16Span(text, col, length) {
  const points = Array.from(text);
  const start = points.slice(0, col).join('').length;
  return { start, length: points.slice(col, col + length).join('').length };
}

function toPosition(document, fsPath, line, column, fileLines) {
  const row = Math.max(0, line - 1);
  const col = Math.max(0, column - 1);
  const text = lineOf(document, fsPath, row, fileLines);
  if (text === undefined) {
    return { start: new vscode.Position(row, col), end: new vscode.Position(row, col + 1) };
  }
  const span = utf16Span(text, col, 1);
  const end = span.start + (span.length || 1);
  return { start: new vscode.Position(row, span.start), end: new vscode.Position(row, end) };
}

// Both the documents' keys and neon's canonical module paths are real paths, so a symlink cannot split one file in two.
async function checkDocument(document) {
  const path = document.uri.fsPath;
  const key = realPathOf(path);
  const generation = nextGeneration(key);
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const { code, signal, stdout, error } = await runNeon(neonPath, ['--check', '--json', path], null);
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
  if (UNREADABLE_EXITS.includes(code)) {
    clearReported(key);
    notifyOnce(neonPath, code, `"${neonPath}" could not read ${path} (exit ${code}).`);
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
  const byFile = new Map([[key, { uri: document.uri, list: [] }]]);
  const openUris = openUrisByRealPath();
  const fileLines = new Map();
  for (const err of errors) {
    const own = err.file === null || err.file === path || err.file === key;
    const file = own ? key : stripVerbatimPrefix(err.file);
    if (!byFile.has(file)) {
      byFile.set(file, { uri: openUris.get(file) ?? vscode.Uri.file(file), list: [] });
    }
    const { start, end } = toPosition(own ? document : null, file, err.line, err.column, fileLines);
    const diagnostic = new vscode.Diagnostic(new vscode.Range(start, end), err.message, vscode.DiagnosticSeverity.Error);
    diagnostic.source = 'neon';
    diagnostic.code = err.code;
    byFile.get(file).list.push(diagnostic);
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
  const key = realPathOf(document.uri.fsPath);
  nextGeneration(key);
  // Drops the closed document's own report and the entries it reported on other URIs; importers' reports on it stay.
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

module.exports = { activate, provideDocumentFormattingEdits };
