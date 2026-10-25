# 0152 — Investigate the adverse event slice and propose its design

Status: OPEN.

Milestone: 3

## Outcome

`sdlc/planning/slices/adverse-event.md` holds the adverse event slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 3 delivers the drug, disease and adverse event slices, and safety questions are part of what it serves.
- Starts from: `planning/slices.md` with its type-before-capture rule; `planning/slices/drug.md` and `disease.md` as worked examples; the adverse event sources in ROADMAP.md; 0.9's adverse event commands; decisions `2026-10-24-no-text-heuristics.md`, `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` and `2026-10-24-sources-are-open-by-default.md`.
- Examples: 20 to 40 real clinical safety questions from papers, labels and public issues, covering reported reactions for a cancer drug, a serious outcome such as death or hospitalization, a drug and reaction pair, a date range, a device report, a vaccine report, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask openFDA FAERS and device reports, VAERS and CVX directly for each example and record counts, speed, requests, returned identifiers, the fields each public tier carries, and terms. Report counts are never presented as incidence or causation.
- Identity: record how a report names its drug and reaction, how that joins to the drug slice's identities without matching on names, and how ambiguous drug names return candidates.
- 0.9: measure 0.9's adverse event commands on the same examples only behind verified network containment, as 0136 did; otherwise mark the comparison unmeasured. Account for every command, option and section.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps, disclosure layers and measured token budget, sources kept and dropped with use labels and groups, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- PM review 2026-10-25 of 21ad15d2, returned with fixes that use the evidence already captured:
  1. Search and get only: remove `count`, `summary`, `sync cvx` and CVX search and get. Counts become `search adverse-event TEXT --count reaction` only if one harvested most-reported-reactions question is added from the F19 capture; the seriousness count is dropped because F13's total answers it.
  2. FAERS is the default source. Input is positional, with no `--query`. The table shows the exact query text built and its echo. Typed filters `--reaction` (exact MedDRA preferred term), `--serious death|hospitalization` and `--received-from` and `--received-to` (first FDA receipt) each tie to a captured positive row; suspect-only, age, sex and reporter filters are dropped. For MAUDE, positional text searches the device generic name.
  3. VAERS and CVX end as dropped from this slice, with retirement messages and reasons. Whether to accept losing 0.9's vaccine search goes to Ian; until he rules, the design records the loss.
  4. Tie every case to its captured facts and date; D03 is the real empty case; A24 is labeled synthetic; A04's fatal filter, A15 and A21 to A22 are marked unanswered or removed.
  5. Source and report links are card fields. Next steps follow architecture 6.10.
  6. Add JSON shapes and field lists for the report header, both cards and each section page, and a case map per command and option.
  7. Prove the `reporter` section's fields from the field inventory or drop it; mark multi-version get unproven.
  8. Count rows reuse the shared `CohortObservation` or state why not; reaction terms are `Concept` without a CURIE; identifiers keep published forms; record openFDA's third-party marks for MedDRA reaction terms and GMDN device names, with use labels.
  9. Every budget row is marked for measurement by its adapter ticket, with a ceiling at measured size plus margin.
- Defers: tickets, which the PM files from the reviewed design.
