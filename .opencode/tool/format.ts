import { tool } from "@opencode-ai/plugin"
export default tool({
  description: "Run cargo fmt",
  args: {
    workdir: tool.schema.string().describe("Absolute path to worktree"),
    check: tool.schema.boolean().optional().describe("Only check formatting"),
  },
  async execute(args) {
    if (!args.workdir) throw new Error("workdir is required")
    const cmd = args.check ? "cargo fmt -- --check" : "cargo fmt"
    try {
      const output = await Bun.$`${cmd}`.text({ cwd: args.workdir })
      return JSON.stringify({ raw: output, check: !!args.check })
    } catch (e) {
      return JSON.stringify({ raw: e.toString(), check: !!args.check, error: true })
    }
  },
})
