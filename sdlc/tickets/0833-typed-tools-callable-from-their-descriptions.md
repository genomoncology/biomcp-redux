# 0833 — typed-tools-callable-from-their-descriptions

Status: OPEN.

Milestone: 7

## Outcome

An agent that sees only a typed tool's name, description and input schema, with no skill and no help call, makes a correct first `search` and a correct first `get` on each of the 13 entity tools. Every search accepts `query`. Every get accepts the record ID as `input` or `id`, as one string or a list. `section` accepts a string or a list, and `sections` is accepted. Each entity tool's description says what a person would ask for, names its sources in full, gives one search example and one get example that pass its own schema, lists the other actions, and says what it is not for.

## Why

Hosting serves the typed tools, and Amazon Quick, live in late November, finds tools by searching their descriptions and loads no skill. In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, current main) Sonnet had 105 typed calls refused by the client before BioMCP could answer. Every refused call read used the nested form correctly; the refusals are argument names and types that differ from tool to tool. The one example agents are given is valid for `gene` only and teaches a list that `drug` and `variant` refuse.

## Proof

- Proof: one MCP case per entity tool sends the guessed forms below and gets the canonical call's reply; each description's two examples run as MCP cases through that tool's schema; the bench's typed mode with no skill shows under 10 client argument refusals a repeat on Sonnet.

## Starts from

- Starts from: `crates/biomcp/src/mcp/typed/description.rs` (the template ends `Call {"action":"get","get":{...}}` and names no arguments), `grouping.rs`, the catalog and front map, `crates/biomcp/typed_front_map.rs` (the one example), `skills/SKILL.md` section "MCP typed tools", the ceilings in `crates/biomcp/src/config/mcp.rs`, and `sdlc/decisions/2026-11-07-entity-tools-in-1-0.md`. The call logs are `baseline-1.0-sonnet/r*/*/biomcp-mcp-tools/calls.md` in the bench's data folder on the build machine.
- What the schema refuses today, from those logs (counts approximate):

| Refused | About | Why |
| --- | --- | --- |
| `search.query` unknown | 48, most on `article` | search text is `query` on 7 tools, `text` on 5, `name` on diagnostic |
| `get.sections` unknown | 25, most on `drug` | the key is `section` |
| `get.input` as a list | 21, all `drug` | a list on gene, trial, article, protein; one string on variant, drug |
| `search.keyword` or `keywords` unknown | 22 | article has neither |
| unknown action | 17 | `approvals` and `label` are sections of `drug` get |
| `get.input` missing | 9 | the ID is `input` on 6 tools and `id` on 7 |

- The tools list is 83,563 bytes of the 131,072 ceiling; descriptions of about 700 bytes each add about 6 KB.
- PM decision: 2026-11-11, after design review. Keep the nested shape and make names uniform. Accepting a flat form was dropped: no observed call used it, and it would double the schema. The advertised schema itself must accept the guessed forms, because clients check arguments before BioMCP sees them.
- PM decision: 2026-11-11. Descriptions follow this order: what a person would ask for with common synonyms; sources by full name; one search and one get example, with any required argument; other actions in a few words; what it is not for, naming the right tool. The description says where `approvals` and `label` live.

## Keeps

- Keeps: the 17 tool names, the actions, every existing call, per-action required arguments and the refusal of unknown keys; the tools-list ceilings; no new tool.

## Defers

- Defers: one tool per entity and action (`search_trial` style) is Ian's decision after Amazon answers how Quick ranks tools. The `mcp__biomcp__` name prefix belongs to the bench's client.

## Changes

- Changes: `crates/biomcp/src/mcp/typed/`, `crates/biomcp/typed_front_map.rs`, `skills/SKILL.md`, `tests/cases/mcp/`
