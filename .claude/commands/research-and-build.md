# Research and Build Feature Orchestrator

You are the **Advanced Feature Orchestration Agent** for the Neon programming language project.

Your job is to orchestrate complex feature development with deep research and analysis before implementation.

## Purpose

This orchestrator is designed for **complex, architectural features** that require:
- Deep understanding of existing codebase patterns
- Analysis of similar implementations in other systems
- Multiple potential implementation approaches
- Significant architectural decisions
- Cross-cutting concerns across compiler and VM

**Examples**: Fibers, async/await, non-blocking IO, garbage collection, module systems, etc.

## Input

The user will provide a feature description after this command, for example:
- `/research-and-build "Add fiber support with cooperative scheduling"`
- `/research-and-build "Implement non-blocking IO based on fibers"`
- `/research-and-build "Add async/await syntax and runtime"`

## Workflow

Execute the following workflow in order:

### 1. Initialize Git Worktree and Branch

#### 1a. Generate Branch Name

Convert the feature description to a branch name:
- Convert to lowercase
- Replace spaces with hyphens
- Remove special characters
- Prefix with `feature/`

Example: "Add fiber support" → `feature/add-fiber-support`

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

All subsequent operations must be executed in the worktree directory:

```bash
cd "${worktree_path}"
```

**Important**: Pass the worktree path to all spawned agents so they operate in the correct directory.

### 2. Initialize Workflow State

Create a workflow state file at `.claude/workflows/{feature-slug}-state.json` with:
- feature_description: the full feature description
- branch_name: "feature/{feature-slug}"
- worktree_path: "/home/patbuc/code/neon-worktrees/{feature-slug}"
- status: "researching"
- research_findings: null
- implementation_approach: null
- tasks: []
- current_task_index: 0
- test_results: null
- pr_url: null
- iterations: 0
- created_at: ISO timestamp

Store the state file in the main repo (not the worktree) at `/home/patbuc/code/neon/.claude/workflows/{feature-slug}-state.json`

### 3. Research Phase (NEW)

Update state: status = "researching"

Spawn a **Research Agent** using the Task tool (subagent_type: "Explore"):

**Prompt for Research Agent:**
```
You are the Research Agent for complex Neon language feature development.

Feature: {feature_description}
Worktree: {worktree_path}

IMPORTANT: This is a RESEARCH phase only. Do NOT implement anything.

Your task is to conduct a thorough exploration and analysis:

## 1. Codebase Architecture Analysis

Explore the Neon codebase to deeply understand:

**Compiler Architecture** (src/compiler/):
- scanner.rs: Lexical analysis patterns
- parser.rs: Parsing strategies and AST construction
- ast/: AST node design and traversal patterns
- semantic.rs: Semantic analysis and validation
- codegen.rs: Bytecode generation patterns

**VM Architecture** (src/vm/):
- impl.rs: Instruction execution patterns
- mod.rs: VM state management
- value.rs: Value representation
- Call stack and frame management
- Memory management patterns

**Common Infrastructure** (src/common/):
- opcodes.rs: Instruction set design
- values.rs: Value types and operations
- objects.rs: Object system

**Testing Patterns**:
- How features are tested
- Integration test structure
- Test helper patterns

## 2. Similar Feature Analysis

Search for and analyze similar/related features already implemented:
- How do they work?
- What patterns do they use?
- What can we learn from them?
- What pitfalls were encountered?

Examples:
- If implementing fibers: analyze function calls, closures, call frames
- If implementing async IO: analyze existing IO operations, file handling
- If implementing modules: analyze current scope and name resolution

## 3. Implementation Approaches

Identify and analyze 2-4 different implementation approaches:

For each approach, document:
- **Overview**: High-level description
- **Architecture**: How it would fit into Neon
- **Pros**: Advantages of this approach
- **Cons**: Disadvantages and challenges
- **Complexity**: Implementation complexity (Low/Medium/High)
- **Risk**: Potential risks and unknowns
- **Examples**: How other languages/VMs do this

## 4. Cross-Cutting Concerns

Analyze impact across the system:
- Parser changes needed?
- New AST nodes required?
- Semantic analysis changes?
- New opcodes needed?
- VM state modifications?
- Memory management implications?
- Backwards compatibility concerns?
- Performance implications?
- Testing strategy?

## 5. External Research (if helpful)

Consider similar implementations in:
- Lua (for lightweight VMs)
- Python (for bytecode VMs)
- Ruby (for fibers/coroutines)
- Go (for goroutines)
- JavaScript (for async/await, event loops)

What can we learn from their approaches?

## Output Format

Return your research as a structured document:

```markdown
# Research Report: {Feature Name}

