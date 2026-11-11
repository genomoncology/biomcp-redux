# 0838 — choose-fields-in-a-reply

Status: OPEN.

Milestone: 7

## Outcome

`--fields a,b,c` (comma-separated, repeatable) on any entity search, and a top-level `fields` list beside `action` on each typed entity tool, returns only those top-level keys of each search row, always keeping the row schema's required fields and the whole envelope (so identifiers and source attribution stay). An unknown or dotted name is a usage error listing the row's valid names; paging and sort steps carry the option; `help search <entity>` gains one "Fields:" line; the typed property is bare (no description). Trial rows' `conditions`, `interventions` and `url` become optional (trial-page/15) so trials can narrow. The research `search` and `fetch` tools refuse `fields`. Gets are not covered: `section` narrows them, extended by 0846.

Design review 2026-11-11 (PM accepted): the original "any search or get" and "narrowing only removes optional fields" did not hold. A get's top level is the envelope, and trial rows' required fields are their bulkiest (narrowing a live 3-row trial search kept 95% of its bytes until those three became optional, then 67%). The opt-in option mainly helps clients with no shell, such as Amazon Quick; a smaller default envelope is ticket 0849.

## Why

Agents trim replies with `jq` or curl today; the baseline's default replies are larger than a trimmed fetch, which is one of the two ways a tool earns its place over scripts.

## Proof

- Proof: narrowed 10-row searches of trial, article and drug validate and are under half the default's bytes; an unknown name is a usage error listing the valid ones; the typed tool list grows by under 1%; the Markdown view-empty cases still pass.

## Starts from

- Starts from: reply rendering in `crates/biomcp-types/src/response/`, the global options in `crates/biomcp/src/grammar.rs`.
- PM decision: 2026-11-11. Design review done 2026-11-11 (see Outcome). Build after 0837, which also touches `mcp/`. Moved up the same day by `sdlc/records/2026-11-11-token-gap-root-cause.md`: smaller replies are the only change that takes the replay below curl; field names must show in help and tool descriptions so an agent can ask on the first call.

## Keeps

- Keeps: default replies are unchanged; schemas are unchanged except trial-page/15; narrowing keeps every required field. Projection runs once on the finished JSON reply after the cache, before Markdown and the MCP reply form.

## Defers

- Defers: fields on gets, after 0846 is measured; dotted paths; CSV export builds on this in 1.1.

## Changes

- Changes: `crates/biomcp-types/src/response/`, `crates/biomcp/src/grammar.rs`, `crates/biomcp/typed_front_map.rs`, the typed parser, trial-page schema, `tests/cases/`
