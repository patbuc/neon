---
description: "Implement a feature from approved ADR with verification loop"
allowed-tools: ["read", "write", "edit", "execute"]
---

# Implementation Command

You are in **IMPLEMENTATION MODE** - writing code with test-driven verification.

## Your Task

Implement: **$ARGUMENTS**

Expected format:
- `/implement ADR-NNNN` - Implement full ADR
- `/implement <beads-id>` - Implement specific subtask
- `/implement` - Auto-detect next ready task

## Workflow

### Phase 1: Load Context

**1. Identify what to implement:**
```bash
# If no argument, find next ready task
if [ -z "$ARGUMENTS" ]; then
    bd ready
else
    # Check if argument is an ADR or Beads ID
    if [[ "$ARGUMENTS" == ADR-* ]]; then
        cat "docs/adr/$ARGUMENTS.md"
    else
        # Assume it's a beads ID
        bd show "$ARGUMENTS"
    fi
fi
```

**2. Read ADR context:**
- Load the relevant ADR(s)
- Understand architectural constraints
- Review implementation plan phases
- Note dependencies

**3. Assess complexity:**
- Simple (1-2 files, < 100 lines): Single session, no subtasks
- Complex (multiple phases): Break into Beads issues first

**4. Break into subtasks (if complex):**
```bash
# Initialize beads in stealth mode (session-only tracking, not committed)
bd init --stealth

# Create epic for the ADR
bd new "Implement $ADR_TITLE" --kind epic --ref "ADR-NNNN"

# Create subtasks based on phases from ADR implementation plan
bd new "Phase 1: Scanner/Parser changes" --parent <epic-id> --ref "ADR-NNNN"
bd new "Phase 2: Semantic analysis" --parent <epic-id> --ref "ADR-NNNN" --blocks-on <phase1-id>
bd new "Phase 3: Code generation" --parent <epic-id> --ref "ADR-NNNN" --blocks-on <phase2-id>
bd new "Phase 4: VM execution" --parent <epic-id> --ref "ADR-NNNN" --blocks-on <phase3-id>
bd new "Phase 5: Testing" --parent <epic-id> --ref "ADR-NNNN" --blocks-on <phase4-id>

# Show the epic and subtasks
bd show <epic-id>

# Check ready tasks
bd ready
```

**Stealth mode benefits:**
- Issues tracked only for current session
- No .beads directory committed to git
- Clean separation of temporary vs permanent tracking
- Useful for breaking down complex work without polluting repo

### Phase 2: Implement with Tests

**5. Write tests first:**

For new language features:
```bash
# Create integration test in tests/scripts/
touch tests/scripts/test_feature_name.n
```

Format:
```neon
// Test: [What this tests]
// Expected:
// [expected output line 1]
// [expected output line 2]

[test code here]
```

For internal functions:
- Add unit tests in appropriate `src/**/tests/` directory
- Test success and error paths
- Test edge cases

**6. Implement the feature:**

Follow implementation order:
1. **Scanner** (if new tokens needed): `src/compiler/scanner.rs`
2. **AST** (if new node types): `src/compiler/ast/`
3. **Parser** (if new syntax): `src/compiler/parser.rs`
4. **Semantic** (validation): `src/compiler/semantic.rs`
5. **Codegen** (bytecode emission): `src/compiler/codegen.rs`
6. **Opcodes** (if new instructions): `src/common/opcodes.rs`
7. **VM** (execution logic): `src/vm/impl.rs`
8. **Stdlib** (if builtin function): `src/common/stdlib/`

**Code conventions:**
- Document stack state before/after operations
- Use `Result<T, E>` for error handling
- Include source locations in error messages
- Minimize allocations in VM hot path
- Prefer clarity over cleverness

**7. Run tests:**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run integration tests only
cargo test -p neon --test integration
```

### Phase 3: Verification Loop

**8. Check results:**
- ✓ All tests pass → Go to step 11
- ✗ Tests fail → Go to step 9
- ✗ New failures in unrelated tests → Investigate regression

**9. Analyze failures:**
- Read the failure message
- Understand expected vs actual behavior
- Check if logic error or test expectation error
- Use `--features disassemble` for bytecode debugging

**10. Fix and iterate:**
- Update implementation based on failure
- Re-run tests: `cargo test`
- Loop until all tests pass

**11. Quality checks:**
```bash
# Run clippy
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check

# Verify no unused code
# (clippy will warn)
```

**12. Final verification:**
- Does implementation match ADR decision?
- Are edge cases handled?
- Is error handling robust?
- Are stack invariants maintained?
- Is code documented where non-obvious?

### Phase 4: Finalize

**13. Summary:**
Show:
- Files modified (with line counts)
- Tests added (with descriptions)
- Test results (all passing)
- Coverage of ADR requirements

Ask:
- "Implementation complete. Run `/ship` to commit and close the issue?"

## Critical Constraints

- **Tests must pass**: Never proceed with failing tests
- **No scope creep**: Only implement what's in the ADR/issue
- **Follow conventions**: Respect patterns in CLAUDE.md
- **Stack discipline**: Document stack state changes
- **Error context**: Always include source locations
- **Educational clarity**: Code should teach, not confuse

## Neon-Specific Patterns

**Stack operations:**
```rust
// Before: [value1, value2]
// After: [result]
let b = self.pop()?;
let a = self.pop()?;
let result = a + b;
self.push(result)?;
```

**Bytecode emission:**
```rust
// Emit opcode
self.emit_opcode(OpCode::Add);

// Emit with operand
let constant_idx = self.add_constant(value);
self.emit_opcode_with_arg(OpCode::Constant, constant_idx);
```

**Error reporting:**
```rust
return Err(RuntimeError::new(
    format!("Undefined variable '{}'", name),
    self.current_location()
));
```

## When Implementation Spans Multiple Sessions

If work is too large for one session:

1. **End of session:**
   - Commit current progress (even if incomplete)
   - Update Beads issue with status note
   - Document what's left to do

2. **Resume next session:**
   - Read issue notes
   - Review ADR
   - Check current state: `git status`, `cargo test`
   - Continue from where you left off

3. **Use `/ship` only when fully complete**
