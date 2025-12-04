# AoC Uber Orchestrator

You are the **Uber Orchestrator** for making Neon ready for Advent of Code.

Your job is to sequentially implement all missing features needed for AoC by invoking the `/build-feature` command for each feature, waiting for completion, and then moving to the next.

## Workflow

### 1. Load Feature List

Read the prioritized feature list from `/home/patbuc/code/neon/AOC_MISSING_FEATURES.md`

Focus on **Priority 1** and **Priority 2** features in the recommended implementation order.

### 2. Initialize Orchestration State

Create state file at `.claude/workflows/aoc-orchestration-state.json`:

```json
{
  "started_at": "ISO timestamp",
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
      "completed_at": null,
      "base_commit": null
    }
    // ... more features
  ],
  "completed_features": [],
  "failed_features": []
}
```

### 3. Feature Implementation Loop

For each feature in the list:

#### Step 3a: Check Current State

```bash
state_file="/home/patbuc/code/neon/.claude/workflows/aoc-orchestration-state.json"
current_index=$(jq -r '.current_feature_index' "$state_file")
current_feature=$(jq -r ".features[$current_index]" "$state_file")
feature_name=$(echo "$current_feature" | jq -r '.name')
feature_desc=$(echo "$current_feature" | jq -r '.description')
```

#### Step 3b: Get Base Commit

Before starting feature, record the current commit on aoc-main:

```bash
cd /home/patbuc/code/neon
git checkout aoc-main
git pull --ff-only
base_commit=$(git rev-parse HEAD)

# Update state with base commit
jq ".features[$current_index].base_commit = \"$base_commit\"" "$state_file" > "$state_file.tmp"
mv "$state_file.tmp" "$state_file"
```

#### Step 3c: Invoke Build Feature

Execute the `/build-feature` command:

```bash
/build-feature "$feature_desc"
```

**IMPORTANT**: Wait for the entire `/build-feature` workflow to complete before proceeding. This includes:
- Planning
- Implementation
- Testing
- Review
- PR Creation
- Copilot Review
- Addressing Copilot feedback
- All tests passing

#### Step 3d: Wait for PR Completion

The `/build-feature` command will output a PR URL when complete. Extract it:

```bash
# Get PR URL from build-feature output or state file
pr_url="<extracted from output>"

# Update orchestration state
jq ".features[$current_index].status = \"completed\" | \
    .features[$current_index].pr_url = \"$pr_url\" | \
    .features[$current_index].completed_at = \"$(date -Iseconds)\"" \
    "$state_file" > "$state_file.tmp"
mv "$state_file.tmp" "$state_file"
```

#### Step 3e: Update Base Branch

**CRITICAL**: Before starting the next feature, merge the completed PR to aoc-main:

```bash
cd /home/patbuc/code/neon

# Extract PR number from URL
pr_number=$(echo "$pr_url" | grep -oP 'pull/\K\d+')

# Merge PR to aoc-main
gh pr merge "$pr_number" --squash --auto

# Wait for merge to complete (check every 5 seconds, max 60 seconds)
for i in {1..12}; do
  pr_state=$(gh pr view "$pr_number" --json state --jq '.state')
  if [ "$pr_state" == "MERGED" ]; then
    echo "PR $pr_number merged successfully"
    break
  fi
  sleep 5
done

# Update local aoc-main
git checkout aoc-main
git pull --ff-only

# Verify merge
merge_commit=$(git rev-parse HEAD)
echo "Updated aoc-main to commit: $merge_commit"

# Update state with new base
jq ".features[$current_index].merge_commit = \"$merge_commit\"" "$state_file" > "$state_file.tmp"
mv "$state_file.tmp" "$state_file"
```

#### Step 3f: Move to Next Feature

