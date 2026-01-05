---
description: "Review code changes for quality, correctness, and conventions"
allowed-tools: ["read", "execute"]
---

# Code Review Guidelines

Perform a thorough code review of current changes.

## Your Task

Review changes for: **$ARGUMENTS**

Default: Review all current changes (`git diff`)

## Workflow

### 1. Load Changes

```bash
# Show unstaged changes
git diff

# Show staged changes
git diff --cached

# Show specific file
git diff <file>

# Show commit
git show <commit-hash>
```

### 2. Review Checklist

#### Correctness

**Logic & Algorithms:**
- [ ] Implements the intended behavior correctly
- [ ] Handles all edge cases (empty input, null, boundary values)
- [ ] No off-by-one errors in loops or array indexing
- [ ] Recursion has proper base cases
- [ ] No infinite loops or stack overflows

**Error Handling:**
- [ ] All `Result` types are properly handled (no `unwrap()` outside tests)
- [ ] Error messages are clear and actionable
- [ ] Error messages include source location (line/column)
- [ ] Appropriate error types used (CompileError vs RuntimeError)

**Type Safety:**
- [ ] Pattern matching is exhaustive
- [ ] No unsafe type casts without validation
- [ ] Value types handled correctly (Number, Boolean, Nil, Object)

#### Neon-Specific Concerns

**Compiler Pipeline:**
- [ ] Scanner: Token types correct, line/column tracking works
- [ ] Parser: AST structure matches grammar, precedence correct
- [ ] Semantic: Symbol table scoping is correct, types validated
- [ ] Codegen: Bytecode emission is correct, constant pool managed

**VM Execution:**
- [ ] **Stack invariants maintained** (document before/after state)
- [ ] No stack underflow/overflow
- [ ] Call frames properly managed
- [ ] Instruction pointer updates correct
- [ ] Locals tracked with correct scope depth

**Bytecode:**
- [ ] Opcode arguments have correct width (8/16/32-bit)
- [ ] Jump addresses properly backpatched
- [ ] Constants correctly indexed
- [ ] String table properly populated

**Memory Management:**
- [ ] `Rc<Object>` used for shared ownership
- [ ] `RefCell` only used when mutation needed
- [ ] No memory leaks (circular references)
- [ ] Proper cleanup in Drop implementations

**Object System:**
- [ ] Object types handled correctly (String, Function, Array, etc.)
- [ ] Method dispatch works for all types
- [ ] Iterator stack properly managed for nested loops

#### Performance

**VM Hot Path:**
- [ ] Minimal allocations in execution loop
- [ ] String interning used appropriately
- [ ] No unnecessary cloning
- [ ] Efficient data structures chosen

**Compilation:**
- [ ] Constant folding where appropriate
- [ ] Dead code elimination considered
- [ ] No redundant passes over AST

**Trade-offs:**
- [ ] Educational clarity prioritized over micro-optimizations
- [ ] Avoid premature optimization

#### Code Quality

**Rust Idioms:**
- [ ] Follows Rust naming conventions (snake_case, CamelCase)
- [ ] Uses `Result<T, E>` for error propagation
- [ ] Pattern matching preferred over conditionals
- [ ] Iterator methods used where appropriate
- [ ] No `unwrap()` or `expect()` except in tests

**Neon Conventions:**
- [ ] Follows patterns in CLAUDE.md
- [ ] Matches style of surrounding code
- [ ] Comments explain "why" not "what"
- [ ] Complex algorithms have explanatory comments
- [ ] Stack operations documented

**Code Clarity:**
- [ ] Variable names are descriptive
- [ ] Functions are focused (single responsibility)
- [ ] No overly complex conditionals (extract to functions)
- [ ] No commented-out code
- [ ] No debug `println!` or `dbg!` statements (use `tracing` crate)

