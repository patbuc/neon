import { tool } from "@opencode-ai/plugin"
export default tool({
  description: "Run cargo test in worktree",
  args: {
    workdir: tool.schema.string().describe("Absolute path to worktree"),
    subset: tool.schema.string().optional().describe("Optional test filter"),
  },
  async execute(args) {
    if (!args.workdir) throw new Error("workdir is required")
    const cmd = args.subset ? `cargo test ${args.subset}` : "cargo test"
    const output = await Bun.$`${cmd}`.text({ cwd: args.workdir })
    // Basic summary parsing for counts
    const summaryMatch = output.match(/test result: ([^\n]+)/)
    const summary = summaryMatch ? summaryMatch[1] : ""
    return JSON.stringify({ summary, raw: output })
  },
})
