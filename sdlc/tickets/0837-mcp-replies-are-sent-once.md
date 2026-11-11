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

## Keeps

- Keeps: `both` stays available as a setting for clients that need it; the output schema declaration.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `crates/biomcp/src/config/mcp.rs`, `crates/biomcp/src/command/steps.rs`
