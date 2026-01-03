---
name: build-feature
description: Feature development workflow with planning, implementation, testing, and PR creation. Use when asked to build, implement, or add a new feature that requires multiple steps.
---

# Feature Development Workflow

You are orchestrating a comprehensive feature development workflow. Follow these phases exactly.

## Workflow State

Track the following state throughout:
- Current phase: planning | implementing | pr-creation
- Attempt counter: 1-3 (for implementation retries)
- Plan approval: pending | approved
- Quality gate status: pending | passed | failed

Use TodoWrite to maintain visible progress.

## Phase 1: Planning (Human Approval Required)

### Step 1.1: Exploration
Use the Task tool with subagent_type="Explore" to understand:
- Relevant existing code and patterns
- Files that will need modification
- Testing patterns used in the project

### Step 1.2: Plan Creation
Use the Task tool with subagent_type="Plan" to create a detailed implementation plan:
- Summary of changes
- Files to modify/create
- Implementation approach
- Testing strategy
- Potential risks

### Step 1.3: Human Approval
Present the plan using AskUserQuestion:
- Show the complete plan
- Ask for explicit approval to proceed
- If feedback given, iterate on plan
- Do NOT proceed to Phase 2 without explicit "approved" or "proceed" response

## Phase 2: Implementation Loop (Autonomous - Max 3 Attempts)

Set attempt_count = 1. Loop while attempt_count <= 3:

### Step 2.1: Implementation
Use Task tool to spawn feature-coder agent:
```
Task(
  subagent_type="general-purpose",
  prompt="[Provide full plan context + specific coding instructions]
          You are the feature-coder agent. Implement the following...
          Reference: .claude/agents/feature-coder.md for guidelines"
)
```

### Step 2.2: Testing
Use Task tool to spawn feature-tester agent:
```
Task(
  subagent_type="general-purpose",
  prompt="[Provide implementation context]
          You are the feature-tester agent. Test the implementation...
          Reference: .claude/agents/feature-tester.md for guidelines"
)
```

### Step 2.3: Code Review
Use Task tool to spawn feature-reviewer agent:
```
Task(
  subagent_type="general-purpose",
  prompt="[Provide implementation + test results]
          You are the feature-reviewer agent. Review the code...
          Reference: .claude/agents/feature-reviewer.md for guidelines"
)
```

### Step 2.4: Quality Gate
Run directly (not in subagent):
```bash
cargo build --release && cargo test && cargo clippy -- -D warnings
```

**On PASS**: Set quality_gate_status = passed, proceed to Phase 3
**On FAIL**:
- Increment attempt_count
- If attempt_count <= 3: Analyze failure, fix issues, repeat from Step 2.1
- If attempt_count > 3: STOP, notify user with detailed failure report

## Phase 3: PR Creation (Autonomous)

1. Create feature branch if not already on one
2. Stage all changes: `git add -A`
3. Commit with descriptive message (no AI watermarks per project conventions)
4. Push to remote: `git push -u origin <branch>`
5. Create PR: `gh pr create --title "..." --body "..."`

Include in PR body:
- Summary of changes
- Test plan
- Any notes from the review phase

## Error Recovery Protocol

If you cannot proceed autonomously at any point:

1. **Document the blocker**:
   - What was attempted
   - Specific error messages
   - What phase/step failed

2. **Assess severity**:
   - Recoverable: Try alternative approach
   - Needs clarification: Ask user via AskUserQuestion
   - Fundamental blocker: Stop and report

3. **Notify user** (triggers notification hook):
   - Clear description of the issue
   - Suggested solutions
   - Wait for input

## Important Constraints

- NEVER proceed past Phase 1 without explicit human approval
- Track ALL progress using TodoWrite
- STOP after 3 failed implementation attempts
- Git commits follow project conventions (no AI watermarks)
- Quality gate is non-negotiable: all three checks must pass