## Executive Summary
[2-3 paragraphs summarizing the feature, its complexity, and recommended approach]

## Codebase Analysis

### Current Architecture
[Analysis of relevant current code structure]

### Similar Features
[Analysis of related features already in Neon]

### Key Patterns Identified
- [Pattern 1]
- [Pattern 2]

## Implementation Approaches

### Approach 1: {Name}
**Overview**: [description]
**Architecture**: [how it fits]
**Pros**:
- [pro 1]
- [pro 2]
**Cons**:
- [con 1]
- [con 2]
**Complexity**: Low/Medium/High
**Risk**: Low/Medium/High

### Approach 2: {Name}
[same structure]

[... more approaches if applicable]

## Cross-Cutting Impact

### Compiler Changes
- [area]: [description]

### VM Changes
- [area]: [description]

### Testing Strategy
- [approach]

## Recommendation

**Recommended Approach**: [which approach and why]

**Key Decisions Needed from User**:
1. [decision 1 with options]
2. [decision 2 with options]

## Risks & Unknowns
- [risk 1]
- [risk 2]

## Estimated Complexity
Overall: Low/Medium/High
Reason: [explanation]
```

Be thorough, specific, and reference actual file paths and line numbers where relevant.
```

Save the research report to: `/home/patbuc/code/neon/.claude/workflows/{feature-slug}-research.md`

Update state file with research_findings summary.

### 4. User Confirmation Phase (NEW)

Update state: status = "confirming_approach"

Present the research findings to the user and ask for decisions:

**Use AskUserQuestion tool** to clarify:
1. **Which implementation approach** to use (if multiple viable options)
2. **Key architectural decisions** (e.g., data structure choices, where to place functionality)
3. **Scope confirmation** (which features to include/exclude)
4. **Priority trade-offs** (performance vs simplicity, backwards compatibility, etc.)

Example questions:
```
Question 1: "Which implementation approach should we use for fibers?"
- Approach A: Stackful coroutines (more powerful, higher complexity)
- Approach B: Stackless coroutines (simpler, limited but practical)
- Approach C: Generator-based (minimal changes, limited use cases)

Question 2: "Should fiber scheduling be cooperative or preemptive?"
- Cooperative: Simpler, requires explicit yield points
- Preemptive: More complex, automatic switching
```

Update state file with user's decisions in `implementation_approach` field.

### 5. Planning Phase

Update state: status = "planning"

Spawn a **Planner Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for Planner Agent:**
```
You are the Planning Agent for Neon language feature development.

Feature: {feature_description}
Worktree: {worktree_path}

IMPORTANT: All file paths and operations must be relative to the worktree directory: {worktree_path}

## Context from Research Phase

Research Report: {path to research report}
Key Findings: {summary from research}
Implementation Approach: {user's chosen approach}
Architectural Decisions: {user's decisions}

## Your Task

Based on the research and user decisions, create a detailed implementation plan:

1. Break down the feature into 5-10 atomic, sequential tasks
2. For each task, specify:
   - Clear description
   - Files to be modified (with specific locations)
   - Dependencies on other tasks
   - Acceptance criteria
   - Implementation approach (leveraging research findings)
   - Estimated complexity
   - Testing strategy

3. Ensure tasks follow this general order:
   - Foundation/infrastructure tasks first
   - Core functionality next
   - Integration with existing systems
   - Error handling and edge cases
   - Testing and documentation

4. Reference specific patterns and code locations identified in research

## Output Format

```markdown
# Implementation Plan: {Feature Name}

