#!/bin/bash
set -euo pipefail

# AoC Uber Orchestrator Runner Script
# This script coordinates sequential feature implementation for AoC readiness

STATE_FILE="/home/patbuc/code/neon/.claude/workflows/aoc-orchestration-state.json"
REPO_DIR="/home/patbuc/code/neon"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[AoC Orchestrator]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[AoC Orchestrator]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[AoC Orchestrator]${NC} $1"
}

log_error() {
    echo -e "${RED}[AoC Orchestrator]${NC} $1"
}

# Initialize orchestration state
initialize_state() {
    log_info "Initializing orchestration state..."

    mkdir -p "$(dirname "$STATE_FILE")"

    cat > "$STATE_FILE" <<'EOF'
{
  "started_at": "",
  "status": "in_progress",
  "current_feature_index": 0,
  "features": [
    {
      "name": "File I/O Operations",
      "description": "Add File.read(), File.readLines(), File.write() for reading input files",
      "priority": 1,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "For-Each Loops",
      "description": "Add for-in loop syntax for iterating over collections (arrays, sets, maps)",
      "priority": 1,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Break/Continue Statements",
      "description": "Add break and continue keywords for early loop termination and iteration control",
      "priority": 1,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "String Methods Extended",
      "description": "Add string methods: trim(), startsWith(), endsWith(), indexOf(), charAt(), toUpperCase(), toLowerCase()",
      "priority": 1,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Array Methods Extended",
      "description": "Add array methods: map(), filter(), reduce(), sort(), reverse(), slice(), join(), indexOf(), sum(), min(), max()",
      "priority": 1,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Command-Line Arguments",
      "description": "Add access to command-line arguments via global args array or similar mechanism",
      "priority": 1,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Range Operator",
      "description": "Add range syntax for loops: for (i in 1..10) and for (i in 1..=10)",
      "priority": 2,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "String Interpolation",
      "description": "Add string interpolation with ${} syntax for embedding expressions in strings",
      "priority": 2,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Integer Division Operator",
      "description": "Add // operator for integer division (floor division)",
      "priority": 2,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Character Type Operations",
      "description": "Add character operations: charAt(), charCode(), fromCharCode() for working with individual characters",
      "priority": 2,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Lambda/Anonymous Functions",
      "description": "Add lambda function syntax for inline function definitions: fn(x) => x * 2",
      "priority": 2,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    },
    {
      "name": "Tuple Type",
      "description": "Add tuple support with destructuring: val (x, y) = get_position()",
      "priority": 2,
      "status": "pending",
      "branch": null,
      "pr_url": null,
      "pr_number": null,
      "completed_at": null,
      "base_commit": null,
      "merge_commit": null
    }
  ],
  "completed_features": [],
  "failed_features": []
}
EOF

    # Set start timestamp
    jq ".started_at = \"$(date -Iseconds)\"" "$STATE_FILE" > "$STATE_FILE.tmp"
    mv "$STATE_FILE.tmp" "$STATE_FILE"

    log_success "State initialized at: $STATE_FILE"
}

# Get current feature
get_current_feature() {
    local index=$(jq -r '.current_feature_index' "$STATE_FILE")
    jq -r ".features[$index]" "$STATE_FILE"
}

# Update feature in state
update_feature_field() {
    local index=$1
    local field=$2
    local value=$3

    jq ".features[$index].$field = $value" "$STATE_FILE" > "$STATE_FILE.tmp"
    mv "$STATE_FILE.tmp" "$STATE_FILE"
}

# Get base commit from aoc-main
update_base_commit() {
    local index=$1

    cd "$REPO_DIR"
    git checkout aoc-main
    git pull --ff-only
    local base_commit=$(git rev-parse HEAD)

    log_info "Base commit: $base_commit"
    update_feature_field "$index" "base_commit" "\"$base_commit\""
}

