# Steve Klabnik-Inspired ADR Workflow for Claude Code

A complete command system for planning complex features with ADR documentation, breaking them into subtasks with Beads, and executing them incrementally.

## Directory Structure

```
.claude/
├── commands/
│   ├── planning/
│   │   ├── adr-plan.md          # Initiate planning & document as ADR
│   │   ├── adr-review.md        # Review existing ADRs
│   │   └── adr-approve.md       # Finalize & approve plan
│   ├── execution/
│   │   ├── subtasks.md          # Break plan into Beads issues
│   │   ├── next-step.md         # Execute next subtask
│   │   ├── verify.md            # Test & verify completion
│   │   └── close-task.md        # Mark task done
│   ├── utils/
│   │   ├── list-adrs.md         # Show all ADRs
│   │   ├── land-plane.md        # Cleanup & commit workflow
│   │   └── status.md            # Show workflow status
│   └── init-workflow.md         # One-time setup
├── CLAUDE.md                    # Main context & conventions
└── hooks/                       # Post-edit verification (optional)
```

## Setup: One-Time Initialization

### 1. Create `.claude/CLAUDE.md`

```markdown
# Project Context

## Build & Test Commands
- Build: `cargo build` or `npm run build`
- Test: `cargo test` or `npm test`
- Verify all tests pass before completing tasks

## Project Conventions
- ADRs stored in `docs/adr/`
- Beads issues in `.beads/`
- Branch naming: `feature/description`
- Commit messages: Imperative mood, reference ADR when applicable

## ADR Format
All ADRs use the template in `docs/adr/TEMPLATE.md`:
- Title (ADR-NNNN: Clear decision name)
- Status (Proposed, Accepted, Deprecated)
- Context (Problem & motivation)
- Decision (Selected approach & rationale)
- Consequences (Positive & negative impacts)
- Alternatives (Options considered & rejected)

## Verification & Testing
Every implementation must:
1. Pass all existing tests
2. Include new tests for new functionality
3. Follow coding conventions in CONTRIBUTING.md
4. Not violate constraints in existing ADRs

## Architecture Guardrails
- [Your project-specific constraints]
- Example: "Use SQLite for persistence, never MongoDB"
- Example: "All network I/O must be async"
- Example: "Module system must support cyclic dependencies"

## Context Efficiency Tips
- Load specs, not raw files. Use structured documents.
- Plan before execute. Use plan mode (Shift+Tab).
- Test immediately. Each feature gets verification loop.
- Document decisions. ADRs prevent "did we decide this?" arguments.
```

### 2. Create `docs/adr/TEMPLATE.md`

```markdown
# ADR-NNNN: [Title]

**Status**: Proposed | Accepted | Deprecated | Superseded

**Date**: YYYY-MM-DD

## Context

Describe the problem or decision that prompted this architectural choice.
What constraints or requirements drove this?
What alternatives were considered at the time?

## Decision

What decision did we make? Be specific and clear.
Include rationale: Why this approach over alternatives?
What does this enable or prevent?

## Consequences

### Positive
- Consequence 1
- Consequence 2

### Negative
- Consequence 1
- Consequence 2

## Alternatives Considered

### Alternative 1: [Title]
**Pros**: ...
**Cons**: ...

### Alternative 2: [Title]
**Pros**: ...
**Cons**: ...

## Related ADRs
- ADR-0001: Previous decision this builds on
- ADR-0003: Future decision affected by this

## Supersedes / Superseded By
If this deprecates previous decisions, list them.
If this gets superseded later, link to replacement.
```

### 3. Initialize Beads (if not already done)

```bash
# One time only
beads init

# Create initial issue for your feature
beads create --title "Feature: [Name]" --description "[Brief description]"
```

---

## Command 1: `/project:planning:adr-plan` — Initiate Planning & Create ADR

**File**: `.claude/commands/planning/adr-plan.md`

```markdown
---
description: "Plan a feature end-to-end, document decision in ADR, get approval"
allowed-tools: ["read", "write", "execute"]
---

# ADR-Based Planning Command

You are in **PLAN MODE** (read-only, analysis phase).

## Your Task

Plan the implementation of: **$ARGUMENTS**

### Planning Process

1. **Read & Understand**
   - Read CLAUDE.md to understand project constraints
   - List all existing ADRs in `docs/adr/` that relate to this feature
   - Read the issue description from `.beads/issues.jsonl` if available

2. **Identify Architectural Decisions**
   - What architectural question does this feature raise?
   - Does it conflict with any existing ADRs?
   - What's the simplest approach that respects all constraints?

3. **Generate Implementation Plan**
   - Break into logical phases (parsing, analysis, codegen, testing, integration)
   - For each phase: list files to create/modify, key algorithms, test cases
   - Estimate: "This requires N subtasks"
   - List dependencies between phases

4. **Document Decision (Draft ADR)**
   - Generate a draft ADR for the architectural decision
   - Status: "Proposed"
   - Reference existing ADRs as context
   - Include alternatives considered (why NOT the other approaches?)
   - DO NOT finalize yet — this is a draft for review

5. **Output**
   - Show your comprehensive step-by-step plan
   - Display the draft ADR as a code block
   - Ask the human to review:
     - "Does the plan align with project architecture?"
     - "Do you approve the architectural decision?"
     - "Any changes before we proceed?"

### Keep This In Mind
- You are READING ONLY in this phase. Do not write code or modify files.
- Reference specific ADRs by filename (ADR-0001.md, ADR-0003.md)
- Be explicit about why each phase is necessary
- Flag any potential conflicts with existing constraints
```

