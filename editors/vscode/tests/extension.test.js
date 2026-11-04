'use strict';

const { test, beforeEach } = require('node:test');
const assert = require('node:assert');
const path = require('path');
const fs = require('fs');
const os = require('os');
const Module = require('module');

const vscodeDir = path.join(__dirname, '..');
const repoRoot = path.join(vscodeDir, '..', '..');
const neonBin = process.env.NEON_BIN || path.join(repoRoot, 'target', 'debug', 'neon');

let neonPath = neonBin;
let errors = [];

class Position {
  constructor(line, character) {
    this.line = line;
    this.character = character;
  }
}

class Range {
  constructor(start, end) {
    this.start = start;
    this.end = end;
  }
}

const fakeVscode = {
  workspace: { getConfiguration: () => ({ get: () => neonPath }) },
  window: { showErrorMessage: (message) => errors.push(message) },
  languages: { registerDocumentFormattingEditProvider: () => ({ dispose() {} }) },
  TextEdit: { replace: (range, newText) => ({ range, newText }) },
  Range,
  Position,
};

const originalLoad = Module._load;
Module._load = function (request, ...rest) {
  return request === 'vscode' ? fakeVscode : originalLoad.call(this, request, ...rest);
};
const { provideDocumentFormattingEdits } = require('../extension.js');

function documentOf(text, uri = { scheme: 'untitled', fsPath: 'Untitled-1' }) {
  return {
    uri,
    getText: () => text,
    positionAt: (offset) => {
      const lines = text.slice(0, offset).split('\n');
      return new Position(lines.length - 1, lines[lines.length - 1].length);
    },
  };
}

beforeEach(() => {
  neonPath = neonBin;
  errors = [];
});

test('formats an unsaved buffer to the output of `neon fmt -`', async () => {
  const edits = await provideDocumentFormattingEdits(documentOf('print(  1+2 )\n'));
  assert.strictEqual(edits.length, 1);
  assert.strictEqual(edits[0].newText, 'print(1 + 2)\n');
  assert.deepStrictEqual(edits[0].range.start, new Position(0, 0));
  assert.deepStrictEqual(edits[0].range.end, new Position(1, 0));
});

test('leaves the document unchanged when `neon fmt -` exits non-zero', async () => {
  const edits = await provideDocumentFormattingEdits(documentOf('print(\n'));
  assert.deepStrictEqual(edits, []);
  assert.deepStrictEqual(errors, []);
});

test('leaves an already-formatted document unchanged', async () => {
  const edits = await provideDocumentFormattingEdits(documentOf('print(1 + 2)\n'));
  assert.deepStrictEqual(edits, []);
});

test('shows one error notification naming the `neon.path` setting when the binary is not found', async () => {
  neonPath = path.join(repoRoot, 'target', 'does-not-exist', 'neon');
  const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'));
  assert.deepStrictEqual(edits, []);
  assert.strictEqual(errors.length, 1);
  assert.match(errors[0], /neon\.path/);
  assert.ok(errors[0].includes(neonPath));
});

test('shows an error notification when the binary fails to start', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = path.join(dir, 'neon');
    fs.writeFileSync(neonPath, '', { mode: 0o644 });
    const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.deepStrictEqual(edits, []);
    assert.strictEqual(errors.length, 1);
    assert.ok(errors[0].includes(neonPath));
    assert.match(errors[0], /EACCES/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('survives a binary that exits without reading a large buffer', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = path.join(dir, 'neon');
    fs.writeFileSync(neonPath, '#!/bin/sh\nexit 1\n', { mode: 0o755 });
    const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'.repeat(500000)));
    assert.deepStrictEqual(edits, []);
    assert.deepStrictEqual(errors, []);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('runs `neon fmt -` from the document\'s directory', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    fs.symlinkSync(neonBin, path.join(dir, 'neon'));
    neonPath = './neon';
    const uri = { scheme: 'file', fsPath: path.join(dir, 'main.n') };
    const edits = await provideDocumentFormattingEdits(documentOf('print(  1+2 )\n', uri));
    assert.deepStrictEqual(errors, []);
    assert.strictEqual(edits.length, 1);
    assert.strictEqual(edits[0].newText, 'print(1 + 2)\n');
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('ignores a `neon.path` set in workspace settings', () => {
  const pkg = require('../package.json');
  assert.strictEqual(pkg.contributes.configuration.properties['neon.path'].scope, 'machine');
});

test('enables format on save for Neon files by default', () => {
  const pkg = require('../package.json');
  assert.strictEqual(pkg.contributes.configurationDefaults['[neon]']['editor.formatOnSave'], true);
});
