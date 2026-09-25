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
    - Resolves every name once, using scoped symbol tables (`src/compiler/symbol_table.rs`) for
      lexical scoping — including which calls dispatch to a native
    - Returns `Resolutions` (`src/compiler/resolutions.rs`): a `Res` per name-use node (keyed by the
      parser-assigned `NodeId`), native-call entries, declarations, per-function params and upvalue
      captures, and which declarations are captured
    - Owns these diagnostics: undefined variable, break/continue outside a loop, postfix operand

4. **Code Generation** (`src/compiler/codegen.rs`)
    - Traverses AST and emits bytecode, consuming `&Resolutions` — it never looks up a name by string,
      and maps each `DeclId` to a stack slot when it defines the local
    - Produces Chunk objects containing instructions and constant pool
    - Compile-time state (locals, scope depth, loop contexts) lives in the per-function
      `FunctionCompiler`, not in the Chunk; upvalue captures come from `Resolutions`
      (`FunctionResolution.upvalues`)

### Runtime Architecture

**VM Core** (`src/vm/impl.rs`)

- Stack-based bytecode interpreter
- Main execution loop processes opcodes
- Call frame stack for function calls (`src/vm/functions.rs`)
- Separate builtin values storage (e.g., Math namespace)

**Bytecode Format** (`src/common/chunk/`)

- Chunk: name, bytecode instructions, constant pool, string table, and a line table of `LineInfo` entries
- Constants pool stores literals referenced by index
- `LineInfo { ip, line, column }` maps instruction offsets to source line/column for error reporting

**Opcodes** (`src/common/opcodes.rs`)

- Instruction set definition as `#[repr(u8)]` enum
- Stack manipulation, arithmetic, control flow, function calls
- Index operands (constants, strings, locals, globals, upvalues, builtins) are a fixed 16 bits; jump/loop offsets are 32 bits

**Value System** (`src/common/mod.rs`)

- Scalars (Number, Boolean, Nil) are stored inline
- Every heap variant (String, Function, Closure, NativeFunction, Struct, Instance, Array, Map, Set, File) holds a single `Rc`; collections and instances use `Rc<RefCell<..>>` for interior mutability
- Strings are `Rc<String>` so `Value` stays 16 bytes
- `Uninitialized` marks a hoisted global/block-level slot before its declaration runs

**Standard Library** (`src/common/stdlib/`)

- Native functions for built-in types
- Math namespace with static methods
- String/Array/Map/Set methods via method registry
- Method registry (`src/common/method_registry.rs`) maps type+method to function index

### Key Type Interactions

- **CallFrame**: Links function object to instruction pointer and stack slot range
- **Locals**: Tracked per-function in the code generator's `FunctionCompiler` — scope depth and capture
  status for closures
- **Iterator Stack**: Supports nested for-in loops by tracking (index, collection) pairs
- **Builtin Storage**: Separate from call stack to avoid polluting stack frames

## Adding New Features

### New Opcode

1. Add variant to `OpCode` enum in `src/common/opcodes.rs` and its byte to `OpCode::from_u8`
2. Implement execution logic in `src/vm/impl.rs` VM loop
3. Emit opcode in `src/compiler/codegen.rs`
4. Update disassembler in `src/common/chunk/disassembler.rs` (if using disassemble feature)
5. Add tests for compilation and execution

### New Language Feature

1. Add token types to `src/compiler/token.rs` if needed
2. Update scanner in `src/compiler/scanner.rs`
3. Extend AST nodes in `src/compiler/ast/mod.rs`
4. Add parsing logic in `src/compiler/parser.rs`
5. Add semantic validation in `src/compiler/semantic.rs`; a new binding form is resolved here too, so
   codegen never has to look it up by name
6. Implement code generation in `src/compiler/codegen.rs`, reading the resolution recorded in step 5
7. Write integration test in `tests/scripts/` with expected output

### New Standard Library Function

1. Implement function in appropriate `src/common/stdlib/*_functions.rs` file
2. Register in method registry if it's a method (see `src/common/method_registry.rs`) — a new namespace (like
   `Math`/`File`) is picked up automatically from there; a new runtime builtin value (like `args`) is declared in
   `BUILTIN_VALUES` (`src/common/stdlib/mod.rs`) and constructed in `create_builtin_objects`
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
- **Name resolution**: Names shadow lexically — a local or user function named like a native (`print`,
  `Math`, `File`) wins; the method registry is consulted only when a name resolves to nothing else
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

## Agent Workflow: Orchestrated Feature Development

This project uses a three-agent workflow for feature development. The agents are defined in `.claude/agents/`.

### Agent Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│ ORCHESTRATOR AGENT (top-level coordinator)              │
│ .claude/agents/orchestrator-agent.md                    │
│                                                         │
│ Responsibilities:                                       │
│ • Break down user request into atomic steps             │
│ • Create and maintain plan file                         │
│ • Get user approval before implementation               │
│ • Spawn sub-agents for each step                        │
│ • Handle commits after passed steps                     │
│ • Create final PR                                       │
│                                                         │
│ Sub-agents it spawns:                                   │
│ ├── coding-agent (for implementation)                   │
│ └── quality-gate-agent (for validation)                 │
└─────────────────────────────────────────────────────────┘
```

### Agent Roles

| Agent | File | Responsibility |
|-------|------|----------------|
| **Orchestrator** | `orchestrator-agent.md` | Coordinates workflow, breaks down problems, manages state |
| **Coding Agent** | `coding-agent.md` | Implements single steps, writes code and tests |
| **Quality Gate Agent** | `quality-gate-agent.md` | Reviews code, determines PASS/FAIL with feedback |

### Workflow Phases

**Phase 1: Planning (Human Approval Required)**
1. Orchestrator analyzes request and explores codebase
2. Breaks down into atomic, testable steps
3. Creates plan file at `.claude/plans/feature-{slug}.md`
4. Presents plan to user for approval
5. Blocks until user approves

**Phase 2: Implementation Loop (Per Step)**
1. Orchestrator spawns coding-agent for current step
2. Coding-agent implements step and runs tests
3. Orchestrator runs quality gate commands
4. Orchestrator spawns quality-gate-agent to review
5. If PASS: commit and move to next step
6. If FAIL: retry (max 3 attempts) or escalate

**Phase 3: Completion**
1. Verify all steps passed
2. Run final quality gate
3. Create feature branch, push, and open PR
4. Clean up plan file

### When Workflow Stops for Human Input

- **Always**: Planning phase requires approval before proceeding
- **On failure**: After 3 failed attempts per step
- **On ambiguity**: When requirements are unclear
- **On fundamental issues**: Type system conflicts, architectural mismatches

### Quality Gate Commands

```bash
cargo fmt                      # Format code
cargo clippy -- -D warnings    # Lint (no warnings allowed)
cargo test                     # All tests must pass
```

### Plan File State Machine

The plan file (`.claude/plans/feature-{slug}.md`) tracks:
- Step descriptions and status (pending, in_progress, passed, failed, skipped)
- Attempt count per step (max 3)
- Commit hashes for passed steps
- Quality gate history with failure feedback

### Invoking the Workflow

Use the `/build-feature` skill to start the workflow:

```
/build-feature Add support for do-while loops
```

Or invoke manually:
```
User: I want to add do-while loops to Neon
Claude: [Spawns orchestrator agent via Task tool]
```

The workflow will:
1. Ask for your approval on the plan before writing any code
2. Implement each step one-by-one with quality validation
3. Commit after each passed step
4. Create a PR when all steps are complete