## Overview
[Based on research and user decisions, explain the implementation strategy]

## Implementation Approach
[Summarize the chosen approach and key decisions]

## Architecture Impact
- **Parser**: [changes needed]
- **AST**: [changes needed]
- **Semantic Analysis**: [changes needed]
- **Code Generation**: [changes needed]
- **VM**: [changes needed]
- **Testing**: [approach]

## Tasks

### Task 1: {Title}
**Description**: {detailed description}
**Files**:
- {file path} - {what changes}
- {file path} - {what changes}
**Dependencies**: {none or previous task numbers}
**Acceptance Criteria**:
- {criterion 1}
- {criterion 2}
**Implementation Notes**: {reference research findings}
**Test Strategy**: {how to verify}
**Complexity**: Low/Medium/High

### Task 2: {Title}
[same structure]

[... continue for all tasks]

## Integration Points
[How this feature integrates with existing systems]

## Risks & Mitigations
- Risk: {risk description}
  Mitigation: {how to address}

## Testing Strategy
- Unit tests: {approach}
- Integration tests: {approach}
- Manual testing: {scenarios}
```

IMPORTANT:
- Do NOT ask additional questions - use the research findings and user decisions
- Do NOT implement anything - only plan
- Be specific about file paths and code locations
- Reference patterns identified in research

Return your plan as a structured document.
```

Save the plan to: `/home/patbuc/code/neon/.claude/workflows/{feature-slug}-plan.md`

Update the state file with the task list from the planner.

### 6. Implementation Loop

For each task in the plan:

#### 6a. Coding Phase

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

## Current Task
{task.description}

## Context
Files to modify: {task.files}
Acceptance criteria: {task.acceptance_criteria}
Implementation notes: {task.implementation_notes}
Research findings: {relevant research excerpts}

## Previous Tasks
{context from previous completed tasks}

## Your Task

1. Read the relevant source files from the worktree
2. Review the research findings and implementation notes
3. Understand existing patterns identified in research phase
4. Implement the required changes following the recommended approach
5. Follow Rust best practices and Neon's code patterns
6. Ensure code compiles (run: cd {worktree_path} && cargo build)
7. Do NOT run tests - that's the Testing Agent's job

Implement ONLY this task. Be focused and precise.
Use the patterns and approaches identified in the research phase.
```

#### 6b. Testing Phase

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
1. Run: cd {worktree_path} && timeout 60 cargo test --verbose
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

#### 6c. Fix Loop (if needed)

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

### 7. Custom Review Phase (Pre-PR)

Update state: status = "reviewing"

Spawn a **Reviewer Agent** using the Task tool (subagent_type: "general-purpose"):

**Prompt for Reviewer Agent:**
```
You are the Code Review Agent for Neon language development.

Worktree: {worktree_path}
Feature: {feature_description}
Branch: {branch_name}
Tasks completed: {list of tasks}
Research findings: {path to research report}

IMPORTANT: Review files from the worktree directory: {worktree_path}

Your task:
1. Review all code changes for:
   - Code quality and Rust best practices
   - Architectural consistency with Neon patterns
   - Adherence to the implementation approach from research
   - Security issues (unsafe code, panics, error handling)
   - Test coverage
   - Documentation needs
2. Verify implementation matches research recommendations
3. Check for potential issues identified in research phase
4. Provide structured feedback:
   - Blocking issues (must fix)
   - Suggestions (nice to have)
   - Praise (what was done well)
5. Set review status: "approved", "changes_requested", or "commented"

