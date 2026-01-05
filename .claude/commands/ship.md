---
description: "Ship completed work: verify, review, commit, close issue"
allowed-tools: ["read", "write", "execute", "bash"]
---

# Ship Completed Work

You are in **SHIPPING MODE** - finalizing and committing completed work in a worktree.

## Your Task

Ship the current work: **$ARGUMENTS**

Expected usage:
- `/ship <beads-id>` - Ship specific issue
- `/ship` - Ship current work in progress

## Workflow

### 1. Verify Tests Pass

```bash
# Run full test suite in the current worktree
cargo test

# Verify no warnings
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check
```

**If tests fail:**
- STOP: Fix tests first
- Return to `/implement` mode
- Do not proceed until all tests pass

### 2. Code Review

Review the changes:
```bash
git diff --cached  # If staged
git diff           # If unstaged
```

**Review checklist:**

**Correctness:**
- [ ] Logic implements ADR decision correctly
- [ ] Edge cases handled
- [ ] Error messages are clear and include source locations
- [ ] No off-by-one errors in loops/indexing

**Neon-specific:**
- [ ] Stack invariants maintained (documented in comments)
- [ ] Bytecode emission is correct
- [ ] Opcode implementation handles all value types
- [ ] Symbol table scoping is correct
- [ ] Memory management (Rc usage) is appropriate

**Performance:**
- [ ] No unnecessary allocations in VM hot path
- [ ] String interning used where appropriate
- [ ] Constant folding opportunities not missed

**Code quality:**
- [ ] Follows conventions in CLAUDE.md
- [ ] Clear variable names
- [ ] Comments explain "why" not "what"
- [ ] No commented-out code
- [ ] No debug println! statements

**Testing:**
- [ ] Integration tests for language features (tests/scripts/)
- [ ] Unit tests for internal functions
- [ ] Tests cover success and error paths
- [ ] Expected output format is correct

**Documentation:**
- [ ] ADR updated if implementation differs from plan
- [ ] Complex algorithms have explanatory comments
- [ ] Stack state documented before/after operations

**Auto-fix issues:**
If you find issues during review:
- Fix them immediately
- Re-run tests
- Do not just report issues - fix them

### 3. Commit, Push and Create PR

**Commit message format:**
```
<type>: <brief description>

<optional body explaining what and why>

Implements: ADR-NNNN
Closes: <beads-id>    # Optional: only if beads issue exists
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code restructuring without behavior change
- `test`: Adding/updating tests
- `docs`: Documentation only
- `chore`: Tooling, dependencies, etc.

**Commit, Push and PR:**
```bash
# Stage all changes
git add .
git commit -m "<message>"

# Push current branch
git push origin HEAD

# Create PR (if gh CLI is available)
gh pr create --fill --web
```
*Note: If `gh pr create` requires interaction, provide title/body explicitly.*

### 4. Close Beads Issue

**Check if beads is initialized:**
```bash
# Check if beads is tracking issues
bd list 2>/dev/null || echo "No beads tracking"
```

**If beads is active:**
```bash
# Close the issue if an argument is provided
if [ -n "$ARGUMENTS" ]; then
    # Close the issue
    bd close "$ARGUMENTS" --reason "Implemented and tested"

    # Verify closure
    bd status "$ARGUMENTS"
fi

# Check next ready tasks
bd ready
```

**If issue ID not provided, look it up:**
```bash
if [ -z "$ARGUMENTS" ]; then
    bd list --status=open
fi
```

### 5. Cleanup Worktree

**Important:**
- Once the PR is created and the issue is closed, you should remove the worktree to clean up.

```bash
wt remove
```

## Summary Output

Show:
```
✓ All tests passed
✓ Code review complete
✓ Changes committed: <hash>
✓ PR Created: <url>
✓ Issue closed: <beads-id>
✓ Worktree removed

Next ready tasks:
- <id1>: <description>
- <id2>: <description>

Suggested next step:
/implement <id1>
```

## Critical Constraints

- **No broken tests**: NEVER commit failing tests
- **Auto-fix review issues**: Don't just report, fix immediately
- **Close before commit**: Beads issue must be closed before committing
- **Reference ADR**: Always link commit to ADR
- **Imperative messages**: "Add feature" not "Added feature"
- **Worktree Isolation**: Ensure you are operating within the correct worktree before running commands.

## When to Use /ship

**Use when:**
- Implementation is complete
- All tests pass
- Code review is satisfactory
- Ready to close the issue and create a PR

**Don't use when:**
- Tests are failing
- Work is incomplete
- You want to save progress mid-task (use regular git commit instead)
