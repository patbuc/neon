# Claude Code Instructions

This file contains custom instructions for Claude Code when working on this project.

## About Neon

Neon is a dynamically-typed programming language implemented in Rust.

**Architecture**:
- `src/compiler/` - Lexer, parser, AST, semantic analysis, code generation
- `src/vm/` - Bytecode interpreter, value stack, call frames, instruction execution
- `src/common/` - Shared types, opcodes, object model, bytecode format
- `tests/` - Integration tests using embedded Neon scripts

**Implementation Flow**:
When implementing language features, changes typically flow through:
1. **Lexer/Scanner** - Tokenize new syntax
2. **Parser** - Build AST nodes
3. **Semantic Analysis** - Validate semantics (if needed)
4. **Code Generation** - Emit bytecode opcodes
5. **VM** - Execute instructions

**Key Patterns**:
- Standard library uses macro-based argument extraction (see `src/vm/stdlib.rs`)
- VM operations use a value stack for operands
- Objects are heap-allocated with reference counting
- Tests use embedded Neon syntax with `#[test]` attributes

## Rust Code Guidelines

**Error Handling**:
- NEVER use `unwrap()` or `expect()` in production code paths (compiler, VM, stdlib)
- Use `Result<T, E>` for recoverable errors
- Use proper error types from `src/common/error.rs`
- Panics are acceptable only in test code or truly unreachable branches

**Testing Requirements**:
- All new language features MUST have integration tests in `tests/`
- VM opcodes need both unit tests (if applicable) and integration tests
- Standard library functions need test coverage
- Run `cargo test` before committing - all tests must pass
- Run `cargo clippy` and address warnings

**Performance Considerations**:
- The VM is performance-critical - avoid unnecessary allocations in hot paths
- Be mindful of clone() operations on heap objects
- Consider using references and borrowing where possible
- Profile performance-sensitive changes if uncertain

**Code Style**:
- Follow standard Rust conventions (rustfmt)
- Use descriptive variable names
- Keep functions focused and modular
- Document complex algorithms with comments explaining WHY, not WHAT

## Git Commit Messages

**IMPORTANT**: Never add watermarks, signatures, or co-authorship attributions to commit messages.

When creating commits:
- Write clear, concise commit messages following conventional commit format
- DO NOT add "Generated with Claude Code" footers
- DO NOT add "Co-Authored-By: Claude" trailers
- DO NOT list changed files or obvious details visible in `git diff`
- Focus on the intent and high-level summary of WHY, not WHAT
- Keep commit messages professional and focused on the change itself

**Commit Types**:
- `feat:` - New feature or language capability
- `fix:` - Bug fix
- `refactor:` - Code restructuring without behavior change
- `perf:` - Performance improvement
- `test:` - Adding or updating tests
- `docs:` - Documentation only
- `chore:` - Build, tooling, dependencies

**Scope Examples**: `feat(parser):`, `fix(vm):`, `refactor(stdlib):`

Example of correct commit message:
```
feat: Add array support to parser

Implement array literal parsing and validation
```

Example of incorrect commit messages (DO NOT DO THIS):
```
feat: Add array support to parser

Implement array literal parsing and validation

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

```
feat: Add array support to parser

Changes:
- Modified parser.rs to add array parsing
- Updated ast.rs with ArrayLiteral node
- Added tests in parser_tests.rs
```

## Contribution Workflow

**Before Implementation**:
- For complex features, consider using `/plan-feature` to create a detailed plan first
- For architectural changes or multiple implementation approaches, ask questions before coding
- Understand existing patterns by reading similar implementations in the codebase

**During Implementation**:
- Keep commits atomic and focused on a single change
- Run tests frequently: `cargo test`
- Check for warnings: `cargo clippy`
- Ensure code compiles at each step

**Before Committing**:
- All tests must pass (`cargo test`)
- No clippy warnings unless explicitly justified
- Code follows Rust conventions
- Commit message follows guidelines above

## Claude Code Orchestration

This project has a sophisticated multi-agent orchestration system in `.claude/`:

**Available Commands**:
- `/build-feature "description"` - Fully automated feature development (planning → implementation → testing → PR)
- `/plan-feature "description"` - Create detailed implementation plan
- `/implement-task N` - Implement specific task from plan
- `/run-tests` - Execute test suite with analysis
- `/create-pr` - Create GitHub pull request
- `/review-pr` - Automated code review

**When to Use**:
- Use `/plan-feature` for complex features before implementation
- Use `/build-feature` for end-to-end automation
- Manual workflow gives more control: plan → implement → test → PR

See `.claude/ORCHESTRATION.md` for complete documentation.

## Summary

**Code Quality**:
- ✅ No `unwrap()` in production code
- ✅ All features have tests
- ✅ Code passes `cargo test` and `cargo clippy`
- ✅ Follow existing patterns in codebase

**Commit Messages**:
- ✅ Clean, professional messages focused on intent
- ✅ Use conventional commit format with types
- ❌ No watermarks or attribution footers
- ❌ No file lists or obvious details from git diff
