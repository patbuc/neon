---
description: "Execute one subtask (or simple feature) with verification loop"
allowed-tools: ["read", "write", "execute"]
---

# Execute Next Step (Verification Loop)

## Context

You are implementing: **$ARGUMENTS**

If no argument provided, run `bd ready` to find the next actionable task.

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
