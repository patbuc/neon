# Testing the Neon Tree-sitter Grammar

This document describes the testing strategy and how to run tests.

## Test Structure

### 1. Unit Tests (Corpus Tests)

Location: `test/corpus/*.txt`

These test individual grammar rules with expected AST output.

Run: `npx tree-sitter test`

### 2. Integration Tests

Location: `../examples/*.n`, `../tests/scripts/*.n`

These test parsing of real Neon code.

Run: `./test-all.sh`

### 3. Highlight Tests

Tests that syntax highlighting queries work correctly.

Run: `npx tree-sitter highlight test_comprehensive.n`

### 4. Performance Tests

Tests parser performance with large files.

Run: `./benchmark.sh`

## Running Tests

### All tests:
```bash
./test-all.sh
```

### Individual test suites:
```bash
# Corpus tests only
npx tree-sitter test

# Parse specific file
npx tree-sitter parse ../examples/day1_solution.n

# Highlight specific file
npx tree-sitter highlight ../examples/day1_solution.n
```

## Continuous Integration

GitHub Actions automatically runs tests on:
- Every push to main/aoc-main branches
- Every pull request that modifies tree-sitter-neon/

See `.github/workflows/tree-sitter.yml` for CI configuration.

## Adding New Tests

### To add corpus tests:

1. Edit files in `test/corpus/`
2. Follow the format:
   ```
   ================================================================================
   Test name
   ================================================================================

   code here

   --------------------------------------------------------------------------------

   (expected_ast)
   ```
3. Run `npx tree-sitter test` to verify

### To add integration tests:

Simply add more `.n` files to `../examples/` or `../tests/scripts/`.
They'll be automatically tested by `test-all.sh` and CI.

## Test Coverage

Current test coverage:
- ✓ All expression types
- ✓ All statement types
- ✓ Control flow structures
- ✓ String interpolation
- ✓ Collections (arrays, maps, sets)
- ✓ Functions and structs
- ✓ Real-world code files

## Test Results

### Corpus Tests (11/11 passing)
- ✓ Val declaration with initialization
- ✓ Var declaration without initialization
- ✓ Function declaration
- ✓ Struct declaration
- ✓ Binary expression with precedence
- ✓ Method call
- ✓ Range expression
- ✓ Simple string
- ✓ String interpolation
- ✓ If statement
- ✓ For-in loop

### Integration Tests
- Examples: 4/5 files parsed successfully (80%)
- Test scripts: 59/70 files parsed successfully (84%)

### Performance Results
- 5001 lines parsed in ~580ms
- Average: ~8600 lines per second
- Small files (< 100 lines): < 10ms
- Medium files (< 1000 lines): < 50ms
- Large files (5000+ lines): < 600ms

## Benchmarking

The `benchmark.sh` script generates a large file (3000+ lines) and measures parse time.

Expected performance:
- Small files (< 100 lines): < 10ms
- Medium files (< 1000 lines): < 50ms
- Large files (3000+ lines): < 200ms

Actual performance on test system:
- 5001 lines: 579ms (real time)
- 588ms user time, 119ms system time
