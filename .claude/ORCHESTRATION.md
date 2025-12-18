# Neon Orchestration System

An automated multi-agent system for building features into the Neon programming language using Claude Code.

## Overview

This orchestration system uses Claude Code's native agent capabilities to manage the complete feature development lifecycle:

```
Planning → Implementation → Testing → Custom Review → PR Creation → Copilot Review → Address Feedback → Iteration
```

## Architecture

### Agents

The system consists of specialized agents, each with a specific role:

1. **Planner Agent** (`/plan-feature`)
   - Analyzes feature requirements
   - Explores codebase patterns
   - Clarifies architectural decisions with user
   - Asks questions about implementation approaches
   - Breaks features into atomic tasks

2. **Coding Agent** (`/implement-task`)
   - Implements specific tasks
   - Follows Neon's architecture
   - Ensures code compiles

3. **Testing Agent** (`/run-tests`)
   - Executes test suites
   - Analyzes failures
   - Provides actionable feedback

4. **PR Agent** (`/create-pr`)
   - Creates comprehensive PRs
   - Generates detailed descriptions
   - Uses gh CLI integration

5. **Review Agent** (`/review-pr`)
   - Performs code review
   - Checks quality, security, architecture
   - Provides structured feedback

6. **Orchestrator Agent** (`/build-feature`)
   - Coordinates all other agents
   - Manages workflow state
   - Handles iteration loops

### State Management

The system tracks workflow state in JSON files:

**Location**: `.claude/workflows/{feature-slug}-state.json`

**Contents**:
```json
{
  "feature": "Feature description",
  "branch": "feature/feature-name",
  "worktree_path": "<worktree-base>/feature-name",
  "status": "planning|coding|testing|reviewing|pr_created|copilot_reviewing|addressing_copilot_feedback|completed",
  "current_phase": "current phase name",
  "tasks": [
    {
      "id": 1,
      "description": "Task description",
      "status": "pending|in_progress|completed",
      "files_modified": ["file1.rs", "file2.rs"],
      "acceptance_criteria": ["criterion 1"],
      "test_strategy": "how to test"
    }
  ],
  "test_results": {
    "last_run": "ISO timestamp",
    "status": "passed|failed|build_failed",
    "total": 96,
    "passed": 94,
    "failed": 2,
    "failures": ["test_name1"],
    "duration_secs": 2.3
  },
  "pr_url": "https://github.com/user/neon/pull/123",
  "custom_review": {
    "performed_at": "ISO timestamp",
    "recommendation": "approve|request_changes|comment",
    "blocking_issues": 0,
    "suggestions": 2,
    "nitpicks": 3
  },
  "copilot_review": {
    "performed_at": "ISO timestamp",
    "review_id": 123456,
    "state": "approved|changes_requested|commented",
    "comments_count": 5,
    "suggestions_addressed": true
  },
  "iterations": 0,
  "created_at": "ISO timestamp"
}
```

## Prerequisites

### Git Worktrees

The orchestration system uses git worktrees to isolate feature development:

- **Worktree Location**: `<repo-parent>/neon-worktrees/{feature-slug}/` (configurable via `NEON_WORKTREE_BASE`)
- **Branch Naming**: `feature/{feature-slug}` (auto-generated from feature description)
- **Benefits**:
  - Isolated development environment per feature
  - No need to stash/commit when switching features
  - Parallel feature development possible
  - Main repo stays clean

The system automatically creates and manages worktrees.

### GitHub Copilot Setup

To use the GitHub Copilot code review integration:

1. **GitHub Copilot Subscription**: You need an active GitHub Copilot subscription (Individual, Business, or Enterprise)

2. **Install gh-copilot-review Extension** (Optional but recommended):
   ```bash
   gh extension install ChrisCarini/gh-copilot-review
   ```

3. **Alternative**: Enable automatic Copilot reviews in your repository settings
   - Go to repository Settings → Rules → Rulesets
   - Enable "Automatically request Copilot code review"

4. **Verify gh CLI Authentication**:
   ```bash
   gh auth status
   gh auth login  # If not authenticated
   ```

Without the extension, you can still manually add Copilot as a reviewer through the GitHub web UI.

## Configuration

### Worktree Location

By default, worktrees are created in a sibling directory to your repository:
- **Repository**: `/path/to/neon`
- **Worktrees**: `/path/to/neon-worktrees/`

