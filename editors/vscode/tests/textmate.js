'use strict';

const fs = require('fs');
const path = require('path');
const vsctm = require('vscode-textmate');
const oniguruma = require('vscode-oniguruma');

function loadOnigLib() {
  const wasmPath = path.join(path.dirname(require.resolve('vscode-oniguruma')), 'onig.wasm');
  const wasmBin = fs.readFileSync(wasmPath).buffer;
  return oniguruma.loadWASM(wasmBin).then(() => ({
    createOnigScanner: (patterns) => new oniguruma.OnigScanner(patterns),
    createOnigString: (s) => new oniguruma.OnigString(s),
  }));
}

function tokenizeLines(grammar, source) {
  const lines = source.replace(/\r\n/g, '\n').split('\n');
  const perLine = [];
  let ruleStack = vsctm.INITIAL;
  for (const line of lines) {
    const result = grammar.tokenizeLine(line, ruleStack);
    perLine.push({ line, tokens: result.tokens });
    ruleStack = result.ruleStack;
  }
  return perLine;
}

module.exports = { loadOnigLib, tokenizeLines };
