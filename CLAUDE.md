# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Neon is a dynamically-typed, bytecode-compiled language with a stack-based VM, written in Rust. The implementation
follows a traditional compiler pipeline: Scanner → Parser → Semantic Analysis → Code Generation → VM Execution.

## Build & Test Commands

### Building

```bash
cargo build              # Debug build
cargo build --release    # Release build
```

### Testing

```bash
cargo test              # Run all tests (unit + integration)
cargo test -p neon      # Run only unit tests
```

Integration tests use `datatest-stable` harness and run all `.n` scripts in `tests/scripts/`. Each script must include
inline expected output:

```neon
// Expected:
// output line 1
// output line 2
```

### Running Scripts

```bash
cargo run -- script.n           # Interpret a Neon script
cargo run -- script.n arg1 arg2 # Pass arguments to script
cargo run                       # Start REPL
```

### WebAssembly

```bash
./build-wasm.sh         # Build for web (requires wasm-pack)
# Output: wasm-pkg/{neon.js, neon_bg.wasm, neon.d.ts}
```

### Debugging

```bash
cargo build --features disassemble  # Enable bytecode disassembly
cargo run --features disassemble -- script.n
```

## Architecture

### Compilation Pipeline

1. **Scanner** (`src/compiler/scanner.rs`)
    - Lexical analysis producing tokens
    - Handles keywords, operators, literals, identifiers
    - Tracks line/column for error reporting

2. **Parser** (`src/compiler/parser.rs`)
    - Builds AST from tokens (defined in `src/compiler/ast/`)
    - Recursive descent parser
    - AST nodes: expressions, statements, declarations

3. **Semantic Analysis** (`src/compiler/semantic.rs`)
    - Type checking and validation
    - Symbol resolution via `src/compiler/symbol_table.rs`
    - Scoped symbol tables with lexical scoping

4. **Code Generation** (`src/compiler/codegen.rs`)
    - Traverses AST and emits bytecode
    - Produces Chunk objects containing instructions and constant pool

### Runtime Architecture

**VM Core** (`src/vm/impl.rs`)

- Stack-based bytecode interpreter
- Main execution loop processes opcodes
- Call frame stack for function calls (`src/vm/functions.rs`)
- Separate builtin values storage (e.g., Math namespace)

**Bytecode Format** (`src/common/chunk/`)

- Chunk: bytecode instructions + constant pool + string table + source locations
- Constants pool stores literals referenced by index
- Source locations map bytecode positions to source line/column for error reporting

**Opcodes** (`src/common/opcodes.rs`)

- Instruction set definition as `#[repr(u8)]` enum
- Stack manipulation, arithmetic, control flow, function calls
- Variable-width operands (8/16/32-bit indices)

**Value System** (`src/common/mod.rs`)

- `Value` enum: Number(f64), Boolean, Nil, Object(Rc<Object>)
- Object types: String, Function, NativeFunction, Struct, Instance, Array, Map, Set, File
- Reference-counted objects with interior mutability where needed (RefCell)

**Standard Library** (`src/common/stdlib/`)

- Native functions for built-in types
- Math namespace with static methods
- String/Array/Map/Set methods via method registry
- Method registry (`src/common/method_registry.rs`) maps type+method to function index

### Key Type Interactions

- **CallFrame**: Links function object to instruction pointer and stack slot range
- **Locals**: Track variable names, scope depth, capture status for closures
- **Iterator Stack**: Supports nested for-in loops by tracking (index, collection) pairs
- **Builtin Storage**: Separate from call stack to avoid polluting stack frames

## Adding New Features

### New Opcode

1. Add variant to `OpCode` enum in `src/common/opcodes.rs`
2. Implement execution logic in `src/vm/impl.rs` VM loop
3. Emit opcode in `src/compiler/codegen.rs`
4. Update disassembler in `src/common/chunk/disassembler.rs` (if using disassemble feature)
5. Add tests for compilation and execution

### New Language Feature

1. Add token types to `src/compiler/token.rs` if needed
2. Update scanner in `src/compiler/scanner.rs`
3. Extend AST nodes in `src/compiler/ast/mod.rs`
4. Add parsing logic in `src/compiler/parser.rs`
5. Add semantic validation in `src/compiler/semantic.rs`
6. Implement code generation in `src/compiler/codegen.rs`
7. Write integration test in `tests/scripts/` with expected output

### New Standard Library Function

1. Implement function in appropriate `src/common/stdlib/*_functions.rs` file
2. Register in method registry if it's a method (see `src/common/method_registry.rs`)
3. For global functions, add to builtin initialization in VM
4. Add tests in corresponding `src/common/stdlib/tests/` file

## Code Conventions

### Rust Patterns

- Use `Result<T, E>` for error propagation, avoid `unwrap()` except in tests
- Pattern matching for AST traversal and opcode dispatch
- Minimize allocations in VM hot path (execution loop)
- Use `Rc` for shared ownership, `RefCell` only when mutation needed
- Prefer `tracing` crate for debug logging, not `println!`

### Compiler/VM Patterns

- **Stack invariants**: Document expected stack state before/after operations in comments
- **Error reporting**: Always include source location (line/column) from tokens
- **Symbol tables**: Maintain proper lexical scope depth
- **Bytecode emission**: Append-only except for jump address backpatching
- **Opcode design**: Keep instruction set minimal and orthogonal

### Testing

- Unit tests in module files or submodule `tests/` directories
- Integration tests in `tests/scripts/` use inline expected output format
- Test both success and error paths
- Include edge cases (empty input, stack overflow, division by zero, etc.)

### Error Handling

- Compilation errors use `src/common/error_renderer.rs` for formatted output
- Runtime errors should include context about what operation failed
- VM returns `Result` enum: Ok, CompileError, RuntimeError

## Project Philosophy

This is a learning project focused on understanding compiler and VM construction. Prioritize:

- Code clarity over performance optimization
- Explicit intermediate representations
- Helpful error messages
- Educational value of features

Avoid:

- Production-grade optimizations that obscure the learning path
- Over-engineering or premature abstraction
