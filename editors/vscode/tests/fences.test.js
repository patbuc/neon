'use strict';

const test = require('node:test');
const assert = require('node:assert');
const fs = require('fs');
const path = require('path');
const vsctm = require('vscode-textmate');
const { loadOnigLib, tokenizeLines } = require('./textmate');

const grammarFiles = {
  'source.neon': path.join(__dirname, '..', 'syntaxes', 'neon.tmLanguage.json'),
  'markdown.neon.codeblock': path.join(__dirname, '..', 'syntaxes', 'neon-markdown.tmLanguage.json'),
  'text.html.markdown': path.join(__dirname, 'fixtures', 'markdown.tmLanguage.json'),
};

const markdown = (async () => {
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
})();

async function tokenize(source) {
  return tokenizeLines(await markdown, source).map(({ line, tokens }) =>
    tokens.map((t) => ({ text: line.slice(t.startIndex, t.endIndex), scopes: t.scopes }))
  );
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
