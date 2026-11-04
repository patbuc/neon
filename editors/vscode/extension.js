'use strict';

const fs = require('fs');
const path = require('path');
const { execFile } = require('child_process');
const vscode = require('vscode');

let notifiedPath;

function isFile(file) {
  try {
    return fs.statSync(file).isFile();
  } catch {
    return false;
  }
}

function resolveNeonPath(neonPath, cwd) {
  if (/[\\/]/.test(neonPath)) {
    return path.resolve(cwd || process.cwd(), neonPath);
  }
  const extensions = process.platform === 'win32' ? ['', ...(process.env.PATHEXT || '.EXE').split(';')] : [''];
  for (const dir of (process.env.PATH || '').split(path.delimiter)) {
    for (const extension of extensions) {
      const candidate = path.join(dir, neonPath + extension);
      if (dir && isFile(candidate)) {
        return candidate;
      }
    }
  }
  return neonPath;
}

function runNeonFmt(neonPath, text, cwd, token) {
  return new Promise((resolve) => {
    const child = execFile(neonPath, ['fmt', '-'], { cwd, maxBuffer: 64 * 1024 * 1024 }, (error, stdout) => {
      if (error && typeof error.code !== 'number' && !error.signal) {
        resolve({ code: null, stdout: '', error });
      } else {
        resolve({ code: error ? (error.code ?? -1) : 0, stdout, error: null });
      }
    });
    if (token) {
      token.onCancellationRequested(() => child.kill());
    }
    // A child that exits without reading all of stdin fails this write with EPIPE; its exit, reported above, decides the outcome.
    child.stdin.on('error', () => {});
    child.stdin.end(text);
  });
}

async function provideDocumentFormattingEdits(document, options, token) {
  const neonPath = vscode.workspace.getConfiguration('neon').get('path');
  const text = document.getText();
  const cwd = document.uri.scheme === 'file' ? path.dirname(document.uri.fsPath) : undefined;
  const { code, stdout, error } = await runNeonFmt(resolveNeonPath(neonPath, cwd), text, cwd, token);
  if (error) {
    if (notifiedPath === neonPath) {
      return [];
    }
    notifiedPath = neonPath;
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

module.exports = { activate, provideDocumentFormattingEdits, resolveNeonPath };
