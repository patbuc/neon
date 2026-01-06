---
description: Clean up worktree after implementation is complete
---

# Cleanup Implementation Worktree

Remove the implementation worktree and return to the main worktree.

## Instructions

### 1. Identify the Worktree

- If an argument is provided, use it as the branch name: `$ARGUMENTS`
- Otherwise, detect the current worktree branch using `git branch --show-current`
- Verify this is an implementation branch (should start with `adr-`)

### 2. Check Beads Status

Extract the ADR number from the branch name and check if the epic is closed:
```bash
bd list --type epic --title "ADR-NNNNNN" --json
```

- If the epic exists and is still open, warn the user that implementation may be incomplete
- Show any remaining open child issues:
  ```bash
  bd list --status open --json | jq '.[] | select(.id | startswith("<epic-id>"))'
  ```

### 3. Confirm Git Status

Before removing, check:
```bash
git status
```

- If there are uncommitted changes, warn the user and ask if they want to proceed
- If the branch has unpushed commits, warn the user

### 4. Remove the Worktree

Run:
```bash
wt remove
```

This will:
- Remove the worktree directory
- Delete the local branch
- Return you to the main worktree

### 5. Confirm Cleanup

- Confirm the worktree was removed successfully
- Inform the user they are back in the main worktree
- Remind them the PR is still open on GitHub for review/merge
- If any beads issues were left open, remind them to address those

---

## Important

- This command is meant to be run AFTER the PR has been created
- The `wt remove` command will return you to the main worktree automatically
- If the branch was already merged, the branch will be deleted automatically
- Use `wt remove -D` if you need to force-delete an unmerged branch
- Beads issues are committed to git, so they persist across worktrees
