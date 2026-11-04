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
    const message =
      error.code === 'ENOENT'
        ? `Could not run "${neonPath}". Set the neon.path setting to the neon binary.`
        : error.code === 'ERR_CHILD_PROCESS_STDIO_MAXBUFFER'
          ? `The output of "${neonPath}" was too large to use.`
          : `Could not run "${neonPath}": ${error.message}`;
    notifyOnce(neonPath, error.code, message);
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

function activate(context) {
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider('neon', { provideDocumentFormattingEdits }),
  );
}

module.exports = { activate, provideDocumentFormattingEdits };
