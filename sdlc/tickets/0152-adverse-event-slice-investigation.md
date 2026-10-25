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
- Defers: tickets, which the PM files from the reviewed design.