The system uses dynamic path detection:
```bash
# Repository root is detected using git
neon_repo=$(git rev-parse --show-toplevel)

# Worktree base defaults to sibling directory
worktree_base="${NEON_WORKTREE_BASE:-$(dirname "${neon_repo}")/neon-worktrees}"
```

#### Customizing Worktree Location

To use a custom worktree location, set the `NEON_WORKTREE_BASE` environment variable:

```bash
# Temporary (current session only)
export NEON_WORKTREE_BASE="/custom/path/to/worktrees"

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export NEON_WORKTREE_BASE="/custom/path/to/worktrees"' >> ~/.bashrc
```

Examples:
- Default: Repository at `/Users/you/neon` → Worktrees at `/Users/you/neon-worktrees/`
- Custom: `NEON_WORKTREE_BASE=/tmp/neon-dev` → Worktrees at `/tmp/neon-dev/`

This makes the configuration portable across:
- Different users
- Different operating systems (Linux, macOS, WSL)
- Different directory structures

## Usage

### Quick Start

Build a complete feature with full automation:

```bash
/build-feature "Add array support to Neon"
```

The orchestrator will:
1. Create a new git worktree with a feature branch
2. Create a workflow state file
3. Spawn a Planner Agent to break down the feature
4. For each task:
   - Spawn a Coding Agent to implement (in worktree)
   - Spawn a Testing Agent to verify (in worktree)
   - Iterate on failures
5. Spawn a Custom Review Agent to review the code
6. Iterate on review feedback if needed
7. Spawn a PR Agent to create the pull request
8. Request GitHub Copilot review on the PR
9. Fetch Copilot suggestions via gh CLI
10. Spawn a Coding Agent to address Copilot feedback
11. Push changes to update the PR

### Manual Workflow

For more control, use individual commands:

#### Step 1: Plan the Feature

```bash
/plan-feature "Add array support"
```

Creates a detailed implementation plan saved to `.claude/workflows/array-support-plan.md`

#### Step 2: Implement Tasks

```bash
/implement-task 1  # Implement first task
/run-tests         # Verify implementation
/implement-task 2  # Next task
/run-tests         # Verify again
# ... continue for all tasks
```

#### Step 3: Create Pull Request

```bash
/create-pr "feat: Add array support to Neon"
```

Creates a PR with comprehensive description and test results.

#### Step 4: Review Before PR (Custom Review)

```bash
/review-pr
```

Performs automated code review and provides structured feedback before creating the PR.

#### Step 5: Create PR After Review

Once the custom review approves:

```bash
/create-pr "feat: Add array support to Neon"
```

Creates a PR with comprehensive description and test results.

#### Step 6: GitHub Copilot Review

After PR creation, Copilot automatically reviews the PR (if orchestration is used), or manually request:

```bash
gh extension install ChrisCarini/gh-copilot-review  # One-time setup
gh copilot-review {pr_number}
```

Or manually add Copilot as a reviewer in the GitHub UI.

#### Step 7: Address Copilot Feedback

The orchestrator fetches Copilot suggestions and spawns agents to address them automatically, pushing updates to the existing PR.

## Commands Reference

### `/build-feature {description}`

**Purpose**: Full automation from planning to PR

**Example**:
```bash
/build-feature "Implement while loops"
```

**Output**:
- Creates state file
- Spawns all necessary agents
- Manages iteration loops
- Reports progress throughout

---

### `/plan-feature {description}`

**Purpose**: Create implementation plan

**Example**:
```bash
/plan-feature "Add hash maps"
```

**Output**:
- Detailed task breakdown
- File-by-file analysis
- Architecture impact assessment
- Testing strategy
- Saved plan file

---

### `/implement-task {task_number|description}`

**Purpose**: Implement a specific task

**Example**:
```bash
/implement-task 1
/implement-task "Add array parsing"
```

**Output**:
- Code changes
- Files modified
- Build verification
- Summary of implementation

---

### `/run-tests [filter]`

**Purpose**: Execute tests and analyze results

**Examples**:
```bash
/run-tests              # All tests
/run-tests unit         # Unit tests only
/run-tests test_arrays  # Specific test
```

**Output**:
- Build status
- Test results summary
- Detailed failure analysis
- Specific fixes needed

---

### `/create-pr [title]`

**Purpose**: Create GitHub pull request

