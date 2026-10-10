# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Neon is a dynamically-typed, bytecode-compiled language with a stack-based VM, written in Rust. The implementation
follows a traditional compiler pipeline: Scanner → Parser → Module Graph → Semantic Analysis → Code Generation → VM Execution.

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

Integration tests use `datatest-stable` harness and run all `.n` scripts in `tests/scripts/` and `benches/`. Each
script must include inline expected output:

```neon
// Expected:
// output line 1
// output line 2
```

A script whose execution ends in a runtime error also needs a `// Expected runtime error: <message>`
line, matched exactly against the error message; it still needs an `// Expected:` block for any output
printed before the error.

A script that fails to compile lives in `tests/compile_errors/` and carries one
`// Expected compile error: <line>:<col> <code> <message>` line per expected error, in reported order (e.g.
`// Expected compile error: 1:7 E0013 Undefined variable 'x'`). It has no `// Expected:` block and no runtime-error
line. The lines are matched exactly against the stored errors from `VirtualMachine::get_compile_errors`, not the
text the CLI renders, and carry no file path. The script need not parse and is not formatted, but must carry at
least one such line. Scripts under `tests/compile_errors_must_fail/` must fail, each naming its expected failure
in a `// Must fail with: <text>` line that the failure must contain, which tests that the harness rejects a mismatch.

Multi-file module cases live in `tests/modules/<case>/main.n`. The harness runs each `main.n` with its own path as
the entry script, both as written and after formatting; the modules it imports sit beside or below it in the case
directory. It also runs `examples/modules/main.n` the same way. Cases under `tests/modules_must_fail/<case>/main.n`
must fail, each naming its expected failure in a `// Must fail with: <text>` line that the failure must contain,
which tests that the harness rejects a mismatch. Module cases
use the same directive format and stay formatted.

### Benchmarks

Neon vs. Python benchmarks live in `benches/` as `<name>.n` / `<name>.py` pairs implementing the same algorithm.

```bash
cargo build --release
python3 benches/run.py               # all benchmarks
python3 benches/run.py fib strings   # name filter
python3 benches/run.py --runs 10     # timed runs per benchmark (default 5)
```

Prints a Markdown table (mean, stddev, min, median per language, and the Neon/Python ratio) and writes
`bench-results.json` in `benchmark-action/github-action-benchmark`'s `customSmallerIsBetter` format. Exits non-zero
naming the benchmark if the Neon and Python checksums differ.

It also prints `CPU: <model>` above the table (and in the GitHub step summary) and sets `"extra": "CPU: <model>"` on
every JSON entry, so the chart tooltip names the runner's CPU. Shared runners vary in hardware, so compare points
from the same CPU. The model is the first `model name` line of `/proc/cpuinfo`, else `platform.processor()`; the run
exits 1 if neither gives one. Test `run.py` from the repo root with `python3 -m unittest benches.test_run` (CI
doesn't run these).

To add a pair: write `benches/<name>.n` and `benches/<name>.py` implementing the same algorithm like for like (the
Python side uses plain loops/classes, not numpy), then add `"<name>": SIZE` to `BENCHMARKS` in `benches/run.py`.
Both scripts read the problem size from the first argument with a small inline default, e.g.
`args.size() > 0 ? args[0].toInt() : 15` in Neon and `int(sys.argv[1]) if len(sys.argv) > 1 else 15` in Python, and
each prints one integer checksum line. Keep the inline default small — `cargo test` runs the `.n` file at that
default in a debug build, and it must stay fast — and give it a `// Expected:` block at that default size. Pick the
`SIZE` registered in `BENCHMARKS` separately, so a release build of Neon takes roughly 200-1000 ms at that size.

`.github/workflows/bench.yml` runs the suite on every push to `main` (and via `workflow_dispatch`), publishing charts
to `https://patbuc.github.io/neon/dev/bench/`. It never fails the build on a regression — shared runners are too
noisy for thresholds.

### Profiling

Build a release binary with line tables first: `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only cargo build --release`.
It compiles to the same code as a plain release build. Full debug info (`=true`) changes code generation — fib runs
~6% more instructions and ~20% slower — so its profile describes a different program than the benchmarks run.

```bash
perf record -g --call-graph dwarf ./target/release/neon benches/fib.n 31
perf report
```

`perf record` needs `kernel.perf_event_paranoid` at 1 or below (e.g. `sudo sysctl kernel.perf_event_paranoid=1`).
Where perf isn't available or a deterministic instruction-count profile is preferred, use callgrind instead (no
elevated privileges required):

