---
description: Design a feature and create an ADR through guided discussion
---

# Design Feature - Architecture Decision Record

You are a software architect helping design a new feature for the Neon programming language.

## Feature Request

$ARGUMENTS

## Workflow

This is a **multi-phase, conversational** design process. Do NOT write the ADR immediately.

### Phase 1: Research & Present

1. **Understand the Request**: Clarify what the user wants if ambiguous
2. **Explore the Codebase**: Use the explore agent to understand:
   - Relevant existing code and patterns
   - How similar features are implemented
   - The compilation pipeline (Scanner -> Parser -> Semantic -> Codegen -> VM)
3. **Analyze Approaches**: Consider viable implementation approaches
4. **Present Findings**: Share with the user:
   - Brief context summary (what you learned from exploring)
   - Your recommended approach (or options if genuinely unclear)
   - Any breaking changes or performance concerns
5. **Ask for Direction**: Wait for user input before proceeding

### Phase 2: Draft ADR

After the user agrees on the direction:

1. **Write Draft ADR**: Create the ADR following the template below
2. **Present for Review**: Show the complete ADR to the user
3. **Status**: Set status to `Proposed`
4. **Iterate**: If user requests changes, update the draft

**IMPORTANT**: The Implementation Plan must be detailed enough that each task can be implemented independently. Include:
- Specific files to modify
- Functions/structs to add or change
- Test cases to write
- Dependencies between tasks (what must be done first)

### Phase 3: Finalize

When the user **explicitly accepts** the ADR (e.g., "looks good", "accepted", "approve", "lgtm"):

1. **Determine ADR Number**: Scan `docs/adr/` for existing ADRs and use the next available 6-digit number (starting at 000001)
2. **Update Status**: Change status to `Accepted`
3. **Save ADR**: Write to `docs/adr/NNNNNN-feature-name.md` (kebab-case slug from title)
4. **Create Beads Epic**: Create the epic and all child issues (see Phase 4)
5. **Confirm**: Tell the user the ADR has been saved and the beads epic is ready for `/implement`

### Phase 4: Create Beads Epic

After saving the ADR, create a beads epic with child issues:

#### 4.1 Create the Epic

```bash
bd create "ADR-NNNNNN: <ADR Title>" \
    -t epic \
    -p 1 \
    -d "<Decision Outcome summary>" \
    --json
```

Save the epic ID (e.g., `bd-a3f8`).

#### 4.2 Create Child Issues for Each Task

For each task in the Implementation Plan, create a child issue:

```bash
bd create "<Task title>" \
    -t task \
    -p 2 \
    --parent <epic-id> \
    -d "<Detailed task description>

## Files to Modify
- path/to/file.rs: <what to change>

## Implementation Details
<Specific instructions from ADR>

## Tests
- <test cases to add>" \
    --json
```

#### 4.3 Model Dependencies

Add dependencies between tasks based on the Implementation Plan:

```bash
bd dep add <dependent-id> <dependency-id>
```

For example, if "Add parser support" depends on "Add token types":
```bash
bd dep add bd-a3f8.2 bd-a3f8.1
```

#### 4.4 Sync Issues

After creating all issues, sync to persist them:
```bash
bd sync
```

#### 4.5 Show Summary

After syncing, show a summary:
```bash
bd dep tree <epic-id>
```

---

## ADR Template

Use this exact structure:

```
# ADR-NNNNNN: [Title]

## Status
[Proposed | Accepted]

## Date
YYYY-MM-DD

## Context
[What is the problem or opportunity? Why is this decision needed?]

## Decision Drivers
- [Driver 1]
- [Driver 2]
- ...

## Decision Outcome
[The chosen approach and rationale - be specific about the technical solution]

## Consequences

### Breaking Changes / Migration
- [What existing behavior changes, if any]
- [Migration steps if needed]
- [Or: "None - this is a new feature with no breaking changes"]

### Performance Implications
- [Expected impact on compilation/runtime]
- [Or: "Negligible - no hot path changes"]

## Implementation Plan

[Detailed, actionable tasks. Each task should be implementable independently once its dependencies are complete.]

### Task 1: [Name]
**Depends on**: None (or list dependencies)
**Files**: `path/to/file.rs`

- [ ] Specific change 1
- [ ] Specific change 2
- [ ] Add tests for X

### Task 2: [Name]
**Depends on**: Task 1
**Files**: `path/to/file.rs`, `path/to/other.rs`

- [ ] Specific change 1
- [ ] Specific change 2
- [ ] Add tests for Y

### Task 3: [Name]
**Depends on**: Task 1, Task 2
**Files**: `path/to/file.rs`

- [ ] Specific change 1
- [ ] Add integration tests

[Continue for all tasks...]

## Test Plan

### Unit Tests
- [ ] Test case 1: description
- [ ] Test case 2: description

### Integration Tests
- [ ] `tests/scripts/feature_name.n`: description of what it tests
```

---

## Important Rules

- This is a **conversation** - do NOT skip to writing the ADR without discussing first
- Do NOT save the ADR file until the user explicitly accepts it
- Do NOT include a "Considered Options" section - only document the final decision
- **Implementation Plan must be detailed** - each task should have:
  - Clear title
  - Dependencies on other tasks (or "None")
  - Specific files to modify
  - Concrete checklist of changes
  - Test cases to add
- The ADR number is determined at save time, not before
- After saving the ADR, ALWAYS create the beads epic with all child issues and dependencies
- The beads issues are the source of truth for `/implement` - make them comprehensive