**Example**:
```bash
/create-pr "feat: Add array support"
/create-pr  # Auto-generate title
```

**Output**:
- PR URL
- Generated description
- Files changed summary
- Next steps

**Prerequisites**:
- `gh` CLI installed and authenticated
- Changes committed to a branch
- Tests passing

---

### `/review-pr [pr_number]`

**Purpose**: Automated code review

**Examples**:
```bash
/review-pr      # Most recent PR
/review-pr 123  # Specific PR number
```

**Output**:
- Overall assessment
- Blocking issues
- Suggestions
- Security review
- Test coverage analysis
- Recommendation (approve/request changes)

---

## Workflow Examples

### Example 1: Simple Feature

```bash
# Full automation
/build-feature "Add support for single-line comments"

# System will:
# 1. Plan the feature (scanner token, parser skip logic)
# 2. Implement task 1: Add comment token
# 3. Test implementation
# 4. Implement task 2: Add parser logic
# 5. Test again
# 6. Create PR
# 7. Review PR
# 8. Report completion
```

### Example 2: Complex Feature with Manual Control

```bash
# Step 1: Plan
/plan-feature "Add struct types with methods"

# Review the plan, then proceed task by task

# Step 2: Implement AST changes
/implement-task 1
/run-tests

# Step 3: Implement parser changes
/implement-task 2
/run-tests

# (Tests fail, need fixes)
/implement-task "Fix struct field parsing"
/run-tests

# Step 4: Continue with remaining tasks
/implement-task 3
/run-tests

# ... continue until all tasks complete

# Step 5: Create PR
/create-pr "feat: Add struct types with methods"

# Step 6: Review
/review-pr

# (Review suggests improvements)
/implement-task "Add bounds checking in struct field access"
/run-tests

# Step 7: Done
```

### Example 3: Bug Fix Workflow

```bash
# Plan the fix
/plan-feature "Fix panic in VM when dividing by zero"

# Implement
/implement-task 1
/run-tests

# Create PR
/create-pr "fix: Handle division by zero gracefully"

# Review
/review-pr

# Merge (manual step in GitHub)
```

## State File Management

### Creating a State File Manually

If you want to start with a pre-defined state:

```bash
cat > .claude/workflows/my-feature-state.json << 'EOF'
{
  "feature": "My feature description",
  "branch": "feature/my-feature",
  "status": "planning",
  "tasks": [],
  "current_task_index": 0,
  "test_results": null,
  "pr_url": null,
  "iterations": 0,
  "created_at": "2025-11-29T10:00:00Z"
}
EOF
```

### Viewing State

```bash
cat .claude/workflows/*-state.json | jq
```

### Cleaning Up Old State Files

```bash
rm .claude/workflows/completed-feature-state.json
```

### Managing Worktrees

List all worktrees:
```bash
git worktree list
```

Remove a completed feature worktree:
```bash
# Replace <worktree-path> with the actual path from git worktree list
git worktree remove <worktree-path>
git branch -d feature/feature-name  # Delete the branch if merged
```

Prune stale worktrees:
```bash
git worktree prune
```

## Best Practices

### 1. Use Descriptive Feature Names

Good:
```bash
/build-feature "Add array literal syntax with indexing"
```

Bad:
```bash
/build-feature "arrays"
```

### 2. Review Plans Before Full Automation

For complex features, plan first:

```bash
/plan-feature "Complex feature"
# Review the plan
# Then decide: /build-feature or manual steps
```

### 3. Let Tests Guide Implementation

Don't skip `/run-tests`:
- Catches issues early
- Provides specific feedback
- Ensures quality

### 4. Iterate on Review Feedback

The Review Agent provides valuable insights:
- Address blocking issues first
- Consider suggestions seriously
- Don't ignore security concerns

### 5. Keep Tasks Atomic

Each task should:
- Have a single clear purpose
- Be independently testable
- Take < 30 minutes to implement

### 6. Commit Frequently

During manual workflows:
```bash
git add .
git commit -m "Complete task 1: Add array AST nodes"
```

This helps with:
- PR history clarity
- Easier rollbacks
- Better review granularity

## Troubleshooting

### Agent Fails to Start

**Problem**: Task tool doesn't spawn agent

**Solution**:
- Check command syntax
- Ensure you're in the Neon repository
- Verify `.claude/commands/` exists

### Tests Keep Failing

**Problem**: Testing Agent reports failures in loop

