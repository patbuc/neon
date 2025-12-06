# Contributing to tree-sitter-neon

Thank you for your interest in contributing! This guide will help you get started.

## Development Setup

1. **Fork and clone:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/neon.git
   cd neon/tree-sitter-neon
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Generate parser:**
   ```bash
   npx tree-sitter generate
   ```

## Making Changes

### Modifying the Grammar

1. Edit `grammar.js`
2. Regenerate: `npx tree-sitter generate`
3. Test: `npx tree-sitter test`

### Adding Tests

1. Add test cases to `test/corpus/*.txt`
2. Follow the format:
   ```
   ================================================================================
   Test name
   ================================================================================

   input code

   --------------------------------------------------------------------------------

   (expected_ast)
   ```
3. Run: `npx tree-sitter test`

### Updating Highlights

1. Edit `queries/highlights.scm`
2. Test: `npx tree-sitter highlight test_comprehensive.n`

## Testing

Run all tests before submitting:

```bash
./test-all.sh
```

Ensure:
- All corpus tests pass
- Real Neon files parse correctly
- Highlighting works

## Submitting Changes

1. Create a feature branch: `git checkout -b feature/my-feature`
2. Make your changes
3. Add tests for new features
4. Run tests: `./test-all.sh`
5. Commit with clear messages
6. Push to your fork
7. Create a pull request

## Code Style

- Follow existing grammar patterns
- Use descriptive names for rules
- Comment complex grammar rules
- Keep precedence consistent with Neon compiler

## Questions?

Open an issue or discussion on GitHub!
