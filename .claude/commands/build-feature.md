# Build Feature Orchestrator

You are the **Feature Orchestration Agent** for the Neon programming language project.

Your job is to orchestrate the complete feature development workflow from planning to PR creation and review.

## Input

The user will provide a feature description after this command, for example:
- `/build-feature "Add array support to Neon"`
- `/build-feature "Implement for loops"`

## Workflow

Execute the following workflow in order:

### 1. Initialize Git Worktree and Branch

#### 1a. Generate Branch Name

Convert the feature description to a branch name:
- Convert to lowercase
- Replace spaces with hyphens
- Remove special characters
- Prefix with `feature/`

Example: "Add array support to Neon" → `feature/add-array-support-to-neon`

```bash
# Generate feature slug
feature_slug=$(echo "{feature_description}" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 ]//g' | tr ' ' '-' | sed 's/--*/-/g')
branch_name="feature/${feature_slug}"
```

#### 1b. Create Git Worktree

Create a new worktree for isolated development:

```bash
# Ensure we're in the main repo
cd /home/patbuc/code/neon

# Get absolute path for worktree
worktree_path="/home/patbuc/code/neon-worktrees/${feature_slug}"

# Create worktree directory if it doesn't exist
mkdir -p /home/patbuc/code/neon-worktrees

# Create new worktree with new branch based on aoc-main
git worktree add "${worktree_path}" -b "${branch_name}" aoc-main

# Verify worktree creation
git worktree list
```

#### 1c. Set Working Directory

All subsequent operations (cargo build, cargo test, file edits, etc.) must be executed in the worktree directory:

```bash
cd "${worktree_path}"
```

**Important**: Pass the worktree path to all spawned agents so they operate in the correct directory.

### 2. Initialize Workflow State

Create a workflow state file at `.claude/workflows/{feature-slug}-state.json` with:
- feature description
- branch name (feature/{feature-slug})
- worktree_path: "/home/patbuc/code/neon-worktrees/{feature-slug}"
- status: "planning"
- tasks: []
- current_task_index: 0
- test_results: null
- pr_url: null
- iterations: 0
- created_at: ISO timestamp

Store the state file in the main repo (not the worktree) at `/home/patbuc/code/neon/.claude/workflows/{feature-slug}-state.json`

### 3. Planning Phase

Spawn a **Planner Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for Planner Agent:**
```
You are the Planning Agent for Neon language feature development.

Feature: {feature_description}
Worktree: {worktree_path}

IMPORTANT: All file paths and operations must be relative to the worktree directory: {worktree_path}

Your task:
1. Analyze the Neon codebase structure (src/compiler/, src/vm/, src/common/)
2. Understand how similar features are implemented
3. Identify architectural decisions and implementation approaches
4. **Use AskUserQuestion tool** to clarify with the user:
   - Multiple valid implementation approaches (e.g., which data structure to use)
   - Architectural decisions (e.g., where to place new functionality)
   - Scope and requirements clarification
   - Trade-offs between different approaches
   - Any ambiguities in the feature description
5. Based on user feedback, break down the feature into 3-7 atomic, sequential tasks
6. For each task, specify:
   - Clear description
   - Files likely to be modified
   - Dependencies on other tasks
   - Acceptance criteria
   - Implementation approach (based on user decisions)

IMPORTANT: Always ask questions when there are multiple valid approaches or unclear requirements.
Do NOT make architectural decisions without user input.
Do NOT implement anything - only plan.

Return your plan as a structured list of tasks. Be specific and actionable.
```

Update the state file with the task list from the planner.

### 4. Implementation Loop

For each task in the plan:

#### 4a. Coding Phase

Update state: status = "coding", current_task_index = {index}

Spawn a **Coding Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for Coding Agent:**
```
You are the Coding Agent for Neon language development.

Worktree: {worktree_path}
Branch: {branch_name}

IMPORTANT: All file operations and commands must be executed in: {worktree_path}
Change to this directory before any operations: cd {worktree_path}

IMPORTANT - Commit Message Guidelines:
- DO NOT add watermarks like "Generated with Claude Code"
- DO NOT add "Co-Authored-By: Claude" trailers
- DO NOT list changed files in commit messages
- Focus on the intent and high-level summary (WHY, not WHAT)
- Keep messages clean and professional

Current Task: {task.description}
Files to modify: {task.files}
Acceptance criteria: {task.acceptance_criteria}

Your task:
1. Read the relevant source files from the worktree
2. Understand existing patterns and architecture
3. Implement the required changes following Rust best practices
4. Ensure code compiles (run: cd {worktree_path} && cargo build)
5. Do NOT run tests - that's the Testing Agent's job

Context from previous tasks: {context}

Implement ONLY this task. Be focused and precise.
```

