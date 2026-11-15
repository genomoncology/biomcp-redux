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
