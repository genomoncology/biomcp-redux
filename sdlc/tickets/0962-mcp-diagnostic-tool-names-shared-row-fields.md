# 0962 — mcp-diagnostic-tool-names-shared-row-fields

Status: OPEN.

Milestone: 7

## Outcome

An agent using BioMCP over MCP learns what 0959 tells CLI agents: FDA CDx rows of both tables carry diagnostic, indication, device, drug and row_date beside FDA's own cells. The typed `search_diagnostic` tool description or another MCP-visible place says it, within the per-tool description budget.

## Why

0959 (2026-11-14): putting the sentence in the command summary pushed the typed `search_diagnostic` description to 2,500 characters against its 2,400 budget (j41040), so it went into CLI help only, and MCP agents still see neither the shared fields nor the two row shapes that broke three dx-lab-director scripts.

## Proof

- Proof: the MCP tool listing case for `search_diagnostic` shows the sentence (or a pointer to where it lives) and stays inside the 2,400 budget; or a measured case shows MCP agents reach it through `biomcp_help`.

## Starts from

- Starts from: 0959's ticket and `crates/biomcp/src/help.rs`; the typed tool description source for `search_diagnostic`; 0905's per-tool budget.

## Keeps

- Keeps: the 2,400 per-tool budget (shorten the description's own words rather than raising the budget); the CLI help wording.

## Defers

- Defers: other tools' descriptions.

## Changes

- Changes: the `search_diagnostic` description or MCP help routing, its case, regenerated docs.

## Progress

- 2026-11-15, design read: over MCP the 13 entity tools replace per-command tools. The 2,400 budget (`ACTION_BUDGET`, `crates/biomcp/tests/runner/typed_schemas.rs`) and the 300-character cap apply to each action's own description and schema; the `diagnostic` tool's own description (what it is for, sources, examples, what it is not for) sits outside both, under the 128 KiB catalog cap only. So the sentence goes there, and the `search` action's wording and size stay as they are.
- Wording: 0959's order-neutral sentence ("carry ... beside FDA's cells"), not 0966's "start with". MCP replies are `serde_json::Value` maps without `preserve_order`, so they list fields alphabetically and `cells` comes before `device` there; "start with" would be false over MCP. The sentence is its own constant in `crates/biomcp/src/mcp/typed/description.rs`, so 0966's help wording change does not reach it, and it is added only while `fda-cdx` is offered.
- Proof: `mcp-parity/typed-description-contract` now checks that the local and hosted `diagnostic` tool descriptions end with the sentence, and that a server started with `--disable-sources fda-cdx` leaves it out.
