---
description: "Run verification tests after implementation"
allowed-tools: ["execute", "read"]
---

# Verify Implementation

Run verification tests to ensure implementation meets quality gates.

## Your Task

Verify the current implementation meets all quality standards.

### Verification Steps

1. **Run All Tests**
   ```bash
   cargo test
   ```
   All tests must pass.

2. **Run Clippy**
   ```bash
   cargo clippy -- -D warnings
   ```
   No warnings allowed.

3. **Build in Release Mode**
   ```bash
   cargo build --release
   ```
   Must succeed without errors.

4. **Check Coverage (Optional)**
   - Are new features covered by tests?
   - Are edge cases tested?

### Output

Show results for each verification step:
```
✓ Tests: All passing (X tests)
✓ Clippy: No warnings
✓ Build: Success
```

If any verification fails:
```
✗ Tests: 3 failures
✗ Clippy: 2 warnings
```

And provide details about what needs to be fixed.
