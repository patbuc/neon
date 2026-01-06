---
description: Implement a feature from an accepted ADR in a new worktree
agent: build
---

# Implement Feature

You are implementing a feature that was previously designed and accepted via `/design`.

## Instructions

### 1. Find the ADR

- If an argument is provided, look for: `docs/adr/*-$ARGUMENTS.md` (glob match)
- Otherwise, list ADRs in `docs/adr/` and use the most recently modified one with status `Accepted`
- If no accepted ADRs exist, inform the user and stop
- Extract the ADR number (NNNNNN) and feature slug from the filename

### 2. Create a Worktree

Create a new worktree for this implementation:

```bash
wt switch --create adr-NNNNNN-feature-slug
```

For example, if implementing `docs/adr/000001-while-loops.md`:
```bash
wt switch --create adr-000001-while-loops
```

**IMPORTANT**: After running `wt switch`, you are now in a NEW directory. The worktree path will be printed by the command. All subsequent work must happen in that new worktree directory. Use the `workdir` parameter for all bash commands to ensure you're working in the correct location.

### 3. Review the ADR

Read the ADR thoroughly to understand:
- The **Context** and **Decision Drivers**
- The **Decision Outcome** (the chosen approach)
- The **Implementation Notes** (phases/steps to follow)

### 4. Plan the Work

- Use the TodoWrite tool to create tasks from the Implementation Notes
- If the ADR has phases, create a todo for each phase
- Break down phases into subtasks as needed

### 5. Implement Phase by Phase

For each phase in the Implementation Notes:

1. Mark the phase todo as `in_progress`
2. Implement the changes for that phase
3. Run `cargo build` to verify compilation
4. Run `cargo test` to ensure nothing is broken
5. **Commit the phase**:
   ```bash
   git add -A && git commit -m "Phase N: <phase name from ADR>"
   ```
6. Mark the phase todo as `completed`
7. Move to the next phase

### 6. Push and Create PR

After all phases are complete:

1. Push the branch to origin:
   ```bash
   git push -u origin adr-NNNNNN-feature-slug
   ```

2. Create a pull request using `gh pr create`. Use a HEREDOC for the body:
   ```bash
   gh pr create --title "<ADR Title>" --body "$(cat <<'EOF'
   ## Summary
   <Brief description from ADR Decision Outcome - 1-3 sentences>

   ## ADR
   See: docs/adr/NNNNNN-feature-slug.md
   EOF
   )"
   ```

3. Return the PR URL to the user

### 7. Inform User About Cleanup

Tell the user:
- The PR has been created
- They are still in the feature worktree
- When ready to clean up, run `/implement-cleanup` to remove the worktree and return to main

---

## Important

- Follow the ADR closely - it represents an accepted architectural decision
- Maintain the project's code conventions (see CLAUDE.md)
- Use proper error handling with `Result<T, E>`
- Include helpful comments for complex logic
- **Commit after each phase** - this makes review easier and provides checkpoints
- If you encounter issues not covered by the ADR, note them and proceed with your best judgment
- Do NOT modify the ADR file itself during implementation
- After `wt switch`, always verify you're in the correct worktree directory
