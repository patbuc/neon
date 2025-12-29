# Claude Code Orchestration System

An automated multi-agent system for building features into the Neon programming language using Claude Code.

## Overview

This orchestration system manages the complete feature development lifecycle using Claude Code's native agent
capabilities:

```
Planning → Implementation → Testing → Custom Review → PR Creation → Copilot Review → Address Feedback → Iteration
```

### Key Benefits

- **Full Automation**: Build complete features with a single command
- **Quality Assurance**: Automated testing and code review at every step
- **Isolated Development**: Git worktrees keep your main branch clean
- **Structured Workflow**: Consistent process for every feature
- **GitHub Integration**: Seamless PR creation and Copilot code review

## Quick Start

Build a complete feature in one command:

```bash
/build-feature "Add array support to Neon"
```

That's it! The system will:

1. Plan the implementation
2. Break it into tasks
3. Implement each task
4. Run tests after each task
5. Perform code review
6. Create a PR
7. Request GitHub Copilot review
8. Address feedback automatically

## Available Commands

| Command           | Purpose               | Example                         |
|-------------------|-----------------------|---------------------------------|
| `/build-feature`  | Full automation       | `/build-feature "Add arrays"`   |
| `/plan-feature`   | Planning only         | `/plan-feature "Add for loops"` |
| `/implement-task` | Code a specific task  | `/implement-task 1`             |
| `/run-tests`      | Execute test suite    | `/run-tests`                    |
| `/create-pr`      | Create pull request   | `/create-pr`                    |
| `/review-pr`      | Automated code review | `/review-pr`                    |

## Usage Examples

### Simple Feature (Full Automation)

```bash
/build-feature "Add modulo operator (%)"
```

The orchestrator handles everything automatically.

### Complex Feature (Manual Control)

```bash
# 1. Plan first
/plan-feature "Add closures with lexical scoping"

# 2. Review the plan, then implement tasks one by one
/implement-task 1
/run-tests

/implement-task 2
/run-tests

# 3. Create PR when done
/create-pr

# 4. Review
/review-pr
```

### Bug Fix Workflow

```bash
/plan-feature "Fix panic when calling undefined function"
/implement-task 1
/run-tests
/create-pr
```

## How It Works

### Agents

The system uses specialized agents, each with a specific role:

1. **Planner Agent** (`/plan-feature`)
    - Analyzes feature requirements
    - Explores codebase patterns
    - Clarifies architectural decisions with user
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

**Key Fields**:

- `feature`: Feature description
- `branch`: Git branch name
- `status`: Current workflow phase
- `tasks`: Array of task objects with status
- `test_results`: Latest test run results
- `pr_url`: Pull request URL
- `custom_review`: Code review results
- `copilot_review`: GitHub Copilot review data

### Workflow Lifecycle

Each feature follows this lifecycle:

1. **Planning**: Break down feature into tasks
2. **Coding**: Implement each task
3. **Testing**: Verify implementation
4. **Review**: Automated code review
5. **PR Creation**: Create GitHub pull request
6. **Copilot Review**: GitHub Copilot reviews the PR
7. **Address Feedback**: Fix issues from Copilot
8. **Completed**: Feature merged

## Configuration

### Git Worktrees

The orchestration system uses git worktrees to isolate feature development:

- **Worktree Location**: `<repo-parent>/neon-worktrees/{feature-slug}/` (configurable via `NEON_WORKTREE_BASE`)
- **Branch Naming**: `feature/{feature-slug}` (auto-generated from feature description)
- **Benefits**:
    - Isolated development environment per feature
    - No need to stash/commit when switching features
    - Parallel feature development possible
    - Main repo stays clean

#### Customizing Worktree Location

By default, worktrees are created in a sibling directory to your repository:

- **Repository**: `/path/to/neon`
- **Worktrees**: `/path/to/neon-worktrees/`

To use a custom location, set the `NEON_WORKTREE_BASE` environment variable:

```bash
# Temporary (current session only)
export NEON_WORKTREE_BASE="/custom/path/to/worktrees"

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export NEON_WORKTREE_BASE="/custom/path/to/worktrees"' >> ~/.bashrc
```

### GitHub Copilot Setup

To use GitHub Copilot code review integration:

