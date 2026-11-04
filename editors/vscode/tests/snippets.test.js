'use strict';

const { test } = require('node:test');
const assert = require('node:assert');
const path = require('path');
const fs = require('fs');
const os = require('os');
const { spawnSync } = require('child_process');

const vscodeDir = path.join(__dirname, '..');
const repoRoot = path.join(vscodeDir, '..', '..');
const neonBin = process.env.NEON_BIN || path.join(repoRoot, 'target', 'debug', 'neon');

const snippets = JSON.parse(fs.readFileSync(path.join(vscodeDir, 'snippets', 'neon.json'), 'utf8'));
const prefixes = ['fn', 'struct', 'impl', 'enum', 'match', 'for', 'while', 'try', 'use'];
const preambles = { impl: 'struct Name {}\n\n' };

function expand(snippet) {
  const lines = Array.isArray(snippet.body) ? snippet.body : [snippet.body];
  return (
    lines
      .join('\n')
      .replace(/\$\{\d+:([^${}]*)\}/g, '$1')
      .replace(/\$\{\d+\}|\$\d+/g, '') + '\n'
  );
}

function neon(args, source) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-snippet-'));
  const file = path.join(dir, 'snippet.n');
  fs.writeFileSync(file, source);
  try {
    return spawnSync(neonBin, [...args, file], { encoding: 'utf8' });
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

test('defines a snippet for each prefix', () => {
  const defined = Object.values(snippets).map((s) => s.prefix);
  assert.deepStrictEqual([...defined].sort(), [...prefixes].sort());
});

for (const snippet of Object.values(snippets)) {
  const source = (preambles[snippet.prefix] || '') + expand(snippet);

  test(`${snippet.prefix} expansion passes --check`, () => {
    const result = neon(['--check'], source);
    assert.strictEqual(result.status, 0, result.stderr + result.stdout);
  });

  test(`${snippet.prefix} expansion is formatted`, () => {
    const result = neon(['fmt', '--check'], source);
    assert.strictEqual(result.status, 0, result.stderr + result.stdout);
    assert.strictEqual(result.stdout, '');
  });
}
