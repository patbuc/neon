---
description: "Design a feature or task: create ADR or Task doc, get approval, finalize"
allowed-tools: ["read", "write", "execute", "bash"]
---

# Design Command

You are in **DESIGN MODE** - planning and architectural decision phase.

## Your Task

Design the implementation of: **$ARGUMENTS**

## Workflow

### 1. Understand Context

**Read project constraints:**
- Read `CLAUDE.md` to understand project architecture and conventions
- Review `docs/adr/TEMPLATE.md` for ADR format (if applicable)

**Survey existing docs:**
```bash
ls -la docs/adr/
ls -la docs/tasks/ 2>/dev/null || echo "No tasks directory yet"
```

### 2. Assess Type & Scope

Determine if this is an **Architectural Change (ADR)** or a **Task/Improvement**.

**A. Architectural Decision Record (ADR)**
- **Criteria**: New language features, significant architectural changes, new dependencies, major refactors.
- **Output**: `docs/adr/ADR-NNNN.md`
- **Examples**: Closures, pattern matching, new bytecode format.

**B. Task Plan**
- **Criteria**: Bug fixes, minor improvements, refactoring existing code without architectural change, implementing known patterns.
- **Output**: `docs/tasks/TASK-NNNN.md` (or similar ID scheme)
- **Examples**: Fix memory leak, add `Math.ceil()`, optimize scanner.

### 3. Design Process

#### For ADRs (Architectural Changes)

1. **Determine next ADR number:**
   ```bash
   ls -1 docs/adr/ADR-*.md | tail -1
   ```
   Next number = highest + 1

2. **Draft ADR:**
   Follow `docs/adr/TEMPLATE.md`. Key sections: Context, Decision, Consequences, Alternatives, Implementation Plan.

#### For Tasks (Improvements/Fixes)

1. **Determine Task ID:**
   If linked to a Beads issue, use that ID (e.g., `TASK-123`). Otherwise, generate a timestamp or sequential ID.

2. **Draft Task Plan:**
   Create a markdown file in `docs/tasks/` (create directory if needed).
   
   Structure:
   ```markdown
   # TASK-ID: [Title]
   
   **Status:** Proposed
   **Date:** [Date]
   
   ## Context
   [What needs to be done and why?]
   
   ## Plan
   [Step-by-step implementation plan]
   
   ## Verification
   [How will we test this?]
   ```

### 4. Review & Approve

**Present draft to user:**
- Show the complete draft (ADR or Task Plan).
- Explain the rationale.
- **Wait for explicit approval.**

### 5. Finalize

**After approval:**

1. **Update Status**: Change "Proposed" to "Accepted".
2. **Save File**:
   - ADR: `docs/adr/ADR-NNNN.md`
   - Task: `docs/tasks/TASK-ID.md`
3. **Commit**:
   ```bash
   git add docs/adr/ADR-NNNN.md  # or docs/tasks/...
   git commit -m "docs: Add [ADR-NNNN/TASK-ID] for [Title]"
   ```

### 6. Output Summary

**Show:**
- File path created.
- Commit hash.
- **Next steps:**
  - "Run `/implement [ADR-NNNN|TASK-ID]` to start implementation."

## Critical Constraints

- **Read-only analysis**: Do not write implementation code in this phase.
- **Architecture first**: Ensure alignment with existing ADRs.
- **Explicit approval**: Never finalize without user confirmation.
