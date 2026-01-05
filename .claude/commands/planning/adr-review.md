---
description: "Review existing ADRs related to a topic"
allowed-tools: ["read", "execute"]
---

# Review Existing ADRs

You are reviewing architectural decisions to understand constraints.

## Your Task

Review ADRs related to: **$ARGUMENTS**

### Review Process

1. **List All ADRs**
   ```bash
   ls -la docs/adr/
   ```

2. **Read Relevant ADRs**
   - Read ADRs that relate to the topic
   - Note their status (Accepted, Deprecated, Superseded)
   - Identify key constraints and decisions

3. **Analyze Relationships**
   - Which ADRs reference each other?
   - Are there any conflicting decisions?
   - What patterns emerge from the decisions?

4. **Summarize Findings**
   - List relevant ADRs with brief summaries
   - Highlight key constraints for the topic
   - Note any gaps in decision coverage

### Output Format

```
## Relevant ADRs for [Topic]

### ADR-0001: [Title]
**Status**: Accepted
**Summary**: [Brief summary]
**Key Constraints**: [List constraints]

### ADR-0003: [Title]
**Status**: Accepted
**Summary**: [Brief summary]
**Key Constraints**: [List constraints]

## Analysis
- Pattern 1: [Observation]
- Pattern 2: [Observation]
- Gaps: [What decisions haven't been made yet?]
```
