---
description: "Finalize the ADR, save to docs/adr/, and prepare for execution"
allowed-tools: ["write", "read", "execute"]
---

# Approve & Finalize ADR

You are transitioning from **PLAN MODE** to **EXECUTION PHASE**.

## Task

Finalize the ADR that was just planned and save it to `docs/adr/`.

### Steps

1. **Get Human Confirmation**
   - Review the feedback from the human
   - Ask: "Should I proceed with these changes to the plan?"
   - Wait for approval

2. **Determine Next ADR Number**
   - Look at existing ADRs in `docs/adr/` (ls -la docs/adr/)
   - Find the highest number (e.g., ADR-0005.md)
   - Next ADR number = highest + 1

3. **Finalize ADR**
   - Change Status from "Proposed" to "Accepted"
   - Set Date to today's date (YYYY-MM-DD)
   - Ensure all sections are complete
   - Ensure all ADR format requirements are met

4. **Save ADR**
   - Write to `docs/adr/ADR-NNNN.md` (replace NNNN with actual number)
   - Create parent directories if needed
   - Commit: `git add docs/adr/ADR-NNNN.md && git commit -m "ADR-NNNN: [Title]"`

5. **Output Summary**
   - Show: "ADR-NNNN saved and committed"
   - Show the full ADR content for the record
   - Note: "Ready to break into subtasks with `/project:execution:subtasks`"