```bash
valgrind --tool=callgrind ./target/release/neon benches/fib.n 22
callgrind_annotate --auto=yes callgrind.out.<pid>
```

Most VM code inlines into `VirtualMachine::run_script`, so read costs per source line in the annotated output
rather than per function. For per-instruction costs, add `--dump-instr=yes`; that works on a plain release build
too.

For a coarser view, the `opcode-stats` feature counts executed instructions:

```bash
cargo run --release --features opcode-stats -- benches/fib.n
```

Prints one line per executed instruction with its count to stderr after the script finishes, sorted by count
descending (ties by name); stdout is unchanged. A second section follows after a blank line, one `Prev->Next <count>`
line per executed instruction pair (consecutive instructions across calls, returns, and native callbacks), sorted the
same way. The Features workflow (`.github/workflows/features.yml`) runs clippy and tests with this feature on every pull request,
every push to `main`, and on demand.

### Versioning

The version the CLI prints is `<major>.<minor>.<merges>`. `build.rs` takes major and minor from `Cargo.toml` and
counts the merge commits on `HEAD`'s first-parent history (`git rev-list --count --merges --first-parent HEAD`), so
every PR merged into `main` bumps the patch. It is exposed as `env!("NEON_VERSION")`; the patch in `Cargo.toml` is
ignored. Bump the minor by hand in `Cargo.toml`. Without git the build warns and uses `Cargo.toml`'s version.

### Running Scripts

```bash
cargo run -- script.n           # Interpret a Neon script
cargo run -- script.n arg1 arg2 # Pass arguments to script
cargo run -- -e 'print(1 + 2)'  # Run a snippet (also --eval); file imports are an error
cargo run -- - < script.n       # Run a script read from stdin; file imports are an error
cargo run -- --check script.n   # Compile without running
cargo run -- fmt path...         # Format .n files in place, recursing into directories
cargo run -- fmt --check path... # Print files that would change, exit 1 if any
cargo run -- fmt -               # Format stdin, write to stdout
cargo run -- --version          # Print the version (also -V)
cargo run                       # Start REPL
```

The REPL drives `VirtualMachine::interpret_line`/`interpret_line_in` (the latter takes the
directory imports resolve from; the CLI passes the current directory) and `Compiler::compile_entry`,
which persist globals and methods across lines via a `GlobalEnv` carried line to line, instead of
resetting per line like `interpret`/`interpret_file` does for a file. A line that imports a new module runs
that module's chunk first; if the module or the line fails, the line's globals are rolled back.

### WebAssembly

```bash
./build-wasm.sh         # Build for web (requires wasm-pack)
# Output: wasm-pkg/{neon.js, neon_bg.wasm, neon.d.ts}
```

Exports `NeonVM`, `interpret_once(source)`, and `format_source(source)`, each returning a `WasmResult`-shaped
value (`{ success, output, error }`). Test the wasm bindings themselves (`tests/wasm.rs`, gated
`#![cfg(target_arch = "wasm32")]`) with `wasm-pack test --node`.

### Debugging

```bash
cargo build --features disassemble  # Enable bytecode disassembly
cargo run --features disassemble -- script.n
```

## Architecture

### Layers

`src/`'s top-level modules are layers with one-way allowed edges: `compiler → common`, `vm → common`,
`vm → compiler`. `main.rs`, `lib.rs`, `wasm.rs`, and `macros.rs` sit outside the layers and are
unrestricted; `tests/` directories and `#[cfg(test)]` items are exempt. `tests/architecture.rs`
enforces these edges in `cargo test`.

### Compilation Pipeline

1. **Scanner** (`src/compiler/scanner.rs`)
    - Lexical analysis producing tokens
    - Handles keywords, operators, literals, identifiers
    - Tracks line/column for error reporting

2. **Parser** (`src/compiler/parser.rs`)
    - Builds AST from tokens (defined in `src/compiler/ast/`)
    - Recursive descent parser
    - AST nodes: expressions, statements, declarations

