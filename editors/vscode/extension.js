'use strict';

const { execFile } = require('child_process');
const vscode = require('vscode');

const PARSE_ERROR_EXIT = 65;
const KILLED = -1;

let notified;

function notifyOnce(neonPath, key, message) {
  if (notified?.path === neonPath && notified.code === key) {
    return;
  }
  notified = { path: neonPath, code: key };
  vscode.window.showErrorMessage(message);
}

function runNeonFmt(neonPath, text, token) {
  return new Promise((resolve) => {
    let sub;
    const child = execFile(neonPath, ['fmt', '-'], { maxBuffer: 64 * 1024 * 1024 }, (error, stdout) => {
      sub?.dispose();
      if (error && typeof error.code !== 'number' && !error.signal) {
        resolve({ code: null, stdout: '', error });
      } else {
        resolve({ code: error ? (error.code ?? -1) : 0, stdout, error: null });
      }
    });
    if (token) {
      sub = token.onCancellationRequested(() => child.kill());
    }
    // A child that exits without reading all of stdin fails this write with EPIPE; its exit, reported above, decides the outcome.
    child.stdin.on('error', () => {});
    child.stdin.end(text);
  });
}

async function provideDocumentFormattingEdits(document, options, token) {
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const text = document.getText();
  const { code, stdout, error } = await runNeonFmt(neonPath, text, token);
  if (error) {
    const message =
      error.code === 'ENOENT'
        ? `Could not run "${neonPath}". Set the neon.path setting to the neon binary.`
        : error.code === 'ERR_CHILD_PROCESS_STDIO_MAXBUFFER'
          ? `The output of "${neonPath}" was too large to use.`
          : `Could not run "${neonPath}": ${error.message}`;
    notifyOnce(neonPath, error.code, message);
    return [];
  }
  if (code !== 0 && code !== PARSE_ERROR_EXIT && code !== KILLED) {
    notifyOnce(
      neonPath,
      code,
      `"${neonPath}" does not support \`fmt\` (exit ${code}). Set the neon.path setting to a current neon binary.`,
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

function activate(context) {
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider('neon', { provideDocumentFormattingEdits }),
  );
}

module.exports = { activate, provideDocumentFormattingEdits };
