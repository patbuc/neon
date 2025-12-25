# AGENTS.md

This file defines conventions and commands for agentic coding agents working in this repository. It covers build/test/lint usage, guidance for running a single test, and code style conventions for the Neon compiler/VM codebase.

## Project Overview
- Language: Rust (2021 edition)
- Purpose: Educational compiler + VM (not production)
- Architecture
  - Compiler: `src/compiler/{scanner.rs, parser.rs, ast/, semantic.rs, codegen.rs, symbol_table.rs}`
  - VM: `src/vm/{impl.rs, functions.rs}`, opcodes in `src/common/opcodes.rs`
  - Common: shared types, errors, bytecode (“bloq”) format under `src/common/`
  - Tests: unit tests in `src/*/tests/` and integration tests in `tests/` (Neon scripts)
- WASM demo: `build-wasm.sh` and `wasm-demo/`

## Build, Run, and Test
- Build (native):
  - `cargo build`
  - Verbose: `cargo build --verbose`
- Run Neon script:
  - `cargo run -- <script.neon>`
  - Examples: `cargo run -- examples/test_math.neon`
- Test (full suite):
  - `cargo test`
  - Verbose: `cargo test --verbose`
- Clippy (lint):
  - `cargo clippy --all-targets --all-features`
  - Optional fix (ask first): `cargo clippy --all-targets --all-features --fix -Z unstable-options`
- Format:
  - Check only: `cargo fmt -- --check`
  - Apply formatting: `cargo fmt`
- Disassembler (feature flag):
  - Add `--features disassemble` to build/test/run when you need bytecode disassembly
- WASM:
  - `./build-wasm.sh`

### Running a Single Test
- Unit tests in Rust modules:
  - Run by module path: `cargo test -p neon <module_or_test_name>`
  - Examples:
    - `cargo test compiler::tests::parser::test_parse_array_with_numbers`
    - `cargo test common::stdlib::tests::array_functions::test_array_slice`
  - Run by name pattern: `cargo test parser -- --exact`
- Integration tests (Neon scripts):
  - `cargo test --test neon_scripts -- --ignored` for selective cases if marked
  - Use `-- --nocapture` to see printed output
- Filtering:
  - `cargo test <substring>` filters tests by substring across the suite
  - `cargo test <name> -- --exact` matches exact test names

### CI Reference
- GitHub Actions workflow `.github/workflows/rust.yml` runs:
  - `cargo build --verbose`
  - `cargo test --verbose`

## Copilot Instructions Summary
See `.github/copilot-instructions.md` for full details. Key points agents must respect:
- Avoid `unwrap()` and prefer explicit error types; use `Result<T, E>` and proper propagation
- VM hot path: minimize allocations and clones; leverage borrowing
- Maintain stack invariants in the interpreter; document where relevant
- Use orthogonal, minimal opcode additions; update disassembler
- All compiler stages and VM changes require tests (unit + integration)
- Use `tracing` for runtime debugging, not `println!`
- End-to-end tests live in `tests/` and should cover error paths

## Code Style and Conventions

### Formatting and Imports
- Use `rustfmt` defaults; run `cargo fmt` (or `--check` in validation)
- Group imports logically, prefer explicit paths over glob imports (`use crate::module::Type`)
- Order: std → external crates → crate modules; avoid unused imports (clippy will warn)
- Prefer `use super::...` in tests to access parent module where appropriate

### Types and Ownership
- Use descriptive, explicit types; avoid `impl Trait` in public interfaces unless necessary
- Prefer borrowing (`&T`, `&mut T`) to avoid clones, especially in VM and codegen
- Avoid unnecessary `clone()`; if cloning is needed, document rationale
- Use `Option` and `Result` for control flow over sentinel values
- Consider `SmallVec` for small, frequent collections (see Copilot instructions)

### Error Handling
- Production paths (compiler, VM, stdlib):
  - NEVER use `unwrap()` or `expect()`
  - Return `Result<T, E>` using error types from `src/common/errors.rs` and render via `src/common/error_renderer.rs`
  - Panics allowed only in tests or truly unreachable branches with justification
- Provide actionable error messages including token/line/column context where available

### Naming Conventions
- Modules and files: `snake_case` (Rust default)
- Types and structs: `UpperCamelCase`
- Functions and methods: `snake_case`
- Constants and statics: `SCREAMING_SNAKE_CASE`
- Opcode names: uppercase mnemonic in `opcodes.rs`; keep instruction set minimal and orthogonal
- Test names: descriptive, reflect behavior (e.g., `test_array_slice_negative_indices`)

### Compiler/VM Patterns
- Compiler flow is explicit and staged:
  1. Scanner → 2. Parser → 3. Semantic → 4. Codegen
- Do not back-patch bytecode except for jump addresses; keep emission append-only semantics
- AST traversal and opcode handling should use exhaustive `match` where practical
- VM execution loop is stack-based; document stack effects for each opcode (inputs/outputs)
- Objects are ref-counted; be mindful of allocation patterns

### Testing Requirements
- New features must include:
  - Compiler unit tests in `src/compiler/tests/`
  - VM tests in `src/vm/tests/` where applicable
  - Integration tests under `tests/` using Neon scripts (add new `.n` or `.neon` files)
- Test both success and error paths; include edge cases (empty, out-of-range, malformed)
- Run locally: `cargo fmt -- --check`, `cargo clippy --all-targets --all-features`, `cargo test`

### Logging and Diagnostics
- Use `tracing` (e.g., `tracing::debug!`) for runtime diagnostics; avoid `println!` in production
- Prefer structured logs where useful; keep noise low in hot paths

### Performance Considerations
- VM and codegen are performance sensitive:
  - Avoid heap allocations in loops; pre-allocate `Vec` when sizes are known
  - Be cautious with string handling in the scanner; prefer slices
  - Profile if unsure; do not micro-optimize at the expense of clarity

### Contribution and Commits
- Keep commits atomic and focused; compile/tests should pass at each step
- Commit messages
  - Clean, professional, focused on intent (no `feat:`/`fix:` prefixes in the subject)
  - Do NOT add watermarks or AI attribution
  - Optional type in the body: Feature/Fix/Refactor/etc.
- Before committing: run `cargo test` and `cargo clippy`; ensure no warnings without justification

## Agent Workflow Notes
- Operate in a git worktree for feature development; avoid writing in the main repo path
- Prefer small, focused diffs aligned to existing patterns
- Super-agent usage (OpenCode):
  - Start with `build-feature` agent; it will ask clarifying questions once, then proceed autonomously
  - It computes `feature/<slug>` and initializes a worktree under `../neon-worktrees/feature/<slug>`
  - It coordinates `plan-feature`, `implement-task`, `run-tests`, `review-pr`, and `create-pr` subagents
  - It runs local Test/Review loops up to 3 times, then proceeds to PR and applies feedback
- Always pass the worktree path to tools (`tests`, `clippy`, `format`) in agent workflows
- Use `worktree_info`/`worktree_make` tools for standardized branch/worktree setup

## File and Directory Pointers
- Opcodes: `src/common/opcodes.rs`
- VM core: `src/vm/impl.rs`, `src/vm/functions.rs`
- Compiler stages: `src/compiler/*`
- Error rendering: `src/common/error_renderer.rs`
- Integration tests: `tests/`
- Examples: `examples/`

## Notes
- This repository includes Copilot instructions in `.github/copilot-instructions.md`; agents should follow their guidance.
- No Cursor rules present (no `.cursor/rules/` or `.cursorrules`). If added in the future, incorporate them here.
- Use `--features disassemble` when debugging bytecode and disassembly in `src/common/disassembler.rs`.
