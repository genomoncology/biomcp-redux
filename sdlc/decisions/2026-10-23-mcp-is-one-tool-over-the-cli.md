# MCP is one tool over the CLI

Decided 2026-10-23 by Ian.

- The MCP server offers one main tool. It invokes the same searches and gets through the CLI's subcommand structure, as a service over what the CLI does.
- Applied by the PM: the tool takes the command path and arguments as a list, never a shell string. The CLI's grammar parses them and the same command functions run. The tool returns the same envelope the CLI prints.
- The tool's input and output schemas come from the same types as the CLI grammar, so the two cannot drift.
- A case may send its request through the CLI or the MCP tool, so the suite proves both agree.

Amended 2026-10-25 by Ian: the server also offers `search` and `fetch` for ChatGPT, whose deep research uses only those two tool names and which refuses servers without them outside developer mode. Both are thin wrappers that run the same CLI commands as the main tool, and they stay consistent with it.
