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

class Diagnostic {
  constructor(range, message, severity) {
    this.range = range;
    this.message = message;
    this.severity = severity;
  }
}

let collection = new Map();
let deleted = [];
const listeners = {};
const fakeCollection = {
  set: (uri, diagnostics) => collection.set(uri.fsPath, diagnostics),
  delete: (uri) => {
    deleted.push(uri.fsPath);
    collection.delete(uri.fsPath);
  },
  dispose() {},
};

const fakeVscode = {
  workspace: {
    getConfiguration: () => ({ get: () => neonPath }),
    textDocuments: [],
    onDidOpenTextDocument: (fn) => ((listeners.open = fn), { dispose() {} }),
    onDidSaveTextDocument: (fn) => ((listeners.save = fn), { dispose() {} }),
    onDidCloseTextDocument: (fn) => ((listeners.close = fn), { dispose() {} }),
  },
  window: { showErrorMessage: (message) => errors.push(message) },
  languages: {
    registerDocumentFormattingEditProvider: () => ({ dispose() {} }),
    createDiagnosticCollection: () => fakeCollection,
  },
  TextEdit: { replace: (range, newText) => ({ range, newText }) },
  Uri: { file: (fsPath) => ({ scheme: 'file', fsPath }) },
  DiagnosticSeverity: { Error: 0 },
  Diagnostic,
  Range,
  Position,
};

const originalLoad = Module._load;
Module._load = function (request, ...rest) {
  return request === 'vscode' ? fakeVscode : originalLoad.call(this, request, ...rest);
};
const { activate, provideDocumentFormattingEdits } = require('../extension.js');
activate({ subscriptions: [] });

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
  collection = new Map();
  deleted = [];
});

test('formats an unsaved buffer to the output of `neon fmt -`', async () => {
  const edits = await provideDocumentFormattingEdits(documentOf('print(  1+2 )\n'));
  assert.strictEqual(edits.length, 1);
  assert.strictEqual(edits[0].newText, 'print(1 + 2)\n');
  assert.deepStrictEqual(edits[0].range.start, new Position(0, 0));
  assert.deepStrictEqual(edits[0].range.end, new Position(1, 0));
});

test('stays silent on exit 65', async () => {
  const edits = await provideDocumentFormattingEdits(documentOf('print(\n'));
  assert.deepStrictEqual(edits, []);
  assert.deepStrictEqual(errors, []);
});

