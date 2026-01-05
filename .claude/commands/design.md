---
description: "Design a feature: create ADR, get approval, finalize"
allowed-tools: ["read", "write", "execute"]
---

# Feature Design Command

You are in **DESIGN MODE** - planning and architectural decision phase.

## Your Task

Design the implementation of: **$ARGUMENTS**

## Workflow

### 1. Understand Context

**Read project constraints:**
- Read `CLAUDE.md` to understand project architecture and conventions
- Review `docs/adr/TEMPLATE.md` for ADR format

**Survey existing ADRs:**
```bash
ls -la docs/adr/
```
- Identify related ADRs that constrain this feature
- Look for conflicts or dependencies

**Check issue context (if applicable):**
```bash
bd show $ARGUMENTS
```

### 2. Assess Scope

Categorize as **small** or **large**:

**Small features** (1-3 files, single session):
- Adding a builtin function
- New opcode for existing pattern
- Minor syntax sugar
- Example: Adding `Math.ceil()` function

**Large features** (multiple files, multiple sessions):
- New language constructs
- Semantic analysis changes
- Bytecode format changes
- Example: Adding closures, structs, pattern matching

### 3. Identify Architectural Decision

**What question does this feature raise?**
- How should this integrate with the VM stack model?
- Does this require new bytecode instructions?
- What's the compilation strategy (single-pass, multi-pass)?
- How does this interact with existing features?

**Constraints from existing ADRs:**
- Stack-based VM architecture (not register-based)
- Reference-counted objects (Rc<Object>)
- Bytecode compilation (not tree-walk)
- Educational clarity over performance optimization

### 4. Create Draft ADR

**Determine next ADR number:**
```bash
ls -1 docs/adr/ADR-*.md | tail -1
```
Next number = highest + 1

**Draft ADR structure:**
```markdown
# ADR-NNNN: [Feature Name]

**Status:** Proposed
**Date:** [Leave blank for now]

## Context

[Problem statement and motivation]
[Why does Neon need this feature?]
[What user pain point does it solve?]

## Decision

[Selected approach and rationale]
[Key implementation choices]
[How it fits into existing architecture]

## Consequences

### Positive
- [Benefit 1]
- [Benefit 2]

### Negative
- [Trade-off 1]
- [Trade-off 2]

## Alternatives Considered

### Alternative 1: [Name]
- [Description]
- Rejected because: [Reason]

### Alternative 2: [Name]
- [Description]
- Rejected because: [Reason]

## Implementation Plan

### Phase 1: [Scanner/Parser changes if needed]
- Files: src/compiler/scanner.rs, src/compiler/parser.rs
- Tasks: [Specific changes]

### Phase 2: [AST/Semantic analysis]
- Files: src/compiler/ast/*.rs, src/compiler/semantic.rs
- Tasks: [Specific changes]

### Phase 3: [Code generation]
- Files: src/compiler/codegen.rs, src/common/opcodes.rs
- Tasks: [Specific changes]

### Phase 4: [VM execution]
- Files: src/vm/impl.rs
- Tasks: [Specific changes]

### Phase 5: [Testing and integration]
- Files: tests/scripts/*.n
- Tasks: [Test cases to add]

### Dependencies
- Phase N depends on Phase M
- [Other dependencies]
```

**Present draft to user:**
- Show the complete draft ADR
- Explain the decision rationale
- Highlight any conflicts with existing constraints
- Ask for feedback

### 5. Get Approval

**Questions to ask:**
- "Does this align with Neon's architecture?"
- "Do you approve this architectural decision?"
- "Any changes before we finalize?"

**Wait for explicit approval** before proceeding.

### 6. Finalize ADR

**After approval:**

1. Update ADR:
   - Change Status: "Proposed" → "Accepted"
   - Set Date: YYYY-MM-DD (today)
   - Incorporate user feedback

2. Save ADR:
   ```bash
   # Write to docs/adr/ADR-NNNN.md
   # Create parent directories if needed
   mkdir -p docs/adr
   ```

3. Commit ADR:
   ```bash
   git add docs/adr/ADR-NNNN.md
   git commit -m "ADR-NNNN: [Title]

Describes the architectural decision for [feature].

Status: Accepted"
   ```

### 7. Output Summary

**Show:**
- ADR number and title
- Path: `docs/adr/ADR-NNNN.md`
- Commit hash
- Implementation complexity estimate

**Next steps:**
- For complex features: "Run `/implement ADR-NNNN` to start implementation"
- For simple features: "Run `/implement ADR-NNNN` for single-session implementation"

## Critical Constraints

- **Read-only analysis**: Do not write implementation code in this phase
- **No issue creation**: Wait for implementation phase to create Beads issues
- **Architecture first**: Ensure alignment with existing ADRs
- **Explicit approval**: Never finalize without user confirmation
- **Educational focus**: Prefer clarity over clever optimizations

## Small vs Large Decision Matrix

| Aspect | Small Feature | Large Feature |
|--------|---------------|---------------|
| Files touched | 1-3 | 4+ |
| ADR needed? | Optional | Required |
| Beads issues | Single issue | Epic with subtasks |
| Sessions | 1 | Multiple |
| Examples | Builtin function, simple opcode | Closures, pattern matching, modules |
