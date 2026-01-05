---
description: "List all ADRs and their current status"
allowed-tools: ["execute", "read"]
---

# List All ADRs

Show all architectural decision records and their status.

## Task

1. List all ADRs
   ```bash
   ls -la docs/adr/
   ```

2. For each ADR, show:
   - Filename (ADR-NNNN.md)
   - Title (extracted from markdown)
   - Status (Proposed, Accepted, Deprecated, Superseded)
   - Date

3. Show relationships
   - Which ADRs reference each other?
   - Which are deprecated/superseded?

4. Output as a formatted table
   - Makes it easy to see what decisions have been made
   - Highlights active constraints

## Usage

Run at any time to understand what architectural decisions are currently active.
