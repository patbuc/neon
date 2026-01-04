---
name: build-feature
description: Feature development workflow with planning, implementation, testing, and PR creation. Use when asked to build, implement, or add a new feature that requires multiple steps.
---

# Feature Development Workflow

You are orchestrating a feature development workflow. This workflow balances autonomous execution with human oversight at critical decision points.

## Design Principles

1. **Minimize context pollution**: Use specialized agents, pass only necessary context
2. **Fail fast**: Run quick checks early, expensive checks later
3. **Human gates at decisions, not mechanics**: Approve plans, not every command
4. **Incremental quality**: Catch issues immediately, don't batch failures
5. **Categorize failures**: Not all errors are equal - handle them appropriately
6. **Never lose work**: Always offer recovery options before discarding progress

## Failure Categories

Categorize every failure before deciding how to handle it:

| Category | Examples | Action |
|----------|----------|--------|
| **TRIVIAL** | Unused import, formatting, typo in comment | Fix inline, don't count as attempt |
| **RECOVERABLE** | Logic error, missing implementation, test failure | Count as attempt, retry with context |
| **FUNDAMENTAL** | Type system conflict, architectural mismatch, impossible requirement | Escalate immediately, don't waste attempts |

## Workflow State

Track in the plan file (`.claude/plans/feature-{slug}.md`):
- `phase`: planning | implementing | finalizing
- `implementation_attempts`: 0-3 (coding/testing failures)
- `review_attempts`: 0-2 (reviewer rejections)
- `checkpoint`: stash reference for recovery

---

## Phase 1: Planning (Human Approval Required)

### Step 1.1: Explore

Use the Task tool with `subagent_type="Explore"` with thoroughness="medium":

```
Explore the codebase to understand:
1. Files and patterns relevant to: [USER REQUEST]
2. Existing test patterns (unit tests location, integration test format)
3. Similar features for reference
4. Recent changes that might conflict

Required findings checklist:
[ ] Files that will need modification (with line ranges if possible)
[ ] Existing similar implementations to reference
[ ] Test file locations and patterns used
[ ] Recent commits on relevant files (git log --oneline -5 for each)

Return: relevant file paths, key patterns, testing conventions, potential conflicts
```

### Step 1.2: Create Plan

Use the Task tool with `subagent_type="Plan"`:

```
Create an implementation plan for: [USER REQUEST]

Based on exploration findings: [EXPLORATION RESULTS]

The plan must include:
1. Summary (1-2 sentences)
2. Files to modify (with brief description of changes)
3. Files to create (if any)
4. Test strategy (what to test, where tests go)
5. Risks or open questions

Keep the plan concise. Focus on WHAT changes, not HOW to write code.
```

### Step 1.3: Persist Plan

Generate a slug from the feature name:
- Lowercase, replace spaces with hyphens
- Use first 20 chars (enough to distinguish "auth-login" from "auth-logout")
- Remove special characters except hyphens

Write the plan to `.claude/plans/feature-{slug}.md`:

```markdown
# Feature Plan: [Feature Name]

## Approved Plan
[Plan from Step 1.2]

## Status
- Phase: planning
- Implementation Attempts: 0/3
- Review Attempts: 0/2
- Checkpoint: (none yet)

## Failure History
(none yet)

## Unplanned Changes
(none yet)
```

### Step 1.4: Human Approval

Present the plan and ask for approval using AskUserQuestion:

```
question: "Does this implementation plan look good?"
options:
  - "Approve and proceed"
  - "Modify the plan" (iterate on feedback)
  - "Cancel"
```

**STOP if not approved.** Iterate on feedback if requested.

---

## Phase 2: Implementation Loop (Max 3 Attempts)

### Step 2.0: Create Checkpoint

Before first implementation attempt, create a reference to the current clean state:

```bash
# Create a stash entry without modifying working tree
CHECKPOINT=$(git stash create)
if [ -n "$CHECKPOINT" ]; then
  git stash store -m "pre-feature-checkpoint-{slug}-$(date +%s)" "$CHECKPOINT"
fi
```

If working tree is already clean, `git stash create` returns empty - that's fine, note "clean" as checkpoint.

