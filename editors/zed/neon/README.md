# Neon Language Extension for Zed

Syntax highlighting and language support for the Neon programming language in Zed editor.

## Features

- Syntax highlighting for all Neon language constructs
- String interpolation support (`${expression}`)
- Auto-closing brackets and quotes
- Comment toggling with Cmd+/
- File extensions: `.neon`, `.n`

## Installation

### Development Installation

From the Neon repository root:

```bash
cd editors/zed
./install-dev.sh
```

Then restart Zed or run "Reload Window" command.

### Manual Installation

1. Copy the `neon` directory to `~/.config/zed/extensions/`
2. Restart Zed
3. Open a `.neon` or `.n` file

## Supported Syntax

- Variables: `val`, `var`
- Functions: `fn name(params) { ... }`
- Structs: `struct Name { fields }`
- Control flow: `if`, `else`, `while`, `for`
- Operators: `+`, `-`, `*`, `/`, `//`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `!`, `++`, `--`, `..`, `..=`
- Collections: Arrays `[1, 2, 3]`, Maps `{"key": "value"}`, Sets `{1, 2, 3}`
- String interpolation: `"Value: ${expression}"`
- Comments: `// line comment`

## Testing

Open any `.neon` or `.n` file in Zed and verify:
- Keywords are highlighted in purple
- Strings are highlighted in green
- Comments are grayed out
- Functions are highlighted in blue
- Numbers are highlighted in orange
- String interpolations show expressions in different colors

## Development

The extension references the Tree-sitter grammar at `../../tree-sitter-neon/`.

To update highlighting:
1. Edit `../../tree-sitter-neon/queries/highlights.scm`
2. Run `npx tree-sitter generate` in the tree-sitter-neon directory
3. Reload Zed window

## Troubleshooting

### Syntax highlighting not working
1. Verify the extension is installed: `ls ~/.config/zed/extensions/`
2. Check Zed logs: `tail -f ~/.config/zed/zed.log`
3. Try restarting Zed completely

### Grammar not found
Ensure the `tree-sitter-neon` directory exists and contains:
- `src/parser.c`
- `queries/highlights.scm`

## Repository

https://github.com/patbuc/neon

## License

MIT
