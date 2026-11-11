# 0837 — mcp-replies-are-sent-once

Status: OPEN.

Milestone: 7

## Outcome

By default an MCP tool reply carries its JSON once, not as text and again as structured content, and the reply names no server file path in its next steps. A typed tool reply is a plain JSON object: no status line before it and no second copy of `next` (added 2026-11-11 from `sdlc/records/2026-11-11-token-gap-root-cause.md`: typed turns add 3,420 tokens against curl's 2,600, about 16% of reply bytes are envelope, and only 16% of typed turns pick fields because the reply arrives as one string).

## Why

`mcp_reply_form` defaults to `both`, so every MCP reply costs its tokens twice in clients that read both; a live hosted reply's next step also carried `--config /etc/biomcp/config.json`.

## Proof

- Proof: an MCP case asserts the default reply holds one copy and that no `next` step contains a filesystem path; a measured reply-size table for ten recorded calls goes in the ticket, before and after.

## Starts from

- Starts from: `mcp_reply_form` in `crates/biomcp/src/config/mcp.rs`, reply building in `crates/biomcp/src/mcp/protocol.rs`, next-step rendering in `crates/biomcp/src/command/steps.rs`.
- PM decision: 2026-11-11. Which single form is the default is decided by a short test on the clients hosting serves (Claude, ChatGPT, Amazon Quick, Claude Code): the form every one of them shows the model. Put the result here before building.

## Client test (2026-11-11, developer)

A stub MCP server sent a reply whose text and `structuredContent` held different markers, and each client's model-facing tool result was read.

| Client | Success reply | Error reply (`isError`) | Evidence |
| --- | --- | --- | --- |
| Claude Code 2.1.322 | shows `structuredContent` only, drops text; with `content: []` it still shows `structuredContent` | shows text only, drops `structuredContent`; with `content: []` it shows "Unknown error" | local `claude -p` runs against the stub, with and without an output schema |
| pi (bench harness) | shows text when any text block exists, `structuredContent` only when `content` is empty; code-mode scripts get `structuredContent` | same | pi-mcp README and `extensions/mcp/tools.js`, `codemode/execute.js` |
| ChatGPT | shows both `content` and `structuredContent` | not documented | OpenAI Apps SDK reference: both "surfaced to the model" |
| Claude API MCP connector | documents only `content` text blocks in `mcp_tool_result` | same | platform.claude.com MCP connector page |
| Claude (claude.ai) and Amazon Quick | not verified | not verified | open: hosting's live check and Amazon's answer |

The MCP spec (2025-11-25 tools) says a tool with an output schema MUST return structured results, and SHOULD also send the JSON as text; the official Python SDK client raises when a tool with an output schema returns no `structuredContent` on success, and skips that check on errors.

So no single form is shown by every client: `both` is double in ChatGPT and single in Claude Code. The one-copy form every verified client shows is `once`: success sends `content: []` plus `structuredContent`; an error sends the JSON as text only. It meets the spec's MUST and the SDK check, and drops only the SHOULD. Local servers default to `once`. Hosted servers keep `both`, as 0587 already ruled, until claude.ai and Amazon Quick are checked; hosting switches by setting `"mcp_reply_form": "once"`.

## Keeps

- Keeps: `both` stays available as a setting for clients that need it; the output schema declaration.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `crates/biomcp/src/config/mcp.rs`, `crates/biomcp/src/command/steps.rs`
