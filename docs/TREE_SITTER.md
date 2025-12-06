# Tree-sitter Grammar for Neon

The Neon language has a complete Tree-sitter grammar providing syntax highlighting and structural analysis for modern editors.

## Features

The Tree-sitter grammar supports all Neon language features:

- Variables (val/var)
- Functions
- Structs
- Control flow (if/while/for)
- All operators with correct precedence
- Collections (arrays, maps, sets)
- String interpolation
- Comments

## Editor Support

### Zed

See [editors/zed/](../editors/zed/) for Zed extension.

### Neovim

Install via nvim-treesitter. See [tree-sitter-neon/README.md](../tree-sitter-neon/README.md).

### Helix

Add to `languages.toml`. See [tree-sitter-neon/README.md](../tree-sitter-neon/README.md).

## Development

The grammar is located in `tree-sitter-neon/` directory.

See [tree-sitter-neon/README.md](../tree-sitter-neon/README.md) for development guide.

## Testing

Comprehensive test suite with:
- Unit tests (corpus tests)
- Integration tests (real files)
- CI/CD pipeline

See [tree-sitter-neon/TESTING.md](../tree-sitter-neon/TESTING.md).
