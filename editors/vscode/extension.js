'use strict';

const { execFile } = require('child_process');
const vscode = require('vscode');

let notified;

function runNeonFmt(neonPath, text, token) {
  return new Promise((resolve) => {
    let sub;
    const child = execFile(neonPath, ['fmt', '-'], { maxBuffer: 64 * 1024 * 1024 }, (error, stdout) => {
      sub?.dispose?.();
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
    if (notified?.path === neonPath && notified.code === error.code) {
      return [];
    }
    notified = { path: neonPath, code: error.code };
    const message =
      error.code === 'ENOENT'
        ? `Could not run "${neonPath}". Set the neon.path setting to the neon binary.`
        : `Could not run "${neonPath}": ${error.message}`;
    vscode.window.showErrorMessage(message);
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
