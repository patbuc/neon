---
name: implement-adr
description: "Implement an approved ADR: subtasks, code, test, commit loop in a worktree"
---

# Implement ADR - Implementation Phase Only

This skill implements an already-approved ADR. Planning and approval must be done separately using `/project:planning:adr-plan` and `/project:planning:adr-approve`.

## Prerequisites

Before using this skill:
1. ADR must already be planned using `/project:planning:adr-plan`
2. ADR must be approved and saved to `docs/adr/` using `/project:planning:adr-approve`
3. `worktrunk` (wt) must be installed.

## Your Task

Implement the approved ADR: **$ARGUMENTS**

Expected input:
- ADR number (e.g., "ADR-0001")
- OR ADR title/description that matches an existing ADR

## Implementation Workflow

### Step 1: Read ADR & Setup Worktree

1. **Locate the ADR**
   - If given ADR number: read `docs/adr/ADR-NNNN.md`
   - If given description: list ADRs and find matching one
   - Verify status is "Accepted"

2. **Create/Switch Worktree**
   - Branch name: `feature/$ADR_NUMBER` (e.g., `feature/ADR-0001`)
   ```bash
   wt switch --create feature/ADR-NNNN
   ```
   - **CRITICAL**: Capture the directory path returned by `wt`. All subsequent file operations and commands MUST run in this directory.

3. **Extract Implementation Details**
   - Read the Decision section
   - Read any implementation plan included in ADR
   - Understand architectural constraints
   - Identify what needs to be built

### Step 2: Assess Complexity

4. **Determine Implementation Strategy**
   - Simple feature (< 30 min, single file/concern)? → Implement directly (skip to step 5)
   - Complex feature (multiple files/phases)? → Break into subtasks (continue to step 4)

### Step 3: Create Subtasks (If Complex)

5. **Create Beads Issues**
   - Use Beads to create subtasks with dependencies (in the worktree context if needed, though beads are usually global/branch-agnostic depending on setup. If beads are file-based in the repo, run `bd init --stealth` in the worktree).
   - Each subtask = one focused, testable unit
   - Verify dependency graph with `bd ready`
   - Show created issues to user

### Step 4: Implementation Loop

6. **For Each Subtask** (or entire feature if simple):

   a. **Read Context**
      - Read the ADR
      - Read the subtask description (if using Beads)
      - Identify files to modify and algorithms needed

   b. **Implement**
      - Write code following CLAUDE.md conventions
      - Include tests (unit + integration)
      - Reference ADR in comments where relevant

   c. **Verification Loop**
      ```bash
      # Run in worktree directory
      cargo test
      ```
      - If tests fail: fix and retry
      - Loop until all tests pass
      - Ensure code quality meets standards

   d. **Verify Against Requirements**
      - Does it solve the issue/requirement?
      - Does it respect ADR constraints?
      - Are edge cases handled?

   e. **Commit Subtask**
      ```bash
      # Run in worktree directory
      git add .
      git commit -m "Complete subtask: [description]

      Addresses: [issue-id if using Beads]
      Related ADR: ADR-NNNN"
      ```

   f. **Close Subtask (If Using Beads)**
      ```bash
      bd close [issue-id] --reason "Implemented and tested"
      bd ready  # Check for next ready task
      ```

   g. **Move to Next Subtask**
      - Repeat steps a-f for each remaining subtask
      - Continue until all subtasks complete

### Step 5: Final Verification & Shipping

7. **Run Full Quality Gates**
   ```bash
   # Run in worktree directory
   cargo test                    # All tests pass
   cargo clippy -- -D warnings   # No warnings
   cargo build --release         # Clean build
   ```

8. **Push and Create PR**
   ```bash
   # Run in worktree directory
   git push -u origin HEAD
   gh pr create --fill --web
   ```

9. **Cleanup**
   ```bash
   wt remove
   ```

10. **Summary Output**
    - Show final git log
    - Show completed Beads issues (if used)
    - Show test results
    - Confirm feature complete and PR created

## Important Rules

### Prerequisites Check
- ADR must exist in `docs/adr/` with Status "Accepted"
- If ADR not found or not approved: STOP and inform user
- User must run planning commands first

### Implementation Phase
- **Work in Worktree**: Ensure all file operations and commands target the worktree directory.
- Commit after EACH subtask completion
- NEVER commit broken code or failing tests
- Keep changes focused (no scope creep)
- Reference ADR in commits

### Quality Standards
- All existing tests must pass
- New functionality must have tests
- No Clippy warnings allowed
- Follow Rust patterns from CLAUDE.md

### Error Handling
- If tests fail after 3 attempts on same subtask: STOP
- Report the issue to user
- Ask for guidance before proceeding
- Do not skip failing tests

## Retry Logic

- Max 3 retry attempts per subtask for test failures
- If verification fails after 3 attempts:
  1. Show the error clearly
  2. Explain what was attempted
  3. Ask user for guidance
  4. Wait for user decision

## Output Format

Throughout the workflow, provide clear status updates:
```
[IMPLEMENTING ADR-0001]
✓ Read ADR: String Interpolation Support
✓ Status verified: Accepted
✓ Implementation plan understood
✓ Worktree created: /path/to/worktree/feature/ADR-0001

[COMPLEXITY ASSESSMENT]
Feature assessed as: Complex (4 subtasks needed)

[CREATING SUBTASKS]
✓ Created Beads issues with dependencies
✓ Ready to start: Subtask 1 (Parser phase)

[IMPLEMENTATION]
[Subtask 1/4] Parser phase
  ✓ Implemented parser changes
  ✓ Tests pass (45 tests)
  ✓ Committed changes

[Subtask 2/4] Semantic analysis
  ✓ Implemented type checking
  ✓ Tests pass (52 tests)
  ✓ Committed changes

[Continue for all subtasks...]

[FINAL VERIFICATION]
✓ All tests pass (125 tests)
✓ Clippy: No warnings
✓ Build: Success

[SHIPPING]
✓ Pushed branch feature/ADR-0001
✓ Created PR: https://github.com/org/repo/pull/123
✓ Removed worktree

[COMPLETE] Feature implemented successfully!
```

## Context Efficiency

- Load structured documents (ADRs, CLAUDE.md) rather than browsing files
- Plan before executing (avoid trial-and-error)
- Test immediately after each subtask
- Use ADRs to prevent architectural drift
