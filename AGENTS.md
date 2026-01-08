# Agent Instructions

This file provides guidance for AI coding agents working in this repository.

## Project Overview

Neon is a dynamically-typed, bytecode-compiled language with a stack-based VM, written in Rust.
Pipeline: Scanner → Parser → Semantic Analysis → Code Generation → VM Execution.

## Issue Tracking (Beads)

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

```bash
bd ready                              # Find available work
bd show <id>                          # View issue details  
bd update <id> --status in_progress   # Claim work
bd close <id>                         # Complete work
bd sync                               # Sync with git
```

## Build Commands

```bash
cargo build                           # Debug build
cargo build --release                 # Release build
cargo build --features disassemble    # Enable bytecode disassembly
```

## Test Commands

```bash
cargo test                            # Run all tests (unit + integration)
cargo test -p neon                    # Run only unit tests (no integration)
cargo test <test_name>                # Run single test by name
cargo test scanner::                  # Run all tests in scanner module
cargo test --test neon_scripts        # Run only integration tests
```

### Running a Single Test

```bash
cargo test can_scan_simple_statement              # By test function name
cargo test -p neon scanner::can_scan              # By module path prefix
cargo test -- --exact can_scan_simple_statement   # Exact match only
```

### Integration Tests

Integration tests use `datatest-stable` and run `.n` scripts in `tests/scripts/`.
Each script requires inline expected output:

```neon
// Expected:
// output line 1
// output line 2
print "output line 1"
print "output line 2"
```

## Running Scripts

```bash
cargo run -- script.n                 # Interpret a Neon script
cargo run -- script.n arg1 arg2       # Pass arguments to script
cargo run                             # Start REPL
cargo run --features disassemble -- script.n  # With bytecode dump
```

## Code Style Guidelines

### Imports

Order imports as: std → external crates → crate-local modules.

### Module Structure

- Use `pub(crate)` for internal visibility, not `pub`
- Place tests in `mod tests` within the file or `tests/` subdirectory
- Conditional test modules: `#[cfg(test)] mod tests;`

### Naming Conventions

- Types: `PascalCase` (e.g., `TokenType`, `ObjFunction`, `CompilationError`)
- Functions/methods: `snake_case` (e.g., `scan_token`, `emit_opcode`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_LOCALS`)
- Enum variants: `PascalCase` (e.g., `TokenType::Identifier`)
- Prefixes: `Obj` for object types (e.g., `ObjString`, `ObjFunction`, `ObjInstance`)

### Error Handling

- Use `Result<T, E>` for fallible operations
- Avoid `unwrap()` except in tests
- Include source location in compilation errors
- Use `CompilationResult<T>` alias for `Result<T, Vec<CompilationError>>`

### Types and Ownership

- Use `Rc<T>` for shared ownership (objects, functions, chunks)
- Use `RefCell<T>` only when interior mutability is required
- Use `Rc<RefCell<T>>` for mutable shared data (arrays, maps, instances)
- Prefer `Rc<str>` over `Rc<String>` for immutable strings

### VM/Compiler Patterns

- Document stack state in comments before/after operations
- Use pattern matching for AST traversal and opcode dispatch
- Minimize allocations in VM execution loop (hot path)
- Bytecode is append-only except for jump backpatching

### Testing Patterns

Unit tests go in module `tests/` subdirectories. Test both success and error paths. Include edge cases.

## Architecture Quick Reference

| Component | Location | Purpose |
|-----------|----------|---------|
| Scanner | `src/compiler/scanner.rs` | Lexical analysis → tokens |
| Parser | `src/compiler/parser.rs` | Syntax analysis → AST |
| AST | `src/compiler/ast/mod.rs` | Abstract syntax tree nodes |
| Semantic | `src/compiler/semantic.rs` | Type checking, symbol resolution |
| Symbol Table | `src/compiler/symbol_table.rs` | Scoped symbol management |
| Codegen | `src/compiler/codegen.rs` | AST → bytecode |
| Opcodes | `src/common/opcodes.rs` | Instruction set definition |
| Chunk | `src/common/chunk/` | Bytecode + constants + source maps |
| VM | `src/vm/impl.rs` | Bytecode interpreter |
| Values | `src/common/mod.rs` | Runtime value types |
| Stdlib | `src/common/stdlib/` | Built-in functions |

## Adding New Features

### New Token/Keyword

1. Add variant to `TokenType` in `src/compiler/token.rs`
2. Add keyword mapping in `src/compiler/scanner.rs`
3. Add scanner test in `src/compiler/tests/scanner.rs`

### New AST Node

1. Add node type in `src/compiler/ast/mod.rs`
2. Add parsing in `src/compiler/parser.rs`
3. Add semantic validation in `src/compiler/semantic.rs`
4. Add codegen in `src/compiler/codegen.rs`

### New Opcode

1. Add variant to `OpCode` in `src/common/opcodes.rs`
2. Implement in VM loop in `src/vm/impl.rs`
3. Update disassembler in `src/common/chunk/disassembler.rs`

## Session Completion (Landing the Plane)

When ending a work session, complete ALL steps:

1. **File issues** for remaining work
2. **Run quality gates**: `cargo test && cargo build`
3. **Update issue status**: Close finished work
4. **Push to remote** (MANDATORY):
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Verify** all changes committed AND pushed

**Critical**: Work is NOT complete until `git push` succeeds.
