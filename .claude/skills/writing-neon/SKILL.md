---
name: writing-neon
description: Use when writing, editing, or debugging Neon source files (`.n`) — test scripts in tests/scripts/, benchmarks in benches/, or examples in docs. Lists where Neon differs from JS/Kotlin/Swift/Lox and the exact native methods that exist. Not for work on the Rust compiler/VM itself.
---

# Writing Neon

Neon looks like JS/Kotlin, but many guesses from those languages fail to
compile. Check this list before writing a construct you haven't seen in Neon
before.

## Find a real example first

`tests/scripts/` has 200+ verified programs; the file names say what they
cover (`for_in_map_keys.n`, `impl_static_method.n`, `closure_counter.n`,
`string_interpolation_nested.n`, ...). Grep there before using a construct
you're unsure of:

```bash
grep -rl "#{" tests/scripts/
```

Then run what you wrote: `cargo run -q -- file.n`. `cargo run -q -- --check file.n` compiles without
running, and a PostToolUse hook runs it automatically after an edit, feeding compile errors back — it
won't catch runtime-only errors (e.g. `s[0]` on a string), so still run the script.

## Where guesses go wrong

**Statements and lines**
- A newline ends a statement. No semicolons (`val x = 1;` is an error), and
  there's no way to put two statements on one line.
- A line continues only if it **ends** with a binary operator. A line that
  begins with `+`, `.`, `&&`, etc. is an error, so no leading-dot method
  chains. Open `(`, `[`, `{` can span lines.
- Comments are `//` only; no `/* */`.

**Declarations**
- `val` (immutable) and `var` (mutable). No `let`, `const`.
- Function parameters are immutable; copy to a `var` to modify.
- `fn name(a, b) { ... }`. Lambdas are `fn(x) { return x * 2 }`; no `=>`
  arrows and no implicit return.
- Struct fields are listed one per line, no commas or types:
  `struct Point {` / `x` / `y` / `}`. Construct with `Point(1, 2)`.
- Methods live in `impl Point { fn len(self) { ... } }`; `self` is an
  explicit first parameter. A method without `self` is static (`Point.origin()`).
  No classes, inheritance, `this`, or `new`.
- Enum variants are declared like struct fields, conventionally one per line,
  no commas: `enum Color {` / `Red` / `Green` / `}`. Top level only. Access is
  always qualified (`Color.Red`); a bare `Color` is a compile error.
  `Color.values()` returns a fresh array of every variant in declaration
  order. No payloads, `impl` blocks, or explicit variant values.

**Control flow**
- Parentheses around conditions are required: `if (x) {`, `while (x) {`.
- Always use braces. `if (c) print(1) else print(2)` on one line doesn't
  compile.
- `for (var i = 0; i < n; i = i + 1) { }` and `for (x in coll) { }`. For-in
  over a map gives keys. `break`/`continue` exist.
- No `switch`/`match`, `do`/`while`, `try`/`catch`, or `throw`.

**Operators**
- Compound assignment (`+=`, `-=`, `*=`, `/=`, `%=`, `**=`) works on variables
  only (local, global, captured); `o.n += 1` and `a[0] += 1` are compile
  errors, write `o.n = o.n + 1`. No bitwise compound operators. `i++` and
  `i--` exist as statements; prefix `++i` does not.
- `/` is always float division (`7 / 2` is `3.5`), even on two ints. Integer floor division is
  `Math.div(a, b)`, not `Math.floor(a / b)` — that round-trips through `f64` and loses precision
  past 2^53.
- A decimal literal with no `.`/exponent (and hex/bin/oct literals) is an int; one with a `.` or
  exponent is a float. `+ - * %` on two ints give an int and raise `integer overflow in <op>` if the
  result doesn't fit `i64`; any float operand makes the result a float.
- `**` is power. `& | ^ ~ << >>` are bitwise and always give an int (float operands truncated).
  `c ? a : b` works.
