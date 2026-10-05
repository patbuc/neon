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

New or edited scripts under `tests/scripts/` and `benches/` must also pass `cargo run -- fmt --check
file.n`; the same hook runs it after the compile check and blocks on an unformatted file, so just run
`cargo run -- fmt file.n` to fix it.

## Where guesses go wrong

**Statements and lines**
- A newline ends a statement. No semicolons (`val x = 1;` is an error), and
  there's no way to put two statements on one line.
- A line continues only if it **ends** with a binary operator, except a line
  that **begins** with `.` or `?.` (not `..`/`..=`), which continues the previous
  statement as a method chain; a blank or comment line in between still ends
  it. A line beginning with `+`, `&&`, etc. is still an error. Open `(`, `[`,
  `{` can span lines.
- Comments are `//` only; no `/* */`.

**Declarations**
- `val` (immutable) and `var` (mutable). No `let`, `const`.
- Function parameters are immutable; copy to a `var` to modify.
- `fn name(a, b) { ... }`. A function's last expression statement is its
  return value, with no `return` needed; a bare `return` or any other kind of
  last statement returns `nil`. Lambdas are `fn(x) { x * 2 }`; no `=>`
  arrows. A named function or method whose whole body is one expression can
  skip the braces with `fn name(a, b) = expr` (lambdas can't use `= expr`).
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
- Conditions are paren-free: `if x {`, `while x {`. Parentheses around a
  condition are just grouping, not required.
- Every branch and loop body needs braces: `if c print(1)` is a compile
  error; write `if c { print(1) } else { print(2) }`.
- `for x in coll { }` is the only `for`; no C-style `for`. Count with a range
  instead: `for i in 0..n { }`. For-in over a map gives keys. `break`/
  `continue` exist.
- No `switch`/`match`, `do`/`while`, `try`/`catch`, or `throw`.

**Operators**
- Compound assignment (`+=`, `-=`, `*=`, `/=`, `%=`, `**=`) works on variables
  (local, global, captured), fields (`o.n += 1`, `self.n += 1`), and indexes
  (`a[0] += 1`, `m["k"] += 1`), evaluating the target's object/index once. It's
  an expression whose value is the new value. No bitwise compound operators.
  There is no `++`/`--`; use `+= 1`/`-= 1`.
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
- `a ?? b` yields `a` unless `a` is nil, else `b`; only `nil` triggers the fallback
  (`false ?? x` is `false`). Precedence is between the ternary and `||`:
  `a ?? b || c` is `a ?? (b || c)`, `c ? a ?? b : d` is `c ? (a ?? b) : d`.
- `a?.f` / `a?.m(args)` is nil if `a` is nil (call args aren't evaluated), else the field or
  method call. Not assignable.

**Collections**
- `{}` is an empty map; `#{}` is an empty set; `#{1, 2}` is a set literal.
- Map keys (and set elements) can be strings, numbers, booleans, enum variants, or arrays. An
  array key is compared by value and frozen at insertion: `m[[1, 2]] = 3` then mutating the
  original array doesn't change the stored key, and `m.keys()`/`entries()`/for-in/`Set.toArray()`
  return a fresh array each time. Every element of an array key must itself be a valid key
  (nested arrays are fine); a cyclic array, or any other type (`nil`, maps, sets, instances,
  functions, files, ranges, priority queues), is a runtime error.
- Missing map keys give `nil` (`m["k"]`, `m.get("k")`), not an error.
- Negative indices work on arrays, ranges, and strings (`a[-1]`, `s[-1]` the last character, counting
  from the end). Strings are indexed by character: `s[0]` is the first character, both as one-character
  strings; an out-of-range index is a runtime error. Strings are immutable, so `s[0] = "x"` is a runtime
  error too. `for ch in s` and `s.chars()` iterate the same way.
- Ranges: `1..10` excludes the end, `1..=10` includes it. They're immutable.

**Sizes and membership use the same names on every collection**

`.size()`, `.contains(x)`, `.isEmpty()` - String, Array, Range, Map, and Set
all use these names (Map checks keys, not values). No property-style
`.length`; everything is a method call.

## Native methods that exist

This is the full list. Anything not here, like `forEach` or `keys` on
arrays, `toFixed`, or `String(x)`, doesn't exist. Add a
helper with `impl Array { fn name(self) { ... } }` if you need one.

- **Global:** `print(a, b, ...)`, `sleep(ms)`, `args` (array of script arguments, strings)
- **Math:** `abs`, `floor`, `ceil`, `sqrt`, `min(...)`, `max(...)`, `div(a, b)`, `round`, `sign`,
  `gcd(a, b)`, `lcm(a, b)`, `mod(a, b)`
- **String:** `size`, `isEmpty`, `substring(start, end)`, `replace(old, new)`,
  `split()` (on Unicode whitespace) / `split(sep)`, `trim`, `startsWith`, `endsWith`, `indexOf`,
  `lastIndexOf`, `contains`, `charCodeAt(index)`, `String.fromCharCode(n)`,
  `repeat(n)`, `padStart(len, fill)`, `padEnd(len, fill)`, `chars()`,
  `toUpperCase`, `toLowerCase`, `toInt`, `toFloat`, `toBool`
- **Number:** `toString`, `toInt`, `toFloat`
- **Boolean:** `toString`
- **Array:** `Array(n, init)`, `push`, `pop`, `size`, `isEmpty`, `contains`, `sort()` / `sort(cmp)`,
  `reverse`, `slice(start, end)`, `join(sep)`, `indexOf`, `sum`, `min`, `max`,
  `map(fn)`, `filter(fn)`, `reduce(fn, initial)`, `forEach(fn)`, `flatMap(fn)`,
  `find(fn)`, `some(fn)`, `every(fn)`, `flat()`, `copy()`, `take(n)`, `drop(n)`,
  `first()`, `last()`, `chunked(n)`, `zip(other)`, `withIndex()`
- **Range:** `size`, `isEmpty`, `contains`, `toArray`, `step(k)`, `slice`, `join`,
  `indexOf`, `sum`, `min`, `max`, `map`, `filter`, `reduce`, `forEach`, `flatMap`,
  `take(n)`, `drop(n)`, `first()`, `last()`, `chunked(n)`, `zip(other)`, `withIndex()`
- **Map:** `get`, `contains`, `remove`, `size`, `isEmpty`, `keys`, `values`, `entries`
- **Set:** `add`, `remove`, `contains`, `size`, `isEmpty`, `clear`, `union`,
  `intersection`, `difference`, `isSubset`, `toArray`
- **File:** `File(path)`, `read`, `readLines`, `write(text)` (creates the file;
  errors if it exists)
- **Stdin:** `Stdin.read()`, `Stdin.readLines()` - read to EOF; `read()` after EOF returns `""`
- **PriorityQueue:** `PriorityQueue()`, `push(priority, value)`, `pop`, `peek`, `size`, `isEmpty` -
  min-heap where priority must be a number; equal priorities pop in insertion order

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