# Show progress
show_progress() {
    local current_index=$(jq -r '.current_feature_index' "$STATE_FILE")
    local total=$(jq '.features | length' "$STATE_FILE")
    local current_feature=$(jq -r ".features[$current_index].name" "$STATE_FILE")

    echo ""
    log_info "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    log_info "Progress: Feature $((current_index + 1)) of $total"
    log_info "Current: $current_feature"
    log_info "State: $STATE_FILE"
    log_info "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

# Main orchestration loop
run_orchestration() {
    log_info "Starting AoC Feature Orchestration..."

    # Initialize if state doesn't exist
    if [ ! -f "$STATE_FILE" ]; then
        initialize_state
    fi

    local total_features=$(jq '.features | length' "$STATE_FILE")

    while true; do
        local current_index=$(jq -r '.current_feature_index' "$STATE_FILE")

        # Check if we're done
        if [ "$current_index" -ge "$total_features" ]; then
            log_success "🎉 All features completed!"
            show_final_summary
            break
        fi

        show_progress

        local feature=$(get_current_feature)
        local feature_name=$(echo "$feature" | jq -r '.name')
        local feature_desc=$(echo "$feature" | jq -r '.description')
        local feature_status=$(echo "$feature" | jq -r '.status')

        # Skip if already completed
        if [ "$feature_status" == "completed" ]; then
            log_info "Feature already completed, moving to next..."
            jq ".current_feature_index += 1" "$STATE_FILE" > "$STATE_FILE.tmp"
            mv "$STATE_FILE.tmp" "$STATE_FILE"
            continue
        fi

        log_info "Starting feature: $feature_name"
        log_info "Description: $feature_desc"

        # Update base commit
        update_base_commit "$current_index"

        # Mark as in progress
        update_feature_field "$current_index" "status" '"in_progress"'

        # Here we would invoke /build-feature via Claude
        # For now, we output instructions for the user
        echo ""
        log_warning "Ready to build feature. Please tell Claude:"
        echo ""
        echo "    /build-feature \"$feature_desc\""
        echo ""
        log_warning "After the PR is created and merged, run this script again to continue."
        echo ""

        # Wait for user to provide PR URL
        read -p "Enter PR URL (or 'skip' to skip, 'abort' to abort): " pr_input

        if [ "$pr_input" == "abort" ]; then
            log_error "Orchestration aborted by user"
            jq '.status = "aborted"' "$STATE_FILE" > "$STATE_FILE.tmp"
            mv "$STATE_FILE.tmp" "$STATE_FILE"
            exit 1
        elif [ "$pr_input" == "skip" ]; then
            log_warning "Skipping feature: $feature_name"
            update_feature_field "$current_index" "status" '"skipped"'
            jq ".current_feature_index += 1" "$STATE_FILE" > "$STATE_FILE.tmp"
            mv "$STATE_FILE.tmp" "$STATE_FILE"
            continue
        fi

        # Extract PR number
        local pr_number=$(echo "$pr_input" | grep -oP 'pull/\K\d+' || echo "")

        if [ -z "$pr_number" ]; then
            log_error "Invalid PR URL format"
            continue
        fi

        log_info "PR #$pr_number recorded"
        update_feature_field "$current_index" "pr_url" "\"$pr_input\""
        update_feature_field "$current_index" "pr_number" "$pr_number"

        # Wait for merge
        log_info "Waiting for PR merge..."
        local merged=false
        for i in {1..60}; do
            local pr_state=$(gh pr view "$pr_number" --json state --jq '.state' 2>/dev/null || echo "UNKNOWN")
            if [ "$pr_state" == "MERGED" ]; then
                log_success "PR #$pr_number merged!"
                merged=true
                break
            fi
            echo -n "."
            sleep 5
        done
        echo ""

        if [ "$merged" == "false" ]; then
            log_error "PR merge timeout. Please merge manually and run again."
            exit 1
        fi

        # Update aoc-main
        cd "$REPO_DIR"
        git checkout aoc-main
        git pull --ff-only
        local merge_commit=$(git rev-parse HEAD)

        log_success "Updated aoc-main to: $merge_commit"
        update_feature_field "$current_index" "merge_commit" "\"$merge_commit\""
        update_feature_field "$current_index" "status" '"completed"'
        update_feature_field "$current_index" "completed_at" "\"$(date -Iseconds)\""

        # Move to next feature
        jq ".current_feature_index += 1" "$STATE_FILE" > "$STATE_FILE.tmp"
        mv "$STATE_FILE.tmp" "$STATE_FILE"

        log_success "Feature completed: $feature_name"
        echo ""
    done
}

# Show final summary
show_final_summary() {
    echo ""
    log_info "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    log_success "AoC Orchestration Complete!"
    log_info "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    echo "Feature Summary:"
    echo ""

    jq -r '.features[] | "  [\(.status)] \(.name) - PR: \(.pr_url // "N/A")"' "$STATE_FILE"

    echo ""
    local completed=$(jq '[.features[] | select(.status == "completed")] | length' "$STATE_FILE")
    local failed=$(jq '[.features[] | select(.status == "failed")] | length' "$STATE_FILE")
    local skipped=$(jq '[.features[] | select(.status == "skipped")] | length' "$STATE_FILE")
    local total=$(jq '.features | length' "$STATE_FILE")

    log_info "Total: $total"
    log_success "Completed: $completed"
    log_warning "Skipped: $skipped"
    log_error "Failed: $failed"
    echo ""
    log_success "Neon is now ready for Advent of Code! 🎄"
    echo ""
}

# Run the orchestration
run_orchestration
