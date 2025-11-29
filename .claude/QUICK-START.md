# Orchestration System - Quick Start Guide

## 🚀 Build a Feature in 1 Command

```bash
/build-feature "Your feature description"
```

That's it! The system will:
- ✅ Plan the implementation
- ✅ Break it into tasks
- ✅ Implement each task
- ✅ Run tests
- ✅ Create a PR
- ✅ Review the code
- ✅ Iterate on feedback

## 📋 Available Commands

| Command | Purpose | Example |
|---------|---------|---------|
| `/build-feature` | Full automation | `/build-feature "Add arrays"` |
| `/plan-feature` | Just planning | `/plan-feature "Add for loops"` |
| `/implement-task` | Code a task | `/implement-task 1` |
| `/run-tests` | Test current code | `/run-tests` |
| `/create-pr` | Create PR | `/create-pr` |
| `/review-pr` | Review code | `/review-pr` |

## 🎯 Workflow Options

### Option 1: Full Automation (Recommended for Simple Features)

```bash
/build-feature "Add single-line comments"
```

Sit back and watch the magic happen!

### Option 2: Manual Control (Recommended for Complex Features)

```bash
# 1. Plan first
/plan-feature "Add struct types"

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

## 🔍 Check Progress

State files track everything:

```bash
# View current state
cat .claude/workflows/*-state.json | jq

# See the plan
cat .claude/workflows/*-plan.md
```

## ⚡ Examples

### Add a Simple Feature
```bash
/build-feature "Add modulo operator (%)"
```

### Add a Complex Feature
```bash
/plan-feature "Add closures with lexical scoping"
# Review plan
/implement-task 1
/run-tests
# Continue...
```

### Fix a Bug
```bash
/plan-feature "Fix panic when calling undefined function"
/implement-task 1
/run-tests
/create-pr "fix: Handle undefined function calls gracefully"
```

## 📚 Full Documentation

See [ORCHESTRATION.md](ORCHESTRATION.md) for complete documentation.

## 🐛 Troubleshooting

**gh CLI not working?**
```bash
gh auth login
```

**Tests failing?**
```bash
/run-tests  # Get detailed failure analysis
```

**Want to start over?**
```bash
git checkout main
git branch -D feature/old-feature
```

## 💡 Tips

1. **Be Specific**: "Add array support with literals and indexing" > "arrays"
2. **Test Often**: Run `/run-tests` after each task
3. **Review Plans**: For complex features, review `/plan-feature` output before proceeding
4. **Trust the Process**: The agents are smart - let them work!

---

Ready to build? Try:
```bash
/build-feature "Add your first feature!"
```
