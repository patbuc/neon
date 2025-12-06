# tree-sitter-neon

Tree-sitter grammar for the [Neon programming language](https://github.com/patbuc/neon).

## Features

- ✅ Full syntax support for Neon language
- ✅ String interpolation with `${expression}`
- ✅ All operators and precedence rules
- ✅ Control flow (if/else, while, for, for-in)
- ✅ Functions and structs
- ✅ Collections (arrays, maps, sets)
- ✅ Syntax highlighting queries for Tree-sitter-compatible editors
- ✅ Comprehensive test suite
- ✅ CI/CD pipeline

## Installation

### For Zed Editor

See [editors/zed/](../../editors/zed/) for Zed extension installation.

### For Neovim (nvim-treesitter)

1. Add Neon parser to your nvim-treesitter config:

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()

parser_config.neon = {
  install_info = {
    url = "https://github.com/patbuc/neon",
    files = {"src/parser.c", "src/scanner.c"},
    branch = "main",
    generate_requires_npm = false,
    requires_generate_from_grammar = false,
  },
  filetype = "neon",
}

vim.filetype.add({
  extension = {
    neon = "neon",
    n = "neon",
  },
})
```

2. Install the parser:

```vim
:TSInstall neon
```

### For Helix

Add to your `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "neon"
scope = "source.neon"
injection-regex = "neon"
file-types = ["neon", "n"]
comment-token = "//"
roots = []

[[grammar]]
name = "neon"
source = { git = "https://github.com/patbuc/neon", rev = "main", subpath = "tree-sitter-neon" }
```

Then run: `hx --grammar fetch && hx --grammar build`

### For Development

```bash
git clone https://github.com/patbuc/neon.git
cd neon/tree-sitter-neon
npm install
npx tree-sitter generate
npx tree-sitter test
```

## Language Features

### Variables

```neon
val immutable = 42
var mutable = 3.14
```

### Functions

```neon
fn add(a, b) {
  return a + b
}

fn greet(name) {
  print "Hello, ${name}!"
}
```

### Structs

```neon
struct Point {
  x
  y
}

val p = Point { x: 10, y: 20 }
```

### Control Flow

```neon
if (x > 5) {
  print "big"
} else {
  print "small"
}

while (i < 10) {
  i++
}

for (item in array) {
  print item
}

for (var i = 0; i < 10; i++) {
  print i
}
```

### Collections

```neon
val arr = [1, 2, 3, 4, 5]
val map = {"key": "value", "count": 42}
val set = {1, 2, 3, 4, 5}
```

### String Interpolation

```neon
val name = "Alice"
val age = 30
print "Name: ${name}, Age: ${age + 1}"
```

### Operators

- Arithmetic: `+`, `-`, `*`, `/`, `//` (floor division), `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Postfix: `++`, `--`
- Range: `..` (exclusive), `..=` (inclusive)

## Testing

```bash
# Run all tests
./test-all.sh

# Run corpus tests only
npx tree-sitter test

# Parse a specific file
npx tree-sitter parse ../examples/day1_solution.n

# Test highlighting
npx tree-sitter highlight test_comprehensive.n

# Performance benchmark
./benchmark.sh
```

See [TESTING.md](TESTING.md) for detailed testing documentation.

## Project Structure

```
tree-sitter-neon/
├── grammar.js          # Grammar definition
├── src/
│   ├── parser.c        # Generated parser
│   ├── scanner.c       # External scanner (string interpolation)
│   └── grammar.json    # Generated grammar spec
├── queries/
│   ├── highlights.scm  # Syntax highlighting
│   ├── indents.scm     # Indentation rules
│   ├── injections.scm  # Language injections
│   └── locals.scm      # Scope analysis
├── test/
│   └── corpus/         # Test cases
├── package.json        # npm package
└── README.md
```

## Development

### Prerequisites

- Node.js 16+
- npm or yarn
- C compiler (for building native bindings)
- tree-sitter-cli

### Building

```bash
npm install
npx tree-sitter generate
npm run build
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new features
5. Run `./test-all.sh` to ensure all tests pass
6. Submit a pull request

## CI/CD

The project uses GitHub Actions for continuous integration. See [.github/workflows/tree-sitter.yml](../../.github/workflows/tree-sitter.yml).

Tests run automatically on:
- Push to main/aoc-main branches
- Pull requests

## License

MIT

## Links

- [Neon Language Repository](https://github.com/patbuc/neon)
- [Tree-sitter](https://tree-sitter.github.io/)
- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)

## Acknowledgments

Built using [Tree-sitter](https://tree-sitter.github.io/), a parser generator tool and incremental parsing library.
