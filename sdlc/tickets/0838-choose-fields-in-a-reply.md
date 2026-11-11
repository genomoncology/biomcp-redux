# 0838 — choose-fields-in-a-reply

Status: OPEN.

Milestone: 7

## Outcome

`--fields a,b,c` on any search or get returns only those fields of each row or card, with identifiers and source attribution always kept; the same option works on every MCP tool.

## Why

Agents trim replies with `jq` or curl today; the baseline's default replies are larger than a trimmed fetch, which is one of the two ways a tool earns its place over scripts.

## Proof

- Proof: routine cases on three entities show a narrowed reply that validates and is under half the default's bytes; an unknown field name is a usage error listing the valid ones.

## Starts from

- Starts from: reply rendering in `crates/biomcp-types/src/response/`, the global options in `crates/biomcp/src/grammar.rs`.
- PM decision: 2026-11-11. Design review before hand-over: a new global option on every command. Moved up the same day by `sdlc/records/2026-11-11-token-gap-root-cause.md`: smaller replies are the only change that takes the replay below curl; field names must show in help and tool descriptions so an agent can ask on the first call.

## Keeps

- Keeps: default replies are unchanged; schemas are unchanged, because narrowing only removes optional fields.

## Defers

- Defers: CSV export builds on this in 1.1.

## Changes

- Changes: `crates/biomcp-types/src/response/`, `crates/biomcp/src/grammar.rs`, `tests/cases/view-empty/`
