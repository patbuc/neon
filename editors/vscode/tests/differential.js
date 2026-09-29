'use strict';

const fs = require('fs');
const path = require('path');
const { execFileSync } = require('child_process');
const vsctm = require('vscode-textmate');
const oniguruma = require('vscode-oniguruma');

const vscodeDir = path.join(__dirname, '..');
const repoRoot = path.join(vscodeDir, '..', '..');
const grammarPath = path.join(vscodeDir, 'syntaxes', 'neon.tmLanguage.json');
const scriptsDir = path.join(repoRoot, 'tests', 'scripts');

const neonBin = process.env.NEON_BIN || path.join(repoRoot, 'target', 'debug', 'neon');

const KEYWORD_FAMILY = {
  Break: 'keyword.control.neon',
  Continue: 'keyword.control.neon',
  Else: 'keyword.control.neon',
  For: 'keyword.control.neon',
  If: 'keyword.control.neon',
  In: 'keyword.control.neon',
  Return: 'keyword.control.neon',
  While: 'keyword.control.neon',
  Fn: 'storage.type.neon',
  Struct: 'storage.type.neon',
  Impl: 'storage.type.neon',
  Val: 'storage.type.neon',
  Var: 'storage.type.neon',
  True: 'constant.language.neon',
  False: 'constant.language.neon',
  Nil: 'constant.language.neon',
};

const STRING_SCOPE = 'string.quoted.double.neon';
const INTERPOLATION_SCOPE = 'meta.interpolation.neon';
const NUMBER_SCOPE = 'constant.numeric.neon';
const WORD_SHAPED = /^[A-Za-z_]\w*$/;

// For each scanner token kind we check, the [startOffset, endOffset, scope]
// spans (codepoint offsets into the lexeme) that must hold that scope.
const SPANS = {
  Number: (len) => [[0, len, NUMBER_SCOPE]],
  String: (len) => [[0, len, STRING_SCOPE]],
  StringStart: (len) => [
    [0, len - 2, STRING_SCOPE],
    [len - 2, len, INTERPOLATION_SCOPE],
  ],
  StringMiddle: (len) => [
    [0, 1, INTERPOLATION_SCOPE],
    [1, len - 2, STRING_SCOPE],
    [len - 2, len, INTERPOLATION_SCOPE],
  ],
  StringEnd: (len) => [
    [0, 1, INTERPOLATION_SCOPE],
    [1, len, STRING_SCOPE],
  ],
};

function fail(message) {
  console.error(message);
  process.exit(1);
}

function loadOnigLib() {
  const wasmPath = path.join(path.dirname(require.resolve('vscode-oniguruma')), 'onig.wasm');
  const wasmBin = fs.readFileSync(wasmPath).buffer;
  return oniguruma.loadWASM(wasmBin).then(() => ({
    createOnigScanner: (patterns) => new oniguruma.OnigScanner(patterns),
    createOnigString: (s) => new oniguruma.OnigString(s),
  }));
}

