---
description: Create a task, bug, or feature issue with a bullet-list plan
---

# Create Task

Quick task creation for work that doesn't need a full ADR. Creates a beads issue with a bullet-list implementation plan.

## Task Request

$ARGUMENTS

## Workflow

### 1. Understand the Request

- If the request is clear, proceed
- If ambiguous, ask ONE clarifying question before continuing
- Determine the issue type:
  - **bug**: Something is broken or behaving incorrectly
  - **task**: Refactoring, cleanup, documentation, infrastructure
  - **feature**: New functionality or enhancement

### 2. Quick Exploration

Do minimal codebase exploration to understand:
- Where the changes need to happen
- What existing code is affected
- Any obvious gotchas or dependencies

Keep this brief - this is not an architectural deep-dive.

### 3. Present the Plan

Present a concise plan to the user:

```
## <Title>

**Type**: bug | task | feature
**Priority**: 0-4

### Summary
<1-2 sentence description of what needs to be done>

### Implementation Steps
- [ ] Step 1
- [ ] Step 2
- [ ] Step 3
...

### Files Affected
- `path/to/file1.rs`
- `path/to/file2.rs`
```

**Priority Guidelines**:
- **P0**: Critical - blocks other work or broken in production
- **P1**: High - important feature or significant bug
- **P2**: Medium - normal work (default)
- **P3**: Low - nice to have
- **P4**: Backlog - someday/maybe

Wait for user approval or feedback.

### 4. Create the Issue

Once the user approves (e.g., "looks good", "create it", "yes"):

```bash
bd create "<Title>" \
    -t <type> \
    -p <priority> \
    -d "<Summary>

## Implementation Steps
- Step 1
- Step 2
- Step 3

## Files Affected
- path/to/file1.rs
- path/to/file2.rs" \
    --json
```

### 5. Confirm

Tell the user:
- The issue ID (e.g., `bd-a3f8`)
- They can run `/implement` to start working on it
- Or run `bd show <issue-id>` to view the issue details

---

## Important

- **Keep it simple** - this is for work that does NOT need an ADR
- **Single issue only** - no child issues; bullets go in the description
- **No worktree** - that's `/implement`'s job
- **No implementation** - just create the issue and stop
- **Complexity check** - if the work needs multiple phases or architectural decisions, suggest `/design` instead