**Modularity:**
- [ ] Appropriate abstraction level
- [ ] No premature abstraction (don't over-engineer)
- [ ] Clear module boundaries
- [ ] Public API is minimal and clear

#### Testing

**Coverage:**
- [ ] Integration tests for language features (`tests/scripts/*.n`)
- [ ] Unit tests for internal functions
- [ ] Tests cover success paths
- [ ] Tests cover error paths
- [ ] Edge cases tested (empty, null, boundary values)

**Test Quality:**
- [ ] Expected output format correct (inline comments)
- [ ] Test names are descriptive
- [ ] Tests are isolated (no interdependencies)
- [ ] Test assertions are clear

**Integration Test Format:**
```neon
// Test: [Description of what this tests]
// Expected:
// [expected output line 1]
// [expected output line 2]

[test code]
```

#### Documentation

**Comments:**
- [ ] Public APIs have doc comments
- [ ] Complex algorithms explained
- [ ] Non-obvious behavior documented
- [ ] Stack state changes documented
- [ ] Source location tracking explained

**ADR Alignment:**
- [ ] Implementation matches ADR decision
- [ ] No undocumented architectural changes
- [ ] Alternatives from ADR were considered

### 3. Category-Specific Checks

#### For Scanner Changes
- [ ] All keywords added to keyword map
- [ ] Token types have proper precedence
- [ ] Line/column tracking correct
- [ ] Multi-character operators handled

#### For Parser Changes
- [ ] Grammar rules implemented correctly
- [ ] Operator precedence correct
- [ ] Error recovery doesn't skip valid code
- [ ] AST nodes properly constructed

#### For Semantic Changes
- [ ] Symbol resolution works across scopes
- [ ] Type checking is sound
- [ ] Undefined variable detection works
- [ ] Redeclaration detection works

#### For Codegen Changes
- [ ] Bytecode matches intended semantics
- [ ] Constant pool correctly managed
- [ ] Jump addresses calculated correctly
- [ ] Local variable slots assigned correctly

#### For VM Changes
- [ ] Opcode dispatch is complete
- [ ] Stack operations are correct
- [ ] Function calls work (including natives)
- [ ] Error messages include context

#### For Stdlib Changes
- [ ] Native functions registered correctly
- [ ] Method dispatch works for all types
- [ ] Argument validation is thorough
- [ ] Return values are correct types

### 4. Classify Issues

**Blocking (must fix before commit):**
- Correctness bugs
- Test failures
- Memory safety issues
- Stack invariant violations
- Convention violations (clippy warnings)

**Non-blocking (create issue for later):**
- Performance optimizations
- Refactoring opportunities
- Documentation improvements
- Nice-to-have features

### 5. Auto-Fix

**For blocking issues:**
- Fix immediately, don't just report
- Run tests after fixing
- Iterate until all issues resolved

**For non-blocking issues:**
- Create Beads issue for tracking
- Document the improvement opportunity
- Continue with shipping

### 6. Output

**Summary format:**
```
Code Review: <description>

✓ Correctness: [PASS/FAIL]
  - Logic: correct
  - Error handling: proper
  - Edge cases: covered

✓ Neon-specific: [PASS/FAIL]
  - Stack invariants: maintained
  - Bytecode: correct
  - Memory: no leaks

✓ Performance: [PASS/FAIL]
  - Hot path: optimized
  - Allocations: minimal

✓ Quality: [PASS/FAIL]
  - Rust idioms: followed
  - Conventions: respected
  - Clarity: good

✓ Testing: [PASS/FAIL]
  - Coverage: adequate
  - Integration tests: added
  - Edge cases: tested

✓ Documentation: [PASS/FAIL]
  - Comments: clear
  - ADR alignment: yes

Blocking issues: 0
Non-blocking issues: 2 (created bd-43, bd-44)

Recommendation: APPROVE for shipping
```

## When to Use

**Before committing:**
- Run as part of `/ship` workflow
- Catches issues before they enter main branch

**During PR review:**
- Review PR changes
- Provide structured feedback

**After implementing:**
- Self-review before asking for human review
- Ensures quality standards met
