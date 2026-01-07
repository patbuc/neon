---
description: Implement work from beads issues (epic, task, bug, or feature)
---

# Implement

Implements work tracked in beads. Can work on epics (from `/design`), or standalone issues (from `/task`).

## Input

$ARGUMENTS

## Instructions

### 1. Determine What to Implement

Based on the input, determine what to work on:

#### Option A: Issue ID Provided (e.g., `bd-a3f8`)

```bash
bd show $ARGUMENTS --json
```

- If the issue exists, use it as the target
- If it's an **epic**: implement all child issues in dependency order
- If it's a **task/bug/feature**: implement just that single issue

#### Option B: ADR Name Provided (e.g., `while-loops`)

Look for: `docs/adr/*-$ARGUMENTS.md` (glob match)

If found, find or create the corresponding epic:

```bash
bd list --type epic --title "ADR-NNNNNN" --json
```

If no epic exists but the ADR does, inform user to run `/design` first to create the epic.

#### Option C: No Input - Smart Pickup

If no argument provided, check for ready work:

```bash
bd ready --json --limit 10
```

**Priority order for pickup:**

1. **In-progress issues**: First check if any issues are already in-progress (resume work)
   ```bash
   bd list --status in_progress --json
   ```
2. **Ready epics**: Prefer epics (larger, planned work)
3. **Ready tasks/bugs/features**: Individual issues by priority

If multiple options exist, present them to the user and ask which to work on:

```
Found ready work:
1. [epic] bd-a3f8: ADR-000001: While Loops (3 child issues)
2. [bug] bd-c5d6: Fix division by zero crash (P1)
3. [task] bd-e7f8: Refactor scanner error handling (P2)

Which would you like to implement? (enter number or issue ID)
```

If only one ready item exists, confirm with user before starting.

If no ready work exists, inform user and suggest `/task` or `/design` to create new work.

### 2. Determine Scope

Based on the target issue type:

| Type             | Scope                          | Branch Name                                 |
|------------------|--------------------------------|---------------------------------------------|
| epic             | All child issues + epic itself | `adr-NNNNNN-slug` or `epic/<issue-id>-slug` |
| task/bug/feature | Single issue only              | `<type>/<issue-id>-slug`                    |

Set the **scope ID** (the root issue to track) and **branch name**.

### 3. Create a Worktree

Create a new worktree for this implementation:

```bash
wt switch --create <branch-name> 
```

For example:

- Epic from ADR: `wt switch --create adr-000001-while-loops`
- Standalone bug: `wt switch --create bug/bd-c5d6-division-by-zero`

**IMPORTANT**: After running `wt switch`, you are now in a NEW directory. The worktree path will be printed by the
command. All subsequent work must happen in that new worktree directory. Use the `workdir` parameter for all bash
commands.

### 4. Implementation Loop

Run the implementation loop until all work in scope is complete:

#### 4.1 Find Ready Work in Scope

```bash
bd ready --json
```

Filter results to only include issues in scope:

- **Epic scope**: Issues where ID equals the epic ID OR starts with `<epic-id>.`
- **Single issue scope**: Only the target issue itself

If no ready issues exist in scope:

- **Epic**: Check if all child issues are closed → close the epic and proceed to step 5
- **Single issue**: Should not happen (the issue itself should be ready)
- **Blocked issues**: Investigate and report blockers to user

#### 4.2 Start Working on the Issue

Mark the issue as in-progress:

```bash
bd update <issue-id> -s in_progress
```

Read the issue details:

```bash
bd show <issue-id>
```

#### 4.3 Implement the Changes

Implement the work described in the issue:

- Follow the issue description
- If this is part of an epic, reference the ADR for context
- Maintain the project's code conventions (see CLAUDE.md)
- Use proper error handling with `Result<T, E>`

#### 4.4 Verify the Implementation

Run verification:

```bash
cargo build
cargo test
```

If tests fail, fix the issues before proceeding.

#### 4.5 Commit the Work

Commit the changes:

```bash
git add -A && git commit -m "<issue-id>: <issue title>"
```

For example:

- `bd-a3f8.1: Add while token and AST node`
- `bd-c5d6: Fix division by zero crash`

#### 4.6 Handle Discovered Work

If during implementation you discover additional work needed:

- **In-scope work** (epic only): Create a child issue
  ```bash
  bd create "Fix edge case in <component>" \
      -t task \
      --parent <epic-id> \
      -d "Discovered during <current-issue-id>: <details>" \
      --json
  ```

- **Out-of-scope work**: Create a standalone issue
  ```bash
  bd create "<title>" \
      -t <bug|task|feature> \
      -p 3 \
      -d "Discovered during <current-issue-id> but out of scope" \
      --json
  ```

#### 4.7 Close the Issue

Mark the issue as closed:

```bash
bd close <issue-id>
```

#### 4.8 Repeat

Go back to step 4.1. Continue until all issues in scope are complete.

### 5. Finalize

Once all work in scope is complete:

#### For Epics

Close the epic:

```bash
bd close <epic-id>
```

#### For All Types

Push and create PR:

1. Push the branch:
   ```bash
   git push -u origin <branch-name>
   ```

2. Create PR:
   ```bash
   gh pr create --title "<issue title>" --body "$(cat <<'EOF'
   ## Summary
   <Brief description from issue>

   ## Issues Completed
   - <issue-id>: <title>
     - <child-id>: <child title>  (if epic)
     ...

   ## ADR
   See: docs/adr/NNNNNN-slug.md  (if epic from ADR, otherwise omit)
   EOF
   )"
   ```

3. Return the PR URL to the user

### 6. Inform User About Cleanup

Tell the user:

- The PR has been created
- They are still in the feature worktree
- All beads issues in scope have been closed
- When ready to clean up, run `/implement-cleanup` to remove the worktree and return to main

---

## Important

- **Scope discipline**: Only work on issues in the current scope. Ignore other ready work.
- **Discovery tracking**: Always file discovered work as issues
- **Commit per issue**: Each issue should result in at least one commit
- **Resume support**: If returning to in-progress work, pick up where you left off
- Maintain the project's code conventions (see CLAUDE.md)
- Use proper error handling with `Result<T, E>`
- After `wt switch`, always verify you're in the correct worktree directory
- For epics, reference the ADR for architectural context
