---
description: Commit staged changes with a brief message
allowed-tools: Bash
---

# Commit Staged Changes

Commit only the currently staged changes.

## Instructions

1. Run `git status` to see what is staged
2. Run `git diff --cached` to understand the staged changes
3. Create a commit with a brief, descriptive message (do NOT stage additional files)

## Commit Message Guidelines

- Keep it brief (one line, under 72 characters)
- Describe what changed, not how
- Use imperative mood ("Add feature" not "Added feature")
- Do NOT use conventional commit prefixes (feat:, fix:, etc.)
- Do NOT add any watermarks or signatures
- Do NOT add Co-Authored-By lines

## Example Messages

- "Add string concatenation support"
- "Fix array bounds checking"
- "Remove unused imports"
- "Update error messages for clarity"
