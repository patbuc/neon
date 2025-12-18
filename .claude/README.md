# Claude Code Configuration

This directory contains the configuration and orchestration system for Claude Code.

## Structure

```
.claude/
├── commands/              # Slash commands (agents)
│   ├── build-feature.md   # Main orchestrator
│   ├── plan-feature.md    # Planning agent
│   ├── implement-task.md  # Coding agent
│   ├── run-tests.md       # Testing agent
│   ├── create-pr.md       # PR creation agent
│   └── review-pr.md       # Code review agent
├── workflows/             # State and plan files
│   ├── .gitignore         # Ignore generated files
│   └── state-template.json # State file template
├── settings.local.json    # Claude Code settings
├── ORCHESTRATION.md       # Full documentation
├── QUICK-START.md         # Quick reference
└── README.md              # This file
```

## Quick Links

- **Getting Started**: See [QUICK-START.md](QUICK-START.md)
- **Full Documentation**: See [ORCHESTRATION.md](ORCHESTRATION.md)

## Slash Commands

All commands are available in Claude Code:

- `/build-feature` - Full automation
- `/plan-feature` - Planning only
- `/implement-task` - Code implementation
- `/run-tests` - Test execution
- `/create-pr` - PR creation
- `/review-pr` - Code review

## State Management

Workflow state is tracked in `workflows/` directory:
- `{feature}-state.json` - Current workflow state
- `{feature}-plan.md` - Implementation plan

These files are gitignored (not committed to the repository).

## How It Works

Each slash command is a specialized agent that:
1. Reads the command markdown file
2. Follows the instructions within
3. Uses Claude Code's tools (Read, Write, Edit, Bash, etc.)
4. Updates state files
5. Reports results to the user

The orchestrator (`/build-feature`) coordinates all agents to build complete features automatically.

## Configuration

### settings.local.json

Contains Claude Code permissions. Currently allows:
- `Bash(find:*)`
- `Bash(cat:*)`
- `Bash(xargs ls:*)`
- `Bash(git log --oneline -15)`
- `Bash(cargo test:*)`

Add more permissions as needed.

## Customization

To customize agent behavior:
1. Edit the relevant command file in `commands/`
2. Follow the existing format
3. Test with Claude Code
4. Commit changes

## Examples

### Build a Complete Feature
```bash
/build-feature "Add array support"
```

### Plan and Review
```bash
/plan-feature "Add closures"
# Review plan
# Then either:
/build-feature "Add closures"  # Full auto
# Or implement manually
```

### Manual Control
```bash
/plan-feature "Add feature"
/implement-task 1
/run-tests
/implement-task 2
/run-tests
/create-pr
/review-pr
```

## Support

For help:
1. Check [ORCHESTRATION.md](ORCHESTRATION.md)
2. Ask Claude Code
3. File an issue on GitHub

## Version

Orchestration System v1.0
Created: 2025-11-29
