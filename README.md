[![CI](https://img.shields.io/github/actions/workflow/status/patbuc/neon/rust.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/patbuc/neon/actions/workflows/rust.yml)
[![Lint](https://img.shields.io/github/actions/workflow/status/patbuc/neon/lint.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=lint)](https://github.com/patbuc/neon/actions/workflows/lint.yml)
[![Bench](https://img.shields.io/github/actions/workflow/status/patbuc/neon/bench.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white&label=bench)](https://github.com/patbuc/neon/actions/workflows/bench.yml)
[![License](https://img.shields.io/github/license/patbuc/neon?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange?style=flat-square&logo=rust&logoColor=white)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
[![Last commit](https://img.shields.io/github/last-commit/patbuc/neon?style=flat-square)](https://github.com/patbuc/neon/commits)
[![Code size](https://img.shields.io/github/languages/code-size/patbuc/neon?style=flat-square)](https://github.com/patbuc/neon)

# <picture><source media="(prefers-color-scheme: dark)" srcset="assets/logo/svg/neon-logo-dark.svg"><img src="assets/logo/svg/neon-logo-light.svg" alt="Neon logo" height="40" align="top"></picture> Neon

A toy language you didn't wait for

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

## Why

Let's be honest — the world doesn't need another programming language.
I'm building neon for one simple reason: to teach myself a few things.

- How to build a compiler
- How to build a virtual machine
- How to use Rust for this

## Status

Neon is a learning project, not a tool for real work. It runs small programs, but expect bugs, missing features,
and breaking changes whenever I feel like changing the language. Don't use it for anything that matters.

I once wanted to make it self-hosted; I'm not sure anymore. For now it does its job: teaching me how compilers and
VMs work.

## Getting Started

You need a Rust toolchain ([rustup.rs](https://rustup.rs)).

```bash
cargo build --release
cargo run -- hello.n            # run a script
cargo run -- --check hello.n    # compile without running
cargo run -- -e 'print(1 + 2)'  # run a snippet
cargo run                       # start the REPL
```

VS Code syntax highlighting for `.n` files ships as a VSIX on the
[GitHub Releases page](https://github.com/patbuc/neon/releases). See
[editors/vscode/README.md](editors/vscode/README.md) for details.

## Documentation

[docs/reference.md](docs/reference.md) covers the language, the standard library, the REPL, and the benchmarks.

## Benchmarks

`benches/` pits Neon against plain Python on the same algorithms. Charts are published at
[patbuc.github.io/neon/dev/bench](https://patbuc.github.io/neon/dev/bench/), updated on every push to `main`; see
[Benchmarks](docs/reference.md#benchmarks) to run them yourself.

## Inspiration

This project was inspired by Robert Nystrom's brilliant book [Crafting Interpreters](https://craftinginterpreters.com/).
It's one of the best reads in tech and I highly recommended it if you're interested in the topic!

Go and get yourself a copy!

That said, neon won't follow Lox, the language developed in the book. It's taking its own path.

## License

This project is licensed under the MIT License - see the
LICENSE [file](https://github.com/patbuc/neon/blob/main/LICENSE) for details.
