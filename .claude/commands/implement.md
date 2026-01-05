---
description: "Implement a feature in a fresh worktree with verification loop"
allowed-tools: ["read", "write", "edit", "execute", "bash"]
---

# Implementation Command

You are in **IMPLEMENTATION MODE** - writing code with test-driven verification in an isolated worktree.

## Your Task

Implement: **$ARGUMENTS**

Expected format:
- `/implement ADR-NNNN` - Implement full ADR (looks in `docs/adr/`)
- `/implement TASK-NNNN` - Implement planned task (looks in `docs/tasks/`)
- `/implement <beads-id>` - Implement specific subtask/issue
- `/implement` - Auto-detect next ready task

## Workflow

### Phase 1: Setup Worktree

**1. Determine Branch Name & Type:**
- If ADR (`ADR-*`): `feature/$ARGUMENTS`
- If Task (`TASK-*`): `task/$ARGUMENTS`
- If Beads ID: `issue/$ARGUMENTS`
- If no argument: Pick top ready task, use `issue/<id>`

**2. Create/Switch Worktree:**
```bash
# Create and switch to new worktree (or switch if exists)
wt switch --create "$BRANCH_NAME"
```

**CRITICAL**: 
- Note the directory path created/returned by `wt`.
- **ALL** subsequent commands (read, write, test, git) **MUST** be executed in this new directory (use `workdir` parameter).
- Do not make changes in the main repository folder.

### Phase 2: Load Context

**3. Identify Requirements:**
- **ADR**: Read `docs/adr/$ARGUMENTS.md`
- **Task**: Read `docs/tasks/$ARGUMENTS.md`
- **Beads**: `bd show $ARGUMENTS`

**4. Assess Complexity:**
- **Simple**: Implement directly in this session.
- **Complex**: Break into subtasks.

**5. (Optional) Break into subtasks:**
```bash
# Initialize beads in stealth mode (session-only tracking)
bd init --stealth

# Create structure
# Prefix with document ID for clarity
bd new "Implement $ARGUMENTS: $TITLE" --kind epic
bd new "Phase 1..." --parent <epic-id>
# ...
```

### Phase 3: Implement with Tests

**6. Write Tests FIRST:**
- New features: Create integration test in `tests/scripts/`
- Internal logic: Create unit tests in `src/**/tests/`

Format for integration tests (`tests/scripts/test_name.n`):
```neon
// Test: [Description]
// Expected:
// [Line 1]
// [Line 2]

print("actual output");
```

**7. Implement Feature:**
Follow pipeline order (Scanner -> AST -> Parser -> Semantic -> Codegen -> VM).

**8. Run Tests:**
```bash
# In worktree directory:
cargo test
cargo test -p neon --test integration
```

### Phase 4: Verification Loop

**9. Check & Iterate:**
- ✓ All tests pass → Proceed.
- ✗ Tests fail → Analyze, Fix, Retry.

**10. Quality Gates:**
```bash
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Phase 5: Ship & Cleanup

**11. Summary:**
- List modified files.
- Confirm test results.

**12. Publish:**
```bash
# Stage and commit
git add .
git commit -m "feat: implement $ARGUMENTS" 

# Push and Create PR
git push -u origin HEAD
gh pr create --fill --web
```
*Note: If `gh pr create` requires interaction, provide title/body explicitly.*

**13. Cleanup:**
```bash
# Remove the worktree
wt remove
```

## Critical Rules
- **Work in Worktree**: Never pollute the main checkout.
- **Tests First**: Always write a failing test before implementation.
- **Atomic PRs**: One task = One Branch = One PR.
