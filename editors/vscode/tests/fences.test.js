'use strict';

const test = require('node:test');
const assert = require('node:assert');
const fs = require('fs');
const path = require('path');
const vsctm = require('vscode-textmate');
const oniguruma = require('vscode-oniguruma');

const grammarFiles = {
  'source.neon': path.join(__dirname, '..', 'syntaxes', 'neon.tmLanguage.json'),
  'markdown.neon.codeblock': path.join(__dirname, '..', 'syntaxes', 'neon-markdown.tmLanguage.json'),
  'text.html.markdown': path.join(__dirname, 'fixtures', 'markdown.tmLanguage.json'),
};

function loadOnigLib() {
  const wasmPath = path.join(path.dirname(require.resolve('vscode-oniguruma')), 'onig.wasm');
  const wasmBin = fs.readFileSync(wasmPath).buffer;
  return oniguruma.loadWASM(wasmBin).then(() => ({
    createOnigScanner: (patterns) => new oniguruma.OnigScanner(patterns),
    createOnigString: (s) => new oniguruma.OnigString(s),
  }));
}

async function loadMarkdown() {
  const registry = new vsctm.Registry({
    onigLib: loadOnigLib(),
    loadGrammar: async (scopeName) => {
      const file = grammarFiles[scopeName];
      return file ? vsctm.parseRawGrammar(fs.readFileSync(file, 'utf8'), file) : null;
    },
    getInjections: (scopeName) =>
      scopeName === 'text.html.markdown' ? ['markdown.neon.codeblock'] : undefined,
  });
  return registry.loadGrammar('text.html.markdown');
}

async function tokenize(source) {
  const grammar = await loadMarkdown();
  let ruleStack = vsctm.INITIAL;
  return source.split('\n').map((line) => {
    const result = grammar.tokenizeLine(line, ruleStack);
    ruleStack = result.ruleStack;
    return result.tokens.map((t) => ({ text: line.slice(t.startIndex, t.endIndex), scopes: t.scopes }));
  });
}

const hasNeonScope = (tokens) => tokens.some((t) => t.scopes.some((s) => s.endsWith('.neon')));

test('gives a keyword inside a ```neon fence the Neon keyword scope', async () => {
  const lines = await tokenize('```neon\nval x = 1\n```');
  const val = lines[1].find((t) => t.text === 'val');
  assert.ok(val.scopes.includes('storage.type.neon'), val.scopes.join(' '));
  assert.ok(val.scopes.includes('meta.embedded.block.neon'), val.scopes.join(' '));
});

test('leaves a ```js fence without Neon scopes', async () => {
  const lines = await tokenize('```js\nval x = 1\n```');
  assert.ok(!hasNeonScope(lines[1]));
});

test('leaves the Markdown after the closing fence without Neon scopes', async () => {
  const lines = await tokenize('```neon\nval x = 1\n```\nval after');
  assert.ok(!hasNeonScope(lines[3]));
});
