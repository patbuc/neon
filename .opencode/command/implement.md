---
description: Implement work from beads issues (epic, task, bug, or feature)
---

# Implement

Implements work tracked in beads using TDD. Can work on epics (from `/design`),
or standalone issues (from `/task`).

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

If found, find the corresponding epic:

```bash
bd list --type epic --title-contains "ADR-NNNNNN" --json
```

If no epic exists but the ADR does, inform user to run `/design` first.

#### Option C: No Input - Smart Pickup

```bash
bd ready --json --limit 10
```

**Priority order:**
1. **In-progress issues**: Resume work first
   ```bash
   bd list --status in_progress --json
   ```
2. **Ready epics**: Prefer epics (larger, planned work)
3. **Ready tasks/bugs/features**: Individual issues by priority

If multiple options, present list and ask user which to work on.
If none, suggest `/task` or `/design` to create work.

### 2. Determine Scope

| Type             | Scope                          | Branch Name                                 |
|------------------|--------------------------------|---------------------------------------------|
| epic             | All child issues + epic itself | `adr-NNNNNN-slug` or `epic/<issue-id>-slug` |
| task/bug/feature | Single issue only              | `<type>/<issue-id>-slug`                    |

### 3. Create Worktree

Create worktree branching from current branch using `@` shortcut:
```bash
wt switch --create <branch-name> --base @
```

**IMPORTANT**: After `wt switch`, you are in a NEW directory. Use `workdir` parameter
for all subsequent bash commands.

### 4. Prepare ADR Context

**Only if scope is an epic with an ADR:**

1. Read the ADR file: `docs/adr/NNNNNN-*.md`
2. Create a concise summary (ADR_SUMMARY) containing:
   - Core design decisions
   - Key code patterns/locations mentioned
   - Constraints or invariants to maintain
3. Keep summary to 5-10 bullet points maximum
4. Store for passing to sub-agents

### 5. Implementation Loop

#### 5.1 Initialize Ready Queue

```bash
bd ready --json
```

Filter results to scope:
- **Epic**: ID equals epic ID OR starts with `<epic-id>.`
- **Single issue**: Only the target issue

Store as READY_QUEUE.

#### 5.2 Process Issues

**WHILE** READY_QUEUE is not empty:

**a) Select issue**: Pick first from READY_QUEUE

**b) Mark in-progress**:
```bash
bd update <issue-id> -s in_progress
```

**c) Get issue details**:
```bash
bd show <issue-id>
```
Extract title and description text only.

**d) Invoke TDD sub-agent**:

Use the Task tool to invoke `@tdd-implement` with this prompt:

```
## Issue: <issue-id>
**Title**: <issue title>

## Description
<issue description - text only, no JSON>

## ADR Context
<ADR_SUMMARY if epic, otherwise omit this section>

## Working Directory
<worktree path>

---

Implement this issue using TDD. Return JSON result when complete.
```

**e) Handle result**:

Parse the JSON response from the sub-agent.

**IF status == "success":**
- Close the issue:
  ```bash
  bd close <issue-id>
  ```
- Log: `✓ <issue-id>: <title>`
- For each item in `discovered_issues`:
  - If `in_scope` is true AND this is an epic:
    ```bash
    bd create "<title>" -t task --parent <epic-id> -d "<description>" --json
    ```
  - Otherwise:
    ```bash
    bd create "<title>" -t task -p 3 -d "<description>" --json
    ```
- Refresh queue:
  ```bash
  bd ready --json
  ```
  Filter to scope, update READY_QUEUE.

**IF status == "failure":**
- **STOP IMMEDIATELY**
- Report to user:
  ```
  ❌ Implementation failed on <issue-id>: <title>

  Error: <error from sub-agent>

  The worktree is preserved at <path>.
  Fix the issue manually, then run `/implement <issue-id>` to retry.
  ```
- **EXIT** - do not continue

**IF status == "blocked":**
- **STOP IMMEDIATELY**
- Report to user:
  ```
  ⚠️ Blocked on <issue-id>: <title>

  Reason: <error from sub-agent>

  Resolve the blocker and run `/implement <issue-id>` to retry.
  ```
- **EXIT** - do not continue

#### 5.3 Complete Epic

If scope is an epic and READY_QUEUE is empty:
- Verify all child issues are closed
- Close the epic:
  ```bash
  bd close <epic-id>
  ```

### 6. Finalize

#### Push and Create PR

```bash
git push -u origin <branch-name>
```

```bash
gh pr create --title "<scope title>" --body "$(cat <<'EOF'
## Summary
<Brief description>

## Issues Completed
- <issue-id>: <title>
  - <child-id>: <child title>  (if epic)
  ...

## ADR
See: docs/adr/NNNNNN-slug.md  (if epic, otherwise omit)
EOF
)"
```

Return the PR URL to the user.

### 7. Inform User

Tell the user:
- PR has been created
- They are still in the feature worktree
- All beads issues in scope have been closed
- Run `/implement-cleanup` when ready to remove worktree

---

## Important

- **TDD enforced**: Sub-agent writes tests before implementation
- **Stop on failure**: Do not continue if any issue fails
- **Context isolation**: Each issue implemented in separate sub-agent
- **ADR read once**: Summary passed to sub-agents, not full document
- **Discovered work**: Auto-created immediately as issues
- **Commit per issue**: Each issue results in one commit
- **Branch from current**: Always branch from the current branch
