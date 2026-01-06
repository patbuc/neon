---
description: Clean up worktree after implementation is complete
agent: build
---

# Cleanup Implementation Worktree

Remove the implementation worktree and return to the main worktree.

## Instructions

### 1. Identify the Worktree

- If an argument is provided, use it as the branch name: `$ARGUMENTS`
- Otherwise, detect the current worktree branch using `git branch --show-current`
- Verify this is an implementation branch (should start with `adr-`)

### 2. Confirm Status

Before removing, check:
```bash
git status
```

- If there are uncommitted changes, warn the user and ask if they want to proceed
- If the branch has unpushed commits, warn the user

### 3. Remove the Worktree

Run:
```bash
wt remove
```

This will:
- Remove the worktree directory
- Delete the local branch
- Return you to the main worktree

### 4. Confirm Cleanup

- Confirm the worktree was removed successfully
- Inform the user they are back in the main worktree
- Remind them the PR is still open on GitHub for review/merge

---

## Important

- This command is meant to be run AFTER the PR has been created
- The `wt remove` command will return you to the main worktree automatically
- If the branch was already merged, the branch will be deleted automatically
- Use `wt remove -D` if you need to force-delete an unmerged branch
