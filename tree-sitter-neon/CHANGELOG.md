# Changelog

All notable changes to tree-sitter-neon will be documented in this file.

## [0.1.0] - 2025-12-06

### Added
- Initial Tree-sitter grammar for Neon language
- Full syntax support for all language features
- String interpolation with external scanner
- Syntax highlighting queries (highlights, indents, injections, locals)
- Comprehensive test suite with 11 corpus tests
- Integration tests (59/70 test files parsing successfully)
- CI/CD pipeline with GitHub Actions
- Performance benchmarks (8600+ lines/second)
- Zed editor extension
- Documentation (README, TESTING, CONTRIBUTING)

### Features
- All expression types with correct precedence
- Control flow statements (if, while, for, for-in)
- Function and struct declarations
- Collections (arrays, maps, sets)
- Range operators (.., ..=)
- Postfix operators (++, --)
- Method calls and field access
- String interpolation with embedded expressions
- Comments

### Testing
- 100% corpus test pass rate (11/11)
- 84% integration test pass rate (59/70)
- Performance: 5001 lines in 579ms
- Automated CI/CD pipeline

[0.1.0]: https://github.com/patbuc/neon/releases/tag/tree-sitter-neon-v0.1.0
