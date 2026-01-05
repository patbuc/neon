---
description: "Handle PR feedback, CI failures, and requested changes"
allowed-tools: ["read", "write", "edit", "execute"]
---

# Revise PR or Fix CI

Handle pull request feedback and CI failures.

## Your Task

Address feedback for: **$ARGUMENTS**

Expected:
- `/revise <pr-number>` - Handle PR feedback
- `/revise` - Fix current PR (auto-detect from branch)

## Workflow

### 1. Identify Issues

**For PR feedback:**
```bash
# View PR details
gh pr view <pr-number>

# View PR comments
gh pr view <pr-number> --comments

# View PR checks
gh pr checks <pr-number>
```

**For CI failures:**
```bash
# Check CI status
gh pr checks

# View specific check logs
gh run view <run-id>
```

**Categorize issues:**
- Code review requests (logic, style, clarity)
- Test failures (unit, integration)
- Lint failures (clippy, fmt)
- Documentation requests
- Performance concerns

### 2. Analyze Failures

**For test failures:**
```bash
# Run tests locally
cargo test

# Run specific failing test
cargo test <test-name> -- --nocapture

# Check with debugging
cargo test --features disassemble
```

**For clippy warnings:**
```bash
cargo clippy -- -D warnings
```

**For formatting:**
```bash
cargo fmt -- --check
```

**Understand the root cause:**
- What is the expected behavior?
- What is the actual behavior?
- Why does the code not match expectations?
- Is this a logic error, edge case, or test expectation issue?

### 3. Make Changes

**Fix the issues:**

Follow priority:
1. **Correctness bugs** - Logic errors, wrong behavior
2. **Test failures** - Make tests pass
3. **Code review feedback** - Improve clarity, style
4. **Documentation** - Add/update comments
5. **Performance** - Optimize if requested

**Code review changes:**
- Improve variable names
- Extract complex logic to functions
- Add explanatory comments
- Simplify conditionals
- Remove dead code

**Test fixes:**
- Fix logic errors in implementation
- Update test expectations if implementation is correct
- Add missing edge case handling
- Fix stack invariant violations

**Documentation:**
- Add doc comments to public APIs
- Explain complex algorithms
- Document stack state changes
- Update ADR if implementation approach changed

### 4. Re-test Locally

```bash
# Run all tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check

# Run specific integration test
cargo test --test integration -- <test-name>
```

**Ensure:**
- All tests pass
- No clippy warnings
- Formatting is correct
- No regressions in unrelated tests

### 5. Commit Changes

**Determine commit strategy:**

**Option A: Separate "fixup" commit** (preferred for review)
```bash
git add .
git commit -m "fixup: Address PR feedback

- Fix edge case in parameter handling
- Improve variable naming in codegen
- Add explanatory comments for stack operations

Addresses feedback on PR #<number>"
```

**Option B: Amend previous commit** (for minor fixes)
```bash
git add .
git commit --amend --no-edit
```

**Option C: Interactive rebase** (for reorganizing)
```bash
# Combine fixup commits
git rebase -i HEAD~N
# Mark fixup commits with 'f' or 'fixup'
```

### 6. Push Changes

```bash
# Push to PR branch (will update PR)
git push origin <branch-name>

# If you amended, force push
git push --force-with-lease origin <branch-name>
```

**Update PR:**
```bash
# Add comment to PR
gh pr comment <pr-number> --body "Addressed feedback:
- Fixed parameter validation edge case
- Improved code clarity in codegen
- Added tests for null handling

All tests passing, ready for re-review."
```

### 7. Verify CI

**Wait for CI to run:**
```bash
# Watch CI status
gh pr checks <pr-number> --watch
```

**If CI still fails:**
- Review new failure messages
- Return to step 2
- Iterate until all checks pass

### 8. Output Summary

```
Revisions Applied:

Changes made:
  ✓ Fixed parameter validation edge case (src/compiler/semantic.rs)
  ✓ Improved variable naming in codegen (src/compiler/codegen.rs)
  ✓ Added explanatory comments (src/vm/impl.rs)
  ✓ Added test for null handling (tests/scripts/test_params_null.n)

Tests:
  ✓ All 247 tests passing
  ✓ No clippy warnings
  ✓ Formatting correct

Commit: a1b2c3d fixup: Address PR feedback
Pushed to: feature/default-params
PR: #42

CI Status: ⏳ Running...
Next: Wait for CI, then request re-review
```

## Common CI Failure Patterns

**Test failures:**
- Integration test output mismatch → Check expected vs actual
- Unit test assertion fail → Review logic
- Flaky tests → Run multiple times, check for race conditions

**Lint failures:**
- Clippy warnings → Run `cargo clippy --fix`
- Formatting → Run `cargo fmt`
- Unused code → Remove or mark with `#[allow(dead_code)]` if intentional

**Platform-specific issues:**
- Different behavior on macOS vs Linux
- File path separators (use `std::path::Path`)
- Line endings (Git handles CRLF)

**Dependency issues:**
- Update Cargo.lock
- Check for version conflicts
- Run `cargo update`

## Iteration Loop

**While CI fails or feedback exists:**
1. Identify issue
2. Fix locally
3. Test locally
4. Commit
5. Push
6. Check CI
7. Repeat if needed

**Exit condition:**
- All CI checks pass ✓
- All feedback addressed ✓
- Re-review requested
- PR approved ✓

## When Not to Revise

**If feedback requests major changes:**
- Architectural changes → Create new ADR
- Scope expansion → Create new issue
- Different approach → Discuss first, possibly close PR

**Suggest:**
- "This feedback requires architectural changes. Should we create a new ADR?"
- "This is expanding scope. Should we create a follow-up issue?"
- "This is a different approach. Let's discuss before revising."

## Force Push Safety

**Use `--force-with-lease` not `--force`:**
```bash
# Safe: Checks remote hasn't changed
git push --force-with-lease

# Unsafe: Overwrites remote unconditionally
git push --force  # DON'T USE
```

**When force pushing is OK:**
- Updating your own PR branch
- After amending or rebasing
- Before PR is merged

**When force pushing is NOT OK:**
- Main branch
- Shared feature branches
- After PR is merged
