[![Rust](https://github.com/patbuc/neon/actions/workflows/rust.yml/badge.svg)](https://github.com/patbuc/neon/actions/workflows/rust.yml)

# ✨ Neon

A toy language you didn't wait for!

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
- **Sets** - Unique value collections (created with set literal syntax)

**Other Types:**
- **Ranges** - Inclusive `1..=10` or exclusive `1..10`
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
val numbers = {1}
numbers.clear()
numbers.add(1)
numbers.add(2)
numbers.add(3)
for (num in numbers) {
    print(num)
}
```

### Operators

**Arithmetic:**
- `+` Addition (also string concatenation)
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `//` Floor division
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
val numbers = {1}
numbers.clear()

// Add elements
numbers.add(1)
numbers.add(2)
numbers.add(3)
numbers.add(2)  // Duplicate ignored

print(numbers.size())  // 3

// Check membership
if (numbers.has(1)) {
    print("Contains 1")
}

// Set operations
val setA = {1}
setA.clear()
setA.add(1)
setA.add(2)
setA.add(3)

val setB = {1}
setB.clear()
setB.add(2)
setB.add(3)
setB.add(4)

val unionSet = setA.union(setB)         // {1, 2, 3, 4}
val intersect = setA.intersection(setB) // {2, 3}
val diff = setA.difference(setB)        // {1}

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

**Example:**
```neon
val arr = [1, 2, 3]
arr.push(4)
print(arr.size())          // 4
print(arr.contains(2))     // true
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
val set = {1}
set.clear()
set.add(1)
set.add(2)
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