3. **Module Graph** (`src/compiler/module_graph.rs`)
    - `ModuleGraph::build` loads the import graph from an `EntryLocation`: the entry file (run,
      `--check`), a directory (REPL), or none (in-process `interpret` and wasm, where a file import is
      an error)
    - Modules are identified by canonical path, parsed once, and returned in dependency order (the
      entry module last); import cycles and unknown or unreadable modules are compile errors. Errors in
      imported modules carry that module's path and are rendered from the sources the graph loaded
      (`Compiler::module_sources`); errors in the entry module carry no file
    - `std/` paths name builtin modules; one not in `method_registry::builtin_modules()` is an
      `UnknownModule` error listing the known ones
    - Every module in the graph compiles as its own unit, in graph order, each against a fresh
      `GlobalEnv` that shares the compile's `Symbols`, `next_decl_id` and `slot_count`. A file import
      with no entry location (in-process `interpret`, wasm) is rejected by the graph (E0056). A builtin
      module has no chunk: its `ExportTable` is built from its registry rows (`ExportTable::builtin`)
      and bound to the path's file stem (`std/pq` → `pq`)
    - The REPL's `GlobalEnv` remembers each compiled module's `ExportTable` by canonical path, so a
      module imported again on a later line, directly or by a new module, is not compiled again and
      its bindings resolve to the original slots

4. **Semantic Analysis** (`src/compiler/semantic.rs`)
    - Type checking and validation
    - Resolves every name once, using scoped symbol tables (`src/compiler/symbol_table.rs`) for
      lexical scoping — including which calls dispatch to a native
    - Returns `Resolutions` (`src/compiler/resolutions.rs`): a `Res` per name-use node (keyed by the
      parser-assigned `NodeId`), native-call entries, declarations, per-function params and upvalue
      captures, which declarations are captured, and a `Symbols` table interning every field, method,
      and type name into a `u16` id (more than 65,536 distinct names is a compile error)
    - Owns these diagnostics: undefined variable, break/continue outside a loop, postfix operand,
      unknown module export, write to an export, module used as a value, wrong-arity export call
    - Each module's exports go into an `ExportTable` (`src/compiler/exports.rs`): name, kind, global
      slot and arity for functions, variables, structs and enums, registry index and arity for a builtin
      module's natives (`Export::Native`, no slot). A `use` binds a compile-time `Module` symbol
      with no slot; `utils.name` resolves against the imported module's table. Enums are compile-time
      only exports and have no slot

5. **Code Generation** (`src/compiler/codegen.rs`)
    - Traverses AST and emits bytecode, consuming `&Resolutions` — it never looks up a name by string,
      and maps each `DeclId` to a stack slot when it defines the local
    - Member access on a module becomes `GetGlobal` of the export's slot (plus `Call`/`TailCall` for a
      call); a call of a builtin module's native emits the same native call as `print(x)`; a `use`
      itself emits nothing. Each module's chunk is named after its module path
    - Produces Chunk objects containing instructions and constant pool
    - Compile-time state (locals, scope depth, loop contexts) lives in the per-function
      `FunctionCompiler`, not in the Chunk; upvalue captures come from `Resolutions`
      (`FunctionResolution.upvalues`)

### Runtime Architecture

**VM Core** (`src/vm/impl.rs`)

- Stack-based bytecode interpreter
- Main execution loop dispatches on `Chunk.code`, the chunk's bytes decoded once into `Instr` values
  (`src/common/chunk/decode.rs`) when it becomes immutable; `ip` is an index into it
- Call frame stack for function calls (`src/vm/functions.rs`)
- The running frame's `ip` and chunk live in `VirtualMachine.ip`/`chunk`; `CallFrame.ip` is only current for the
  frames below the top (`push_frame`/`pop_frame` save and restore it)
- `TailCall`/`TailInvoke` reuse the running frame for a call in tail position (codegen emits them there), so tail
  recursion isn't bounded by `MAX_FRAMES` (except in a `try` body, where codegen emits plain calls); the replaced frame vanishes from runtime-error traces
- Separate builtin values storage (e.g., `args`)
- Runs a program's module chunks in dependency order, then the entry as the script frame; globals live in one
  shared area, so a module's exports are plain globals

**Bytecode Format** (`src/common/chunk/`)

- Chunk: name, bytecode instructions, constant pool, a line table of `LineInfo` entries, the
  symbol table shared by every chunk of the compile, and `file`, the unit's source file when known
