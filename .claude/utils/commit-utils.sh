#!/bin/bash
# Lightweight commit utilities for orchestration

# Watermark patterns to block (CRITICAL - currently being added!)
WATERMARK_PATTERNS=(
  "🤖.*[Gg]enerated with.*[Cc]laude"
  "[Gg]enerated with.*[Cc]laude [Cc]ode"
  "Co-Authored-By:.*[Cc]laude"
  "Co-Authored-By:.*noreply@anthropic.com"
  "claude\\.com/claude-code"
)

validate_no_watermarks() {
  local message="$1"
  for pattern in "${WATERMARK_PATTERNS[@]}"; do
    if echo "$message" | grep -Eq "$pattern"; then
      echo "ERROR: Watermark detected in commit message!"
      echo "Pattern: $pattern"
      return 1
    fi
  done
  return 0
}

has_uncommitted_changes() {
  [[ -n "$(git status --porcelain)" ]]
}

generate_task_commit_message() {
  local task_description="$1"

  # Determine commit type from task description
  local commit_type="feat"
  if echo "$task_description" | grep -Eiq "^(fix|bug)"; then
    commit_type="fix"
  elif echo "$task_description" | grep -Eiq "^test"; then
    commit_type="test"
  elif echo "$task_description" | grep -Eiq "^refactor"; then
    commit_type="refactor"
  fi

  # Simple semantic format (NO WATERMARKS!)
  echo "$commit_type: $task_description"
}

safe_task_commit() {
  local task_description="$1"

  # Check for changes
  if ! has_uncommitted_changes; then
    echo "INFO: No changes to commit"
    return 0
  fi

  # Generate message
  local commit_message=$(generate_task_commit_message "$task_description")

  # Validate no watermarks
  if ! validate_no_watermarks "$commit_message"; then
    echo "ERROR: Watermark validation failed!"
    return 1
  fi

  # Stage and commit
  git add .
  if git commit -m "$commit_message"; then
    echo "✓ Committed: $commit_message"
    return 0
  else
    echo "ERROR: Git commit failed"
    git reset HEAD
    return 1
  fi
}

categorize_copilot_issue() {
  local issue_body="$1"

  # Simple keyword-based categorization
  if echo "$issue_body" | grep -Eiq "(unsafe|security|vulnerability|overflow|bounds)"; then
    echo "security"
  elif echo "$issue_body" | grep -Eiq "(performance|slow|optimize|allocat|expensive)"; then
    echo "performance"
  elif echo "$issue_body" | grep -Eiq "(style|format|naming|convention|idiomatic)"; then
    echo "style"
  elif echo "$issue_body" | grep -Eiq "(test|coverage|assert)"; then
    echo "testing"
  else
    echo "refactoring"
  fi
}

get_category_commit_message() {
  local category="$1"

  case "$category" in
    security)      echo "fix: Address security issues from Copilot review" ;;
    performance)   echo "perf: Optimize performance based on Copilot feedback" ;;
    style)         echo "style: Apply code style improvements from Copilot" ;;
    testing)       echo "test: Add test coverage per Copilot suggestions" ;;
    refactoring)   echo "refactor: Refactor code per Copilot review" ;;
  esac
}

safe_copilot_commit() {
  local category="$1"

  # Check for changes
  if ! has_uncommitted_changes; then
    echo "INFO: No changes for category $category"
    return 0
  fi

  # Generate message
  local commit_message=$(get_category_commit_message "$category")

  # Validate no watermarks
  if ! validate_no_watermarks "$commit_message"; then
    echo "ERROR: Watermark validation failed!"
    return 1
  fi

  # Stage and commit
  git add .
  if git commit -m "$commit_message"; then
    echo "✓ Committed: $commit_message (category: $category)"
    return 0
  else
    echo "ERROR: Git commit failed"
    git reset HEAD
    return 1
  fi
}
