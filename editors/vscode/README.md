# Neon for VS Code

TextMate grammar and language configuration for Neon (`.n` files): highlighting
for keywords, strings, numbers, and operators, plus function and method calls,
parameters, `self`, properties, type names, enum variants, struct fields, and
invalid string escapes; comment toggling, bracket matching/auto-closing, and
indentation on brackets. `.n` files get the Ne icon in the explorer and tabs when the active file
icon theme has none of its own.

## Install

Download `neon-X.Y.Z.vsix` from the [GitHub Releases page](https://github.com/patbuc/neon/releases),
then install it:

```bash
code --install-extension neon-X.Y.Z.vsix
```

The extension isn't published to the Marketplace; releases are the only distribution channel.

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

## Release

Bump `version` in `package.json`, merge to `main`, then push a tag `vscode-vX.Y.Z` matching
that version. The `vscode-release.yml` workflow packages the extension and attaches the VSIX
to a GitHub Release for that tag.