**Solution**:
- Read test failure details carefully
- Check if implementation matches task requirements
- Verify understanding of existing code patterns
- Consider manual debugging

### PR Creation Fails

**Problem**: gh CLI errors

**Solutions**:
```bash
# Check gh authentication
gh auth status

# If not authenticated
gh auth login

# Check remote setup
git remote -v

# Add remote if missing
gh repo set-default
```

### State File Corruption

**Problem**: State file has invalid JSON

**Solution**:
```bash
# Validate JSON
cat .claude/workflows/feature-state.json | jq

# Fix or recreate the file
```

## Advanced Usage

### Custom Agent Prompts

You can modify the command files in `.claude/commands/` to customize agent behavior:

```bash
# Edit planner behavior
vim .claude/commands/plan-feature.md

# Edit coding agent behavior
vim .claude/commands/implement-task.md
```

### Integration with CI/CD

The orchestration system works well with CI:

```yaml
# .github/workflows/feature-validation.yml
name: Validate Feature Branch
on:
  push:
    branches:
      - 'feature/**'

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo test --verbose
```

### Parallel Task Implementation

For independent tasks, you can run multiple coding agents in parallel:

```bash
# In separate terminal sessions
/implement-task 1  # Terminal 1
/implement-task 2  # Terminal 2 (if independent)
```

## File Structure

```
neon/                              # Main repository
├── .claude/
│   ├── commands/
│   │   ├── build-feature.md      # Main orchestrator
│   │   ├── plan-feature.md       # Planning agent
│   │   ├── implement-task.md     # Coding agent
│   │   ├── run-tests.md          # Testing agent
│   │   ├── create-pr.md          # PR agent
│   │   └── review-pr.md          # Review agent
│   ├── workflows/
│   │   ├── {feature}-state.json  # State files (gitignored)
│   │   └── {feature}-plan.md     # Plan files (gitignored)
│   ├── settings.local.json       # Claude Code settings
│   └── ORCHESTRATION.md          # This file
└── ... (Neon source code)

neon-worktrees/                    # Worktree directory (separate)
├── add-array-support/            # Feature worktree 1
│   └── ... (full Neon codebase)
├── implement-for-loops/          # Feature worktree 2
│   └── ... (full Neon codebase)
└── ...
```

## Contributing to the Orchestration System

### Adding a New Agent

1. Create command file: `.claude/commands/my-agent.md`
2. Define agent role and responsibilities
3. Specify input/output format
4. Add state file integration
5. Update this documentation
6. Test with a real feature

### Improving Existing Agents

1. Edit the relevant `.claude/commands/{agent}.md` file
2. Test changes with `/agent-name`
3. Update examples in this documentation
4. Commit changes

## Examples of Real Features Built

### Feature: Array Support

**Workflow Used**: Full automation (`/build-feature`)

**Tasks Generated**:
1. Add array literal AST nodes
2. Implement array parsing
3. Add array opcodes
4. Implement VM array handling
5. Add array indexing
6. Add comprehensive tests

**Outcome**: Successful PR, merged

**Iterations**: 1 (minor test fix)

---

### Feature: For Loops

**Workflow Used**: Manual (better control over complex feature)

**Tasks**:
1. Plan for loop syntax
2. Add AST nodes
3. Implement parsing
4. Add semantic validation
5. Implement code generation
6. Add VM loop instructions
7. Test extensively

**Outcome**: Successful after review feedback

**Iterations**: 2 (architectural adjustments)

---

## Future Enhancements

Potential improvements to the orchestration system:

- [x] GitHub Copilot code review integration
- [ ] Parallel task execution for independent tasks
- [ ] Integration with GitHub Issues
- [ ] Automatic benchmarking on feature branches
- [ ] Documentation generation for new features
- [ ] Rollback capabilities for failed features
- [ ] Multi-PR support for large features
- [ ] Integration with external linters
- [ ] Performance regression detection
- [ ] Automated migration guides for breaking changes
- [ ] Smarter Copilot feedback parsing and categorization
- [ ] Metrics tracking for Copilot suggestion acceptance rate

## Support

For issues with the orchestration system:

1. Check this documentation
2. Review command files in `.claude/commands/`
3. Check state files in `.claude/workflows/`
4. Ask Claude Code for help
5. File an issue on GitHub

## License

This orchestration system is part of the Neon project and follows the same license.