---

## Command 2: `/project:planning:adr-approve` — Finalize & Approve ADR

**File**: `.claude/commands/planning/adr-approve.md`

```markdown
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
```

---

## Command 3: `/project:execution:subtasks` — Break Into Beads Issues

**File**: `.claude/commands/execution/subtasks.md`

```markdown
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
   beads create --title "Subtask 1: Parser phase" \
     --description "Parse feature X, output AST"
   
   beads create --title "Subtask 2: Typechecker phase" \
     --description "Type-check feature X AST" \
     --blocks "Subtask 3"  # Blocks the codegen step
   
   beads create --title "Subtask 3: Codegen phase" \
     --description "Generate code for feature X"
   
   beads create --title "Subtask 4: Test suite" \
     --description "Write tests covering all cases" \
     --related "Subtask 1"  # Thematic link, not blocking
   ```

4. **Verify Graph**
   - Run: `beads ready` — shows next actionable tasks
   - Should show subtasks with zero unmet dependencies
   - If all are blocked, cycle exists; fix and retry

5. **Output**
   - Show the created issues (IDs and titles)
   - Show: `beads ready` output
   - Next: Use `/project:execution:next-step <issue-id>`
```

---

## Command 4: `/project:execution:next-step` — Implement One Subtask

**File**: `.claude/commands/execution/next-step.md`

```markdown
---
description: "Execute one subtask (or simple feature) with verification loop"
allowed-tools: ["read", "write", "execute"]
---

# Execute Next Step (Verification Loop)

## Context

You are implementing: **$ARGUMENTS**

If no argument provided, run `beads ready` to find the next actionable task.

## Two-Phase Execution

### Phase 1: Understand & Code

1. **Read Context**
   - Read the issue description from Beads or from the plan
   - Read the relevant ADR in `docs/adr/`
   - Identify: What files will change? What algorithms needed?

