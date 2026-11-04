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
const preambles = {
  impl: 'struct Name {}\n\n',
  match: 'val value = 1\n',
  while: 'val condition = false\n',
  for: 'val items = [1, 2]\n',
};

function expandPlaceholders(text) {
  const filled = new Map();
  const at = (re, i) => {
    re.lastIndex = i;
    return re.exec(text);
  };

  function sequence(i, inBraces) {
    let out = '';
    while (i < text.length) {
      const c = text[i];
      if (inBraces && c === '}') break;
      if (c === '\\' && '$}\\'.includes(text[i + 1] ?? '')) {
        out += text[i + 1];
        i += 2;
        continue;
      }
      let m;
      if ((m = at(/\$\{(\d+)\|([^|]*)\|\}/y, i))) {
        const first = m[2].split(',')[0];
        if (!filled.has(m[1])) filled.set(m[1], first);
        out += first;
        i += m[0].length;
      } else if ((m = at(/\$\{(\d+):/y, i))) {
        const [value, end] = sequence(i + m[0].length, true);
        if (!filled.has(m[1])) filled.set(m[1], value);
        out += value;
        i = end + 1;
      } else if ((m = at(/\$\{(\d+)\}|\$(\d+)/y, i))) {
        out += filled.get(m[1] ?? m[2]) ?? '';
        i += m[0].length;
      } else {
        out += c;
        i += 1;
      }
    }
    return [out, i];
  }

  sequence(0, false);
  return sequence(0, false)[0];
}

function expand(snippet) {
  const lines = Array.isArray(snippet.body) ? snippet.body : [snippet.body];
  return expandPlaceholders(lines.join('\n')) + '\n';
}

function neon(args, source) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'neon-snippet-'));
  const file = path.join(dir, 'snippet.n');
  fs.writeFileSync(file, source);
  try {
    const result = spawnSync(neonBin, [...args, file], { encoding: 'utf8' });
    if (result.error) assert.fail(`cannot run ${neonBin}: ${result.error.message}`);
    return result;
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

test('defines a snippet for each prefix', () => {
  const defined = Object.values(snippets).map((s) => s.prefix);
  assert.deepStrictEqual([...defined].sort(), [...prefixes].sort());
});

test('expands repeated, nested, choice and escaped placeholders like VS Code', () => {
  const text = '${1:a} $1 ${0:print($1)} ${2|x,y|} \\$ \\} \\\\ $3 ${4:\\}}';
  assert.strictEqual(expandPlaceholders(text), 'a a print(a) x $ } \\  }');
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
