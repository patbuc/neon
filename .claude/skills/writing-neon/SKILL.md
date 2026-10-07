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
won't catch runtime-only errors (e.g. `"n=" + 3`), so still run the script.

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
  `{` can span lines. For example:
  ```neon
  val ys = [1, 2, 3]
      .map { it * 2 }
      .filter { it > 2 }
  ```
- Comments are `//` only; no `/* */`.

**Declarations**
- `val` (immutable) and `var` (mutable). No `let`, `const`.
- Function parameters are immutable; copy to a `var` to modify.
- `fn name(a, b) { ... }`. A function's last expression statement is its
  return value, with no `return` needed; a bare `return` or any other kind of
  last statement returns `nil`. Lambdas are `fn(x) { x * 2 }`; no `=>`
  arrows. A named function or method whose whole body is one expression can
  skip the braces with `fn name(a, b) = expr` (lambdas can't use `= expr`). A
  statement starting with `fn(` is always a lambda, not a declaration, so it
  can be called right away: `fn(x) { print(x) }(5)`.
- A function literal that's a call's last argument can be written as a
  trailing block instead: `xs.map { it * 2 }`, `xs.reduce(0) { acc, x -> acc + x }`,
  `twice { print("hi") }`. Named params use a `name, name ->` header; with no
  header, the block gets an implicit `it` parameter only if its body reads a
  free `it` (a block with its own `it` — a parameter or `val`/`var it` —
  doesn't; nested blocks each bind their own `it`; a `for it in ...` loop
  variable only exists inside that loop's body); a block that mentions no
  `it` takes zero params. `return` inside returns from the block. Not
  allowed in `if`/`while` conditions or a `for ... in` collection —
  parenthesize the call instead.
- Struct fields are listed one per line, no commas or types:
  `struct Point {` / `x` / `y` / `}`. Construct with `Point(1, 2)`.
- Methods live in `impl Point { fn len(self) { ... } }`; `self` is an
  explicit first parameter. A method without `self` is static (`Point.origin()`).
  No classes, inheritance, `this`, or `new`.
  An `impl` must be in the struct's own module (`impl utils.Point` is an error);
  `impl Array` and other builtin impls are program-wide for modules compiled after.
- Enum variants are declared like struct fields, conventionally one per line,
  no commas: `enum Color {` / `Red` / `Green` / `}`. Top level only. Access is
  always qualified (`Color.Red`); a bare `Color` is a compile error.
  `Color.values()` returns a fresh array of every variant in declaration
  order (a compile error if any variant has a payload). A variant can take named
  payload fields: `Rect(w, h)`; construct with `Shape.Rect(1, 2)` (a bare
  `Shape.Rect` is a constructor value), read with `s.w` (immutable). Prints
  `Shape.Rect(1, 2)`; equality is structural. No `impl` blocks or explicit
  variant values.
- `import` and `export` are keywords. `import "path"` and `import "path" as name`
  are top level only. The path resolves relative to the importing file (the current
  directory in the REPL) with `.n` appended, and a missing file or an
  import cycle is a compile error. A file name that isn't an identifier (`my-utils`) needs `as`.
  Exports are reached as `utils.name` (or the `as` alias) and are
  read-only from outside the module. An unknown export, a module used as a value (`print(utils)`) and a
  wrong-arity call of an exported function are compile errors. Each module runs once, before its
  importers. `import "std/math"` (also `std/file`, `std/stdin`, `std/pq`) binds a builtin module;
  any other `std/` path is a compile error.
  `export` goes before a top-level `fn`, `val`, `var`, `struct` or `enum` (one
  plain name for `val`/`var`; not `impl`).

**Control flow**
- Conditions are paren-free: `if x {`, `while x {`. Parentheses around a
  condition are just grouping, not required.
- Every branch and loop body needs braces: `if c print(1)` is a compile
  error; write `if c { print(1) } else { print(2) }`.
- `if`/`else` is also an expression: its value is the branch's last
  expression statement (`nil` if that branch's last statement isn't an
  expression). `else` is required in expression position — omitting it is a
  compile error (`if expression requires else`). The ternary (`c ? a : b`)
  stays the shorter form for one-liners.
- A branch ending in an `if`/`else` *statement* also yields `nil` — the
  parser can't tell it apart from a nested if-expression. Bind it to a
  `val` first (`val sign = if n > 100 { "big" } else { "small" }`) and use
  that as the branch's last statement instead.
- `for x in coll { }` is the only `for`; no C-style `for`. Count with a range
  instead: `for i in 0..n { }`. For-in over a map gives keys, or
  `[key, value]` pairs with `for (k, v) in m { }`. `val`/`var`/`for` also
  destructure a tuple pattern (`val (a, b) = [1, 2]`), `_` skipping a
  position; a non-array or wrong-size value is a runtime error (`Cannot
  destructure number into 2 names`). `break`/`continue` exist.
- No `switch`, `do`/`while`, `try`/`catch`, or `throw`.
- `match x { pattern, pattern -> body ... }` is an expression and a statement.
  A pattern is a number/string/bool/`nil` literal, `Enum.Variant`, an integer
  range (`1..5`, `1..=5`), `_`, or a bare name that binds the value
  (immutable, arm-scoped, always shadows; comma alternatives must bind the
  same names); an array pattern `[a, 1, _]` matches an array of exactly that
  length (`..` matches any number of elements, `..rest` binds a copy; never
  counts for coverage); a variant pattern `Shape.Rect(w, 0)` matches that
  variant and its fields by position against any sub-pattern (count must
  equal the field count; write a payload variant with parentheses, a unit
  variant without; no `..` inside); unguarded with only bindings/`_` it
  covers its variant, a refutable sub-pattern covers nothing; anything else
  (a call) is a compile error. A guard,
  `n if n > 0 -> body`, runs after the pattern, sees bindings, and falls
  through when false. A body is an expression or a `{ }` block. A match on an enum
  needs every variant or a `_`/unguarded binding arm; guarded arms never count; a repeated pattern or one after `_` is
  `unreachable pattern`. No matching arm is the runtime error
  `No match arm for <value>`.

**Operators**
- Compound assignment (`+=`, `-=`, `*=`, `/=`, `%=`, `**=`) works on variables
  (local, global, captured), fields (`o.n += 1`, `self.n += 1`), and indexes
  (`a[0] += 1`, `m["k"] += 1`), evaluating the target's object/index once. It's
  an expression whose value is the new value. No bitwise compound operators.
  There is no `++`/`--`; use `+= 1`/`-= 1`.
- `/` is always float division (`7 / 2` is `3.5`), even on two ints. Integer floor division is
  `math.div(a, b)` (`std/math`), not `math.floor(a / b)` — that round-trips through `f64` and loses precision
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
- Map keys (and set elements) can be strings, numbers, booleans, enum variants (payload fields must be valid keys too), or arrays. An
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

This is the full list. Anything not here, like `keys` on
arrays, `toFixed`, or `String(x)`, doesn't exist. Add a
helper with `impl Array { fn name(self) { ... } }` if you need one.
The `std/` entries are builtin modules: `import "std/math"` binds `math`,
then call `math.abs(x)`. Exports are values: `val abs = math.abs`, `xs.map(math.abs)`.

- **Global:** `print(a, b, ...)`, `sleep(ms)`, `args` (array of script arguments, strings)
- **String:** `size`, `isEmpty`, `substring(start, end)`, `replace(old, new)`,
  `split()` (on Unicode whitespace) / `split(sep)`, `trim`, `startsWith`, `endsWith`, `indexOf`,
  `lastIndexOf`, `contains`, `charCodeAt(index)`, `String.fromCharCode(n)`,
  `repeat(n)`, `padStart(len, fill)`, `padEnd(len, fill)`, `chars()`,
  `toUpperCase`, `toLowerCase`, `toInt`, `toFloat`, `toBool`
- **Number:** `toString`, `toInt`, `toFloat`
- **Boolean:** `toString`
- **Array:** `Array(n, init)`, `push`, `pop`, `size`, `isEmpty`, `contains`, `sort()` / `sort(cmp)`,
  `reverse`, `slice(start, end)`, `join(sep)`, `indexOf`, `sum`, `min`, `max`,
  `map(fn)`, `filter(fn)`, `reduce(initial, fn)`, `forEach(fn)`, `flatMap(fn)`,
  `find(fn)`, `some(fn)`, `every(fn)`, `flat()`, `copy()`, `take(n)`, `drop(n)`,
  `first()`, `last()`, `chunked(n)`, `zip(other)`, `withIndex()`, `sortBy(fn)`,
  `minBy(fn)`, `maxBy(fn)`, `groupBy(fn)`, `tally()`, `takeWhile(fn)`, `dropWhile(fn)`,
  `partition(fn)`, `distinct()`, `scan(initial, fn)`, `windowed(n)`
- **Range:** `size`, `isEmpty`, `contains`, `toArray`, `step(k)`, `slice`, `join`,
  `indexOf`, `sum`, `min`, `max`, `map`, `filter`, `reduce`, `forEach`, `flatMap`,
  `take`, `drop`, `first`, `last`, `chunked`, `zip`, `withIndex`, `sortBy`, `minBy`, `maxBy`,
  `groupBy`, `tally`, `takeWhile`, `dropWhile`, `partition`, `distinct`, `scan`, `windowed`
- **Map:** `get`, `contains`, `remove`, `size`, `isEmpty`, `keys`, `values`, `entries`,
  `forEach(fn)`, `map(fn)`, `filter(fn)`, `mapValues(fn)`, `some(fn)`, `every(fn)`
- **Set:** `add`, `remove`, `contains`, `size`, `isEmpty`, `clear`, `union`,
  `intersection`, `difference`, `isSubset`, `toArray`
- **File:** `read`, `readLines`, `write(text)` (creates the file; errors if it exists)
- **PriorityQueue:** `push(priority, value)`, `pop`, `peek`, `size`, `isEmpty` -
  min-heap where priority must be a number; equal priorities pop in insertion order
- **std/math:** `math.abs`, `math.floor`, `math.ceil`, `math.sqrt`, `math.min(...)`, `math.max(...)`,
  `math.div(a, b)`, `math.round`, `math.sign`, `math.gcd(a, b)`, `math.lcm(a, b)`, `math.mod(a, b)`
- **std/file:** `file.open(path)` - returns a File
- **std/stdin:** `stdin.read()`, `stdin.readLines()` - read to EOF; `read()` after EOF returns `""`
- **std/pq:** `pq.new()` - returns a PriorityQueue

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
