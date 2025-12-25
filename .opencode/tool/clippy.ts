import { tool } from "@opencode-ai/plugin"
export default tool({
  description: "Run cargo clippy with all targets",
  args: {
    workdir: tool.schema.string().describe("Absolute path to worktree"),
    fix: tool.schema.boolean().optional().describe("Apply fixes where possible"),
  },
  async execute(args) {
    if (!args.workdir) throw new Error("workdir is required")
    const flags = "--all-targets --all-features"
    const cmd = args.fix ? `cargo clippy ${flags} --fix -Z unstable-options` : `cargo clippy ${flags}`
    const output = await Bun.$`${cmd}`.text({ cwd: args.workdir })
    return JSON.stringify({ raw: output, fixApplied: !!args.fix })
  },
})
