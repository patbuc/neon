---
description: "Break the plan into Beads subtasks (only if needed)"
allowed-tools: ["read", "write", "execute"]
---

# Break Plan Into Subtasks (Conditional)

## Decision Point

Is this feature **simple** (< 30 minutes of work)?
- Simple → Skip this step. Implement directly with `/project:execution:next-step`
- Complex → Break into subtasks. Continue below.

## If Complex: Create Beads Issues

You are designing a dependency graph of subtasks.

### Steps

1. **Read the ADR**
   - Read the most recent ADR in `docs/adr/`
   - Extract the "Implementation Plan" sections
   - Identify logical boundaries between phases

2. **Design Subtask Graph**
   - Each subtask = one focused, testable unit of work
   - Can subtask B start before subtask A completes? If yes: not blocked
   - If B depends on A: B should specify `--blocks` or A should specify `--blocks B`
   - Total subtasks: 3-7 is typical. 10+ suggests over-decomposition

3. **Create Beads Issues**
   ```bash
   bd create --title "Subtask 1: Parser phase" \
     --description "Parse feature X, output AST"

   bd create --title "Subtask 2: Typechecker phase" \
     --description "Type-check feature X AST" \
     --blocks "Subtask 3"  # Blocks the codegen step

   bd create --title "Subtask 3: Codegen phase" \
     --description "Generate code for feature X"

   bd create --title "Subtask 4: Test suite" \
     --description "Write tests covering all cases" \
     --related "Subtask 1"  # Thematic link, not blocking
   ```

4. **Verify Graph**
   - Run: `bd ready` — shows next actionable tasks
   - Should show subtasks with zero unmet dependencies
   - If all are blocked, cycle exists; fix and retry

5. **Output**
   - Show the created issues (IDs and titles)
   - Show: `bd ready` output
   - Next: Use `/project:execution:next-step <issue-id>`
