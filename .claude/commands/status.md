---
description: "Show workflow status: git, ADRs, Beads progress"
allowed-tools: ["execute", "read"]
---

# Workflow Status Dashboard

Show comprehensive project status across all workflow dimensions.

## Your Task

Display current state of: **$ARGUMENTS**

Default: Show full dashboard (git + ADRs + Beads)

## Workflow

### 1. Git Status

```bash
# Current branch and status
git status

# Recent commits
git log --oneline -10

# Branch comparison
git rev-list --count HEAD ^main
```

**Display:**
- Current branch
- Uncommitted changes (staged/unstaged)
- Commits ahead of main
- Last commit hash and message

### 2. ADR Status

```bash
# List all ADRs
ls -1 docs/adr/ADR-*.md 2>/dev/null | wc -l

# Show recent ADRs
ls -lt docs/adr/ADR-*.md | head -5
```

**For each ADR, extract:**
- Number and title
- Status (Accepted, Proposed, Deprecated, Superseded)
- Date

**Categorize:**
- Total ADRs
- Accepted
- Proposed (awaiting approval)
- Deprecated/Superseded

**Display recent ADRs:**
```
ADR-0007: Default Function Parameters (Accepted, 2024-01-05)
ADR-0006: Closures and Upvalues (Accepted, 2024-01-03)
ADR-0005: Struct Instances (Accepted, 2024-01-02)
```

### 3. Beads Progress

**Check if beads is initialized:**
```bash
# Check if beads is active (regular or stealth mode)
bd list 2>/dev/null
```

**If beads is active:**
```bash
# Show ready tasks
bd ready

# Count by status
bd list | wc -l                    # total
bd list --status=closed | wc -l   # completed
bd list --status=open | wc -l     # open

# Show epics and their progress
bd list --kind=epic

# Detect mode
if [ -d .beads ]; then
  echo "Mode: Regular (committed tracking)"
else
  echo "Mode: Stealth (session-only tracking)"
fi
```

**If beads not initialized:**
- Display: "No active issue tracking"
- Note: "Use `/implement` with complex features to enable tracking"

**Calculate (if beads active):**
- Total issues
- Open issues
- Closed issues
- In-progress (if tracked)
- Blocked (has `blocks-on` dependencies not yet closed)
- Ready (no unresolved dependencies)
- Mode (Regular or Stealth)

**Group by epic (if applicable):**
```
Epic: Implement ADR-0007 (3/5 complete) [Stealth mode]
  ✓ Phase 1: Scanner changes (closed)
  ✓ Phase 2: Semantic analysis (closed)
  ✓ Phase 3: Code generation (closed)
  ○ Phase 4: VM execution (ready)
  ○ Phase 5: Testing (blocked on Phase 4)
```

### 4. Feature Status

Analyze current work:

**What's in progress?**
- Issues with recent activity
- Uncommitted changes related to which feature?

**What's ready to start?**
- Issues with no blocking dependencies
- Highest priority tasks

**What's blocked?**
- Issues waiting on dependencies
- Issues waiting on ADR approval

### 5. Test Status

```bash
# Quick test run
cargo test --quiet 2>&1 | tail -5
```

**Show:**
- Last test run result (pass/fail)
- Number of tests
- Any failing tests

### 6. Dashboard Output

```
=== Neon Workflow Status ===

Git:
  Branch: feature/default-params
  Ahead of main: 12 commits
  Uncommitted: 3 files modified
  Last commit: d23a6fd Remove unused new_function method

ADRs:
  Total: 7
  Accepted: 6
  Proposed: 1 (ADR-0008: Pattern Matching)

  Recent:
  - ADR-0007: Default Function Parameters (Accepted, 2025-01-05)
  - ADR-0006: Closures and Upvalues (Accepted, 2025-01-03)
  - ADR-0005: Struct Instances (Accepted, 2025-01-02)

Beads Issues: [Stealth Mode]
  Total: 8
  Completed: 5 ✓
  In Progress: 1 ⚙
  Ready: 1 ○
  Blocked: 1 ✗

  Active Epic: Implement ADR-0007 (5/5 complete)
    ✓ neon-42: Phase 1: Scanner changes
    ✓ neon-43: Phase 2: Semantic analysis
    ✓ neon-44: Phase 3: Code generation
    ✓ neon-45: Phase 4: VM execution
    ✓ neon-46: Phase 5: Integration tests

  Note: Session-only tracking (not committed)

Tests:
  Last run: PASSED (247 tests)
  Coverage: Good

Next Action:
  All subtasks complete for ADR-0007!
  Suggested: /ship neon-46

  Or start new feature:
  /design <feature description>
```

## Filtering

**Show specific component:**
```bash
/status git      # Git status only
/status adrs     # ADRs only
/status beads    # Beads issues only
/status tests    # Test status only
```

## When to Use

**Regular checkpoints:**
- Start of session: "Where did I leave off?"
- End of session: "What's the current state?"
- Before starting new work: "What's ready?"

**Planning:**
- Decide what to work on next
- See progress on current feature
- Identify blockers

**Reporting:**
- Share status with team
- Document progress

## Output Modes

**Full (default):**
- All sections included
- Detailed information

**Summary:**
```bash
/status --summary
```
- One-line status per component
- Quick overview

**Verbose:**
```bash
/status --verbose
```
- Include all Beads issues (not just summary)
- Show full git log
- List all ADRs

## Critical Information

**Always show:**
- Next actionable task
- Blocking issues
- Failing tests (if any)
- Uncommitted changes (risk of loss)

**Highlight problems:**
- Tests failing
- Blocked tasks with no progress path
- Proposed ADRs awaiting approval
- Uncommitted changes for > 1 day
