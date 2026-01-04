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

## Workflow State

Track using TodoWrite:
- `phase`: planning | implementing | finalizing
- `attempt`: 1-3 (implementation retries)
- `plan_approved`: boolean

---

## Phase 1: Planning (Human Approval Required)

### Step 1.1: Explore

Use the Task tool with `subagent_type="Explore"` with thoroughness="medium":

```
Explore the codebase to understand:
1. Files and patterns relevant to: [USER REQUEST]
2. Existing test patterns (unit tests location, integration test format)
3. Similar features for reference

Return: relevant file paths, key patterns, testing conventions
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

### Step 1.3: Human Approval

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

Set `attempt = 1`. Loop while `attempt <= 3`:

### Step 2.1: Implement

Spawn the feature-coder agent:

```
Task(
  subagent_type="feature-coder",
  prompt="Implement this feature:

PLAN:
[Insert approved plan]

CONSTRAINTS:
- Follow patterns in CLAUDE.md
- Minimal changes - only what the plan requires
- Use Result<T, E> for errors, no unwrap() outside tests
- Add inline comments only where logic is non-obvious

Report: files changed, summary of implementation"
)
```

### Step 2.2: Format and Lint (Fail Fast)

Run immediately after implementation:

```bash
cargo fmt
```

Then check for issues:

```bash
cargo clippy -- -D warnings 2>&1
```

**On clippy failure:**
- If fixable (unused imports, naming): fix inline and continue
- If architectural (wrong pattern, missing trait): increment attempt, retry Step 2.1

### Step 2.3: Test

Spawn the feature-tester agent:

```
Task(
  subagent_type="feature-tester",
  prompt="Validate the implementation:

WHAT WAS IMPLEMENTED:
[Summary from coder]

FILES CHANGED:
[List from coder]

TASKS:
1. Run existing tests: cargo test
2. If tests fail, analyze and report (do NOT fix - coder will fix)
3. Assess if new tests are needed based on:
   - New public functions/methods without test coverage
   - New code paths (branches, error cases)
   - Integration points with existing features
4. Add tests ONLY where coverage is genuinely missing
5. Do NOT add tests for: trivial getters/setters, already-covered paths, obvious one-liners

Report: test results, new tests added (with justification), coverage assessment"
)
```

### Step 2.4: Quality Gate

Run the full quality check:

```bash
cargo build --release && cargo test && cargo clippy -- -D warnings
```

**On PASS:** Proceed to Phase 3

**On FAIL:**
- Parse the specific failure (build error, test failure, clippy warning)
- If `attempt < 3`:
  - Create a focused fix task for the coder agent
  - Increment attempt, retry from Step 2.1
- If `attempt >= 3`:
  - **STOP** and report to user with:
    - What failed (exact error)
    - What was tried
    - Suggested manual intervention

---

## Phase 3: Finalization

### Step 3.1: Final Review

Spawn the feature-reviewer agent (reviews the FINAL state):

```
Task(
  subagent_type="feature-reviewer",
  prompt="Review the completed implementation:

ORIGINAL REQUEST:
[User request]

PLAN:
[Approved plan]

FILES CHANGED:
[List all modified/created files]

Review for:
1. Does implementation match the plan?
2. Code quality (idioms, clarity, error handling)
3. Test coverage adequate?
4. Any security concerns?

Return: APPROVED or NEEDS_CHANGES with specific issues"
)
```

**On NEEDS_CHANGES:** Fix issues and re-run quality gate (counts as attempt)

### Step 3.2: Create PR

1. Create feature branch if needed:
   ```bash
   git checkout -b feature/[descriptive-name]
   ```

2. Stage and commit:
   ```bash
   git add -A
   git commit -m "[concise description of feature]"
   ```

3. Push and create PR:
   ```bash
   git push -u origin HEAD
   gh pr create --title "[Feature title]" --body "## Summary
   [1-2 sentence summary]

   ## Changes
   [Bulleted list of changes]

   ## Testing
   [How to verify]"
   ```

---

## Error Recovery

When blocked:

1. **Identify the blocker type:**
   - Ambiguous requirement → AskUserQuestion
   - Technical limitation → Document and ask user
   - Repeated failure → Stop after 3 attempts

2. **Provide actionable context:**
   - Exact error message
   - What was attempted
   - Specific question or suggested fix

3. **Do not spin:** If the same error occurs twice, escalate to user immediately.

---

## Anti-Patterns to Avoid

- **Over-testing:** Don't add tests for trivial code or already-covered paths
- **Context dumping:** Don't pass entire file contents between agents
- **Retry without change:** Each retry must address the specific failure
- **Formatting at the end:** Run `cargo fmt` immediately after coding, not in quality gate
- **Reviewing early:** Review the final state, not intermediate states