async function loadGrammar() {
  const content = fs.readFileSync(grammarPath, 'utf8');
  const rawGrammar = vsctm.parseRawGrammar(content, grammarPath);
  const registry = new vsctm.Registry({
    onigLib: loadOnigLib(),
    loadGrammar: (scopeName) =>
      Promise.resolve(scopeName === rawGrammar.scopeName ? rawGrammar : null),
  });
  return registry.loadGrammar(rawGrammar.scopeName);
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

// Converts a 1-based, codepoint-counted column (as the Rust scanner reports
// it) to the 0-based UTF-16 code unit index vscode-textmate uses.
function utf16Index(lineText, column) {
  return Array.from(lineText).slice(0, column - 1).join('').length;
}

// Position of the codepoint at `offset` into a token's lexeme, accounting
// for a lexeme whose earlier content contains a raw newline.
function positionAtOffset(token, offset) {
  const codepoints = Array.from(token.lexeme).slice(0, offset);
  const newlineCount = codepoints.filter((c) => c === '\n').length;
  if (newlineCount === 0) {
    return { line: token.line, column: token.column + offset };
  }
  const lastNewline = codepoints.lastIndexOf('\n');
  const afterLastNewline = codepoints.slice(lastNewline + 1);
  return { line: token.line + newlineCount, column: afterLastNewline.length + 1 };
}

function scopesAt(perLine, line, column) {
  const entry = perLine[line - 1];
  if (!entry) return null;
  const idx = utf16Index(entry.line, column);
  const token = entry.tokens.find((t) => idx >= t.startIndex && idx < t.endIndex);
  return token ? token.scopes : null;
}

function checkScope(mismatches, file, perLine, line, column, expected) {
  const scopes = scopesAt(perLine, line, column);
  if (!scopes || !scopes.includes(expected)) {
    mismatches.push(
      `${file}:${line}:${column}: expected ${expected}, got ${scopes ? scopes.join(' ') : '<no token>'}`
    );
  }
}

function checkSpan(mismatches, file, perLine, token, startOffset, endOffset, scope) {
  const codepoints = Array.from(token.lexeme);
  for (let offset = startOffset; offset < endOffset; offset++) {
    // A newline ends a TextMate line, so no token covers it.
    if (codepoints[offset] === '\n') continue;
    const pos = positionAtOffset(token, offset);
    checkScope(mismatches, file, perLine, pos.line, pos.column, scope);
  }
}

function checkToken(mismatches, file, perLine, token) {
  if (token.kind.startsWith('Error')) {
    mismatches.push(
      `${file}:${token.line}:${token.column}: scanner rejected the script (${token.kind}: ${token.lexeme})`
    );
    return;
  }

  const family = KEYWORD_FAMILY[token.kind];
  if (family) {
    checkScope(mismatches, file, perLine, token.line, token.column, family);
    return;
  }

  if (token.kind !== 'Identifier' && WORD_SHAPED.test(token.lexeme)) {
    mismatches.push(
      `${file}:${token.line}:${token.column}: token kind "${token.kind}" (lexeme "${token.lexeme}") is not in KEYWORD_FAMILY`
    );
    return;
  }

  const spansFor = SPANS[token.kind];
  if (!spansFor) return;
  const len = Array.from(token.lexeme).length;
  for (const [start, end, scope] of spansFor(len)) {
    checkSpan(mismatches, file, perLine, token, start, end, scope);
  }
}

function scanTokens(file) {
  let stdout;
  try {
    stdout = execFileSync(neonBin, ['--tokens', file], { encoding: 'utf8' });
  } catch (err) {
    fail(`failed to run "${neonBin} --tokens ${file}": ${err.message}`);
  }
  return JSON.parse(stdout);
}

async function main() {
  if (!fs.existsSync(neonBin)) {
    fail(
      `Neon binary not found at ${neonBin}. Build it with "cargo build" first, ` +
        `or point NEON_BIN at a built binary.`
    );
  }

  const grammar = await loadGrammar();
  const files = fs
    .readdirSync(scriptsDir)
    .filter((f) => f.endsWith('.n'))
    .sort()
    .map((f) => path.join(scriptsDir, f));

  const mismatches = [];
  for (const file of files) {
    const tokens = scanTokens(file);
    const source = fs.readFileSync(file, 'utf8');
    const perLine = tokenizeLines(grammar, source);
    const relFile = path.relative(repoRoot, file);
    for (const token of tokens) {
      checkToken(mismatches, relFile, perLine, token);
    }
  }

  if (mismatches.length > 0) {
    for (const m of mismatches) console.error(m);
    fail(`${mismatches.length} scope mismatch(es) across ${files.length} script(s).`);
  }

  console.log(`differential: ${files.length} script(s), all scanner tokens match their grammar scopes.`);
}

main().catch((err) => fail(err.stack || String(err)));
