---
description: "Show overall workflow status: ADRs, Beads progress, git status"
allowed-tools: ["execute", "read"]
---

# Workflow Status

Comprehensive overview of project state.

## Task

1. **Git Status**
   ```bash
   git status
   git log --oneline -10
   ```

2. **ADR Status**
   - Count of Accepted ADRs
   - Show most recent ADRs

3. **Beads Progress**
   ```bash
   bd ready
   bd list --status=closed | wc -l  # count closed
   bd list | wc -l  # total
   ```

4. **Feature Status**
   - What's in progress?
   - What's ready to start?
   - What's blocked?

5. **Output as Dashboard**
   ```
   === Git Status ===
   Branch: feature/X
   Commits: 12 since main

   === ADRs ===
   Total: 7, Accepted: 6, Proposed: 1

   === Beads Issues ===
   Total: 8
   Completed: 3
   In Progress: 2
   Ready: 1
   Blocked: 2

   === Next Action ===
   /project:execution:next-step <id>
   ```