2. **Implement**
   - Write code that satisfies the issue
   - Follow conventions from CLAUDE.md
   - Reference the ADR: "Implementing ADR-NNNN: [decision]"
   - Keep changes focused to this issue (don't scope creep)

3. **Write Tests**
   - Unit tests for new functions
   - Integration tests if needed
   - Tests should verify the issue is resolved

### Phase 2: Verify & Iterate

4. **Run Tests**
   ```bash
   # Run all tests
   cargo test  # or npm test
   
   # Or run specific test
   cargo test test_name
   ```

5. **Check Results**
   - All tests pass? ✓ Move to step 7
   - Tests fail? Go to step 6
   - New failures? Investigate

6. **Fix Failures**
   - Read the test failure
   - Understand why the code doesn't match the test
   - Update code
   - Rerun tests
   - Loop until tests pass

7. **Verify Against Issue**
   - Does the code solve the issue?
   - Does it respect all ADR constraints?
   - Are edge cases handled?
   - Code quality OK?

8. **Output**
   - Show: Test results (all passing)
   - Show: Files modified
   - Show: Summary of what was implemented
   - Ask: Ready to close this issue and move to next?
```

---

## Command 5: `/project:execution:close-task` — Mark Task Complete

**File**: `.claude/commands/execution/close-task.md`

```markdown
---
description: "Mark a Beads issue complete and move to next task"
allowed-tools: ["execute", "read"]
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

2. **Update Beads**
   ```bash
   beads close $ARGUMENTS --reason "Implemented and tested"
   
   # Show the update
   beads status $ARGUMENTS
   ```

3. **Check What's Ready Now**
   ```bash
   beads ready
   ```
   Tasks that were blocked on this one may now be ready.

4. **Suggest Next Task**
   - Show the highest-priority ready task
   - Suggest: "Next: `/project:execution:next-step <next-issue-id>`"

5. **Output**
   - Show closed task confirmation
   - Show next actionable tasks
   - Show progress: "X of N subtasks complete"
```

---

## Command 6: `/project:utils:list-adrs` — Show ADR History

**File**: `.claude/commands/utils/list-adrs.md`

```markdown
---
description: "List all ADRs and their current status"
allowed-tools: ["execute", "read"]
---

# List All ADRs

Show all architectural decision records and their status.

## Task

1. List all ADRs
   ```bash
   ls -la docs/adr/
   ```

2. For each ADR, show:
   - Filename (ADR-NNNN.md)
   - Title (extracted from markdown)
   - Status (Proposed, Accepted, Deprecated, Superseded)
   - Date

3. Show relationships
   - Which ADRs reference each other?
   - Which are deprecated/superseded?

4. Output as a formatted table
   - Makes it easy to see what decisions have been made
   - Highlights active constraints

## Usage

Run at any time to understand what architectural decisions are currently active.
```

---

## Command 7: `/project:utils:land-plane` — Cleanup & Commit Workflow

**File**: `.claude/commands/utils/land-plane.md`

```markdown
---
description: "Land the plane: commit progress, identify next session work"
allowed-tools: ["execute", "read", "write"]
---

# Land the Plane

End-of-session cleanup and summary for next developer/session.

## Task

1. **Check Status**
   - Show current git status: `git status`
   - Show Beads progress: `beads ready` and `beads list --status=closed`

2. **Commit Progress**
   ```bash
   git add .
   git commit -m "Progress checkpoint: [brief summary of work done]"
   ```

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
```

---

## Command 8: `/project:utils:status` — Show Workflow Status

**File**: `.claude/commands/utils/status.md`

```markdown
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
   beads ready
   beads list --status=closed | wc -l  # count closed
   beads list | wc -l  # total
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
```

---

## Usage Workflow

### Complete Session Flow

```bash
# 1. Start a feature
/project:planning:adr-plan Implement feature X

# 2. Review plan, approve ADR
[Human reviews]
/project:planning:adr-approve

# 3. If complex, break into tasks
/project:execution:subtasks

# 4. Execute tasks one by one
/project:execution:next-step <issue-1>
/project:execution:close-task <issue-1>
/project:execution:next-step <issue-2>
/project:execution:close-task <issue-2>

# 5. Check status anytime
/project:utils:status

# 6. End of session
/project:utils:land-plane

# 7. Next session, check what's ready
/project:utils:status
/project:execution:next-step <next-ready-issue>
```

### Quick Feature (No Subtasks)

```bash
# Simple features skip the subtask step
/project:planning:adr-plan Implement simple feature Y
[Human approves]
/project:planning:adr-approve
/project:execution:next-step  # No issue ID = use from plan
/project:execution:close-task  # Simple features don't need Beads
/project:utils:land-plane
```

---

## Key Principles

1. **Plan Always Comes First**
   - Never implement without planning in ADR
   - Plan Mode prevents hallucination through read-only analysis

2. **ADRs Are Checkpoints**
   - Every architectural decision gets documented
   - Prevents "I thought we decided..." arguments
   - Future features reference ADRs as constraints

3. **Beads = Task Memory**
   - Distributed across sessions
   - Synced via Git
   - No agent/human memory loss

4. **Verification Loop = Quality Guarantee**
   - Tests before completion
   - Failures addressed immediately
   - No broken commits

5. **Decomposition Is Optional**
   - Simple changes: Plan → Execute → Done
   - Complex changes: Plan → Subtasks → Execute each → Done
   - Decide based on scope and risk

---

## Integration with Your Project

1. Copy these commands to `.claude/commands/` in your project
2. Customize CLAUDE.md with your project's conventions
3. Create docs/adr/TEMPLATE.md in your repo
4. Run `/project:init-workflow` first time (creates directories)
5. Start with `/project:planning:adr-plan <your feature>`

---

## Optional: Verify Hook (Advanced)

If you want Claude to auto-verify tests after edits, create:

**File**: `.claude/hooks/verify_tests.py`

```python
#!/usr/bin/env python3
import os
import subprocess
import sys

# After any code edit, run tests
# This hook prevents committing broken code

def run_tests():
    """Run test suite"""
    try:
        # Adjust for your project (cargo test, npm test, pytest, etc)
        result = subprocess.run(
            ["cargo", "test"],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        if result.returncode != 0:
            print("❌ Tests failed after edit!")
            print(result.stdout)
            print(result.stderr)
            return False
        
        print("✅ Tests passing")
        return True
    except Exception as e:
        print(f"⚠️ Could not run tests: {e}")
        return True  # Don't block on hook errors

if __name__ == "__main__":
    if not run_tests():
        sys.exit(1)
```

---

## Tips & Troubleshooting

### "My plan doesn't fit the architecture"
→ Go back to Plan Mode, adjust approach to respect ADRs
→ Create a new ADR for the new decision
→ Replan and get approval

### "A subtask is blocked but shouldn't be"
→ Check Beads dependencies: `beads list --depends=<issue-id>`
→ Update blocking relationship if wrong: `beads update <id> --no-blocks <other-id>`

### "Tests are failing in the middle of implementation"
→ Normal. Fix the failing test or code, then rerun
→ Don't close task until all tests pass

### "Two features conflict architecturally"
→ Check ADRs - there may be a constraint violated
→ Create an ADR to resolve the conflict
→ Replan features accordingly

### "The session is getting too long"
→ Use `/project:utils:land-plane` to save progress
→ Create `.session-summary.md` for next developer
→ Come back fresh next session