Update plan file with the stash reference or "clean" marker.

### Step 2.1: Implement

Spawn the feature-coder agent with rich context:

```
Task(
  subagent_type="feature-coder",
  prompt="Implement this feature.

PLAN FILE: .claude/plans/feature-{slug}.md

CURRENT ATTEMPT: [N] of 3

[If retry, include:]
PREVIOUS FAILURE:
- Category: [TRIVIAL|RECOVERABLE|FUNDAMENTAL]
- Error: [exact error message]
- Files affected: [list]
- What to fix: [specific guidance]

CONSTRAINTS:
- Follow patterns in CLAUDE.md
- Minimal changes - only what the plan requires
- Use Result<T, E> for errors, no unwrap() outside tests
- Add inline comments only where logic is non-obvious

Report: files changed, summary of implementation, any UNPLANNED_CHANGES"
)
```

### Step 2.1b: Check for Drift

After coder returns, check output for `UNPLANNED_CHANGES` section:
- If empty or minor: continue
- If significant deviation: update plan file and ask user to approve

### Step 2.2: Format and Lint (Fail Fast)

Run immediately after implementation:

```bash
cargo fmt
cargo clippy -- -D warnings 2>&1
```

**On clippy failure, categorize:**
- **TRIVIAL** (unused imports, naming conventions): fix inline, continue
- **RECOVERABLE** (wrong pattern, missing trait impl): increment attempt, retry Step 2.1
- **FUNDAMENTAL** (type system conflict, impossible cast): escalate to user immediately

### Step 2.3: Test

Spawn the feature-tester agent with context:

```
Task(
  subagent_type="feature-tester",
  prompt="Validate the implementation.

PLAN FILE: .claude/plans/feature-{slug}.md
Verify tests cover the PLANNED requirements, not just what was coded.

WHAT WAS IMPLEMENTED:
[Summary from coder]

FILES CHANGED:
[List from coder]

TASKS:
1. Run existing tests: cargo test
2. If tests fail, analyze and report (do NOT fix - coder will fix)
3. Assess coverage using the coverage heuristics
4. Add tests ONLY where coverage is genuinely missing
5. Report with coverage checklist

Report: test results, coverage checklist, new tests added (with justification)"
)
```

**On tester reporting test failures:**
1. Categorize the failure (usually RECOVERABLE)
2. Update plan file with test failure context
3. If `implementation_attempts < 3`, increment and retry Step 2.1 with failure details
4. If `implementation_attempts >= 3`, go to Step 2.5 (Escalate)

**On tester success:** Proceed to Step 2.4

### Step 2.4: Quality Gate

Run the full quality check:

```bash
cargo build --release && cargo test && cargo clippy -- -D warnings
```

**On PASS:** Proceed to Phase 3

**On FAIL:**
1. Categorize the failure (TRIVIAL, RECOVERABLE, or FUNDAMENTAL)
2. Update the plan file with failure info:
   ```
   ### Implementation Attempt [N]
   - Category: [TRIVIAL|RECOVERABLE|FUNDAMENTAL]
   - Error: [brief description]
   - Cause: [one sentence analysis]
   ```
