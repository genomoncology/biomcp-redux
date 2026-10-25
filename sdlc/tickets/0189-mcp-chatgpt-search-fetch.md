# 0189 — Add ChatGPT-compatible search and fetch MCP tools over the CLI

Status: OPEN.

Milestone: 4

## Outcome

The MCP server lists `search` and `fetch` beside the main `biomcp` tool. `search(query)` runs the CLI's own `search` command and returns its rows as results, each with an `id`, `title` and `url`. `fetch(id)` runs the CLI's own `get` command for that `id` and returns the record's `id`, `title`, Markdown text, `url` and metadata. Both tools reach their answers only through the same command functions as the main tool, so their output cannot drift from the CLI.

- Why: Ian amended the one-tool decision on 2026-10-25. ChatGPT deep research uses only tools named `search` and `fetch`, and ChatGPT refuses servers without both outside developer mode. 0.9 shipped these tools for ChatGPT users (public issues 21 and 37).
- Starts from: the amended decision `decisions/2026-10-23-mcp-is-one-tool-over-the-cli.md`; the MCP tool and protocol code; the Markdown view; 0.9's search and fetch tools; OpenAI's current published MCP and deep research tool contract, read by the developer and cited in architecture.md before code.
- Rules: if the query's first word names a BioMCP entity (such as `trial`, `variant` or `gene`), `search` runs `search ENTITY REST`; otherwise it runs `search article QUERY`. Each result's `id` is the exact `get` argument list for that row, such as `article 17611497`, and `fetch` accepts only such an id. Anything else is the same usage error the CLI gives. The `url` is the record's source URL and is never empty. The default page size and ceilings are the CLI's. No new search logic, ranking, synonym expansion or entity guessing beyond the first-word rule. Source terms, problems and outcomes appear in the metadata as the CLI reports them. The tool schemas follow OpenAI's published shapes and are checked by the existing schema compatibility check.
- Proof: outside-in cases first, replaying existing captures: `search` with an article query, with `trial ...` and with `variant ...`; `fetch` of an article, a trial and a variant id taken from those results; a bad id; and `tools/list` showing the three tools within a measured description ceiling. Each case asserts that the wrapper's output matches the CLI output for the same command. No provider call. Then make landing-gate runs through pm job.