- `Chunk::decode` fills `code`, `instr_lines` (one location per instruction, read by `instr_line_info`),
  `closure_upvalues`, and `fused_field_lines`; jump targets become instruction indices. Decode also fuses adjacent
  instructions into one `Instr` (`GetLocal`+`GetField` → `GetLocalField`, `SetLocal`+`Pop` → `StoreLocal`,
  `SetField`+`Pop` → `StoreField`, `GetLocal`+`Add`/`Subtract`/`Multiply`/`Divide` → `*Local`, `GetLocalField`+operator
  → `*LocalField`, `Constant`+`Add`/`Subtract`/`Multiply`/`Modulo`/`Greater`/`GreaterEqual`/`Less`/`LessEqual` →
  `*Constant` when the pool entry is a Number or Int, a comparison followed by `PopJumpIfFalse` →
  `<Cmp>JumpIfFalse`, and a `<Cmp>Constant` followed by it → `<Cmp>ConstantJumpIfFalse`, and that after a `GetLocal` of slot ≤ 255 →
  `Local<Cmp>ConstantJumpIfFalse`), never across a jump target,
  keeping the failing part's location, so `code` isn't one-to-one with the bytecode. The pass cascades: a fused result
  is tried again with the instruction before it, so `GetLocal` + number `Constant` +
  `Add`/`Subtract`/`Multiply`/`Modulo` becomes `GetLocal<Op>Constant`, and that followed by a `StoreLocal` of the same
  slot becomes `IncrementLocal`; no part of a fusion may be a jump target. A `*LocalField` keeps the operator's
  location in `instr_lines` and the field read's in `fused_field_lines`, which it uses when the field read fails
- Constants pool stores literals referenced by index
- `LineInfo { ip, line, column }` maps instruction offsets to source line/column for error reporting;
  runtime errors and call-trace frames also name `Chunk.file` when it is set (errors in REPL lines and in-process
  runs keep the old format, but a runtime error inside an imported module prints that module's file, REPL
  included)

**Opcodes** (`src/common/opcodes.rs`)

- Instruction set definition as `#[repr(u8)]` enum
- Stack manipulation, arithmetic, control flow, function calls (`Call`/`Invoke` and their tail variants
  `TailCall`/`TailInvoke`)
- Index operands (constants, locals, globals, upvalues, builtins, and symbol ids for field/method/type
  names) are a fixed 16 bits; jump/loop offsets are 32 bits

**Value System** (`src/common/mod.rs`)

- Scalars (Number, Int, Boolean, Nil) are stored inline
- Every heap variant (String, Function, Closure, NativeFunction, Struct, Instance, Array, Map, Set, File, Range, EnumVariant) holds a single `Rc`; collections and instances use `Rc<RefCell<..>>` for interior mutability
- Strings are `Rc<String>` so `Value` stays 16 bytes
- `Uninitialized` marks a hoisted global/block-level slot before its declaration runs
- Range is an immutable `Rc<ObjRange>` of integer bounds; for-in iterates it without allocating an array
- Generator is an `Rc<ObjGenerator>`: a function whose own body has `yield` (`ObjFunction.is_generator`) returns one
  from the call instead of running. `next()` (`src/vm/functions.rs`) resumes it by pushing its saved frame onto the
  frame stack, with no Rust recursion; the `Yield` opcode saves the frame, its stack slice, open upvalues and `try`
  handlers back into the generator and returns the value to the `next()` caller. States are NotStarted, Suspended,
  Running, Done; `next()` on the last two is a runtime error. The VM tracks the generators it is running, with their
  frame depth, in its `running_generators` stack, not on `CallFrame`

**Standard Library** (`src/common/stdlib/`)

- Native functions for built-in types
- Builtin `std/` modules (`std/math`, `std/file`, `std/stdin`, `std/pq`), registered as `std/<name>` rows in the
  method registry
- String/Array/Map/Set/Range methods via method registry
- Method registry (`src/common/method_registry.rs`) maps type+method to function index
- Runtime `Invoke` dispatch of native methods goes through the per-compile `NativeMethodTable` held on the VM,
  indexed by method symbol and builtin type symbol, not through name lookups
- User methods from `impl` blocks live on the struct: `ObjStruct.methods`, which `DefineMethod` appends to after
  loading the struct value. `impl` on a builtin type goes through `DefineBuiltinMethod` into the VM's
  `builtin_methods` table, indexed by builtin type symbol, and is program-wide. The semantic analyzer and
  `GlobalEnv` key struct methods by declaration (`DeclId`) and keep builtin-type methods in a separate map
  keyed by type name. The VM journals the structs `DefineMethod` touched during a REPL line so a runtime
  error can roll those methods back.

### Key Type Interactions

- **CallFrame**: Links function object to a saved instruction pointer and stack slot range
- **Locals**: Tracked per-function in the code generator's `FunctionCompiler` — scope depth and capture
  status for closures
