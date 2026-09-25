[![CI](https://img.shields.io/github/actions/workflow/status/patbuc/neon/rust.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/patbuc/neon/actions/workflows/rust.yml)
[![Lint](https://img.shields.io/github/actions/workflow/status/patbuc/neon/lint.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=lint)](https://github.com/patbuc/neon/actions/workflows/lint.yml)
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
```

### Hello World

Create a file `hello.n`:

```neon
print("Hello, Neon!")
```

Run it:

```bash
cargo run -- hello.n
```

## State

Neon is a functional dynamically-typed interpreter with a comprehensive feature set. It's an active learning project and includes:

- Complete lexer, parser, and bytecode compiler
- Stack-based virtual machine
- Rich standard library with collection types and methods
- 80+ integration tests validating all features
- String interpolation and first-class functions

While Neon is functional for many programs, it remains experimental. Expect rough edges, missing features, and occasional crashes as development continues.

## Language Features

### Data Types

**Primitives:**
- **Numbers** - 64-bit floating-point (e.g., `42`, `3.14`)
- **Booleans** - `true` and `false`
- **Strings** - Unicode text with escapes (e.g., `"hello"`, `"world\n"`)
- **Nil** - Null value represented as `nil`

**Collections:**
- **Arrays** - Ordered, mutable, indexed collections (e.g., `[1, 2, 3]`)
- **Maps** - Key-value dictionaries (e.g., `{"name": "Alice", "age": 30}`)
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

### Functions

```neon
fn add(a, b) {
    return a + b
}

fn greet(name) {
    print("Hello, ${name}!")
}

fn fibonacci(n) {
    if (n <= 1) {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

Functions are first-class values: they can be stored in variables, passed as
arguments, and returned from other functions.

```neon
fn double(x) {
    return x * 2
}

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
if (x > 10) {
    print("Greater than 10")
} else if (x > 5) {
    print("Greater than 5")
} else {
    print("5 or less")
}
```

**While Loops:**

```neon
var i = 0
while (i < 5) {
    print(i)
    i = i + 1
}
```

**For Loops:**

```neon
// Traditional for loop
for (var i = 0; i < 10; i = i + 1) {
    print(i)
}

// For-in loop over arrays
for (item in [1, 2, 3, 4, 5]) {
    print(item)
}

// For-in loop over ranges
for (i in 1..=10) {
    print(i)
}

// For-in loop over map keys
val person = {"name": "Alice", "age": 30}
for (key in person) {
    print("${key}: ${person[key]}")
}

// For-in loop over set
val numbers = #{1, 2, 3}
for (num in numbers) {
    print(num)
}
```

Both loop forms give every iteration its own binding of the loop variable, so a closure
created in the body keeps that iteration's value. In a traditional `for` loop, a write to
the variable in the body carries over to the increment and the next iteration.

### Operators

**Arithmetic:**
- `+` Addition (also string concatenation)
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo
- `-x` Negation (unary)

**Comparison:**
- `==` Equal
- `!=` Not equal
- `<` Less than
- `<=` Less than or equal
- `>` Greater than
- `>=` Greater than or equal

**Logical:**
- `&&` Logical AND (short-circuit)
- `||` Logical OR (short-circuit)
- `!` Logical NOT (unary)

**Other:**
- `..` Range (exclusive)
- `..=` Range (inclusive)

**Operator Precedence:** `||` has lower precedence than `&&`, so `a || b && c` is evaluated as `a || (b && c)`.

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
- Methods are registered before the program runs, so they can be called from
  code that appears before their `impl` block. A method body can see
  functions, structs, builtins, and top-level variables.

A builtin type (`Array`, `String`, `Map`, `Set`, `Number`, `Boolean`, `File`)
can have an `impl` block too, adding an instance method callable on any value
of that type. A method can't share a name with a native method of the type -
that's a compile error, since a native method can never be redefined. Unlike
a struct, a builtin type only supports instance methods; every method must
take `self`.

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

## Code Examples

### Fibonacci

```neon
fn fibonacci(n) {
    if (n <= 1) {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

for (var i = 0; i < 10; i = i + 1) {
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
if (numbers.contains(5)) {
    print("Found 5!")
}

// Iterate
for (num in numbers) {
    print(num)
}

// Get length
print("Length: ${numbers.length()}")
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
for (key in person) {
    print("${key}: ${person[key]}")
}

// Check if key exists
if (person.has("age")) {
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
if (numbers.has(1)) {
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
if (setA.isSubset(setB)) {
    print("A is subset of B")
}

// Convert to array for iteration control
val asArray = numbers.toArray()
for (var i = 0; i < asArray.size(); i = i + 1) {
    print(asArray[i])
}
```

### String Operations

```neon
val text = "Hello World"

// String methods
print(text.len())                      // 11
print(text.toUpperCase())              // "HELLO WORLD"
print(text.toLowerCase())              // "hello world"
print(text.substring(0, 5))            // "Hello"
print(text.replace("World", "Neon"))   // "Hello Neon"

// Split into array
val words = "one,two,three".split(",")  // ["one", "two", "three"]
for (word in words) {
    print(word)
}

// Type conversions
val num = "42".toInt()
val pi = "3.14".toFloat()
val flag = "true".toBool()
```

## Standard Library

### Global Functions

- `print(value, ...)` - Output values to stdout (variadic)

### Math (Static Methods)

- `Math.abs(n)` - Absolute value
- `Math.floor(n)` - Round down to nearest integer
- `Math.ceil(n)` - Round up to nearest integer
- `Math.sqrt(n)` - Square root
- `Math.min(a, b, ...)` - Minimum value (variadic)
- `Math.max(a, b, ...)` - Maximum value (variadic)

**Example:**
```neon
print(Math.abs(-5))        // 5
print(Math.sqrt(16))       // 4
print(Math.max(3, 7, 2))   // 7
print(Math.floor(a / b))   // integer division
```

### String Methods

- `.len()` - String length (character count)
- `.substring(start, end)` - Extract substring (supports negative indices)
- `.replace(old, new)` - Replace all occurrences
- `.split(separator)` - Split into array
- `.toUpperCase()` - Convert to uppercase
- `.toLowerCase()` - Convert to lowercase
- `.toInt()` - Convert to integer
- `.toFloat()` - Convert to float
- `.toBool()` - Convert to boolean (case-insensitive)

**Example:**
```neon
val text = "Hello World"
print(text.toUpperCase())             // "HELLO WORLD"
print(text.substring(0, 5))           // "Hello"
print("one,two,three".split(","))     // ["one", "two", "three"]
print("42".toInt() + 8)               // 50
```

### Array Methods

- `.push(value)` - Add element to end
- `.size()` / `.length()` - Get array length
- `.contains(value)` - Check if contains value
- `.map(fn)` - New array with `fn` applied to each element
- `.filter(fn)` - New array of the elements for which `fn` is truthy
- `.reduce(fn, initial)` - Fold the array from the left, calling `fn(accumulator, element)`

**Example:**
```neon
val arr = [1, 2, 3]
arr.push(4)
print(arr.size())          // 4
print(arr.contains(2))     // true
print(arr.map(fn(x) { return x * 2 }))          // [2, 4, 6, 8]
print(arr.filter(fn(x) { return x % 2 == 0 }))  // [2, 4]
print(arr.reduce(fn(acc, x) { return acc + x }, 0))  // 10
```

`map`, `filter` and `reduce` accept a named function, a closure, or a lambda,
and can call back into other Neon functions (including nested `map`/`filter`/
`reduce` calls). Callbacks passed to `map`, `filter` or `reduce` can nest at
most 32 levels deep before reporting a "Stack overflow" error.

### Range Methods

- `.size()` / `.length()` - Number of integers the range covers, computed from its bounds
- `.contains(value)` - Check if value is an integer within the range, computed from its bounds
- `.toArray()` - Convert to an array
- `.slice(start, end)`, `.join(delimiter)`, `.indexOf(value)`, `.sum()`, `.min()`, `.max()`, `.map(fn)`, `.filter(fn)`, `.reduce(fn, initial)` - Same as the Array methods, applied to the range's elements

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
- `.has(key)` - Check if key exists
- `.keys()` - Get array of keys
- `.values()` - Get array of values
- `.entries()` - Get array of [key, value] pairs
- `[key]` - Direct index access to get/set values

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
- `.has(value)` - Check if contains value
- `.size()` - Number of elements
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
print(set.has(1))         // true

val arr = set.toArray()
print(arr)                // [1, 2] (order may vary)
```

### Type Conversions

**String Conversions:**
- `String(value)` - Convert value to string
- `.toString()` - Available on numbers and booleans

**Number Methods:**
- `.toString()` - Convert to string

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
