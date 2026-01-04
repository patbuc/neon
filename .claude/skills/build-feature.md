# Build Feature Skill

Invoke the orchestrated feature development workflow.

## Usage

```
/build-feature [description of the feature]
```

## What This Does

This skill spawns the orchestrator agent to coordinate feature development through a three-phase workflow:

1. **Planning**: Analyzes request, breaks into steps, creates plan file, gets your approval
2. **Implementation**: For each step, spawns coding-agent then quality-gate-agent
3. **Completion**: Creates PR when all steps pass

## Instructions for Claude

When this skill is invoked, spawn the orchestrator agent using the Task tool:

```
Task(
  subagent_type="general-purpose",
  description="Orchestrate feature development",
  prompt="You are the ORCHESTRATOR AGENT for the Neon project's feature development workflow.

Read your full instructions from: .claude/agents/orchestrator-agent.md

USER REQUEST:
[user's feature request goes here]

IMPORTANT:
- Follow the orchestrator-agent.md instructions exactly
- Create plan file at .claude/plans/feature-{slug}.md
- Get user approval before implementing
- Spawn sub-agents for implementation (see below)

When spawning the CODING AGENT, use:
Task(subagent_type='general-purpose', model='sonnet', description='Implement step N', prompt='You are the CODING AGENT. Read instructions from .claude/agents/coding-agent.md. [step details]...')

When spawning the QUALITY GATE AGENT, use:
Task(subagent_type='general-purpose', model='haiku', description='Review step N', prompt='You are the QUALITY GATE AGENT. Read instructions from .claude/agents/quality-gate-agent.md. [review context]...')"
)
```

## Example

```
User: /build-feature Add support for do-while loops

Claude: I'll start the orchestrated feature development workflow for do-while loops.
[Spawns orchestrator agent which will create a plan and ask for approval]
```
