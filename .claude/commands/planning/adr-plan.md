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
