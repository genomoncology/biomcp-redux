# 0154 — Investigate the diagnostic slice and propose its design

Status: COMPLETE.
Landed: a5805fea2005425e2678ba5fa2fed2b0a3205704

Milestone: 4

## Outcome

`sdlc/planning/slices/diagnostic.md` holds the diagnostic slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 4 delivers the remaining entities, and clinicians ask which tests detect a biomarker, such as approved companion diagnostics for a targeted therapy.
- Starts from: `planning/slices.md` with its type-before-capture rule; the drug, variant, gene and disease designs; the diagnostic sources in ROADMAP.md; 0.9's diagnostic commands; decisions on text heuristics, data bundles, keyed sources and open-by-default terms.
- Examples: 20 to 40 real clinical diagnostic questions from labels, papers and public issues, covering a companion diagnostic for a drug, tests for a gene or biomarker, a device approval, a laboratory test listing, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask NCBI GTR, WHO in vitro diagnostics and openFDA devices directly for each example and record counts, speed, requests, identifiers, the fields each public tier carries, bundle sizes and update cadence, and terms, with use labels and groups.
- Identity: record how each source names tests, genes, biomarkers and drugs, and how they join to other slices without name matching.
- 0.9: measure 0.9's commands only behind verified network containment, otherwise mark them unmeasured; account for every command, option and section.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps, disclosure layers and measured token budget, sources and bundles kept and dropped, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- PM review 2026-10-25 of c1e88722, returned with fixes that use the evidence already captured. PM rulings: approve a separate `fda-cdx` bundle built from FDA's companion diagnostic tables, because 13 of 24 questions need device, drug and indication rows that no structured source carries; the sync checks the exact header set of both tables and fails whole on any change, cells stay whole text, and IDs come only from their own cells or links; this differs from SEER's no-HTML ruling because these are fixed-column regulatory tables, not narrative pages. The conflicting GTR accession keeps both rows, returns an identity-conflict problem and no card, shows once in search with a conflict status and no get step, and the sync receipt counts duplicate identities. 510(k) stays deferred, with one exact-ID request for a K number from the companion table approved in the adapter ticket; until then K and DEN numbers show without a get step. WHO's use label comes from WHO's terms page, or stays `noncommercial` with the reason stated. Selectors are `--gene-id` (exclusive with `--gene`), `--gene`, `--condition-id` for MedGen once a positive exists, and `--type`. Fixes:
  1. Add header, card and page field lists with JSON shapes for each record kind, from the captured columns.
  2. Add per-command next steps under architecture 6.10, with gene, drug and PMA pivots by exact ID.
  3. End each contingent selector as proven or deferred, add `--gene-id` to the command table, case map and accounting, and name the field `--gene` matches.
  4. Show from the captured FDA tables that NDA or BLA and device numbers sit in their own cells or links and state their mapping to the drug slice's spelling; otherwise the selector is the whole cell.
  5. Defer or drop `search --source fda-pma --query`, GTR `--laboratory` and WHO `--manufacturer` and `--type`.
  6. WHO gets a card only. Define PMA get with and without `--supplement`, and its supplements section.
  7. Companion and GTR relations are not Statements; reuse `Concept`, `Curie`, `GeneRef` and `NcbiGeneId`; keep all WHO rows unfiltered.
  8. Propose default page sizes and provisional ceilings from measured source projections, with product and 0.9 figures marked unmeasured and the contained-run ticket named.
  9. Align source names with the registry. "Ian's questions" says None.
- PM recheck 2026-10-25 of 2601694b: accepted with these amendments, applied on the branch before it lands:
  1. Replace `get diagnostic P150013 --source fda-cdx [--supplement S020]` with `search diagnostic --source fda-cdx --device-id P150013[/S020]`, which returns an association page, because get returns one card.
  2. `--drug-application` selects rows whose own drug cell link carries that exact application; rows without an owned typed ID are reached by device search. `--drug-cell` is dropped because no question needs it. `--biomarker` asserts the captured P110027/S012 table-0 cell, or it is deferred.
  3. The next-step table says sync and fresh, not help.
  4. The accounting adds `fda-cdx` and `fda-pma` as approved.
  5. With no WHO search, Q23 is answered only for a known WHO product ID; the design says so.
- Defers: tickets, which the PM files from the reviewed design.
