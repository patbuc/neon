---
description: "Mark a Beads issue complete and move to next task"
allowed-tools: ["execute", "read", "write"]
---

# Close Completed Task

## Task

Mark issue **$ARGUMENTS** as complete and identify next actionable task.

### Steps

1. **Verify Tests Pass**
   ```bash
   cargo test  # or npm test
   ```
   If tests fail, fix first. Don't close incomplete work.

2. **Commit Changes**
   ```bash
   git add .
   git commit -m "Complete subtask: [Brief description of what was implemented]

   Addresses: $ARGUMENTS
   Related ADR: ADR-NNNN"
   ```
   - Use a clear, concise commit message
   - Reference the issue ID
   - Reference the related ADR if applicable
   - Follow conventional commit format if desired

3. **Update Beads**
   ```bash
   bd close $ARGUMENTS --reason "Implemented and tested"

   # Show the update
   bd status $ARGUMENTS
   ```

4. **Check What's Ready Now**
   ```bash
   bd ready
   ```
   Tasks that were blocked on this one may now be ready.

5. **Suggest Next Task**
   - Show the highest-priority ready task
   - Suggest: "Next: `/project:execution:next-step <next-issue-id>`"

6. **Output**
   - Show commit confirmation with commit hash
   - Show closed task confirmation
   - Show next actionable tasks
   - Show progress: "X of N subtasks complete"