#### 4b. Testing Phase

Update state: status = "testing"

Spawn a **Testing Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for Testing Agent:**
```
You are the Testing Agent for Neon language development.

Worktree: {worktree_path}
Branch: {branch_name}

IMPORTANT: All commands must be executed in: {worktree_path}

Just completed: {task.description}
Files modified: {list of modified files}

Your task:
1. Run: cd {worktree_path} && cargo test --verbose
2. Run: cd {worktree_path} && cargo build --verbose
3. Analyze any failures
4. If tests fail:
   - Identify root cause
   - Provide specific feedback for the Coding Agent
   - Set status to "needs_fixes"
5. If tests pass:
   - Update test results in state file
   - Set status to "task_completed"

Return detailed test results and analysis.
```

#### 4c. Fix Loop (if needed)

If status == "needs_fixes":
- Increment iterations counter
- Re-spawn Coding Agent with test feedback
- Re-run Testing Agent
- Limit to 3 iterations per task before escalating to user

If status == "task_completed":
- **Create commit for this task**:
  ```bash
  cd {worktree_path}

  # Source commit utilities
  source /home/patbuc/code/neon/.claude/utils/commit-utils.sh

  # Get current task info from state file
  state_file="/home/patbuc/code/neon/.claude/workflows/{feature-slug}-state.json"
  current_task=$(jq -r ".tasks[.current_task_index]" "$state_file")
  task_description=$(echo "$current_task" | jq -r '.description')

  # Commit the task (with watermark validation)
  if ! safe_task_commit "$task_description"; then
    echo "ERROR: Failed to commit task. Pausing workflow."
    jq '.status = "error" | .error = "commit_failed"' "$state_file" > "$state_file.tmp"
    mv "$state_file.tmp" "$state_file"
    exit 1
  fi
  ```
- Mark task as completed in state file
- Move to next task

### 5. Custom Review Phase (Pre-PR)

Update state: status = "reviewing"

Spawn a **Reviewer Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for Reviewer Agent:**
```
You are the Code Review Agent for Neon language development.

Worktree: {worktree_path}
Feature: {feature_description}
Branch: {branch_name}
Tasks completed: {list of tasks}

IMPORTANT: Review files from the worktree directory: {worktree_path}

Your task:
1. Review all code changes for:
   - Code quality and Rust best practices
   - Architectural consistency with Neon patterns
   - Security issues (unsafe code, panics, error handling)
   - Test coverage
   - Documentation needs
2. Provide structured feedback:
   - Blocking issues (must fix)
   - Suggestions (nice to have)
   - Praise (what was done well)
3. Set review status: "approved", "changes_requested", or "commented"

Return your review as structured feedback.
```

### 6. Pre-PR Iteration Phase (if needed)

If review status == "changes_requested":
- Extract blocking issues from review
- Create new tasks for fixes
- Go back to step 3 (Implementation Loop)
- Limit to 2 review iterations before requesting user intervention

If review status == "approved":
- Proceed to PR creation

### 7. PR Creation Phase

After custom review approves:

Update state: status = "creating_pr"

Spawn a **PR Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for PR Agent:**
```
You are the PR Agent for Neon language development.

Worktree: {worktree_path}
Feature: {feature_description}
Branch: {branch_name}
Tasks completed: {list of tasks}
Files modified: {all modified files}

IMPORTANT: Execute all git commands from the worktree directory: {worktree_path}

IMPORTANT - Commit Message Guidelines:
- DO NOT add watermarks like "Generated with Claude Code"
- DO NOT add "Co-Authored-By: Claude" trailers
- DO NOT list changed files in commit messages
- Focus on the intent and high-level summary (WHY, not WHAT)
- Keep messages clean and professional

Your task:
1. Create a comprehensive PR description including:
   - Feature overview
   - Implementation approach
   - Testing performed
   - Files changed summary
2. Ensure branch is pushed: cd {worktree_path} && git push -u origin {branch_name}
3. Use gh CLI to create the PR from the worktree:
   cd {worktree_path} && gh pr create --title "{feature_description}" --body "{description}"
4. Return the PR URL

The PR will be created from the worktree branch.
```

Update state with pr_url.

### 8. GitHub Copilot Review Phase

Update state: status = "copilot_reviewing"

#### 8a. Request Copilot Review

Use one of these methods to request a GitHub Copilot code review:

**Method 1: Using gh-copilot-review Extension** (Recommended)
```bash
# Check if extension is installed
gh extension list | grep copilot-review

# If not installed, install it
gh extension install ChrisCarini/gh-copilot-review

# Request Copilot review
gh copilot-review {pr_number}
```

**Method 2: Manual Assignment**
If the extension is not available, inform the user to:
1. Open the PR URL in a browser
2. Click on the "Reviewers" menu
3. Select "Copilot" from the list

