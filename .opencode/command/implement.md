---
description: Implement a feature from an accepted ADR using beads issue tracking
---

# Implement Feature

You are implementing a feature that was previously designed and accepted via `/design`. The implementation is tracked using beads (`bd`) for persistent task management.

## Instructions

### 1. Ensure Beads is Initialized

Check if beads is initialized:
```bash
ls -la .beads 2>/dev/null || echo "NOT_INITIALIZED"
```

If NOT_INITIALIZED, run:
```bash
bd init
```

### 2. Find the ADR

- If an argument is provided, look for: `docs/adr/*-$ARGUMENTS.md` (glob match)
- Otherwise, list ADRs in `docs/adr/` and use the most recently modified one with status `Accepted`
- If no accepted ADRs exist, inform the user and stop
- Extract the ADR number (NNNNNN) and feature slug from the filename

### 3. Create or Find the Epic Issue

Check if an epic already exists for this ADR:
```bash
bd list --type epic --title "ADR-NNNNNN" --json
```

**If no epic exists**, create one from the ADR:
```bash
bd create "ADR-NNNNNN: <ADR Title>" \
    -t epic \
    -p 1 \
    -d "<Decision Outcome summary from ADR>" \
    --json
```

Save the epic ID (e.g., `bd-a3f8`) for later use.

**If epic exists**, retrieve its ID and check its status. If already closed, inform user and stop.

### 4. Create a Worktree

Create a new worktree for this implementation:

```bash
wt switch --create adr-NNNNNN-feature-slug
```

For example, if implementing `docs/adr/000001-while-loops.md`:
```bash
wt switch --create adr-000001-while-loops
```

**IMPORTANT**: After running `wt switch`, you are now in a NEW directory. The worktree path will be printed by the command. All subsequent work must happen in that new worktree directory. Use the `workdir` parameter for all bash commands to ensure you're working in the correct location.

### 5. Create Child Issues from ADR Phases

Read the ADR thoroughly and extract the Implementation Notes (phases/steps).

For each phase, create a child issue under the epic:
```bash
bd create "Phase N: <phase name>" \
    -t task \
    -p 2 \
    --parent <epic-id> \
    -d "<phase details from ADR>" \
    --json
```

This creates hierarchical IDs like `bd-a3f8.1`, `bd-a3f8.2`, etc.

If phases have dependencies on each other (e.g., Phase 2 depends on Phase 1):
```bash
bd dep add <phase2-id> <phase1-id>
```

### 6. Implementation Loop

Run the implementation loop until all work for this epic is complete:

#### 6.1 Find Ready Work

Get the next ready issue scoped to this epic:
```bash
bd ready --json
```

Filter the results to only include issues that:
- Are the epic itself, OR
- Have IDs starting with the epic ID (e.g., `bd-a3f8.1`, `bd-a3f8.2`)

If no ready issues exist for this epic, check if the epic itself is done:
- If all child issues are closed, close the epic and proceed to step 7
- If there are blocked issues, investigate and report the blockers

#### 6.2 Start Working on the Issue

Mark the issue as in-progress:
```bash
bd update <issue-id> -s in_progress
```

#### 6.3 Implement the Changes

Implement the feature/fix described in the issue:
- Follow the issue description and any linked ADR context
- Maintain the project's code conventions (see CLAUDE.md)
- Use proper error handling with `Result<T, E>`

#### 6.4 Verify the Implementation

Run verification:
```bash
cargo build
cargo test
```

If tests fail, fix the issues before proceeding.

#### 6.5 Commit the Work

Commit the changes:
```bash
git add -A && git commit -m "<issue-id>: <issue title>"
```

For example: `bd-a3f8.1: Phase 1: Add token types`

#### 6.6 Handle Discovered Work

If during implementation you discover additional work needed:
- **In-scope work**: Create a child issue under the epic
  ```bash
  bd create "Fix edge case in <component>" \
      -t task \
      --parent <epic-id> \
      -d "Discovered during <current-issue-id>: <details>" \
      --json
  ```
- **Out-of-scope work**: Create a standalone issue (not under this epic)
  ```bash
  bd create "Unrelated improvement: <title>" \
      -t task \
      -p 3 \
      -d "Discovered during ADR-NNNNNN implementation but out of scope" \
      --json
  ```

#### 6.7 Close the Issue

Mark the issue as closed:
```bash
bd close <issue-id>
```

#### 6.8 Repeat

Go back to step 6.1 to find the next ready issue. Continue until all issues under the epic are complete.

### 7. Close the Epic

Once all child issues are closed:
```bash
bd close <epic-id>
```

### 8. Push and Create PR

After all work is complete:

1. Push the branch to origin:
   ```bash
   git push -u origin adr-NNNNNN-feature-slug
   ```

2. Create a pull request using `gh pr create`. Use a HEREDOC for the body:
   ```bash
   gh pr create --title "<ADR Title>" --body "$(cat <<'EOF'
   ## Summary
   <Brief description from ADR Decision Outcome - 1-3 sentences>

   ## ADR
   See: docs/adr/NNNNNN-feature-slug.md

   ## Issues Completed
   - <epic-id>: <epic title>
     - <child-id>: <child title>
     - <child-id>: <child title>
     ...
   EOF
   )"
   ```

3. Return the PR URL to the user

### 9. Inform User About Cleanup

Tell the user:
- The PR has been created
- They are still in the feature worktree
- All beads issues for this epic have been closed
- When ready to clean up, run `/implement-cleanup` to remove the worktree and return to main

---

## Important

- **Scope discipline**: Only work on issues related to the current epic. Ignore other ready work in beads.
- **Discovery tracking**: Always file discovered work as issues - either under the epic (in-scope) or standalone (out-of-scope)
- **Commit per issue**: Each issue should result in at least one commit
- Follow the ADR closely - it represents an accepted architectural decision
- Maintain the project's code conventions (see CLAUDE.md)
- Use proper error handling with `Result<T, E>`
- Include helpful comments for complex logic
- If you encounter issues not covered by the ADR, note them and proceed with your best judgment
- Do NOT modify the ADR file itself during implementation
- After `wt switch`, always verify you're in the correct worktree directory
