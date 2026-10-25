# 0194 — Investigate the study slice and propose its design

Status: OPEN.

Milestone: 4

## Outcome

`sdlc/planning/slices/study.md` holds the study slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 4 delivers the remaining entities; clinicians and researchers ask which public cancer genomics cohorts profile a tumor type and how often a gene is altered in them (get only; analysis and charts are out of scope).
- Starts from: `planning/slices.md` with its quality-floor, capture-before-types and public-issue rules; the landed slice designs, especially those this entity joins; the shared biodata model; the study sources in ROADMAP.md; 0.9's study commands; decisions on text heuristics, data bundles, keyed sources, open-by-default terms and the amended medical-question scope.
- Examples: 20 to 40 real clinical questions from labels, papers and public issues, covering studies for a cancer type, a study's sample counts and profiled data types, a gene's alteration frequency in one study, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask cBioPortal studies (bundle) directly for each example and record counts, speed, requests, identifiers, the fields each public tier carries, bundle sizes and update cadence where relevant, and terms, with use labels and groups. Keys are inherited by name only and never printed.
- Identity: record study, cancer type (OncoTree) and gene identifiers and how they join to disease, gene and variant, reusing CohortObservation with stated profiled and cohort sample counts and no computed denominator.
- 0.9: measure 0.9's commands only behind verified network containment, otherwise mark them unmeasured; account for every command, option and section, and every closed 0.9 public issue touching this entity.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps under architecture 6.10, disclosure layers and measured token budget, sources and bundles kept and dropped, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
