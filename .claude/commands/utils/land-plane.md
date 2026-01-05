---
description: "Land the plane: commit progress, identify next session work"
allowed-tools: ["execute", "read", "write"]
---

# Land the Plane

End-of-session cleanup and summary for next developer/session.

## Task

1. **Check Status**
   - Show current git status: `git status`
   - Show Beads progress: `bd ready` and `bd list --status=closed`

2. **Commit Any Uncommitted Changes (If Any)**
   ```bash
   # Check if there are uncommitted changes
   git status

   # If changes exist (shouldn't normally, as subtasks are committed on close):
   git add .
   git commit -m "Work in progress: [brief summary of incomplete work]"
   ```
   Note: If the workflow is followed correctly, each completed subtask should already be committed via `/project:execution:close-task`.

3. **Create Summary Document**
   - Create `.session-summary.md` with:
     - What was completed in this session
     - What Beads issues are still open
     - What's the next priority (highest-priority ready task)
     - Any blockers or challenges encountered
     - Reference to the ADR(s) that guided this work

4. **Show Next Steps**
   - What feature should be worked on next?
   - What command to use: `/project:execution:next-step <issue-id>`

5. **Commit Summary**
   ```bash
   git add .session-summary.md
   git commit -m "Session summary: [brief description]"
   ```

## Output
- Summary of session work
- Next developer's starting point
- Git log showing clean commit history