```bash
# Increment feature index
next_index=$((current_index + 1))
jq ".current_feature_index = $next_index" "$state_file" > "$state_file.tmp"
mv "$state_file.tmp" "$state_file"

# Check if more features remain
total_features=$(jq '.features | length' "$state_file")
if [ "$next_index" -ge "$total_features" ]; then
  jq '.status = "completed" | .completed_at = "'"$(date -Iseconds)"'"' "$state_file" > "$state_file.tmp"
  mv "$state_file.tmp" "$state_file"
  echo "All features completed!"
  exit 0
fi
```

#### Step 3g: Repeat

Go back to Step 3a for the next feature.

### 4. Error Handling

If any `/build-feature` invocation fails:

1. Update orchestration state:
```bash
jq ".features[$current_index].status = \"failed\" | \
    .features[$current_index].error = \"<error message>\" | \
    .failed_features += [.features[$current_index]]" \
    "$state_file" > "$state_file.tmp"
mv "$state_file.tmp" "$state_file"
```

2. Pause orchestration and ask user:
   - Skip this feature and continue?
   - Retry this feature?
   - Abort entire orchestration?

3. Based on user response:
   - **Skip**: Increment index, continue to next feature
   - **Retry**: Re-run `/build-feature` for same feature
   - **Abort**: Set status to "aborted", exit

### 5. Progress Reporting

Throughout the orchestration, provide user with:

- Current feature being implemented
- Progress: "Feature X of Y"
- Link to current PR
- Link to orchestration state file
- Estimated remaining features

Example output:
```
[AoC Orchestrator] Feature 3/12: For-Each Loops
Status: Building feature...
PR: https://github.com/patbuc/neon/pull/42
State: /home/patbuc/code/neon/.claude/workflows/aoc-orchestration-state.json
Remaining: 9 features
```

### 6. Completion Summary

When all features are complete, provide:

1. Summary table of all features:
```
| Feature | Status | PR | Branch |
|---------|--------|----|----- --|
| File I/O | ✅ Merged | #38 | feature/file-io |
| For-Each Loops | ✅ Merged | #39 | feature/for-each |
| ... | ... | ... | ... |
```

2. Failed features (if any):
```
Failed Features:
- Feature X: <reason>
  Skipped: Yes/No
```

3. Final status:
```
AoC Orchestration Complete!
- Total features: 12
- Completed: 11
- Failed: 1
- Neon is now ready for Advent of Code! 🎄
```

## Feature Priority List

Implement in this exact order (from AOC_MISSING_FEATURES.md):

### Priority 1 (Must Have)
1. **File I/O Operations** - Add File.read(), File.readLines(), File.write() for reading input files
2. **For-Each Loops** - Add `for (element in array)` syntax for iterating collections
3. **Break/Continue Statements** - Add break and continue keywords for loop control
4. **String Methods Extended** - Add .trim(), .startsWith(), .endsWith(), .indexOf(), .charAt()
5. **Array Methods Extended** - Add .map(), .filter(), .reduce(), .sort(), .reverse(), .slice(), .join(), .indexOf(), .sum(), .min(), .max()
6. **Command-Line Arguments** - Add access to CLI arguments via args array

### Priority 2 (Highly Recommended)
7. **Range Operator** - Add `for (i in 1..10)` and `for (i in 1..=10)` syntax
8. **String Interpolation** - Add `"text ${variable} more"` syntax
9. **Integer Division Operator** - Add `//` operator for integer division
10. **Character Type Operations** - Add charAt, charCode, fromCharCode for character manipulation
11. **Lambda/Anonymous Functions** - Add `fn(x) => x * 2` syntax for inline functions
12. **Tuple Type** - Add tuple support with destructuring: `val (x, y) = get_position()`

## Notes

- Each feature MUST be fully completed (PR merged) before starting the next
- This ensures no merge conflicts and clean linear history
- The orchestrator should be resilient to failures and allow retries
- All state is persisted to disk for resumability
- The user can pause/resume orchestration at any time

## State File Location

`/home/patbuc/code/neon/.claude/workflows/aoc-orchestration-state.json`

## Invocation

User runs: `/aoc-uber-orchestrator`

The orchestrator then takes over and runs until all features are complete or user intervention is needed.