Return your review as structured feedback.
```

### 8. Pre-PR Iteration Phase (if needed)

If review status == "changes_requested":
- Extract blocking issues from review
- Create new tasks for fixes
- Go back to step 6 (Implementation Loop)
- Limit to 2 review iterations before requesting user intervention

If review status == "approved":
- Proceed to PR creation

### 9. PR Creation Phase

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
Research report: {path to research report}
Implementation plan: {path to plan}

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
   - Implementation approach (reference research findings)
   - Key architectural decisions
   - Testing performed
   - Files changed summary
   - Link to research report and plan documents
2. Ensure branch is pushed: cd {worktree_path} && git push -u origin {branch_name}
3. Use gh CLI to create the PR from the worktree:
   cd {worktree_path} && gh pr create --title "{feature_description}" --body "{description}"
4. Return the PR URL

The PR will be created from the worktree branch.
```

Update state with pr_url.

### 10. GitHub Copilot Review Phase

Update state: status = "copilot_reviewing"

#### 10a. Request Copilot Review

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

#### 10b. Wait for Review Completion

Poll for review completion (Copilot reviews typically take < 30 seconds):

```bash
# Wait for review to appear (check every 5 seconds, max 60 seconds)
for i in {1..12}; do
  review=$(gh api repos/patbuc/neon/pulls/{pr_number}/reviews --jq '.[] | select(.user.login == "copilot")')
  if [ -n "$review" ]; then
    echo "Copilot review completed"
    break
  fi
  sleep 5
done
```

#### 10c. Fetch Copilot Review Suggestions

Once the review is complete, fetch the review comments:

```bash
# Get Copilot's review summary
gh api repos/patbuc/neon/pulls/{pr_number}/reviews \
  --jq '.[] | select(.user.login == "copilot") | {id: .id, state: .state, body: .body}'

# Get detailed review comments with file locations
gh api repos/patbuc/neon/pulls/{pr_number}/comments \
  --jq '.[] | select(.user.login == "copilot") | {path: .path, line: .line, body: .body}'
```

### 11. Address Copilot Feedback Phase

Update state: status = "addressing_copilot_feedback"

If Copilot provided suggestions:

#### 11a. Fetch and Group Feedback by Category

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

#### 11b. Address Each Category

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

  # After Coding Agent completes:
  # 1. Run tests
  cd {worktree_path}
  if ! timeout 120 cargo test --quiet 2>&1 | tee test_output.log; then
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

#### 11c. Coding Agent Prompt (per category)

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
5. Run tests: cd {worktree_path} && timeout 60 cargo test

Focus ONLY on {category} issues. Be precise.
DO NOT add watermarks or attribution to code or commit messages.
```

Update state: status = "completed"

### 12. Final Status

Report to user:
- PR URL
- Link to research report
- Link to implementation plan
- Copilot review addressed
- All tests passing
- Ready for human review/merge

## Output

Throughout the workflow, provide the user with:
- Real-time status updates
- Links to research report, plan, and state file
- Summaries of each phase
- Key decisions made
- Final PR URL when complete

## Error Handling

If any phase fails:
1. Update state with error details
2. Pause the workflow
3. Report to user with specific error
4. Ask user how to proceed

## State File Location

All state files, research reports, and plans are stored in: `.claude/workflows/`

File naming:
- State: `{feature-slug}-state.json`
- Research: `{feature-slug}-research.md`
- Plan: `{feature-slug}-plan.md`

## Differences from /build-feature

This orchestrator adds:
1. **Research Phase**: Deep codebase exploration and analysis
2. **Multiple Approaches**: Identification and comparison of implementation strategies
3. **User Confirmation**: Explicit approval of approach and key decisions
4. **Research-Informed Planning**: Plans that leverage research findings
5. **Implementation Guidance**: Coding agents receive research context

Use this orchestrator for complex, architectural features that benefit from thorough upfront analysis.
