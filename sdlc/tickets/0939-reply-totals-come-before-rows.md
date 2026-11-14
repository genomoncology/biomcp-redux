# 0939 — reply-totals-come-before-rows

Status: OPEN.

Milestone: 7

## Outcome

Every paged reply BioMCP writes as JSON states its total (and offset and truncated flag) before its rows, so an agent that reads only the start of a reply sees how much there is. This covers the replies 0937 left: drug search pages (`DrugPage`, and the FDA, WHO and recall pages), adverse-event record pages, and the envelope's top-level `page`, which today follows `result`.

## Why

0937's review (2026-11-14): 0937 moved the total ahead of the rows for drug sections, CPIC and ClinPGx, because the dx-lab-director persona cut DDInter's reply at 1,200 bytes and reported the first 5 of 614 interactions as the whole (bench `personas/dx-lab-director/runs/2026-11-14/analysis.md`). The same cut on the replies listed above still hides the total. This is the paper's W2: the first page read as the whole.

## Proof

- Proof: a contract test per reply family asserts the total's byte offset comes before the first row's; a recorded case for a drug search page and an adverse-event search shows the total within the first 1,200 bytes.

## Starts from

- Starts from: 0937's change and tests (`crates/biomcp-types/tests/response.rs`, `pgx_contracts.rs`); `crates/biomcp-types/src/drug/` page types (`fda.rs:188`, `who.rs:47`, `recall.rs:73` per the review); the envelope type that writes `result` then `page`.

## Keeps

- Keeps: the 1.0.0 schema rules (field order only; generated schemas list properties alphabetically, so no schema file should change); MCP text output, which is alphabetical.

## Defers

- Defers: nothing.

## Changes

- Changes: field order in the page and envelope types, contract tests.
