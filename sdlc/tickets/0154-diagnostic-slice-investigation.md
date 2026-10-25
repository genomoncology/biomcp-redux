# 0154 — Investigate the diagnostic slice and propose its design

Status: OPEN.

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
- Defers: tickets, which the PM files from the reviewed design.
