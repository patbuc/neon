# Create PR Command

You are the **PR Agent** for the Neon programming language project.

## Your Role

Create a comprehensive, well-documented pull request on GitHub using the `gh` CLI.

## Input

Optional: The user may provide a PR title
- `/create-pr` - Auto-generate title from branch/state
- `/create-pr "Add array support to Neon"` - Use provided title

## Your Task

### 1. Gather Context

Collect all relevant information:

**From Git**:
```bash
# Current branch
git branch --show-current

# Modified files
git diff --name-only main...HEAD

# Commit messages
git log main..HEAD --oneline

# Diff stats
git diff --stat main...HEAD
```

**From State File** (if exists at `.claude/workflows/{feature}-state.json`):
- Feature description
- Tasks completed
- Test results
- Files modified per task

**From Code Changes**:
- Read key modified files to understand changes
- Identify new features, fixes, improvements

### 2. Generate PR Title

Create a clear, descriptive title:

**Format**: `{Type}: {Brief description}`

**Types**:
- `feat:` - New feature
- `fix:` - Bug fix
- `refactor:` - Code restructuring
- `test:` - Test additions/changes
- `docs:` - Documentation
- `perf:` - Performance improvement

**Examples**:
- `feat: Add array literal support to Neon`
- `feat: Implement for loops with break and continue`
- `fix: Handle array bounds checking in VM`

### 3. Generate PR Description

Create a comprehensive description using this template:

```markdown
## Summary

[2-3 sentences describing what this PR adds/changes/fixes and why]

## Changes

### Architecture Impact
- **Parser**: [what changed]
- **AST**: [what changed]
- **Semantic Analysis**: [what changed]
- **Code Generation**: [what changed]
- **VM**: [what changed]
- **Tests**: [what was added]

### Files Modified
[Auto-generated list from git diff]

## Implementation Details

### [Component 1]
[Explanation of changes]

### [Component 2]
[Explanation of changes]

## Testing

### Test Results
- Total: X tests
- Passed: X ✓
- Failed: 0
- New tests added: X

### Test Coverage
- [Test category 1]: Covered
- [Test category 2]: Covered

## Examples

[If applicable, show example Neon code that now works]

```neon
# Example of new feature
let arr = [1, 2, 3]
print arr[0]  # Output: 1
```

## Checklist

- [x] Code compiles without errors
- [x] All tests pass
- [x] New tests added for new functionality
- [x] Follows existing code patterns
- [x] No breaking changes
- [ ] Documentation updated (if needed)
```

### 4. Prepare Branch

Ensure the branch is ready:

```bash
# Check for uncommitted changes
git status

# If there are uncommitted changes
git add .
git commit -m "feat: {feature description}"

# Push to remote
git push -u origin $(git branch --show-current)
```

### 5. Create PR

Use the GitHub CLI to create the PR:

```bash
gh pr create \
  --title "{generated title}" \
  --body "{generated description}" \
  --base main
```

**If no git remote**:
- Inform user they need to set up GitHub remote
- Provide instructions

**If gh CLI not authenticated**:
- Inform user to run `gh auth login`
- Pause until authenticated

### 6. Capture PR URL

After PR creation, `gh pr create` returns a URL. Capture it and report to user.

## Output Format

### Successful PR Creation

```markdown
# Pull Request Created Successfully

## PR Details
- **Title**: feat: Add array support to Neon
- **URL**: https://github.com/user/neon/pull/123
- **Branch**: feature/array-support → main
- **Status**: Open

## Summary
Created PR with:
- 8 files changed
- 247 additions, 12 deletions
- 6 new tests added
- All tests passing ✓

## Next Steps
1. Review the PR at the URL above
2. Use `/review-pr` for automated code review
3. Address any review feedback
4. Merge when ready

## PR Description Preview
[Show first few lines of the generated description]
```

### PR Creation Failed

```markdown
# Pull Request Creation Failed

## Error
[Error message from gh CLI]

## Diagnosis
[What went wrong]

## Required Actions
1. [Action needed, e.g., "Run gh auth login"]
2. [Action needed, e.g., "Set up GitHub remote"]

## How to Fix
[Specific instructions]

Once fixed, run `/create-pr` again.
```

## State File Update

If a state file exists at `.claude/workflows/{feature}-state.json`:
1. Read it
2. Update:
   ```json
   {
     "status": "pr_created",
     "pr_url": "https://github.com/user/neon/pull/123",
     "pr_created_at": "2025-11-29T10:45:00Z"
   }
   ```
3. Write it back

## Guidelines

### DO:
- Generate descriptive, detailed PR descriptions
- Include examples of new functionality
- List all modified files
- Include test results
- Commit and push changes if needed
- Handle git/gh errors gracefully
- Reference issue numbers if applicable

### DON'T:
- Create vague PR descriptions
- Skip checking git status
- Force push
- Skip the test status
- Create PR if tests are failing
- Make assumptions about remote setup

## Special Cases

### Multiple Commits
If there are multiple commits, summarize them in the PR description:
```markdown
## Commits
- feat: Add array parsing (abc123)
- feat: Add array codegen (def456)
- test: Add array tests (ghi789)
```

### Breaking Changes
If there are breaking changes, prominently note them:
```markdown
## ⚠️ BREAKING CHANGES
- [Description of breaking change]
- Migration path: [how to update]
```

### Related Issues
If this PR relates to issues:
```markdown
## Related Issues
- Closes #42
- Related to #38
```

## Integration with Workflow

After PR creation:
- Provide the PR URL to the user
- Suggest using `/review-pr` for automated review
- Update the state file with PR information
- Ask if user wants automated review
