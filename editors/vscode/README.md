# Neon for VS Code

TextMate grammar and language configuration for Neon (`.n` files): keyword,
string, number, and operator highlighting, comment toggling, and bracket
matching/auto-closing.

## Try it locally

```bash
ln -s "$(pwd)/editors/vscode" ~/.vscode/extensions/neon
```

Then reload VS Code window (`Developer: Reload Window`) and open a `.n` file.

## Tests

```bash
cargo build
cd editors/vscode
npm ci
npm test
```

`npm test` runs the `vscode-tmgrammar-test` assertion tests in `tests/*.test.n`, then
`tests/differential.js`, which tokenizes every script in `tests/scripts/` with both the Neon
scanner (via `neon --tokens`) and the grammar, and fails if a keyword, string, interpolation, or
number token lands in the wrong scope. The differential test looks for the Neon binary at
`target/debug/neon`; set `NEON_BIN` to point at a different build.
