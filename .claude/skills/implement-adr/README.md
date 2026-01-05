# Implement ADR Skill

This skill implements an already-approved ADR. Planning and approval are separate steps.

## Prerequisites

Before using this skill, you must:
1. Plan the feature: `/project:planning:adr-plan <feature description>`
2. Review and approve the plan
3. Finalize the ADR: `/project:planning:adr-approve`

## Usage

After ADR is approved:
```bash
/implement-adr ADR-0001
# OR
/implement-adr String interpolation support
```

## What It Does

The skill automates the implementation phase only:

### 1. Read ADR
- Locates the approved ADR in `docs/adr/`
- Verifies status is "Accepted"
- Extracts implementation details

### 2. Assess Complexity
- Simple feature? → Implements directly
- Complex feature? → Creates Beads subtasks

### 3. Implementation
- Assesses complexity (simple vs complex)
- For complex features: creates Beads subtasks with dependencies
- For simple features: implements directly
- For each subtask:
  - Implements code with tests
  - Runs verification loop (retries on failure, max 3 attempts)
  - Commits changes
  - Closes subtask in Beads
  - Moves to next subtask

### 4. Final Verification
- Runs all quality gates:
  - `cargo test` (all tests must pass)
  - `cargo clippy -- -D warnings` (no warnings)
  - `cargo build --release` (clean build)

## Human Interaction Points

The skill will STOP and wait for your input:

1. **If ADR not found or not approved**: You must run planning commands first
2. **After 3 failed test attempts**: If tests fail 3 times on same subtask
3. **On blocking errors**: Issues that require architectural decisions

## Commit Strategy

Every completed subtask gets its own commit:
```
* Complete subtask: Parser phase (Addresses: #123, ADR-0001)
* Complete subtask: Semantic analysis (Addresses: #124, ADR-0001)
* Complete subtask: Codegen (Addresses: #125, ADR-0001)
* ADR-0001: Implement async/await support
```

## Quality Guarantees

- No broken commits (tests must pass before commit)
- Each subtask is independently verifiable
- ADR provides architectural context for all changes
- Full test coverage required

## Example Session

### Step 1: Planning (Separate)
```bash
$ /project:planning:adr-plan Add string interpolation support

[PLANNING MODE - READ ONLY]
✓ Read CLAUDE.md
✓ Reviewed existing ADRs (3 related)
✓ Generated implementation plan (4 phases)
✓ Drafted ADR-0001: String Interpolation

--- Draft ADR ---
[ADR content shown here]

Do you approve this plan? (yes/no)

> yes

$ /project:planning:adr-approve

[FINALIZING ADR]
✓ Saved docs/adr/ADR-0001.md
✓ Committed: "ADR-0001: String Interpolation"

Ready for implementation. Run: /implement-adr ADR-0001
```

### Step 2: Implementation (Automated)
```bash
$ /implement-adr ADR-0001

[IMPLEMENTING ADR-0001]
✓ Read ADR: String Interpolation Support
✓ Status verified: Accepted
✓ Implementation plan understood

[COMPLEXITY ASSESSMENT]
Feature assessed as: Complex (4 subtasks)
✓ Created Beads issues

[Subtask 1/4] Parser: string interpolation syntax
  ✓ Modified src/compiler/parser.rs
  ✓ Added AST node for interpolation
  ✓ Tests pass (12 new tests)
  ✓ Committed

[Subtask 2/4] Semantic analysis
  ✓ Modified src/compiler/semantic.rs
  ✓ Tests pass (8 new tests)
  ✓ Committed

[Subtask 3/4] Code generation
  ✓ Modified src/compiler/codegen.rs
  ✓ Emits OpInterpolate opcode
  ✓ Tests pass (15 new tests)
  ✓ Committed

[Subtask 4/4] VM execution
  ✓ Modified src/vm/impl.rs
  ✓ Implemented OpInterpolate handler
  ✓ Tests pass (10 new tests)
  ✓ Committed

[FINAL VERIFICATION]
✓ cargo test: All pass (125 tests)
✓ cargo clippy: No warnings
✓ cargo build --release: Success

[COMPLETE] String interpolation implemented successfully!

Git log:
  abc1234 Complete subtask: VM execution (ADR-0001)
  def5678 Complete subtask: Code generation (ADR-0001)
  9abcdef Complete subtask: Semantic analysis (ADR-0001)
  1234567 Complete subtask: Parser phase (ADR-0001)
  89abcde ADR-0001: String Interpolation
```

## Comparison with Manual Workflow

### Full Workflow with Skill

**Step 1: Planning (Manual)**
```bash
/project:planning:adr-plan Add feature X
# [Review draft ADR]
/project:planning:adr-approve
```

**Step 2: Implementation (Automated)**
```bash
/implement-adr ADR-NNNN
# [Watch it execute automatically]
```

### Full Workflow with Individual Commands

**Step 1: Planning (Manual)**
```bash
/project:planning:adr-plan Add feature X
# [Review draft ADR]
/project:planning:adr-approve
```

**Step 2: Implementation (Manual)**
```bash
/project:execution:subtasks
/project:execution:next-step 123
/project:execution:close-task 123
/project:execution:next-step 124
/project:execution:close-task 124
# [Repeat for each subtask...]
```

The skill automates the repetitive implementation loop while keeping planning as an explicit, thoughtful step.
