# 0343 — Treat a closed output pipe as success and drop empty Markdown query lines

Status: OPEN.

Milestone: 5

## Outcome

When the reader closes stdout early, as in `biomcp help | head`, BioMCP stops writing and exits 0 without printing `biomcp: output failed`. Markdown output leaves out the query line when the query is null, so `key list --format md` no longer starts with `**query:** null`.

- Why: the docs lead's product findings report on main a01667d (2026-10-27), rechecked by the PM on the release candidate build from hosted run 40542150253. Piping into `head` exits 5 with `biomcp: output failed`, and agents pipe output this way often.
- Starts from: the binary's output writer and exit mapping, and the Markdown renderer's query section.
- Rules: other write failures still exit 5. PM ruling 2026-10-27 (m10137): this covers ordinary CLI output only; the MCP stdio server keeps exiting 5 on a broken stdout write.
- Proof: a CLI case pipes `biomcp help` into a reader that closes after ten bytes and sees exit 0 and empty stderr; `key list --format md` starts with its result.
- Changes: the output writer, the Markdown renderer and cases.
- Defers: nothing.
