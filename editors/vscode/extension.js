'use strict';

const { execFile } = require('child_process');
const vscode = require('vscode');

const PARSE_ERROR_EXIT = 65;

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
const reported = new Map();

function isNeonFile(document) {
  return document.uri.scheme === 'file' && document.languageId === 'neon';
}

async function checkDocument(document) {
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const { code, signal, stdout, error } = await runNeon(
    neonPath,
    ['--check', '--json', document.uri.fsPath],
    null,
  );
  if (error) {
    notifyOnce(neonPath, error.code, startFailureMessage(neonPath, error));
    return;
  }
  if (signal) {
    notifyOnce(neonPath, signal, `"${neonPath}" was killed by ${signal} while checking.`);
    return;
  }
  if (code !== 0 && code !== PARSE_ERROR_EXIT) {
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
    notifyOnce(neonPath, 'json', `"${neonPath}" printed output that is not valid JSON while checking.`);
    return;
  }
  notified = undefined;
  const byFile = new Map([[document.uri.fsPath, { uri: document.uri, list: [] }]]);
  for (const err of errors) {
    const uri = err.file === null ? document.uri : vscode.Uri.file(err.file);
    if (!byFile.has(uri.fsPath)) {
      byFile.set(uri.fsPath, { uri, list: [] });
    }
    const range = new vscode.Range(
      new vscode.Position(err.line - 1, err.column - 1),
      new vscode.Position(err.line - 1, err.column),
    );
    const diagnostic = new vscode.Diagnostic(range, err.message, vscode.DiagnosticSeverity.Error);
    diagnostic.source = 'neon';
    diagnostic.code = err.code;
    byFile.get(uri.fsPath).list.push(diagnostic);
  }
  for (const [fsPath, uri] of reported.get(document.uri.fsPath) ?? []) {
    if (!byFile.has(fsPath)) {
      diagnostics.delete(uri);
    }
  }
  const now = new Map();
  for (const [fsPath, { uri, list }] of byFile) {
    diagnostics.set(uri, list);
    now.set(fsPath, uri);
  }
  reported.set(document.uri.fsPath, now);
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
  diagnostics.delete(document.uri);
  for (const uri of (reported.get(document.uri.fsPath) ?? new Map()).values()) {
    diagnostics.delete(uri);
  }
  reported.delete(document.uri.fsPath);
}

function activate(context) {
  diagnostics = vscode.languages.createDiagnosticCollection('neon');
  reported.clear();
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
