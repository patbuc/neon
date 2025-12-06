# Neon Language Syntax Highlighting

This document describes the syntax highlighting queries implemented for the Neon programming language in Tree-sitter.

## Query Files

The following query files have been created in the `queries/` directory:

### 1. `highlights.scm`
Defines syntax highlighting rules for different language constructs. This is the main file that editors like Zed, Neovim, and Helix use to colorize code.

### 2. `indents.scm`
Defines automatic indentation rules for blocks, functions, and control structures.

### 3. `injections.scm`
Defines language injection rules, specifically for highlighting string interpolation expressions as embedded Neon code.

### 4. `locals.scm`
Defines scoping rules for local variables, parameters, and references. Used for features like "go to definition" and semantic highlighting.

## Supported Highlight Groups

The highlights query uses the following standard Tree-sitter capture names:

### Keywords
- **`@keyword`** - Language keywords
  - `val`, `var`, `fn`, `struct`
  - `if`, `else`, `while`, `for`, `in`
  - `return`, `print`
  - `break`, `continue` (as statement nodes)

### Operators
- **`@operator`** - All operators
  - Arithmetic: `+`, `-`, `*`, `/`, `//`, `%`
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Logical: `&&`, `||`, `!`
  - Postfix: `++`, `--`
  - Range: `..`, `..=`
  - Assignment: `=`

### Punctuation
- **`@punctuation.bracket`** - Brackets: `(`, `)`, `[`, `]`, `{`, `}`
- **`@punctuation.delimiter`** - Delimiters: `,`, `;`, `:`
- **`@punctuation.special`** - Special punctuation: `.`, `${`, `}`

### Literals
- **`@number`** - Numeric literals (integers and floats)
- **`@string`** - String literals and content
- **`@string.escape`** - Escape sequences in strings (`\n`, `\t`, `\xHH`, etc.)
- **`@boolean`** - Boolean literals: `true`, `false`
- **`@constant.builtin`** - Built-in constants: `nil`

### String Interpolation
- **`@embedded`** - Expressions inside string interpolation `${...}`
- String interpolation markers (`${` and `}`) are highlighted as `@punctuation.special`

### Comments
- **`@comment`** - Line comments starting with `//`

### Functions and Methods
- **`@function`** - Function definitions (in `fn` declarations)
- **`@function.call`** - Function calls (e.g., `add(1, 2)`)
- **`@function.method`** - Method calls (e.g., `array.len()`)

### Types
- **`@type`** - Struct names in definitions

### Variables and Identifiers
- **`@variable`** - Variable names in declarations and general identifiers
- **`@parameter`** - Function parameters
- **`@property`** - Object field access (e.g., `point.x`)

### Local Scoping (from `locals.scm`)
- **`@local.definition`** - Variable, function, and struct definitions
- **`@local.reference`** - References to defined identifiers
- **`@local.scope`** - Scope boundaries (functions, blocks, loops)

## Testing

To test the syntax highlighting:

```bash
cd /home/patbuc/code/neon/tree-sitter-neon

# Test on specific files
npx tree-sitter highlight test_comprehensive.n
npx tree-sitter highlight test_highlight_simple.n
npx tree-sitter highlight ../tests/scripts/string_interpolation.n

# Verify queries are valid
npx tree-sitter test
```

## Examples

### Basic Highlighting

```neon
// Comment (gray, italic)
val x = 5           // val: keyword, x: variable, 5: number
var y = 10          // var: keyword, =: operator

fn add(a, b) {      // fn: keyword, add: function, a,b: parameters
  return a + b      // return: keyword, +: operator
}
```

### String Interpolation

```neon
val name = "Alice"
print "Hello, ${name}!"  // ${name} highlighted as embedded code
```

### Control Flow

```neon
if (x < y) {        // if: keyword, <: operator
  break             // break: keyword (statement node)
} else {            // else: keyword
  continue          // continue: keyword (statement node)
}
```

### Methods and Fields

```neon
val length = array.len()     // len: method, (): punctuation
val x_coord = point.x        // x: property/field
```

## Editor Integration

### Zed
Zed will automatically use these queries when the tree-sitter-neon grammar is installed.

### Neovim
Install the parser and queries, then Neovim's Tree-sitter integration will use them automatically.

### Helix
Add the language configuration and queries to your Helix runtime directory.

## Color Scheme Mapping

The capture names map to theme colors depending on the editor:

- **Keywords** → typically bold and colored (often blue/purple)
- **Strings** → typically green
- **Numbers** → typically orange/yellow, often bold
- **Comments** → typically gray/dimmed, often italic
- **Operators** → typically bold gray or colored
- **Functions** → typically cyan/blue
- **Types** → typically yellow/gold

Each editor's theme determines the exact colors used for each capture group.

## Future Enhancements

Possible future improvements to the highlighting queries:

1. Add more specific captures for built-in functions vs user functions
2. Distinguish between different types of collections (arrays, maps, sets)
3. Add semantic highlighting for constant values vs mutable variables
4. Highlight regex patterns if/when added to the language
5. Add more granular captures for different operator types

## Validation

All queries have been validated with:
- ✅ No syntax errors in query files
- ✅ Successfully highlights test files
- ✅ String interpolation works correctly
- ✅ All language features covered
- ✅ Break/continue statements highlighted properly

Last updated: 2025-12-06
