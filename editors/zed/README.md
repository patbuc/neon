# Neon Language Extension for Zed Editor

This directory contains the Zed editor extension for the Neon programming language.

## Directory Structure

```
/home/patbuc/code/neon/editors/zed/
├── install-dev.sh              # Development installation script
├── INSTALL_ZED.md              # Instructions for installing Zed editor
├── TEST_IN_ZED.md              # Testing checklist for the extension
├── README.md                   # This file
└── neon/                       # Extension root
    ├── extension.toml          # Extension metadata and configuration
    ├── README.md               # Extension-specific README
    ├── grammars/
    │   └── neon.toml          # Grammar reference configuration
    └── languages/
        └── neon/
            ├── config.toml     # Language configuration (brackets, comments, etc.)
            └── highlights.scm  # Symlink to Tree-sitter highlight queries
```

## Quick Start

### Prerequisites

1. Install Zed editor (see INSTALL_ZED.md if not installed)
2. Ensure Tree-sitter grammar is generated in `/home/patbuc/code/neon/tree-sitter-neon/`

### Installation

Run the development installation script:

```bash
cd /home/patbuc/code/neon/editors/zed
./install-dev.sh
```

This will:
- Create `~/.config/zed/extensions/` directory
- Create a symlink to the Neon extension
- Provide next steps for testing

### Testing

1. Restart Zed or run "Reload Window" command (Cmd+Shift+P → "Reload Window")
2. Open a test file: `zed /home/patbuc/code/neon/examples/day1_solution.n`
3. Follow the checklist in `TEST_IN_ZED.md`

## Features

The Neon extension provides:

- **Syntax Highlighting**: Full support for Neon syntax
  - Keywords (val, var, fn, struct, if, else, while, for, return, print)
  - Operators (arithmetic, comparison, logical, range)
  - Literals (strings, numbers, booleans)
  - Comments
  - String interpolation (`${expression}`)

- **Auto-closing Pairs**:
  - Brackets: `{}`, `[]`, `()`
  - Quotes: `""`

- **Comment Toggle**: Cmd+/ or Ctrl+/ to toggle line comments

- **File Extensions**: `.neon` and `.n`

## Configuration Files

### extension.toml
Defines extension metadata, version, and grammar references. Points to the Tree-sitter grammar in the `tree-sitter-neon` directory.

### languages/neon/config.toml
Configures language-specific settings:
- File extensions
- Comment syntax
- Bracket pairs for auto-closing
- Auto-pairing behavior

### languages/neon/highlights.scm
Symlinked to `/home/patbuc/code/neon/tree-sitter-neon/queries/highlights.scm`. This file defines syntax highlighting rules using Tree-sitter queries.

### grammars/neon.toml
References the Tree-sitter grammar repository and path.

## Development Workflow

### Making Changes to Syntax Highlighting

1. Edit `/home/patbuc/code/neon/tree-sitter-neon/queries/highlights.scm`
2. The changes are immediately reflected through the symlink
3. Reload Zed window (Cmd+Shift+P → "Reload Window")
4. Test with sample files

### Updating Grammar

1. Edit `/home/patbuc/code/neon/tree-sitter-neon/grammar.js`
2. Regenerate parser: `cd tree-sitter-neon && npx tree-sitter generate`
3. Reload Zed window
4. Test with sample files

### Publishing Extension

When ready to publish to Zed's extension marketplace:

1. Ensure all tests pass
2. Update version in `extension.toml`
3. Commit and push to GitHub
4. Follow Zed's extension publishing guidelines

## Testing Files

Good test files to verify highlighting:

1. `/home/patbuc/code/neon/examples/day1_solution.n` - Real-world Advent of Code solution
2. `/home/patbuc/code/neon/tree-sitter-neon/test_comprehensive.n` - Comprehensive feature test
3. `/home/patbuc/code/neon/tests/scripts/string_interpolation.n` - String interpolation

## Troubleshooting

### Extension Not Loading

Check that the symlink exists and points to the correct location:
```bash
ls -la ~/.config/zed/extensions/neon
```

Should show: `~/.config/zed/extensions/neon -> /home/patbuc/code/neon/editors/zed/neon`

### Syntax Highlighting Not Working

1. Verify Tree-sitter grammar is compiled:
   ```bash
   ls /home/patbuc/code/neon/tree-sitter-neon/src/parser.c
   ```

2. Check highlight queries exist:
   ```bash
   ls /home/patbuc/code/neon/tree-sitter-neon/queries/highlights.scm
   ```

3. Check Zed logs:
   ```bash
   tail -f ~/.config/zed/zed.log | grep -i neon
   ```

### Symlink Broken

If the highlights.scm symlink is broken, recreate it:
```bash
cd /home/patbuc/code/neon/editors/zed/neon/languages/neon
rm highlights.scm
ln -s /home/patbuc/code/neon/tree-sitter-neon/queries/highlights.scm highlights.scm
```

## Implementation Status

Phase 6 (Zed Extension) - COMPLETE

- [x] Directory structure created
- [x] Extension configuration (extension.toml)
- [x] Language configuration (config.toml)
- [x] Grammar reference (grammars/neon.toml)
- [x] Highlight queries linked
- [x] Installation script created
- [x] Documentation created
- [x] Testing checklist created

## Next Steps

After completing Phase 6:
- **Phase 7**: VS Code Extension (if desired)
- **Phase 8**: Documentation and publishing

## Repository

https://github.com/patbuc/neon

## License

MIT
