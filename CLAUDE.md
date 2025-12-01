# Claude Code Instructions

This file contains custom instructions for Claude Code when working on this project.

## Git Commit Messages

**IMPORTANT**: Never add watermarks, signatures, or co-authorship attributions to commit messages.

When creating commits:
- Write clear, concise commit messages following conventional commit format
- DO NOT add "Generated with Claude Code" footers
- DO NOT add "Co-Authored-By: Claude" trailers
- DO NOT list changed files or obvious details visible in `git diff`
- Focus on the intent and high-level summary of WHY, not WHAT
- Keep commit messages professional and focused on the change itself

Example of correct commit message:
```
feat: Add array support to parser

Implement array literal parsing and validation
```

Example of incorrect commit messages (DO NOT DO THIS):
```
feat: Add array support to parser

Implement array literal parsing and validation

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

```
feat: Add array support to parser

Changes:
- Modified parser.rs to add array parsing
- Updated ast.rs with ArrayLiteral node
- Added tests in parser_tests.rs
```

## Summary

- ✅ Clean, professional commit messages focused on intent
- ❌ No watermarks or attribution footers
- ❌ No file lists or obvious details from git diff
