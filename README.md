[![Rust](https://github.com/patbuc/neon/actions/workflows/rust.yml/badge.svg)](https://github.com/patbuc/neon/actions/workflows/rust.yml)

# ✨ Neon

A toy language you didn't wait for

## Why

Let’s be honest — the world doesn’t need another programming language.
I’m building neon for one simple reason: to teach myself a few things.

- How to build a compiler
- How to build a virtual machine
- How to use Rust for this

## State

Right now, neon doesn't do much — it's purely experimental and entirely useless and will happily crash in interesting
ways.

## Language Features

Neon currently supports the following operators:

### Logical Operators

- `&&` (logical AND) - Returns `true` if both operands are truthy, otherwise returns `false`
- `||` (logical OR) - Returns `true` if either operand is truthy, otherwise returns `false`
- `!` (logical NOT) - Negates a boolean value

**Short-circuit evaluation:** Both `&&` and `||` use short-circuit evaluation, meaning the right operand is only
evaluated if necessary.

**Operator precedence:** `||` has lower precedence than `&&`, so `a || b && c` is evaluated as `a || (b && c)`.

Example:
```neon
val t = true
val f = false
print t && f    // false
print t || f    // true
print !t        // false
```

### Comparison Operators

- `==` (equal)
- `!=` (not equal)
- `<` (less than)
- `<=` (less than or equal)
- `>` (greater than)
- `>=` (greater than or equal)

### Arithmetic Operators

- `+` (addition and string concatenation)
- `-` (subtraction)
- `*` (multiplication)
- `/` (division)

## Goal

I wanted to make neon self-hosted someday, but I am not sure anymore. Goals may appear (or disappear) as the project
evolves.

## Inspiration

This project was inspired by Robert Nystrom’s brilliant book [Crafting Interpreters](https://craftinginterpreters.com/).
It’s one of the best reads in tech and I highly recommended it if you’re interested in the topic!

Go and get yourself a copy!

That said, neon won’t follow Lox, the language developed in the book. It’s taking its own path.

## License

This project is licensed under the MIT License - see the
LICENSE [file](https://github.com/patbuc/neon/blob/main/LICENSE) for details.