1. **GitHub Copilot Subscription**: You need an active subscription (Individual, Business, or Enterprise)

2. **Install gh-copilot-review Extension** (Optional):
   ```bash
   gh extension install ChrisCarini/gh-copilot-review
   ```

3. **Alternative**: Enable automatic Copilot reviews in repository settings
    - Go to Settings → Rules → Rulesets
    - Enable "Automatically request Copilot code review"

4. **Verify gh CLI Authentication**:
   ```bash
   gh auth status
   gh auth login  # If not authenticated
   ```

### Settings Configuration

The `settings.local.json` file contains Claude Code permissions:

```json
{
  "allowedCommands": [
    "Bash(find:*)",
    "Bash(cat:*)",
    "Bash(cargo test:*)",
    "Bash(git log --oneline -15)"
  ]
}
```

Add more permissions as needed.

## Detailed Command Reference

### `/build-feature {description}`

**Purpose**: Full automation from planning to PR

**Example**:

```bash
/build-feature "Implement while loops"
```

**Process**:

1. Creates git worktree with feature branch
2. Creates workflow state file
3. Spawns Planner Agent to break down feature
4. For each task:
    - Spawns Coding Agent to implement
    - Spawns Testing Agent to verify
    - Iterates on failures
5. Spawns Review Agent to review code
6. Iterates on review feedback if needed
7. Spawns PR Agent to create pull request
8. Requests GitHub Copilot review
9. Fetches Copilot suggestions
10. Spawns Coding Agent to address feedback
11. Pushes changes to update PR

**Output**:

- State file with complete workflow tracking
- Feature branch with all implementations
- Pull request with comprehensive description
- Test results and code review feedback

---

### `/plan-feature {description}`

**Purpose**: Create detailed implementation plan

**Example**:

```bash
/plan-feature "Add hash maps"
```

**Process**:

1. Analyzes feature requirements
2. Explores codebase for existing patterns
3. Asks clarifying questions if needed
4. Breaks feature into atomic tasks
5. Saves plan to `.claude/workflows/{feature}-plan.md`

**Output**:

- Detailed task breakdown
- File-by-file analysis
- Architecture impact assessment
- Testing strategy
- Saved plan file

**Use When**:

- Complex features requiring careful planning
- Architectural changes
- Multiple implementation approaches possible
- You want to review before implementation

---

### `/implement-task {task_number|description}`

**Purpose**: Implement a specific task from the plan

**Examples**:

```bash
/implement-task 1
/implement-task "Add array parsing"
```

**Process**:

1. Reads task from state file
2. Implements required changes
3. Ensures code compiles
4. Updates state file with files modified
5. Marks task as completed

**Output**:

- Code changes in the feature branch
- List of files modified
- Build verification
- Implementation summary

**Notes**:

- Can be used standalone or as part of manual workflow
- Always run `/run-tests` after to verify

---

### `/run-tests [filter]`

**Purpose**: Execute tests and analyze results

**Examples**:

```bash
/run-tests              # All tests
/run-tests unit         # Unit tests only
/run-tests test_arrays  # Specific test
```

**Process**:

1. Builds the project (`cargo build`)
2. Runs test suite (`cargo test`)
3. Analyzes test output
4. Categorizes failures
5. Updates state file with results

**Output**:

- Build status (success/failed)
- Test results summary (passed/failed/total)
- Detailed failure analysis for each failing test
- Specific fixes needed
- Duration of test run

**Notes**:

- Run after every task implementation
- Provides actionable feedback for failures
- Tracks test history in state file

---

### `/create-pr [title]`

**Purpose**: Create GitHub pull request

**Examples**:

```bash
/create-pr "feat: Add array support"
/create-pr  # Auto-generate title from feature
```

**Process**:

1. Verifies all changes are committed
2. Pushes feature branch to remote
3. Generates comprehensive PR description
4. Creates PR using `gh pr create`
5. Updates state file with PR URL

**Output**:

- PR URL
- Generated description including:
    - Feature summary
    - Implementation details
    - Test results
    - Files changed
- Next steps

**Prerequisites**:

- `gh` CLI installed and authenticated
- Changes committed to feature branch
- Tests passing (recommended)

---

### `/review-pr [pr_number]`

