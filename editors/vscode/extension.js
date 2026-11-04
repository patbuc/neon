'use strict';

const path = require('path');
const { execFile } = require('child_process');
const vscode = require('vscode');

function runNeonFmt(neonPath, text, cwd) {
  return new Promise((resolve) => {
    const child = execFile(neonPath, ['fmt', '-'], { cwd, maxBuffer: 64 * 1024 * 1024 }, (error, stdout) => {
      if (error && typeof error.code !== 'number') {
        resolve({ code: null, stdout: '', error });
      } else {
        resolve({ code: error ? error.code : 0, stdout, error: null });
      }
    });
    // A child that exits without reading all of stdin fails this write with EPIPE; its exit, reported above, decides the outcome.
    child.stdin.on('error', () => {});
    child.stdin.end(text);
  });
}

async function provideDocumentFormattingEdits(document) {
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const text = document.getText();
  const cwd = document.uri.scheme === 'file' ? path.dirname(document.uri.fsPath) : undefined;
  const { code, stdout, error } = await runNeonFmt(neonPath, text, cwd);
  if (error) {
    const message =
      error.code === 'ENOENT'
        ? `Could not run "${neonPath}". Set the neon.path setting to the neon binary.`
        : `Could not run "${neonPath}": ${error.message}`;
    vscode.window.showErrorMessage(message);
    return [];
  }
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
