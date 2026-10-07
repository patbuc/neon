[![CI](https://img.shields.io/github/actions/workflow/status/patbuc/neon/rust.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/patbuc/neon/actions/workflows/rust.yml)
[![Lint](https://img.shields.io/github/actions/workflow/status/patbuc/neon/lint.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=lint)](https://github.com/patbuc/neon/actions/workflows/lint.yml)
[![Bench](https://img.shields.io/github/actions/workflow/status/patbuc/neon/bench.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=bench)](https://github.com/patbuc/neon/actions/workflows/bench.yml)
[![License](https://img.shields.io/github/license/patbuc/neon?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange?style=flat-square&logo=rust&logoColor=white)](https://doc.rust-lang.org/edition-guide/rust-2021/index.html)
[![Last commit](https://img.shields.io/github/last-commit/patbuc/neon?style=flat-square)](https://github.com/patbuc/neon/commits)
[![Code size](https://img.shields.io/github/languages/code-size/patbuc/neon?style=flat-square)](https://github.com/patbuc/neon)

# ✨ Neon

A toy language you didn't wait for

## Why

Let's be honest — the world doesn't need another programming language.
I'm building neon for one simple reason: to teach myself a few things.

- How to build a compiler
- How to build a virtual machine
- How to use Rust for this

## Getting Started

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))

### Building

```bash
cargo build --release
```

### Running Neon Scripts

```bash
# Using cargo
cargo run -- script.n

# Or use the compiled binary
./target/release/neon script.n

# Compile without running, to check for errors
cargo run -- --check script.n
```

### REPL

```bash
cargo run
```

Run with no arguments to start an interactive REPL. It reads one physical line per entry, so a
`struct`, `enum`, or `impl` body must fit on that line (e.g. `struct P { x }` or
`impl P { fn f(self) { return 1 } }`) — it can't be split across multiple lines the way a file can.
Multiple statements can still share a line wherever the syntax allows it. It keeps state between
entries:

- A `val`, `var`, `fn`, `struct`, `enum`, or `impl` from an earlier line is visible to later
  ones.
- A line that fails to compile leaves earlier definitions untouched and defines nothing of
  its own.
- A line that fails at runtime keeps any assignments it made to earlier globals, but drops
  the names it tried to introduce; re-entering the line works.
- A later line may redeclare an earlier `val`, `var`, or `fn` — a function defined between
  the two still sees the value it closed over. Structs and enums can't be redeclared, and
  an existing method can't be redefined, though `impl` blocks accumulate.
- Methods persist across lines. A line that fails at runtime adds none of the
  methods it defined.
- Type `exit` or press Ctrl+D (EOF) to quit.

### Hello World

Create a file `hello.n`:

```neon
print("Hello, Neon!")
```

Run it:

```bash
cargo run -- hello.n
```

### Editor Support

VS Code syntax highlighting for `.n` files ships as a VSIX on the
[GitHub Releases page](https://github.com/patbuc/neon/releases) — install it with
`code --install-extension neon-X.Y.Z.vsix`. See
[editors/vscode/README.md](editors/vscode/README.md) for details.

## Benchmarks

`benches/` holds paired `<name>.n` / `<name>.py` benchmarks (fib, closures, structs, collections, sieve, and more)
that implement the same algorithm in Neon and in plain Python, so the two can be timed head to head.

```bash
cargo build --release
python3 benches/run.py               # all benchmarks
python3 benches/run.py fib strings   # run a subset by name
python3 benches/run.py --runs 10     # timed runs per benchmark (default 5)
```

This prints a table with mean, stddev, min, and median per language plus the Neon/Python ratio, and fails if the two
implementations disagree on the result. Charts tracking these benchmarks over time are published at
[patbuc.github.io/neon/dev/bench](https://patbuc.github.io/neon/dev/bench/), updated on every push to `main`.

To add a benchmark, write `benches/<name>.n` and `benches/<name>.py` doing the same work, each reading the problem
size from the first argument with a small default and printing one integer checksum, then register a driver size for
it in `BENCHMARKS` in `benches/run.py`. The `.n` file's `// Expected:` checksum, at its default size, runs as part of
`cargo test`.

## State

Neon is a functional dynamically-typed interpreter with a comprehensive feature set. It's an active learning project and includes:

- Complete lexer, parser, and bytecode compiler
- Stack-based virtual machine
- Rich standard library with collection types and methods
- 200+ integration tests validating all features
- String interpolation and first-class functions

While Neon is functional for many programs, it remains experimental. Expect rough edges, missing features, and occasional crashes as development continues.

## Language Features

### Data Types

**Primitives:**
- **Numbers** - An integer literal (decimal, or `0xFF` hex, `0b101` binary, `0o17` octal) with no `.` and
  no exponent is a 64-bit int; a literal with a `.` or exponent (`3.14`, `1.5e-3`, `2E+2`) is a 64-bit
  float. An int literal that doesn't fit in `i64` is a compile error.
- **Booleans** - `true` and `false`
- **Strings** - Unicode text with escapes (e.g., `"hello"`, `"world\n"`)
- **Nil** - Null value represented as `nil`

**Collections:**
- **Arrays** - Ordered, mutable, indexed collections (e.g., `[1, 2, 3]`)
- **Maps** - Key-value dictionaries (e.g., `{"name": "Alice", "age": 30}`). A key can be a string,
  number, boolean, enum variant, or array.
- **Sets** - Unique value collections (e.g., `#{1, 2, 3}`)

**Other Types:**
- **Ranges** - Immutable, lazy sequences of integers: exclusive `1..10` or inclusive `1..=10`. Prints as `1..4` / `1..=4`, compares equal by bounds, and indexes like an array (including negative indices) without materializing its elements. Not usable as a map key.
- **Functions** - First-class values
- **Structs** - User-defined data structures

### Variables

```neon
var x = 10        // Mutable variable
val name = "Bob"  // Immutable variable
```

`val`/`var` also destructure a tuple pattern - an Array of exactly the right
length - binding each element to a name; `_` skips a position without
declaring anything. Any other value is a runtime error (`Cannot destructure
number into 2 names`, `Cannot destructure array of size 1 into 2 names`):

```neon
val (a, b) = [1, 2]
print(a, b)  // 1 2

var (x, _, z) = [1, 2, 3]
```

### Statements and Newlines

A newline ends a statement, so two statements can't share a line without one:

```neon
val x = 1 val y = 2
// error[E0005]: expecting '\n' or '\0' after value declaration.
```

An expression continues onto the next line only when the line ends with a
binary operator. A line that instead begins with an operator starts a new
statement, so `+ 2` is a syntax error (`+` isn't a valid statement start)
while `- 2` compiles as its own expression statement (unary negation):

```neon
val x = 1 +
    2
print(x)  // 3
```

A line that begins with `.` or `?.` (but not `..`/`..=`) is the exception: it
continues the previous statement as a method chain. A blank line or a
comment line in between still ends the statement.

```neon
val r = [1, 2, 3, 4]
    .slice(0, 2)
    .join(",")
print(r)  // 1,2
```

### Functions

```neon
fn add(a, b) {
    a + b
}

fn greet(name) {
    print("Hello, ${name}!")
}

fn fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

A function's last expression statement is its return value — no `return`
needed. Any other kind of last statement (like `greet`'s `print`, above)
returns `nil`, and so does a bare `return`. When the whole body is one
expression, a named function or method can skip the braces with `= expr`:

```neon
fn sq(x) = x * x
print(sq(3))  // 9
```

Lambdas can't use `= expr`, but their last expression is still their return
value:

```neon
val inc = fn(x) { x + 1 }
print(inc(4))  // 5
```

Parameters are immutable inside the function body: assigning to one is a
compile error, naming the parameter.

```neon
fn f(x) {
    x = x + 1  // error[E0015]: cannot assign to immutable variable 'x'
    return x
}
```

Functions are first-class values: they can be stored in variables, passed as
arguments, and returned from other functions.

```neon
fn double(x) = x * 2

fn apply(f, value) {
    return f(value)
}

val g = double
print(apply(g, 3))  // 6
```

Functions can be nested, and a nested function closes over the variables of
the functions enclosing it: each call to `counter()` gets its own `n`, and
`inc` keeps reading and writing that same `n` even after `counter()` has
returned.

```neon
fn counter() {
    var n = 0
    fn inc() {
        n = n + 1
        return n
    }
    return inc
}

val c = counter()
print(c())  // 1
print(c())  // 2

val other = counter()
print(other())  // 1, independent of c
```

Anonymous functions (`fn(params) { body }`) can be written directly where a
value is expected, and close over their surroundings just like a named
function.

```neon
fn apply(f, value) {
    return f(value)
}

val double = fn(x) {
    return x * 2
}
print(apply(double, 3))               // 6
print(apply(fn(x) { return x + 1 }, 3))  // 4
```

A statement starting with `fn(` is a lambda expression, not a named function
declaration, so it can be called immediately:

```neon
fn(x) {
    print(x)
}(5)  // 5
```

#### Tail Calls

A call in tail position reuses the caller's frame, so tail recursion isn't limited by the
call-depth limit (1,000,000 frames). Tail position is the operand of `return` and a function's
last expression (including `fn f() = expr`). When an `if`/`else` expression, a `match` or a
ternary is in tail position, so are its branches and arms. Parentheses don't change tail
position. Method calls count too, including a closure stored in a field. Operands of `&&`, `||` and `??` and script-level code are not tail
positions. A tail call to a native function, a struct constructor or a native method behaves
like a call followed by a return.

```neon
fn count(n, limit) = n >= limit ? n : count(n + 1, limit)

print(count(0, 2000000))  // 2000000
```

A frame replaced by a tail call is gone, so the stack trace of a runtime error lists only the
frames still live.

#### Trailing Blocks

When a function literal is a call's last (or only) argument, it can be
written as a block right after the call instead of `fn(...) { ... }`. Any
other arguments stay inside the parentheses, and the block takes the
function's place as the last one; a call with no other arguments can drop
the parentheses entirely:

```neon
print([1, 2, 3].map { it * 2 })                   // [2, 4, 6]
print([1, 2, 3].reduce(0) { acc, x -> acc + x })  // 6
twice { print("hi") }
```

Named parameters go in a `name, name ->` header before the body, just like a
lambda's parameter list:

```neon
print(["a", "bb", "ccc"].sortBy { s -> s.size() })
```

A block with no `->` header takes an implicit parameter named `it`, but only
when its body actually reads or assigns a free `it`; a block that never
mentions `it` takes zero parameters instead, same as `fn() { ... }`:

```neon
print([1, 2, 3].map { it * 2 })   // it is the element
twice { print("hi") }             // no parameters at all
```

A block that declares its own `it` — as a parameter or a `val it`/`var it` —
doesn't get the implicit one, and nested blocks each bind their own `it`
independently of any outer one. A `for it in ...` loop variable only exists
inside that loop's body; the block's `it` is unaffected outside it:

```neon
print([[1, 2], [3]].map { it.map { it * 10 } })   // inner it shadows the outer one
```

`return` inside a trailing block returns from the block itself, same as any
other lambda, not from the function the block was passed into.

Trailing blocks aren't allowed in an `if`/`while` condition or a `for ... in`
collection — Neon can't tell the block apart from the statement body that
follows. Parenthesize the call instead:

```neon
if ([1, 2, 3].filter { it > 1 }.isEmpty()) {
    print("empty")
}
```

#### Hoisting

Every top-level `fn`, `struct`, `val`, and `var` is visible throughout the
file. Functions and structs are ready before any statement runs, so a
top-level function can be called above its own declaration, and its body can
name any top-level declaration, including ones that come later:

```neon
print(f())  // 1
fn f() { return 1 }
```

```neon
fn f() { return y }
val y = 2
print(f())  // 2
```

`val`/`var` are initialized when their statement runs, in order. Reading one
from inside a function that gets called too early is a runtime error, naming
the variable and the reading line:

```neon
fn f() { return y }
print(f())
val y = 2
// [1:17] variable 'y' used before initialization
```

At the top level itself, naming a `val`/`var` before its declaration is a
compile error instead:

```neon
print(b)
val b = 1
// error[E0017]: cannot use 'b' before its declaration
```

```neon
val x = x
// error[E0016]: cannot read 'x' in its own initializer
```

The same hoisting applies inside a block or function body: a block's own
`fn` declarations are visible throughout that block, so nested functions can
call each other in any order. Calling one before its `fn` line has run is
the same use-before-initialization error, naming the function:

```neon
fn outer() {
    fn a(n) { return b(n) }
    fn b(n) { return n }
    return a(1)
}
print(outer())  // 1
```

`val`/`var` declared in a block stay ordered: a nested function can't name a
block variable declared after it.

### Control Flow

**If/Else:**

```neon
if x > 10 {
    print("Greater than 10")
} else if x > 5 {
    print("Greater than 5")
} else {
    print("5 or less")
}
```

`if`/`else` is also an expression: its value is the branch's last expression
statement, or `nil` if the branch's last statement isn't an expression. In
expression position `else` is required — a missing one is a compile error
(`if expression requires else`):

```neon
val n = 5
val label = if n > 0 {
    "pos"
} else if n < 0 {
    "neg"
} else {
    "zero"
}
print(label)  // pos
```

For a one-liner, the ternary (`c ? a : b`) is still the shorter choice.

A branch ending in an `if`/`else` *statement* yields `nil`, the same as any
other non-expression last statement — the parser can't tell it apart from a
nested if-expression. Bind it to a `val` first and use that instead:

```neon
val n = 5
val label = if n > 0 {
    val sign = if n > 100 { "big" } else { "small" }
    sign
} else {
    "non-positive"
}
print(label)  // small
```

**While Loops:**

```neon
var i = 0
while i < 5 {
    print(i)
    i = i + 1
}
```

**For Loops:**

```neon
// For-in loop over a range
for i in 0..10 {
    print(i)
}

// For-in loop over arrays
for item in [1, 2, 3, 4, 5] {
    print(item)
}

// For-in loop over ranges
for i in 1..=10 {
    print(i)
}

// For-in loop over map keys
val person = {"name": "Alice", "age": 30}
for key in person {
    print("${key}: ${person[key]}")
}

// For-in loop over map entries, destructuring each [key, value] pair
for (key, value) in person {
    print("${key}: ${value}")
}

// For-in loop over set
val numbers = #{1, 2, 3}
for num in numbers {
    print(num)
}
```

`for-in` also destructures a tuple pattern, the same as `val`/`var`: each
element (or, over a Map, each `[key, value]` pair) is bound by position, and
`_` skips a position without declaring anything.

`for-in` gives every iteration its own binding of the loop variable (or
variables), so a closure created in the body keeps that iteration's value.

**Match:**

```neon
enum Color {
    Red
    Green
    Blue
}

fn status(code) {
    return match code {
        200 -> "ok"
        301, 302 -> "moved"
        400..500 -> "client error"
        _ -> "other"
    }
}

print(status(302))  // moved
print(status(404))  // client error
print(status("x"))  // other

val name = match Color.Green {
    Color.Red -> "red"
    Color.Green -> {
        print("checking green")
        "green"
    }
    Color.Blue -> "blue"
}
print(name)  // green

enum Shape {
    Circle(radius)
    Rect(w, h)
}

fn area(s) {
    return match s {
        Shape.Circle(r) -> r * r * 3
        Shape.Rect(w, 0) -> 0
        Shape.Rect(w, h) -> w * h
    }
}
print(area(Shape.Rect(2, 5)))  // 10

val size = match 42 {
    n if n < 10 -> "small"
    n -> "big " + n.toString()
}
print(size)  // big 42
```

`match` is an expression, and also a statement. It tries the arms top to
bottom and runs the first one with a matching pattern; its value is the
arm's value.

- A pattern is a number, string, `true`/`false` or `nil` literal (a leading
  `-` is fine), an `Enum.Variant` or `Enum.Variant(patterns)`, an integer range (`400..500`,
  `1..=5`), `_`, which matches anything, or a bare name, which matches
  anything and binds it (see below). Anything else, such as a call, is a
  compile error (`Invalid match pattern`).
- An arm lists one or more patterns separated by commas and runs if any of
  them matches. A range pattern only matches a number; any other value falls
  through to the next arm.
- A bare name binds the matched value to a new immutable name, visible in
  the arm's guard and body only. It always shadows an outer name of the
  same name. Unguarded, it counts like `_` for coverage. Comma alternatives
  must bind the same names.
- A guard, `pattern if condition -> body`, runs after the pattern matches
  and sees its bindings. If it is false the match falls through to the next
  arm. A guarded arm never counts toward coverage.
- An array pattern, `[1, x, _]`, matches an array of exactly that length;
  its elements are literal, `_`, binding, range, nested array or enum
  variant patterns. One `..` anywhere matches any number of elements, and
  `..rest` binds them to a new array (a copy). A non-array never matches.
  Array patterns never count toward coverage.
- A variant pattern, `Shape.Rect(w, 0)`, matches a payload variant and its
  fields by position against any sub-pattern (literal, `_`, binding, range,
  array or nested variant). The count must equal the variant's field count.
  A payload variant is written with parentheses and a unit variant without,
  and `..` is not allowed inside. Unguarded, a variant pattern whose
  sub-patterns are all bindings or `_` covers its variant; one with a
  refutable sub-pattern covers nothing.
- An arm's body is an expression, or a `{ ... }` block whose value is its
  last expression statement.
- A match whose patterns include an enum variant is an enum match: every
  other pattern must belong to the same enum, and without a `_` arm every
  variant must be covered.
- A pattern that repeats an earlier one, or comes after `_`, is a compile
  error (`unreachable pattern`).
- If no arm matches, the program stops with a runtime error
  (`No match arm for <value>`).

### Operators

**Arithmetic:**
- `+` Addition (also string concatenation)
- `-` Subtraction
- `*` Multiplication
- `/` Division (always float, even for two ints)
- `%` Modulo
- `**` Exponentiation
- `-x` Negation (unary)

`+ - * %` on two ints give an int; `**` gives an int for a non-negative int exponent, a float for a
negative one. Any float operand makes the result a float. An int result outside `i64` raises a runtime
error (`integer overflow in <op>`); `%` by zero raises `modulo by zero`. Use `Math.div(a, b)` for integer
floor division.

**Comparison:**
- `==` Equal
- `!=` Not equal
- `<` Less than
- `<=` Less than or equal
- `>` Greater than
- `>=` Greater than or equal

Ints and floats compare by exact numeric value (`1 == 1.0` is `true`), and an int is the same map key or
set element as an equal integral float.

**Logical:**
- `&&` Logical AND (short-circuit)
- `||` Logical OR (short-circuit)
- `!` Logical NOT (unary)

**Bitwise:**
- `&` AND, `|` OR, `^` XOR
- `~` NOT (unary)
- `<<` Left shift, `>>` Right shift

Bitwise operators always give an int; a float operand is truncated first.

**Other:**
- `..` Range (exclusive)
- `..=` Range (inclusive)
- `c ? a : b` Ternary
- `a ?? b` Nil coalescing: `a` unless `a` is nil, otherwise `b`. Only `nil` triggers the fallback —
  `false ?? x` is `false`.
- `a?.f`, `a?.m(args)` Optional chaining: nil if `a` is nil (without evaluating the method's
  arguments), otherwise the field or method call. Not assignable.
- `x += e`, `-=`, `*=`, `/=`, `%=`, `**=` Compound assignment

Compound assignment works on variables, fields (`o.n += 1`, `self.n += 1`, `o.inner.n += 1`), and
indexes (`a[i] *= 2`, `m["k"] += 1`), evaluating the target's object/index once. A compound
assignment is an expression whose value is the new value. There is no `++`/`--`; use `+= 1`/`-= 1`.

**Operator Precedence:** `||` has lower precedence than `&&`, so `a || b && c` is evaluated as `a || (b && c)`.
`??` binds looser than `||` but tighter than the ternary, so `a ?? b || c` is `a ?? (b || c)`, and
`c ? a ?? b : d` is `c ? (a ?? b) : d`.

### String Interpolation

```neon
val name = "Alice"
val age = 30
print("Name: ${name}, Age: ${age}")

val x = 5
val y = 10
print("${x} + ${y} = ${x + y}")  // "5 + 10 = 15"
```

The expression inside `${...}` may itself contain strings and braces, e.g. `"${m["k"]}"`.

### Escape Sequences

String literals support `\n`, `\t`, `\r`, `\\`, `\"`, `\$` and `\u{XXXX}` (1 to 6 hex digits).
`\$` keeps `${` from starting an interpolation. Any other escape is a compile error.

```neon
print("say \"hi\"")     // say "hi"
print("a\nb")           // prints two lines
print("\u{1F600}")      // 😀
print("\${name}")       // ${name}
```

### Structs

```neon
struct Point {
    x
    y
}

val pt = Point(10, 20)
print(pt.x)  // 10

pt.x = 15    // Fields are mutable
print(pt.x)  // 15
```

A struct can't be named after a builtin type (`Array`, `String`, `Map`,
`Set`, `Number`, `Boolean`, `File`).

#### Methods

```neon
impl Point {
    fn len(self) {
        return Math.sqrt(self.x * self.x + self.y * self.y)
    }

    fn origin() {
        return Point(0, 0)
    }
}

print(Point(3, 4).len())  // 5
print(Point.origin().x)   // 0
```

- `self` is the first parameter; assigning a field through `self` is visible
  to the caller. A method without `self` is static and is called on the type
  itself, e.g. `Point.origin()`. Calling a method the wrong way - a static
  method on a value, or an instance method on the type - is an error.
- Methods may be spread over several `impl` blocks for the same struct. An
  `impl` block must appear at the top level, and its type must be a declared
  struct or a builtin type. A method can't share a name with a field or with
  another method of the same struct.
- Methods belong to the struct they are declared for. Two modules' structs
  with the same name keep separate methods, and an exported struct carries its
  methods to the modules that import it. An `impl` block must be in the
  struct's own module: `impl utils.Point` is a compile error.
- Methods are registered before the program runs, so they can be called from
  code that appears before their `impl` block. A method body can see
  functions, structs, builtins, and top-level variables.

A builtin type (`Array`, `String`, `Map`, `Set`, `Number`, `Boolean`, `File`)
can have an `impl` block too, adding an instance method callable on any value
of that type. A method can't share a name with a native method of the type -
that's a compile error, since a native method can never be redefined. Unlike
a struct, a builtin type only supports instance methods; every method must
take `self`.

An `impl` on a builtin type is program-wide: every module compiled after the
one declaring it sees the method, in dependency order.

```neon
impl Array {
    fn second(self) {
        return self[1]
    }
}

print([1, 2, 3].second())  // 2
```

#### Calling function-valued fields

A field holding a function can be called like a method, `s.f(x)`, but it does
not receive the instance - it's called with just the arguments given.

```neon
struct Adder {
    add
}

val a = Adder(fn(x, y) { return x + y })
print(a.add(2, 3))  // 5
```

### Enums

```neon
enum Color {
    Red
    Green
    Blue
}

print(Color.Red)              // Color.Red
print(Color.Red == Color.Red) // true
print(Color.Red == Color.Green) // false

for c in Color.values() {
    print(c)
}

enum Shape {
    Circle(radius)
    Rect(w, h)
    Square
}

val r = Shape.Rect(1, 2)
print(r)                      // Shape.Rect(1, 2)
print(r.w)                    // 1
print(Shape.Circle(2) == Shape.Circle(2)) // true
```

- An enum must be declared at the top level. Access to a variant is always
  qualified (`Color.Red`); a bare `Color` is a compile error, as is an unknown
  variant (`Color.Purple`).
- Two variants are equal when they're the same variant of the same enum. A
  variant never equals a number, a string, or a same-named variant of a
  different enum.
- `Color.values()` returns a fresh array of every variant, in declaration
  order, each time it's called.
- A variant can carry named payload fields: `Circle(radius)`, `Rect(w, h)`.
  `Shape.Circle(2)` constructs one positionally, and `s.radius` reads a field
  (fields are immutable). A bare `Shape.Circle` is a constructor value you can
  pass around, e.g. `[1, 2].map(Shape.Circle)`. A payload variant prints as
  `Shape.Rect(1, 2)` and is equal to another of the same variant with equal
  fields. `Shape.values()` on an enum with payload variants is a compile error.
  `match` destructures them with `Shape.Rect(w, h)` patterns (see Match).
- Enums don't support `impl` blocks or explicit variant values.

### Modules

A file can export names and another file can import them.

```neon
// lib/utils.n
export val VERSION = 1
export var counter = 0

export fn double(x) {
    x * 2
}

export fn bump() {
    counter += 1
}

export struct Point {
    x
    y
}
```

```neon
// lib/my-math.n
export fn square(x) = x * x
```

```neon
// main.n
import "lib/utils"
import "lib/my-math" as math

print(utils.double(21))   // 42
utils.bump()
print(utils.counter)      // 1
print(utils.Point(1, 2).x) // 1
print(math.square(5))     // 25
```

- `export` goes before a top-level `fn`, `val`, `var`, `struct` or `enum`, and
  exports one name. Everything else stays private to the file.
- `import "path"` and `import "path" as name` are top-level statements. The
  module is bound to its file name (`lib/utils` becomes `utils`) or to the
  `as` name. A file name that isn't an identifier, like `my-math`, needs `as`.
- A path is relative to the importing file, and may start with `./`, `../` or
  `/`. `.n` is appended unless the path already ends in `.n`. In the REPL,
  paths resolve from the current directory. A `std/` path imports one of the
  builtin modules (`std/math`, `std/file`, `std/stdin`, `std/pq`; see
  [Standard Library](#standard-library)); any other `std/` path is a compile error.
- Exports are reached as `utils.name`. They are read-only from outside:
  `utils.counter = 5` and `utils.counter += 1` are compile errors. They are
  live, so `utils.counter` sees the updates the module makes.
- A module is not a value: `val m = utils` is a compile error. So is an unknown
  export (`utils.nope`), or a call with the wrong number of arguments.
- Each module runs once, before the files that import it, in dependency order.
  Import cycles are compile errors.
- The browser build can't import files.

[`examples/modules/`](examples/modules/) is a small multi-file program that
exports a struct, an enum and a variable, and imports a module under an alias.
Run it with `cargo run -- examples/modules/main.n`.

## Code Examples

### Fibonacci

```neon
fn fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

for i in 0..10 {
    print(fibonacci(i))
}
```

### Working with Arrays

```neon
val numbers = [5, 2, 8, 1, 9]

// Add elements
numbers.push(3)

// Access by index
print(numbers[0])

// Check if contains value
if numbers.contains(5) {
    print("Found 5!")
}

// Iterate
for num in numbers {
    print(num)
}

// Get length
print("Length: ${numbers.size()}")
```

### Working with Maps

```neon
val person = {
    "name": "Alice",
    "age": 30,
    "city": "New York"
}

// Access values
print(person["name"])

// Add new entries
person["email"] = "alice@example.com"

// Iterate over keys
for key in person {
    print("${key}: ${person[key]}")
}

// Iterate over entries, destructuring each [key, value] pair
for (key, value) in person {
    print("${key}: ${value}")
}

// Check if key exists
if person.contains("age") {
    print("Age: ${person["age"]}")
}

// Get all keys, values, or entries
val allKeys = person.keys()
val allValues = person.values()
val allEntries = person.entries()
```

### Working with Sets

```neon
// Create set with literal syntax
val numbers = #{1, 2, 3}
numbers.add(2)  // Duplicate ignored

print(numbers.size())  // 3

// Check membership
if numbers.contains(1) {
    print("Contains 1")
}

// #{} is an empty set; {} is an empty map
val empty = #{}
print(empty)  // #{}

// Set operations
val setA = #{1, 2, 3}
val setB = #{2, 3, 4}

val unionSet = setA.union(setB)         // #{1, 2, 3, 4}
val intersect = setA.intersection(setB) // #{2, 3}
val diff = setA.difference(setB)        // #{1}

// Check subset
if setA.isSubset(setB) {
    print("A is subset of B")
}

// Convert to array for iteration control
val asArray = numbers.toArray()
for i in 0..asArray.size() {
    print(asArray[i])
}
```

### String Operations

```neon
val text = "Hello World"

// String methods
print(text.size())                     // 11
print(text.toUpperCase())              // "HELLO WORLD"
print(text.toLowerCase())              // "hello world"
print(text.substring(0, 5))            // "Hello"
print(text.replace("World", "Neon"))   // "Hello Neon"

// Split into array
val words = "one,two,three".split(",")  // ["one", "two", "three"]
for word in words {
    print(word)
}

// Type conversions
val num = "42".toInt()
val pi = "3.14".toFloat()
val flag = "true".toBool()
```

## Standard Library

`Math`, `File`, `String`, `Array`, `Stdin` and `PriorityQueue` are namespaces: their static methods
(`Math.abs(n)`, `File(path)`, `String.fromCharCode(n)`, `Array(n, init)`, `Stdin.read()`,
`PriorityQueue()`) are only callable through the namespace name, and
redefining that name at top level (`val String = ...`, `fn Math() {}`) is a compile error — a local of
the same name inside a function still shadows it, same as any other name.

### Global Functions

- `print(value, ...)` - Output values to stdout (variadic)
- `sleep(ms)` - Block the current thread for `ms` milliseconds

The builtin modules are imported like any other module and bound to their last path segment:
`import "std/math"` binds `math`, and `import "std/math" as m` binds `m`.

### std/math Methods

- `math.abs(n)` - Absolute value (keeps ints as ints)
- `math.floor(n)` - Round down, returns an int (runtime error if the result doesn't fit in `i64` or is NaN)
- `math.ceil(n)` - Round up, returns an int (same error cases as `floor`)
- `math.sqrt(n)` - Square root
- `math.min(a, b, ...)` - Minimum value (variadic, keeps ints as ints)
- `math.max(a, b, ...)` - Maximum value (variadic, keeps ints as ints)
- `math.div(a, b)` - Floor division on two ints, returns an int; errors on a float argument, division by
  zero, or overflow
- `math.round(n)` - Round half away from zero, returns an int (same error cases as `floor`)
- `math.sign(n)` - `-1`, `0` or `1` as an int (`-0.0` is `0`; `NaN` is a runtime error)
- `math.gcd(a, b)` - Greatest common divisor of two ints, always non-negative
- `math.lcm(a, b)` - Least common multiple of two ints, always non-negative
- `math.mod(a, b)` - Euclidean modulo in `[0, |b|)`: an int for two ints, a float otherwise

**Example:**
```neon
import "std/math"

print(math.abs(-5))        // 5
print(math.max(3, 7, 2))   // 7
print(math.div(7, 2))      // 3, integer division
```

### std/file Methods

- `file.open(path)` - Open a file handle for `path`; returns a `File` (see [File](#file) for its methods)

### std/stdin Methods

- `stdin.read()` - All remaining standard input as a string, read to EOF (`""` once EOF is reached)
- `stdin.readLines()` - Array of lines from standard input, split the same way as `File.readLines()`

### std/pq Methods

- `pq.new()` - New, empty min-priority-queue; returns a `PriorityQueue` (see
  [PriorityQueue Methods](#priorityqueue-methods) for its methods)

**Example:**
```neon
import "std/pq"

val queue = pq.new()
queue.push(2, "b")
queue.push(1, "a")
print(queue.pop())   // a
```

### Math (Static Methods)

- `Math.abs(n)` - Absolute value (keeps ints as ints)
- `Math.floor(n)` - Round down, returns an int (runtime error if the result doesn't fit in `i64` or is NaN)
- `Math.ceil(n)` - Round up, returns an int (same error cases as `floor`)
- `Math.sqrt(n)` - Square root
- `Math.min(a, b, ...)` - Minimum value (variadic, keeps ints as ints)
- `Math.max(a, b, ...)` - Maximum value (variadic, keeps ints as ints)
- `Math.div(a, b)` - Floor division on two ints, returns an int; errors on a float argument, division by
  zero, or overflow (`Math.div(i64::MIN, -1)`)
- `Math.round(n)` - Round half away from zero, returns an int (same error cases as `floor`)
- `Math.sign(n)` - `-1`, `0` or `1` as an int (`-0.0` is `0`; `NaN` is a runtime error)
- `Math.gcd(a, b)` - Greatest common divisor of two ints, always non-negative (`gcd(0, 0)` is `0`);
  errors on a float argument or overflow
- `Math.lcm(a, b)` - Least common multiple of two ints, always non-negative (`0` if either argument is
  `0`); errors on a float argument or overflow (`integer overflow in lcm()`)
- `Math.mod(a, b)` - Euclidean modulo in `[0, |b|)`: an int for two ints, a float otherwise; errors if
  `b` is `0` or if either argument is infinite or `NaN`

**Example:**
```neon
print(Math.abs(-5))        // 5
print(Math.sqrt(16))       // 4
print(Math.max(3, 7, 2))   // 7
print(Math.div(7, 2))      // 3, integer division
print(Math.round(2.5))     // 3
print(Math.gcd(12, 18))    // 6
print(Math.lcm(4, 6))      // 12
print(Math.mod(-7, 3))     // 2
```

### String Methods

- `.size()` - String length (character count)
- `.isEmpty()` - Whether the string has no characters
- `.substring(start, end)` - Extract substring (supports negative indices)
- `.replace(old, new)` - Replace all occurrences
- `.split()` - Split on runs of Unicode whitespace, dropping leading and trailing empties
- `.split(separator)` - Split into array
- `.toUpperCase()` - Convert to uppercase
- `.toLowerCase()` - Convert to lowercase
- `.toInt()` - Convert to integer
- `.toFloat()` - Convert to float
- `.toBool()` - Convert to boolean (case-insensitive)
- `.trim()` - Remove leading and trailing whitespace
- `.startsWith(prefix)` / `.endsWith(suffix)` - Check prefix / suffix
- `.indexOf(substring)` - Position of the first occurrence, or `-1`
- `.charCodeAt(index)` - Unicode code point of the character at `index`
- `String.fromCharCode(n)` - One-character string for the Unicode code point `n`
- `.repeat(n)` - Concatenate the string with itself `n` times (`n` a non-negative integer)
- `.padStart(len, fill)` / `.padEnd(len, fill)` - Pad with `fill` (cycled, non-empty) until `len` chars long
- `.lastIndexOf(substring)` - Position of the last occurrence, or `-1`
- `.contains(substring)` - Whether `substring` occurs anywhere in the string
- `.chars()` - Array of the string's characters, each a one-character string

Strings are indexable by character: `s[i]` returns the character at index `i` as a one-character
string, and a negative index counts from the end (`s[-1]` is the last character). An index outside
the string's bounds is a runtime error. Strings are immutable, so index assignment (`s[i] = v`) is
also a runtime error. `for ch in s` iterates the string's characters in the same order as `.chars()`.

**Example:**
```neon
val text = "Hello World"
print(text.toUpperCase())             // "HELLO WORLD"
print(text.substring(0, 5))           // "Hello"
print("one,two,three".split(","))     // ["one", "two", "three"]
print("42".toInt() + 8)               // 50
print(text[0])                        // "H"
print(text[-1])                       // "d"
for ch in "abc" {
    print(ch)
}
```

### Array Methods

- `.push(value)` - Add element to end
- `.pop()` - Remove and return the last element (`nil` if empty)
- `.size()` - Get array length
- `.isEmpty()` - Whether the array has no elements
- `.contains(value)` - Check if contains value
- `.indexOf(value)` - Position of the first match, or `-1`
- `.sort()` / `.sort(cmp)` / `.reverse()` - Sort (returns the same array) / reverse in place
- `.sortBy(fn)` - New array sorted ascending by `fn`'s key for each element (stable); the receiver
  is unchanged. Keys must be all numbers or all strings
- `.minBy(fn)` / `.maxBy(fn)` - First element with the smallest / largest key from `fn`, or `nil` on
  an empty array. Keys must be all numbers or all strings
- `.groupBy(fn)` - Map from each element's key (from `fn`) to an array of the elements that produced
  it, in first-key insertion order. Keys must be valid map keys
- `.tally()` - Map from each distinct element to how many times it occurs, in first-occurrence order.
  Elements must be valid map keys
- `.distinct()` - New array without repeated elements, keeping the first of each. Elements must be
  valid map keys
- `.scan(initial, fn)` - Like `reduce`, but returns every running accumulator as a new array, starting
  with `initial` (one element longer than the receiver)
- `.windowed(n)` - New array of every `n` consecutive elements, sliding one at a time; `[]` if `n` is
  larger than the array; `n` must be an integer >= 1
- `.takeWhile(fn)` - New array of the leading elements for which `fn` is truthy, stopping at the first that isn't
- `.dropWhile(fn)` - New array of the elements from the first one for which `fn` is falsy onwards
- `.partition(fn)` - `[matching, nonMatching]`: two new arrays split by whether `fn` is truthy
- `.slice(start, end)` - New array of the elements from `start` up to `end` (supports negative indices)
- `.join(delimiter)` - Join the elements into a string
- `.sum()`, `.min()`, `.max()` - Sum, minimum, maximum of the elements. `.sum()` is an int if every
  element is an int, a float if any element is a float (an int sum outside `i64` raises an overflow
  error); `.min()`/`.max()` return the chosen element unchanged
- `.map(fn)` - New array with `fn` applied to each element
- `.filter(fn)` - New array of the elements for which `fn` is truthy
- `.reduce(initial, fn)` - Fold the array from the left, calling `fn(accumulator, element)`
- `.forEach(fn)` - Call `fn` with each element in order; returns `nil`
- `.flatMap(fn)` - New array concatenating the arrays `fn` returns for each element; a `fn`
  returning a non-array is a runtime error (`flatMap() callback must return an array, got number`)
- `.find(fn)` - First element for which `fn` is truthy, or `nil`
- `.some(fn)` / `.every(fn)` - Whether `fn` is truthy for any / all elements; stop calling `fn` after
  the deciding element. `some` is `false` and `every` is `true` on an empty array
- `.flat()` - New array with one level of nested arrays spliced in; other elements kept as they are
- `.copy()` - Shallow copy: a new array with the same elements (heap values like nested arrays are
  still shared)
- `.take(n)` / `.drop(n)` - New array of the first `n` elements / with the first `n` elements
  removed, clamped to the array's length; `n` must be a non-negative integer
- `.first()` / `.last()` - First / last element, or `nil` if the array is empty
- `.chunked(n)` - New array of arrays of `n` elements each; the last chunk may be shorter; `n` must
  be an integer >= 1
- `.zip(other)` - New array pairing each element with the element at the same position in `other`
  (an array or range), stopping at the shorter length; anything else is a runtime error
  (`zip() other must be an array or range, got number`)
- `.withIndex()` - New array of `[index, element]` pairs
- `Array(n, init)` - New array of `n` elements. If `init` is a closure or function, it's called with
  each index from `0` to `n - 1` and its result becomes that element; otherwise `init` is stored (the
  same reference, for a heap value) in every element. `n` must be a non-negative integer

**Example:**
```neon
val arr = [1, 2, 3]
arr.push(4)
print(arr.size())          // 4
print(arr.contains(2))     // true
print(arr.map(fn(x) { return x * 2 }))          // [2, 4, 6, 8]
print(arr.filter(fn(x) { return x % 2 == 0 }))  // [2, 4]
print(arr.reduce(0, fn(acc, x) { return acc + x }))  // 10

val nums = [3, 1, 2]
print(nums.sort())                              // [1, 2, 3], same array as nums
print([3, 1, 2].sort(fn(a, b) { return b - a })) // [3, 2, 1]

print(Array(3, 0))                           // [0, 0, 0]
print(Array(3, fn(i) { return i * i }))      // [0, 1, 4]
val grid = Array(2, fn(y) { return Array(2, ".") })
grid[0][0] = "#"
print(grid)                                  // [["#", "."], [".", "."]]
```

`map`, `filter`, `reduce` and `sort` accept a named function, a closure, or a
lambda, and can call back into other Neon functions (including nested
`map`/`filter`/`reduce`/`sort` calls). Callbacks passed to `map`, `filter`,
`reduce` or `sort` can nest at most 32 levels deep before reporting a "Stack
overflow" error.

With no argument, `sort()` sorts in place (numbers ascending among
themselves, strings alphabetically among themselves) and returns the same
array. With a comparator `sort(fn(a, b) { ... })`, the function is called
with pairs of elements; a negative result puts `a` first, a positive result
puts `b` first, and `0` keeps their existing order (the sort is stable). The
comparator must return a number.

### Range Methods

- `.size()` - Number of integers the range covers, computed from its bounds
- `.isEmpty()` - Whether the range covers no integers
- `.contains(value)` - Check if value is an integer within the range, computed from its bounds
- `.toArray()` - Convert to an array
- `.step(k)` - Array of the range's values from its start, every k-th, honoring the end bound; `k` must be an integer >= 1
- `.slice(start, end)`, `.join(delimiter)`, `.indexOf(value)`, `.sum()`, `.min()`, `.max()`, `.map(fn)`, `.filter(fn)`, `.reduce(initial, fn)`, `.forEach(fn)`, `.flatMap(fn)`, `.take(n)`, `.drop(n)`, `.first()`, `.last()`, `.chunked(n)`, `.zip(other)`, `.withIndex()`, `.sortBy(fn)`, `.minBy(fn)`, `.maxBy(fn)`, `.groupBy(fn)`, `.tally()`, `.takeWhile(fn)`, `.dropWhile(fn)`, `.partition(fn)`, `.distinct()`, `.scan(initial, fn)`, `.windowed(n)` - Same as the Array methods, applied to the range's elements

Ranges are immutable: `.push()`, `.pop()`, `.sort()`, `.reverse()` and index assignment (`r[i] = v`) are all runtime errors.

**Example:**
```neon
val r = 1..4
print(r.size())            // 3
print(r.contains(2))       // true
print(r.toArray())         // [1, 2, 3]
print(r.map(fn(x) { return x * 2 }))  // [2, 4, 6]
```

### Map Methods

- `.size()` - Number of entries
- `.isEmpty()` - Whether the map has no entries
- `.contains(key)` - Check if key exists
- `.get(key)` - Value for `key`, or `nil` if absent
- `.remove(key)` - Remove `key` and return its value
- `.keys()` - Get array of keys
- `.values()` - Get array of values
- `.entries()` - Get array of [key, value] pairs
- `.forEach(fn)` - Call `fn(key, value)` for each entry in insertion order
- `.map(fn)` - Array of `fn(key, value)` for each entry
- `.filter(fn)` - New map of the entries for which `fn(key, value)` is truthy
- `.mapValues(fn)` - New map with the same keys and `fn(key, value)` as each value
- `.some(fn)` - Whether `fn(key, value)` is truthy for any entry; stops at the first
- `.every(fn)` - Whether `fn(key, value)` is truthy for every entry; stops at the first that isn't
- `[key]` - Direct index access to get/set values

A key can be a string, number, boolean, enum variant, or array (same rule for set elements). An
enum variant with payload fields is a valid key only if each field is itself a valid key. An
array key is copied into a frozen value when it's inserted — comparisons go by content, and
mutating the original array afterwards doesn't change the stored key. Every element of an array
key must itself be a valid key, and a self-referencing array is a runtime error.

**Example:**
```neon
val map = {"a": 1, "b": 2, "c": 3}
print(map.keys())         // ["a", "b", "c"]
print(map.values())       // [1, 2, 3]
print(map.size())         // 3
print(map["a"])           // 1
```

### Set Methods

- `.add(value)` - Add element (returns true if added, false if duplicate)
- `.remove(value)` - Remove element (returns true if removed, false if not found)
- `.contains(value)` - Check if contains value
- `.size()` - Number of elements
- `.isEmpty()` - Whether the set has no elements
- `.clear()` - Remove all elements
- `.union(other)` - Set union
- `.intersection(other)` - Set intersection
- `.difference(other)` - Set difference
- `.isSubset(other)` - Check if this set is a subset of another
- `.toArray()` - Convert set to array

**Example:**
```neon
val set = #{1, 2}
print(set.size())         // 2
print(set.contains(1))    // true

val arr = set.toArray()
print(arr)                // [1, 2] (order may vary)
```

### File

- `File(path)` - Open a file handle for `path`
- `.read()` - Whole file as a string
- `.readLines()` - Array of lines
- `.write(text)` - Create the file with `text`; a runtime error if it already exists

### Stdin Methods

- `Stdin.read()` - All remaining standard input as a string, read to EOF (`""` once EOF is reached)
- `Stdin.readLines()` - Array of lines from standard input, split the same way as `File.readLines()`

**Example:**
```neon
// echo -e "a\nb" | neon script.n
print(Stdin.readLines())   // [a, b]
```

### PriorityQueue Methods

- `PriorityQueue()` - New, empty min-priority-queue
- `.push(priority, value)` - Add `value` with the given `priority` (a number); returns `nil`
- `.pop()` - Remove and return the value with the smallest priority, or `nil` if empty. Equal
  priorities pop in insertion order
- `.peek()` - Same as `.pop()` but leaves the queue unchanged
- `.size()` - Number of entries
- `.isEmpty()` - Whether the queue has no entries

**Example:**
```neon
val pq = PriorityQueue()
pq.push(3, "c")
pq.push(1, "a")
pq.push(2, "b")
print(pq.pop())   // a
print(pq.pop())   // b
print(pq.pop())   // c
print(pq.pop())   // nil
```

### Type Conversions

**Number Methods:**
- `.toString()` - Convert to string
- `.toInt()` - Convert to int (truncates a float)
- `.toFloat()` - Convert to float

**Boolean Methods:**
- `.toString()` - Convert to string

**Example:**
```neon
val n = 42
print(n.toString())       // "42"

val b = true
print(b.toString())       // "true"
```

## Goal

I wanted to make neon self-hosted someday, but I'm not sure anymore. Goals may appear (or disappear) as the project evolves. For now, Neon serves its primary purpose: teaching me how compilers and VMs work.

## Inspiration

This project was inspired by Robert Nystrom's brilliant book [Crafting Interpreters](https://craftinginterpreters.com/).
It's one of the best reads in tech and I highly recommended it if you're interested in the topic!

Go and get yourself a copy!

That said, neon won't follow Lox, the language developed in the book. It's taking its own path.

## License

This project is licensed under the MIT License - see the
LICENSE [file](https://github.com/patbuc/neon/blob/main/LICENSE) for details.