3. Based on category:
   - **TRIVIAL**: Fix inline, re-run quality gate (don't count as attempt)
   - **RECOVERABLE**: If `implementation_attempts < 3`, increment and retry Step 2.1
   - **FUNDAMENTAL**: Go to Step 2.5 (Escalate)

4. If `implementation_attempts >= 3`: Go to Step 2.5 (Escalate)

### Step 2.5: Escalate (On Exhausted Attempts or Fundamental Failure)

Present recovery options via AskUserQuestion:

```
question: "Implementation attempts exhausted or hit fundamental blocker. How should we proceed?"
options:
  - "Create draft PR with known issues" (commits progress, creates draft PR documenting issues)
  - "Save to branch for manual completion" (commits to feature branch, no PR)
  - "Abandon and restore checkpoint" (git stash pop, discard all changes)
  - "Let me investigate" (pause for user to debug)
```

Based on choice:
- **Draft PR**: Create branch, commit with WIP prefix, create draft PR with issues in body
- **Save to branch**: Create branch, commit, tell user branch name
- **Abandon**: Discard changes and restore checkpoint:
  ```bash
  git checkout .
  git clean -fd
  # Restore from checkpoint if one was created
  git stash list | grep "pre-feature-checkpoint-{slug}" | head -1 | cut -d: -f1 | xargs -I{} git stash pop {} 2>/dev/null || true
  ```
- **Investigate**: Stop and wait for user guidance

---

## Phase 3: Finalization

### Step 3.1: Final Review

Spawn the feature-reviewer agent:

```
Task(
  subagent_type="feature-reviewer",
  prompt="Review the completed implementation.

PLAN FILE: .claude/plans/feature-{slug}.md

ORIGINAL REQUEST:
[User request]

FILES CHANGED:
[List all modified/created files]

Review for:
1. Does implementation match the plan?
2. Code quality (idioms, clarity, error handling)
3. Test coverage adequate?
4. Any security concerns?

Return: APPROVED, AUTO_FIX (with specific fixes), or NEEDS_CHANGES (with specific issues)"
)
```

**On APPROVED:** Proceed to Step 3.2

**On AUTO_FIX:**
- Apply the reviewer's specified fixes directly
- Re-run quality gate
- If passes, proceed to Step 3.2 (doesn't count as attempt)

**On NEEDS_CHANGES:**
- Increment `review_attempts`
- If `review_attempts < 2`:
  - Update plan file with issues
  - Return to Step 2.1 with reviewer feedback as context
- If `review_attempts >= 2`:
  - Go to Step 2.5 (Escalate) with review issues

### Step 3.2: Create PR

Detect the git forge and create PR appropriately:

```bash
REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "")
```

1. Create feature branch:
   ```bash
   git checkout -b feature/{slug}
   ```

2. Stage and commit:
   ```bash
   git add -A
   git commit -m "[concise description of feature]"
   ```

3. Push and create PR based on forge:

   **GitHub** (URL contains `github.com`):
   ```bash
   git push -u origin HEAD
   gh pr create --fill --title "[Feature title]" --body "## Summary
   [1-2 sentence summary]

   ## Changes
   [Bulleted list of changes]

   ## Testing
   [How to verify]"
   ```

   **GitLab** (URL contains `gitlab.com`):
   ```bash
   git push -u origin HEAD
   glab mr create --title "[Feature title]" --description "..."
   ```

   **Other** (Bitbucket, self-hosted, etc.):
   ```bash
   git push -u origin HEAD
   ```
   Then inform user: "Branch pushed. Please create PR/MR manually at your forge."

### Step 3.3: Cleanup

**Only perform cleanup after confirming PR was created successfully** (i.e., PR URL was returned).

If PR creation failed or was skipped, keep the plan file for recovery.

Delete the plan file:

```bash
rm .claude/plans/feature-{slug}.md
```

Clear the checkpoint stash (use pattern match since we added timestamp):

```bash
git stash list | grep "pre-feature-checkpoint-{slug}" | head -1 | cut -d: -f1 | xargs -I{} git stash drop {} 2>/dev/null || true
```

---

## Error Recovery

When blocked:

1. **Identify the blocker type:**
   - Ambiguous requirement → AskUserQuestion
   - Technical limitation → Categorize as FUNDAMENTAL, escalate
   - Repeated failure → After 3 attempts, escalate with options

2. **Provide actionable context:**
   - Exact error message
   - What was attempted (with attempt numbers)
   - Category of failure
   - Specific question or suggested fix

3. **Do not spin:** If the same error occurs twice, escalate immediately.

4. **Never lose work:** Always offer to save progress before abandoning.

---

## Anti-Patterns to Avoid

- **Over-testing:** Don't add tests for trivial code or already-covered paths
- **Context dumping:** Pass specific failure context, not entire file contents
- **Retry without change:** Each retry must address the specific failure with new information
- **Formatting at the end:** Run `cargo fmt` immediately after coding
- **Reviewing early:** Review only the final state, not intermediate states
- **Counting trivial fixes as attempts:** Unused imports shouldn't exhaust retry budget
- **Losing work on failure:** Always offer recovery options