- `+` needs two numbers or two strings: `"n=" + 3` is a runtime error. Use
  interpolation `"n=${n}"` or `n.toString()`.
- Only `nil` and `false` are falsy; `0` and `""` are truthy. `&&`/`||`
  return an operand, not a boolean.
- `==` compares arrays by value.

**Collections**
- `{}` is an empty map; `#{}` is an empty set; `#{1, 2}` is a set literal.
- Map keys can only be strings, numbers, or booleans.
- Missing map keys give `nil` (`m["k"]`, `m.get("k")`), not an error.
- Negative indices work on arrays and ranges (`a[-1]`).
- Strings cannot be indexed: `s[0]` is a runtime error. Use `s.charAt(0)`.
- Ranges: `1..10` excludes the end, `1..=10` includes it. They're immutable.

**Sizes and membership use different names per type**

| Type   | Size                        | Membership       |
|--------|-----------------------------|------------------|
| String | `.len()`                    | `.indexOf(s) != -1` |
| Array  | `.size()` / `.length()`     | `.contains(v)`   |
| Range  | `.size()` / `.length()`     | `.contains(v)`   |
| Map    | `.size()`                   | `.has(k)`        |
| Set    | `.size()`                   | `.has(v)`        |

No property-style `.length`; everything is a method call.

## Native methods that exist

This is the full list. Anything not here, like `forEach`, `find`, `keys` on
arrays, `toFixed`, `padStart`, or `String(x)`, doesn't exist. Add a
helper with `impl Array { fn name(self) { ... } }` if you need one.

- **Global:** `print(a, b, ...)`, `sleep(ms)`, `args` (array of script arguments, strings)
- **Math:** `abs`, `floor`, `ceil`, `sqrt`, `min(...)`, `max(...)`, `div(a, b)`, `round`, `sign`,
  `gcd(a, b)`, `lcm(a, b)`, `mod(a, b)`
- **String:** `len`, `substring(start, end)`, `replace(old, new)`,
  `split()` (on Unicode whitespace) / `split(sep)`, `trim`, `startsWith`, `endsWith`, `indexOf`,
  `charAt`, `charCodeAt(index)`, `String.fromCharCode(n)`,
  `toUpperCase`, `toLowerCase`, `toInt`, `toFloat`, `toBool`
- **Number:** `toString`, `toInt`, `toFloat`
- **Boolean:** `toString`
- **Array:** `push`, `pop`, `size`, `length`, `contains`, `sort()` / `sort(cmp)`,
  `reverse`, `slice(start, end)`, `join(sep)`, `indexOf`, `sum`, `min`, `max`,
  `map(fn)`, `filter(fn)`, `reduce(fn, initial)`
- **Range:** `size`, `length`, `contains`, `toArray`, `slice`, `join`,
  `indexOf`, `sum`, `min`, `max`, `map`, `filter`, `reduce`
- **Map:** `get`, `has`, `remove`, `size`, `keys`, `values`, `entries`
- **Set:** `add`, `remove`, `has`, `size`, `clear`, `union`,
  `intersection`, `difference`, `isSubset`, `toArray`
- **File:** `File(path)`, `read`, `readLines`, `write(text)` (creates the file;
  errors if it exists)

The source of truth is `src/common/method_registry.rs`; if it disagrees
with this list, trust the registry.

`sort()` sorts in place and returns the same array (not a copy). With a
comparator, `sort(fn(a, b) { ... })` calls it for pairs of elements: negative
puts `a` first, positive puts `b` first, `0` keeps their order (the sort is
stable). The comparator must return a number.

## Test script format

Scripts in `tests/scripts/` carry their expected stdout inline:

```neon
// Expected:
// 3
// hello

print(1 + 2)
print("hello")
```

A script that ends in a runtime error also needs
`// Expected runtime error: <message>`, matched exactly. Check one with
`cargo test --test neon_scripts -- <name>`.
