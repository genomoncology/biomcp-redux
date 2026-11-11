# 0833 — typed-tools-callable-from-their-descriptions

Status: OPEN.

Milestone: 7

## Outcome

An agent that sees only a typed tool's name, description and input schema, with no skill and no help call, makes a correct first `search` and a correct first `get` on each of the 13 entity tools. A flat call such as `{"action":"search","query":...}` is accepted as well as the nested form, and a short tool name is never the reason a call fails on BioMCP's side.

## Why

Hosting serves the typed tools, and Amazon Quick, live in late November, finds tools by searching their descriptions and loads no skill. In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) Sonnet had 105 typed calls refused by the client for arguments that did not fit the schema and 23 for the tool's name, before BioMCP could answer; the skill's single typed example is itself refused.

## Proof

- Proof: a routine MCP case per entity tool calls it with the flat form and with the nested form and gets the same reply; a check asserts every entity tool's description contains one runnable example for search and one for get, and that each example passes that tool's own schema; the bench's typed mode rerun with no skill shows client refusals under 10 a repeat on Sonnet.

## Starts from

- Starts from: `crates/biomcp/src/mcp/typed/` (`description.rs`, `grouping.rs`, `catalog`), `crates/biomcp/typed_front_map.rs`, `skills/SKILL.md` section 'MCP typed tools', the tools-list ceilings in `crates/biomcp/src/config/mcp.rs`, and `sdlc/decisions/2026-11-07-entity-tools-in-1-0.md`.
- PM decision: 2026-11-11. Design review before hand-over: this changes a public input schema. The reviewer weighs accepting both forms in the schema against flattening it, and what each costs in tools-list size.
- PM decision: 2026-11-11. Descriptions say what a person would ask for, in plain words, before any abbreviation of sources, because a host that searches descriptions matches on those words.

## Keeps

- Keeps: the 17 tool names, the actions, and every existing nested call; the tools-list byte ceiling; no tool profile or new tool.

## Defers

- Defers: whether descriptions should be longer and written in users' words for search-based hosts waits for Amazon's answers on how Quick ranks tools; record them in this ticket when they arrive.

## Changes

- Changes: `crates/biomcp/src/mcp/typed/`, `skills/SKILL.md`, `tests/cases/mcp/`