- **For-in State**: Each for-in loop keeps its collection and index in two hidden locals, which
  `IteratorDone`/`IteratorNext` address by slot
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
8. A new keyword also goes in `KEYWORDS` (`src/compiler/scanner.rs`) and in
   `editors/vscode/syntaxes/neon.tmLanguage.json` — a test enforces they match — and in
   `KEYWORD_FAMILY` (`editors/vscode/tests/differential.js`), which `npm test` enforces

### New Standard Library Function

1. Implement function in appropriate `src/common/stdlib/*_functions.rs` file
2. Register in method registry if it's a method (see `src/common/method_registry.rs`) — a new builtin module (like
   `std/math`) is a set of `std/<name>` rows, picked up automatically from there; a new runtime builtin value
   (like `args`) is declared in `BUILTIN_VALUES` (`src/common/stdlib/mod.rs`) and constructed in `create_builtin_objects`
3. For global functions, add to builtin initialization in VM
4. Add tests in corresponding `src/common/stdlib/tests/` file

## Code Conventions

### Rust Patterns

- Use `Result<T, E>` for error propagation; `unwrap()`/`expect()` outside tests are denied by
  `clippy::unwrap_used`/`clippy::expect_used` — a kept `expect` states the invariant and carries an
  item-level `#[allow(clippy::expect_used)]`
- Pattern matching for AST traversal and opcode dispatch
- Minimize allocations in VM hot path (execution loop)
- Use `Rc` for shared ownership, `RefCell` only when mutation needed
- Prefer `log` macros (`info!`, `debug!`) for debug logging, not `println!` — enable with `RUST_LOG`

### Compiler/VM Patterns

- **Stack invariants**: Document expected stack state before/after operations in comments
- **Error reporting**: Always include source location (line/column) from tokens
- **Symbol tables**: Maintain proper lexical scope depth
- **Name resolution**: Names shadow lexically — a local or user function named like a native (`print`,
  `Array`) wins; the method registry is consulted only when a name resolves to nothing else
- **Bytecode emission**: Append-only except for jump address backpatching
- **Opcode design**: Keep instruction set minimal and orthogonal

### Testing

- Unit tests in module files or submodule `tests/` directories
- Integration tests in `tests/scripts/` use inline expected output format
- Before writing or editing `.n` files, load the `writing-neon` skill (`.claude/skills/writing-neon/`): where Neon
  syntax differs from JS/Kotlin and the full list of native methods
- A PostToolUse hook (`.claude/hooks/check-neon.sh`) runs `--check` on any `.n` file after it's edited or
  written, feeding compile errors back automatically; for files under `tests/scripts/`, `tests/modules/`,
  `tests/modules_must_fail/`, `benches/`, and `examples/` it then runs `neon fmt --check` and blocks with feedback to run
  `cargo run -- fmt <file>` if it's unformatted. A module case expecting a compile error (the line in its case's
  `main.n` under `tests/modules/` or `tests/modules_must_fail/`) skips `--check` but is still format-checked; files
  under `tests/compile_errors/` and `tests/compile_errors_must_fail/` skip both
- A PostToolUse hook (`.claude/hooks/check-arch.sh`) runs `cargo test --test architecture` after any
  `src/*.rs` file is edited or written, blocking with the test's layer-violation output if it fails
- New or edited `.n` files under `tests/scripts/`, `tests/modules/`, `tests/modules_must_fail/`, `benches/`, and
  `examples/` must pass `neon fmt --check`; the Lint CI workflow runs the same check over those directories
- Test both success and error paths
- Include edge cases (empty input, stack overflow, division by zero, etc.)

### Error Handling

- Compilation errors use `src/common/error_renderer.rs` for formatted output
- Runtime errors should include context about what operation failed
- VM returns `InterpretResult` enum: Ok, CompileError, RuntimeError

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
cargo fmt                                    # Format code
cargo clippy --all-targets -- -D warnings    # Lint (no warnings allowed)
cargo test                                   # All tests must pass
```

`print_stdout`/`print_stderr`/`unwrap_used`/`expect_used` are denied via `[lints.clippy]` in `Cargo.toml`;
test code is exempt through `allow-print-in-tests`/`allow-unwrap-in-tests`/`allow-expect-in-tests` in
`clippy.toml`, and the few legitimate production sites (the `print` native, the disassembler dump, CLI
output in `main.rs`; stack-underflow and similar invariants in the VM, compiler, and wasm bindings) carry
an item-level `#[allow(clippy::print_stdout)]`/`#[allow(clippy::print_stderr)]`/`#[allow(clippy::expect_used)]`
on the smallest enclosing fn.

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