test('shows a notification when the binary exits with a code other than 65', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = path.join(dir, 'neon');
    fs.writeFileSync(neonPath, '#!/bin/sh\nexit 2\n', { mode: 0o755 });
    const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.deepStrictEqual(edits, []);
    assert.strictEqual(errors.length, 1);
    assert.match(errors[0], /exited with code 2/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
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
    fs.writeFileSync(neonPath, '#!/bin/sh\nexit 65\n', { mode: 0o755 });
    const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'.repeat(500000)));
    assert.deepStrictEqual(edits, []);
    assert.deepStrictEqual(errors, []);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('does not run a `neon` found in the document\'s directory', async () => {
  const docDir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  const binDir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  const originalPath = process.env.PATH;
  try {
    fs.writeFileSync(path.join(docDir, 'neon'), '#!/bin/sh\necho hijacked\n', { mode: 0o755 });
    fs.symlinkSync(neonBin, path.join(binDir, 'neon'));
    const uri = { scheme: 'file', fsPath: path.join(docDir, 'main.n') };
    neonPath = 'neon';
    process.env.PATH = '.' + path.delimiter + binDir;
    const edits = await provideDocumentFormattingEdits(documentOf('print(  1+2 )\n', uri));
    process.env.PATH = originalPath;
    assert.deepStrictEqual(errors, []);
    assert.strictEqual(edits.length, 1);
    assert.strictEqual(edits[0].newText, 'print(1 + 2)\n');
  } finally {
    process.env.PATH = originalPath;
    fs.rmSync(docDir, { recursive: true, force: true });
    fs.rmSync(binDir, { recursive: true, force: true });
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

test('shows the missing-binary notification once per `neon.path` value', async () => {
  neonPath = path.join(repoRoot, 'target', 'missing-once-a', 'neon');
  await provideDocumentFormattingEdits(documentOf('print(1)\n'));
  await provideDocumentFormattingEdits(documentOf('print(1)\n'));
  assert.strictEqual(errors.length, 1);
  neonPath = path.join(repoRoot, 'target', 'missing-once-b', 'neon');
  await provideDocumentFormattingEdits(documentOf('print(1)\n'));
  assert.strictEqual(errors.length, 2);
});

test('reports a binary that goes missing again after a successful run', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = path.join(dir, 'neon');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 1);
    fs.symlinkSync(neonBin, neonPath);
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 1);
    fs.rmSync(neonPath);
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 2);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('reports a different failure for the same `neon.path` value', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = path.join(dir, 'neon');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    fs.writeFileSync(neonPath, '', { mode: 0o644 });
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 2);
    assert.match(errors[1], /EACCES/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('kills the formatter when the request is cancelled', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = path.join(dir, 'neon');
    fs.writeFileSync(neonPath, '#!/bin/sh\nsleep 30\n', { mode: 0o755 });
    let listener;
    let disposed = false;
    const token = {
      onCancellationRequested: (fn) => {
        listener = fn;
        return { dispose: () => (disposed = true) };
      },
    };
    const started = Date.now();
    const pending = provideDocumentFormattingEdits(documentOf('print(1)\n'), {}, token);
    listener();
    assert.deepStrictEqual(await pending, []);
    assert.ok(Date.now() - started < 5000);
    assert.ok(disposed);
    assert.deepStrictEqual(errors, []);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

function stubNeon(dir, name, script) {
  const stub = path.join(dir, name);
  fs.writeFileSync(stub, `#!/bin/sh\n${script}\n`, { mode: 0o755 });
  return stub;
}

test('reports an output over the buffer limit as too large', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = stubNeon(dir, 'neon', "head -c 70000000 /dev/zero | tr '\\0' x");
    const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.deepStrictEqual(edits, []);
    assert.strictEqual(errors.length, 1);
    assert.match(errors[0], /too large/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('shows the foreign-exit notification once per `neon.path` value and exit code', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = stubNeon(dir, 'neon-a', 'exit 2');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 1);
    assert.match(errors[0], /exited with code 2/);
    stubNeon(dir, 'neon-a', 'exit 3');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 2);
    assert.match(errors[1], /exited with code 3/);
    neonPath = stubNeon(dir, 'neon-b', 'exit 3');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 3);
    assert.match(errors[2], /exited with code 3/);
    neonPath = path.join(dir, 'neon-c');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 4);
    stubNeon(dir, 'neon-c', 'exit 2');
    await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.strictEqual(errors.length, 5);
    assert.match(errors[4], /exited with code 2/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('reports a formatter that dies from a signal', async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
  try {
    neonPath = stubNeon(dir, 'neon', 'kill -SEGV $$');
    const edits = await provideDocumentFormattingEdits(documentOf('print(1)\n'));
    assert.deepStrictEqual(edits, []);
    assert.strictEqual(errors.length, 1);
    assert.match(errors[0], /SIGSEGV/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

function tempDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'neon-ext-'));
}

function fileDocument(fsPath, languageId = 'neon') {
  const lines = () => fs.readFileSync(fsPath, 'utf8').split('\n');
  return {
    uri: { scheme: 'file', fsPath },
    languageId,
    get lineCount() {
      return lines().length;
    },
    lineAt: (line) => ({ text: lines()[line] }),
  };
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Runs the real binary, then holds its output for `seconds`; with `firstOnly`, only the first run is held.
function slowNeon(dir, seconds, firstOnly) {
  const marker = path.join(dir, 'ran');
  const hold = firstOnly ? `if [ ! -e "${marker}" ]; then touch "${marker}"; sleep ${seconds}; fi` : `sleep ${seconds}`;
  return stubNeon(dir, 'slow-neon', `out=$("${neonBin}" "$@")\ncode=$?\n${hold}\nprintf '%s' "$out"\nexit $code`);
}

test('shows an error diagnostic at the reported line and column on save', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'print(1)\nval x = y\n');
    await listeners.save(fileDocument(file));
    const found = collection.get(file);
    assert.strictEqual(found.length, 1);
    assert.strictEqual(found[0].range.start.line, 1);
    assert.strictEqual(found[0].range.start.character, 8);
    assert.strictEqual(found[0].range.end.character, 9);
    assert.strictEqual(found[0].severity, 0);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('shows the error code and message on the diagnostic', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'print(1)\nval x = y\n');
    await listeners.save(fileDocument(file));
    const [d] = collection.get(file);
    assert.strictEqual(d.code, 'E0013');
    assert.strictEqual(d.source, 'neon');
    assert.match(d.message, /y/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("puts an imported module's error on that module's file", async () => {
  const dir = tempDir();
  try {
    const main = path.join(dir, 'main.n');
    const lib = path.join(dir, 'lib.n');
    fs.writeFileSync(main, 'use "lib"\nprint(lib.x)\n');
    fs.writeFileSync(lib, 'val x = nope\n');
    await listeners.save(fileDocument(main));
    assert.strictEqual(collection.get(lib).length, 1);
    assert.strictEqual(collection.get(lib)[0].code, 'E0013');
    assert.deepStrictEqual(collection.get(main), []);
    fs.writeFileSync(lib, 'val x = 1\n');
    await listeners.save(fileDocument(main));
    assert.ok(!collection.has(lib));
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('clears the diagnostics when a later save compiles cleanly', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    await listeners.save(fileDocument(file));
    assert.strictEqual(collection.get(file).length, 1);
    fs.writeFileSync(file, 'val x = 1\n');
    await listeners.save(fileDocument(file));
    assert.deepStrictEqual(collection.get(file), []);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("clears a file's diagnostics when it is closed", async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    await listeners.open(fileDocument(file));
    assert.strictEqual(collection.get(file).length, 1);
    listeners.close(fileDocument(file));
    assert.ok(deleted.includes(file));
    assert.ok(!collection.has(file));
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('shows no diagnostics and one error notification when the binary is not found', async () => {
  const dir = tempDir();
  try {
    neonPath = path.join(dir, 'missing', 'neon');
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    await listeners.save(fileDocument(file));
    await listeners.save(fileDocument(file));
    assert.strictEqual(collection.size, 0);
    assert.strictEqual(errors.length, 1);
    assert.match(errors[0], /neon\.path/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("keeps the newest check's result when an older check finishes later", async () => {
  const dir = tempDir();
  try {
    neonPath = slowNeon(dir, 1, true);
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    const older = listeners.save(fileDocument(file));
    await sleep(300);
    fs.writeFileSync(file, 'val x = 1\n');
    await listeners.save(fileDocument(file));
    await older;
    assert.deepStrictEqual(collection.get(file), []);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('publishes nothing for a file closed while its check runs', async () => {
  const dir = tempDir();
  try {
    neonPath = slowNeon(dir, 0.5, false);
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    const pending = listeners.open(fileDocument(file));
    await sleep(100);
    listeners.close(fileDocument(file));
    await pending;
    assert.ok(!collection.has(file));
    assert.deepStrictEqual(errors, []);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('ignores untitled and non-neon documents', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    const untitled = { ...fileDocument(file), uri: { scheme: 'untitled', fsPath: file } };
    await listeners.open(untitled);
    await listeners.save(untitled);
    await listeners.open(fileDocument(file, 'plaintext'));
    await listeners.save(fileDocument(file, 'plaintext'));
    assert.strictEqual(collection.size, 0);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('checks the documents already open at activation', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    const other = path.join(dir, 'other.txt');
    fs.writeFileSync(file, 'val x = y\n');
    fs.writeFileSync(other, 'val x = y\n');
    fakeVscode.workspace.textDocuments = [fileDocument(file), fileDocument(other, 'plaintext')];
    activate({ subscriptions: [] });
    for (let i = 0; i < 100 && !collection.has(file); i++) {
      await sleep(50);
    }
    assert.strictEqual(collection.get(file).length, 1);
    assert.ok(!collection.has(other));
  } finally {
    fakeVscode.workspace.textDocuments = [];
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("clears a module's diagnostics when the file that reported them closes", async () => {
  const dir = tempDir();
  try {
    const main = path.join(dir, 'main.n');
    const lib = path.join(dir, 'lib.n');
    fs.writeFileSync(main, 'use "lib"\nprint(lib.x)\n');
    fs.writeFileSync(lib, 'val x = nope\n');
    await listeners.open(fileDocument(main));
    assert.strictEqual(collection.get(lib).length, 1);
    listeners.close(fileDocument(main));
    assert.ok(!collection.has(lib));
    assert.ok(!collection.has(main));
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('notifies once when the output is not JSON, and again after a later failure following a success', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = 1\n');
    const broken = stubNeon(dir, 'broken', 'echo not json');
    neonPath = broken;
    await listeners.save(fileDocument(file));
    await listeners.save(fileDocument(file));
    assert.strictEqual(errors.length, 1);
    assert.match(errors[0], /not valid JSON/);
    neonPath = neonBin;
    await listeners.save(fileDocument(file));
    assert.strictEqual(errors.length, 1);
    neonPath = broken;
    await listeners.save(fileDocument(file));
    assert.strictEqual(errors.length, 2);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('clamps a zero line and column to the first character', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = 1\n');
    neonPath = stubNeon(
      dir,
      'neon',
      `echo '[{"file":null,"line":0,"column":0,"code":"E0000","message":"m"}]'\nexit 65`,
    );
    await listeners.save(fileDocument(file));
    const [d] = collection.get(file);
    assert.deepStrictEqual(d.range.start, new Position(0, 0));
    assert.deepStrictEqual(d.range.end, new Position(0, 1));
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('clears the previous diagnostics when the binary cannot run', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    await listeners.save(fileDocument(file));
    assert.strictEqual(collection.get(file).length, 1);
    neonPath = path.join(dir, 'missing', 'neon');
    await listeners.save(fileDocument(file));
    assert.ok(!collection.has(file));
    assert.strictEqual(errors.length, 1);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('names the file when neon cannot read it', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'gone.n');
    await listeners.save(fileDocument(file));
    assert.strictEqual(errors.length, 1);
    assert.ok(errors[0].includes(file));
    assert.match(errors[0], /could not read/);
    assert.doesNotMatch(errors[0], /neon\.path setting/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('places the marker by UTF-16 units after an emoji on the same line', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val a = "\u{1F600}" + \u{1F600}\n');
    await listeners.save(fileDocument(file));
    const [d] = collection.get(file);
    assert.strictEqual(d.range.start.character, 15);
    assert.strictEqual(d.range.end.character, 17);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('matches the checked file by its real path', async () => {
  const base = fs.realpathSync(tempDir());
  try {
    const real = path.join(base, 'real');
    const link = path.join(base, 'link');
    fs.mkdirSync(real);
    fs.symlinkSync(real, link);
    const file = path.join(link, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    neonPath = stubNeon(
      base,
      'neon',
      `echo '[{"file":"${path.join(real, 'main.n')}","line":1,"column":9,"code":"E0013","message":"m"}]'\nexit 65`,
    );
    await listeners.save(fileDocument(file));
    assert.strictEqual(collection.get(file).length, 1);
    assert.ok(!collection.has(path.join(real, 'main.n')));
  } finally {
    fs.rmSync(base, { recursive: true, force: true });
  }
});

test("keeps a module's own diagnostics when an importer closes", async () => {
  const dir = tempDir();
  try {
    const main = path.join(dir, 'main.n');
    const lib = path.join(dir, 'lib.n');
    fs.writeFileSync(main, 'use "lib"\nprint(lib.x)\n');
    fs.writeFileSync(lib, 'val x = nope\n');
    await listeners.open(fileDocument(lib));
    await listeners.open(fileDocument(main));
    listeners.close(fileDocument(main));
    assert.strictEqual(collection.get(lib).length, 1);
    assert.strictEqual(collection.get(lib)[0].code, 'E0013');
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("keeps a module's diagnostics reported by another open importer when one importer closes", async () => {
  const dir = tempDir();
  try {
    const a = path.join(dir, 'a.n');
    const b = path.join(dir, 'b.n');
    const lib = path.join(dir, 'lib.n');
    fs.writeFileSync(a, 'use "lib"\nprint(lib.x)\n');
    fs.writeFileSync(b, 'use "lib"\nprint(lib.x)\n');
    fs.writeFileSync(lib, 'val x = nope\n');
    await listeners.open(fileDocument(a));
    await listeners.open(fileDocument(b));
    listeners.close(fileDocument(a));
    assert.strictEqual(collection.get(lib).length, 1);
    assert.strictEqual(collection.get(lib)[0].code, 'E0013');
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test('clears the previous diagnostics on every failed check', async () => {
  const dir = tempDir();
  try {
    const file = path.join(dir, 'main.n');
    fs.writeFileSync(file, 'val x = y\n');
    const failures = [
      [stubNeon(dir, 'exits', 'exit 2'), /exited with code 2/],
      [stubNeon(dir, 'killed', 'kill -SEGV $$'), /SIGSEGV/],
      [stubNeon(dir, 'unreadable', `exec "${neonBin}" --check --json "${dir}"`), /could not read .* \(exit 74\)/],
      [stubNeon(dir, 'garbled', 'echo not json'), /not valid JSON/],
    ];
    for (const [stub, message] of failures) {
      neonPath = neonBin;
      await listeners.save(fileDocument(file));
      assert.strictEqual(collection.get(file).length, 1);
      neonPath = stub;
      errors = [];
      await listeners.save(fileDocument(file));
      assert.ok(!collection.has(file));
      assert.strictEqual(errors.length, 1);
      assert.match(errors[0], message);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("places a module's marker by UTF-16 units after an emoji", async () => {
  const dir = tempDir();
  try {
    const main = path.join(dir, 'main.n');
    const lib = path.join(dir, 'lib.n');
    fs.writeFileSync(main, 'use "lib"\nprint(lib.a)\n');
    fs.writeFileSync(lib, 'val a = "\u{1F600}" + \u{1F600}\n');
    await listeners.save(fileDocument(main));
    const [d] = collection.get(lib);
    assert.strictEqual(d.range.start.character, 15);
    assert.strictEqual(d.range.end.character, 17);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
