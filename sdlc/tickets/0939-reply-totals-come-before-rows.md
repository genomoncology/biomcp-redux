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

## Progress

- Measured before on origin/main 7ee90318a (live job j40566, replays j40607) and after on the branch (live job j40571 on 8f153a744); reply sizes are unchanged. Byte offset of the total, before → after:
  - `search drug imatinib --limit 10` (mychem `DrugPage`): `result.total` 2,143 → 183, envelope `page.total` 2,188 → 161; first row was at 139.
  - `search drug imatinib --source openfda-label --limit 10` (`DrugPage`): `page.total` 2,778 → 251; recorded replay the same, 2,787 → 251.
  - `search drug imatinib --source drugsfda --limit 10` (`DrugFdaPage`, 11,369 bytes): `result.total` 9,152 → 272, `page.total` 9,227 → 250.
  - `search drug metformin --source fda-drug-recall --limit 10` (`DrugRecallPage`): `result.total` 4,397 → 313, `page.total` 4,442 → 261.
  - `search drug artemether --source who-pq --limit 10` (`DrugWhoPage`): `result.total` 3,748 → 189, `page.total` 3,793 → 167.
  - `get drug imatinib civic --source civic` (`DrugCivicPage`): evidence `page` 1,659 → 559 (rows start at 698), assertions `page` 3,723 → 1,810.
  - `get drug clopidogrel interactions` (DDInter, 0937's case): envelope `page.total` 5,188 → 176.
  - `search adverse-event pembrolizumab --source faers --limit 10`: `page.total` 2,298 → 256; first row now at 1,113. Recorded synthetic replay (`tests/cases/adverse-event/examples.yaml`, 5 rows): 1,078 → about 260, where the first row started at 459.
  - `get adverse-event 26940567 drugs` (`AdverseEventRecords`): `drugs.total` 1,754 → 401; `reactions` 560 → 413.
- Changed field order only (no field added, removed or renamed; schemas list properties sorted, so no schema file changed): `Envelope` and `DrugSectionReply` write `page` before `result`; `DrugPage`, `DrugFdaPage`, `DrugWhoPage` and `DrugRecallPage` write `total` before `items`; `DrugCivicPage` writes `page` before `records`; `AdverseEventRecords` and `AdverseEventIdentifierRecords` write `offset` and `total` before `records`. MCP text is serialized through a sorted JSON value, so it stays alphabetical. pgx types untouched (0936).
- Tests: `crates/biomcp-types/tests/totals_first.rs` decodes synthetic wire JSON for each family and asserts each total's byte offset precedes the rows. The case runner gains a step-level `starts_within` map (JSON pointer → byte count, raw output; `crates/biomcp/tests/runner/assertions/offsets.rs`, documented in the runner README, with a proof trial `schema-discovery/starts-within-reads-raw-bytes`). `tests/cases/drug/openfda-label-page.yaml` (recorded public replies) asserts `/page/total` and `/result/total` start within 1,200 bytes (2,787 on main). The recorded adverse-event searches are all limit 1 or synthetic and already showed the total before byte 1,200 on main, so `tests/cases/adverse-event/examples.yaml` asserts `/page/total` within 450 bytes, before the first row's old start (459); it fails on main at 1,078. The live limit-10 FAERS search above shows the 1,200-byte cut (2,298 → 256).
- Not changed here: other families still write their rows before their `page`, outside this ticket's list: variant hotspot and OncoKB records (`response/variant_hotspots.rs`, `variant_oncokb.rs`), study pages (`study/response.rs`), gwas association and study pages (`gwas/response.rs`) and the shared `page!` macro rows (`response_page.rs`). The envelope `page` now leads in all of them.
