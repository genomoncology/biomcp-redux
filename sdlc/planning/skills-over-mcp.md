# Skills over MCP, Tasks and Elicitation

Decided 2026-11-27, after Claude added Skills over MCP in public beta and made Tasks and Elicitation generally available. The facts and sources are in [the MCP skills record](../records/2026-11-04-mcp-skills-and-design-research.md#update-2026-11-27). Ticket status lives in pm.

## What BioMCP ships today

- `skills/SKILL.md`, a 4.6 KB command guide in CLI syntax, and `skills/use-cases/`, 18 worked examples that each give the CLI command and the typed call. The binary embeds both. `biomcp skill render|list|install|status` reads or installs them, and the Claude Code plugin ships the folder.
- The MCP server declares only `tools`. Its always-on guide is the server instructions: the generated front map, under Claude Code's 4,096-character cut. The `help` tool returns one command's options, sections and sources. The hosted service runs the same server with `--hosted`, behind a wrapper that passes MCP bodies through unparsed.

## What goes where

Each channel has one job, so nothing is said twice.

| Channel | When the model sees it | What it holds |
|---|---|---|
| Server instructions | Every session, always | The map: tools, call shape, the few rules every call needs, and one line naming the skill URI. |
| Tool descriptions | When the tool loads | What the tool does and its arguments. |
| `help` tool | On request | One command's options, sections, sources and examples. |
| MCP skill `biomcp-research` | Its name and description always; the body only when the model or user loads it | How to work a multi-step question: which calls to make in what order, and what to check before answering. Not the map. |

The MCP skill is the 18 use cases plus a short `SKILL.md` that routes a question type to its use case. It does not copy the command guide, because the instructions already carry the map and our runs showed a re-read guide costs tokens. It holds no scripts, so no host asks for approval to run code, and it stays far under the scan limits (about 25 KB against 400 KB). Its description says when to use it, for example "Plan a multi-step biomedical evidence question with BioMCP: treatment, variant, trial screen, drug label, pathway or disease review." The current `description: Read biomedical records.` is too thin for a model to choose it.

`skills/SKILL.md` stays as the CLI guide for coding agents that call the binary, through the plugin or `skill install`. It is a different skill with a different name, so a plugin user who also gets the MCP skill sees two skills with two jobs, not a duplicate.

## Local and hosted

- **Local `biomcp serve` ships it** over stdio and HTTP, on by default. A setting turns it off (default on), so an operator can withdraw it without a release.
- **The hosted service ships it** through the same server code. In hosted mode a use case whose calls need a source the hosted service does not offer is left out of the skill, the same way the hosted instructions leave those sources out. The wrapper needs no code change because it passes `skills/list`, `skills/get` and `resources/read` through; the hosting team repins core and adds the skills check to its deployed checks.
- Files are served byte for byte from the embedded folder, so digests are stable for a build and change only when the text changes.
- The skill is declared wherever the negotiated revision defines the `extensions` capability: in `server/discover` under 2026-07-28, and in `initialize` under any older revision whose schema has the field. The skill files are also listed by `resources/list` on every revision that has resources, so a client without the extension can still read them by URI from the instructions pointer, the route the spec guarantees everywhere.

## How it is tested

1. **Outside-in cases in core:** over stdio and HTTP, a client sees the extension in discovery, lists one skill with a complete manifest, gets it by URI, reads every file and finds each digest, size and frontmatter matching the listing; an unknown URI returns `-32602`. A hosted-mode case shows a use case that needs an absent source is left out. These run in the normal landing gate.
2. **Interop in the landing gate:** the pinned MCP Inspector CLI runs `--method skills/list --verify` against `biomcp serve`, and the pinned TypeScript SDK's skills client lists and reads the skill. This catches the schema slips that only real clients find.
3. **Hosted:** after the repin, the hosting team's deployed check runs the same Inspector verify against the live service.
4. **By hand, once per release:** in an interactive Claude Code session (v2.1.331 or later) and in claude.ai with the connector, the skill shows as `biomcp:biomcp-research` (or the connector's prefix) and loads. This goes on the release's manual checks because `claude -p` cannot load MCP skills.
5. **Use, not just delivery:** whether models actually load it is measured on the bench when a scripted client can load MCP skills, as arm E of the record's test plan. It does not gate the build. If it is never loaded and adds no accuracy, the instructions pointer stays and nothing else is added.

## Tasks: not now

BioMCP's calls are bounded at 20 to 60 seconds and return one JSON reply. A task helps only when work runs for minutes, and a claude.ai connector must still answer within its per-request timer, so a task is just a handle for work that continues afterward. The hosted service runs two stateless containers, so a task would need a shared, durable job store that both can read. That cost buys nothing for lookups that finish in a minute. Revisit if a call must run for minutes, such as a bulk export or a whole-corpus text read.

## Elicitation: one narrow use

BioMCP is a read-only lookup server, and the agent is better placed than the server to ask the user what they meant, so elicitation adds nothing to research calls. The one exception is saved work: `saved delete` with `all` removes every saved search, list and note at once, which is exactly the "confirm before destroying data" case elicitation exists for. When the client declares form elicitation, that call asks the user to confirm before deleting. Clients without it keep today's behavior. Nothing else uses elicitation.

## Order

1. [MCP serves the research skill](../tickets/1199-mcp-serves-research-skill.md): the core server and its cases.
2. [Skill interop check in the landing gate](../tickets/1200-mcp-skill-interop-verify.md), after the interop stage's pinned Node.
3. Hosting repin and a deployed skills check, in the hosting repo.
4. [Saved delete-all asks to confirm](../tickets/1201-saved-delete-all-asks-to-confirm.md), independent of the others.