**Purpose**: Automated code review before or after PR creation

**Examples**:

```bash
/review-pr      # Most recent PR
/review-pr 123  # Specific PR number
```

**Process**:

1. Analyzes all changed files
2. Checks code quality and style
3. Reviews architecture decisions
4. Identifies security concerns
5. Validates test coverage
6. Provides structured feedback

**Output**:

- Overall assessment (approve/request changes/comment)
- Blocking issues (must fix)
- Suggestions (should consider)
- Nitpicks (optional improvements)
- Security review
- Test coverage analysis
- Recommendation with reasoning

**Categories**:

- **Blocking**: Critical issues that must be fixed
- **Suggestion**: Improvements worth considering
- **Nitpick**: Minor style/preference items

---

## State Management

### State File Format

```json
{
  "feature": "Feature description",
  "branch": "feature/feature-name",
  "worktree_path": "<worktree-base>/feature-name",
  "status": "planning|coding|testing|reviewing|pr_created|completed",
  "current_phase": "current phase name",
  "tasks": [
    {
      "id": 1,
      "description": "Task description",
      "status": "pending|in_progress|completed",
      "files_modified": [
        "file1.rs",
        "file2.rs"
      ],
      "acceptance_criteria": [
        "criterion 1"
      ],
      "test_strategy": "how to test"
    }
  ],
  "test_results": {
    "last_run": "ISO timestamp",
    "status": "passed|failed|build_failed",
    "total": 96,
    "passed": 94,
    "failed": 2,
    "failures": [
      "test_name1"
    ],
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

### Viewing State

```bash
# View current state
cat .claude/workflows/*-state.json | jq

# See the plan
cat .claude/workflows/*-plan.md
```

### Managing Worktrees

List all worktrees:

```bash
git worktree list
```

Remove a completed feature worktree:

```bash
git worktree remove <worktree-path>
git branch -d feature/feature-name  # Delete branch if merged
```

Prune stale worktrees:

```bash
git worktree prune
```

### Cleaning Up Old State Files

```bash
# Remove completed feature state
rm .claude/workflows/completed-feature-state.json
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

Descriptive names help with:

- Clear branch names
- Better commit messages
- Understandable PR titles
- Easier state file identification

### 2. Review Plans Before Full Automation

For complex features, plan first:

```bash
/plan-feature "Add closures with lexical scoping"
# Review the plan
# Then decide: /build-feature or manual steps
```

This helps you:

- Understand the scope
- Identify potential issues early
- Make architectural decisions upfront
- Provide feedback on the approach

### 3. Let Tests Guide Implementation

Don't skip `/run-tests`:

- Catches issues early
- Provides specific feedback
- Ensures quality at each step
- Prevents cascading failures

Run tests after every task:

```bash
/implement-task 1
/run-tests  # Always run tests
```

### 4. Iterate on Review Feedback

The Review Agent provides valuable insights:

- Address blocking issues first (critical bugs, security)
- Consider suggestions seriously (architecture, best practices)
- Evaluate nitpicks based on project standards
- Don't ignore security concerns

### 5. Keep Tasks Atomic

Each task should:

- Have a single clear purpose
- Be independently testable
- Be completable in one session
- Not depend on unimplemented tasks

Good task:

- "Add array literal parsing to parser"

Bad task:

- "Implement arrays" (too broad)

### 6. Commit Frequently

During manual workflows:

```bash
git add .
git commit -m "Add array AST nodes"
```

Benefits:

- Clear PR history
- Easier rollbacks
- Better review granularity
- Progress tracking

## Troubleshooting

### Agent Fails to Start

**Problem**: Task tool doesn't spawn agent

**Solutions**:

- Check command syntax (e.g., `/build-feature "description"`)
- Ensure you're in the Neon repository root
- Verify `.claude/commands/` directory exists
- Check that command files are valid markdown

### Tests Keep Failing

**Problem**: Testing Agent reports failures in loop

**Solutions**:

- Read test failure details carefully
- Check if implementation matches task requirements
- Verify understanding of existing code patterns
- Run tests manually: `cargo test --verbose`
- Check test output for specific error messages
- Consider manual debugging with print statements

### PR Creation Fails

**Problem**: `gh` CLI errors when creating PR

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

**Solutions**:

```bash
# Validate JSON
cat .claude/workflows/feature-state.json | jq

# If invalid, fix manually or recreate
# Use .claude/workflows/state-template.json as reference
```

### Worktree Already Exists

**Problem**: Cannot create worktree, path already exists

**Solutions**:

```bash
# List existing worktrees
git worktree list

# Remove the conflicting worktree
git worktree remove <path>

# Or choose a different feature name
```

### GitHub Copilot Review Not Working

**Problem**: Copilot review not requested or fails

**Solutions**:

```bash
# Verify gh extension is installed
gh extension list

# Install if missing
gh extension install ChrisCarini/gh-copilot-review

# Check Copilot subscription
# Go to GitHub settings → Copilot

# Manually request review via GitHub UI as fallback
```

## Advanced Usage

### Custom Agent Prompts

You can modify agent behavior by editing command files:

```bash
# Edit planner behavior
vim .claude/commands/plan-feature.md

# Edit coding agent behavior
vim .claude/commands/implement-task.md

# Test changes
/plan-feature "test feature"
```

After modifying, test thoroughly before relying on changes.

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

For independent tasks, implement in parallel:

```bash
# In separate terminal sessions or worktrees
/implement-task 1  # Terminal 1
/implement-task 2  # Terminal 2 (if tasks are independent)
```

Note: Ensure tasks don't conflict (modify same files).

### Creating Manual State Files

Start with a pre-defined state:

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
│   │   ├── .gitignore            # Ignore state files
│   │   ├── {feature}-state.json  # State files (gitignored)
│   │   └── {feature}-plan.md     # Plan files (gitignored)
│   ├── settings.local.json       # Claude Code settings
│   └── README.md                 # This file
└── ... (Neon source code)

neon-worktrees/                    # Worktree directory (sibling to repo)
├── add-array-support/            # Feature worktree 1
│   └── ... (full Neon codebase)
├── implement-for-loops/          # Feature worktree 2
│   └── ... (full Neon codebase)
└── ...
```

## Contributing

### Adding a New Agent

1. Create command file: `.claude/commands/my-agent.md`
2. Define agent role and responsibilities clearly
3. Specify input/output format
4. Add state file integration if needed
5. Update this documentation
6. Test with a real feature

Example agent structure:

```markdown
# Agent Name

## Role

What this agent does

## Inputs

- Input 1: description
- Input 2: description

## Process

1. Step 1
2. Step 2

## Outputs

- Output 1: description
- Output 2: description
```

### Improving Existing Agents

1. Edit `.claude/commands/{agent}.md`
2. Test changes with the command
3. Update examples in this documentation
4. Commit changes with clear description

### Testing Changes

Always test agent changes:

```bash
# Test planning
/plan-feature "test feature for agent changes"

# Test implementation
/implement-task "test task"

# Verify state management still works
cat .claude/workflows/*-state.json | jq
```

## Real-World Examples

### Example 1: Array Support

**Command**:

```bash
/build-feature "Add array support with literals and indexing"
```

**Tasks Generated**:

1. Add array literal AST nodes
2. Implement array parsing
3. Add array opcodes
4. Implement VM array handling
5. Add array indexing
6. Add comprehensive tests

**Outcome**: Successful PR, merged after Copilot review

**Iterations**: 1 (minor test fix)

**Duration**: ~2 hours

---

### Example 2: For Loops

**Workflow**: Manual (complex feature)

```bash
/plan-feature "Implement for-in loops for arrays"
/implement-task 1  # Add AST nodes
/run-tests
/implement-task 2  # Parsing
/run-tests
/implement-task 3  # Code generation
/run-tests
/implement-task 4  # VM execution
/run-tests
/create-pr
/review-pr
```

**Outcome**: Successful after addressing review feedback

**Iterations**: 2 (architectural adjustments)

**Duration**: ~4 hours

---

## Future Enhancements

Potential improvements:

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
- [ ] Smarter Copilot feedback parsing
- [ ] Metrics tracking for suggestion acceptance rate

## Support

For issues with the orchestration system:

1. Check this documentation
2. Review command files in `.claude/commands/`
3. Check state files in `.claude/workflows/`
4. Ask Claude Code for help
5. File an issue on GitHub

