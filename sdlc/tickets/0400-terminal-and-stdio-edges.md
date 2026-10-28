# 0400 — Close terminal, proxy and stdio edge cases

Status: COMPLETE.
Landed: 5a62251d9109eaaa985f1d5e820f6aef94e54b7c

Milestone: 5

## Outcome

Markdown output replaces control and bidirectional override characters from upstream text with visible escapes. BioMCP also reads lower-case `http_proxy`, `https_proxy` and `no_proxy`, preferring lower case when both are set, and help names both. `help network proxy` says publisher PDF downloads are refused when a proxy applies. `--format` comes only from the parsed grammar, and a dispatch failure prints its own message instead of "output failed". An oversized MCP stdio frame is refused and skipped, and the session keeps serving.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, code findings 4, 5, 6, 8 and 9.
- Starts from: `crates/biomcp/src/view/escape.rs`, `config.rs`, `help.rs`, `output.rs`, `main.rs`, `exit.rs`, `mcp.rs` and `framing.rs`.
- Rules: JSON output is unchanged; exit codes keep their contract; no new framework.
- Proof: cases fail before the fix for an ESC byte in Markdown output, a lower-case-only proxy setting, a dispatch error message and an oversized stdio frame followed by a valid request.
- Changes: the files above and their cases.
- Defers: nothing.
