import { tool } from "@opencode-ai/plugin"

function slugify(input: string): string {
  const base = input
    .toLowerCase()
    .replace(/[^a-z0-9\s-_]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
    .replace(/^[-_]+|[-_]+$/g, "")
  const trimmed = base.slice(0, 64)
  return trimmed.length ? trimmed : "feature"
}

export default tool({
  description: "Create or use a git worktree for feature/<slug>",
  args: {
    feature: tool.schema.string().describe("Human-readable feature description"),
    repoPath: tool.schema.string().describe("Absolute path to main repo"),
    worktreesParent: tool.schema.string().describe("Absolute path where worktrees live"),
  },
  async execute(args) {
    const slug = slugify(args.feature)
    const branch = `feature/${slug}`
    const worktreePath = `${args.worktreesParent}/${branch}`

    // Ensure parent dir exists
    await Bun.$`mkdir -p ${args.worktreesParent}`.text()

    // Check if worktree dir exists; if not, add it
    const exists = await Bun.$`[ -d ${worktreePath} ] && echo exists || echo missing`.text()
    let created = false
    if (exists.trim() === "missing") {
      // Create branch and worktree (branch may not exist)
      try {
        await Bun.$`git -C ${args.repoPath} worktree add ${worktreePath} -b ${branch}`.text()
        created = true
      } catch (e) {
        // If branch exists, add without -b
        await Bun.$`git -C ${args.repoPath} worktree add ${worktreePath} ${branch}`.text()
        created = true
      }
    }

    return JSON.stringify({ slug, branch, worktreePath, created })
  },
})