#### 8b. Wait for Review Completion

Poll for review completion (Copilot reviews typically take < 30 seconds):

```bash
# Wait for review to appear (check every 5 seconds, max 60 seconds)
for i in {1..12}; do
  review=$(gh api repos/{owner}/{repo}/pulls/{pr_number}/reviews --jq '.[] | select(.user.login == "copilot")')
  if [ -n "$review" ]; then
    echo "Copilot review completed"
    break
  fi
  sleep 5
done
```

#### 8c. Fetch Copilot Review Suggestions

Once the review is complete, fetch the review comments:

```bash
# Get Copilot's review summary
gh api repos/{owner}/{repo}/pulls/{pr_number}/reviews \
  --jq '.[] | select(.user.login == "copilot") | {id: .id, state: .state, body: .body}'

# Get detailed review comments with file locations
gh api repos/{owner}/{repo}/pulls/{pr_number}/comments \
  --jq '.[] | select(.user.login == "copilot") | {path: .path, line: .line, body: .body}'
```

### 9. Address Copilot Feedback Phase (UPDATED)

Update state: status = "addressing_copilot_feedback"

If Copilot provided suggestions:

#### 9a. Fetch and Group Feedback by Category

```bash
cd {worktree_path}

# Source utilities
source /home/patbuc/code/neon/.claude/utils/commit-utils.sh

# Get state file
state_file="/home/patbuc/code/neon/.claude/workflows/{feature-slug}-state.json"

# Fetch Copilot review comments
pr_number=$(jq -r '.pr_number' "$state_file")
copilot_comments=$(gh api "repos/patbuc/neon/pulls/$pr_number/comments" \
  --jq '.[] | select(.user.login == "github-copilot[bot]") | .body')

# Categorize all comments
declare -A categories
while IFS= read -r comment; do
  if [[ -n "$comment" ]]; then
    category=$(categorize_copilot_issue "$comment")
    categories["$category"]=1
  fi
done <<< "$copilot_comments"

# Get unique categories
unique_categories="${!categories[@]}"
echo "Copilot feedback categories: $unique_categories"
```

#### 9b. Address Each Category

For each category, spawn Coding Agent, then commit:

```bash
for category in $unique_categories; do
  echo "Processing Copilot feedback category: $category"

  # Get all issues for this category
  category_issues=$(echo "$copilot_comments" | while read -r comment; do
    if [[ -n "$comment" ]] && [[ "$(categorize_copilot_issue "$comment")" == "$category" ]]; then
      echo "$comment"
      echo "---"
    fi
  done)

  # Count issues
  issue_count=$(echo "$category_issues" | grep -c "---" || echo "0")
  echo "Found $issue_count issue(s) in category: $category"

  # Spawn Coding Agent to address this category
  # (Use Task tool with subagent_type: "general-purpose")
  # See prompt below

  # After Coding Agent completes:
  # 1. Run tests
  cd {worktree_path}
  if ! cargo test --quiet 2>&1 | tee test_output.log; then
    echo "ERROR: Tests failed after addressing $category issues"
    cat test_output.log
    exit 1
  fi

  # 2. Commit this category (with watermark validation)
  if ! safe_copilot_commit "$category"; then
    echo "ERROR: Failed to commit $category fixes"
    exit 1
  fi
done

# Push all category commits
git push
```

#### 9c. Coding Agent Prompt (per category)

Spawn a **Coding Agent** for each category:

```
You are the Coding Agent addressing GitHub Copilot review feedback.

Worktree: {worktree_path}
PR: {pr_url}
Category: {category}

IMPORTANT: Work in the worktree directory: {worktree_path}

Copilot Review Feedback ({category}):
{category_issues}

Your task:
1. Read each mentioned file from {worktree_path}
2. Understand Copilot's {category} suggestions
3. Implement the changes
4. Ensure compilation: cd {worktree_path} && cargo build
5. Run tests: cd {worktree_path} && cargo test

Focus ONLY on {category} issues. Be precise.
DO NOT add watermarks or attribution to code or commit messages.
```

Update state: status = "completed"

### 11. Final Status

Report to user:
- PR URL
- Copilot review addressed
- All tests passing
- Ready for human review/merge

## Output

Throughout the workflow, provide the user with:
- Real-time status updates
- Links to the state file
- Summaries of each phase
- Final PR URL when complete

## Error Handling

If any phase fails:
1. Update state with error details
2. Pause the workflow
3. Report to user with specific error
4. Ask user how to proceed

## State File Location

All state files are stored in: `.claude/workflows/`

File naming: `{feature-slug}-state.json` where feature-slug is the feature name converted to kebab-case.
