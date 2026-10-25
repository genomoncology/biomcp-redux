# 0192 — Investigate the phenotype slice and propose its design

Status: OPEN.

Milestone: 4

## Outcome

`sdlc/planning/slices/phenotype.md` holds the phenotype slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 4 delivers the remaining entities; clinicians ask which hereditary cancer syndromes or genes fit a patient's phenotype terms, and what a given HPO term means.
- Starts from: `planning/slices.md` with its quality-floor, capture-before-types and public-issue rules; the landed slice designs, especially those this entity joins; the shared biodata model; the phenotype sources in ROADMAP.md; 0.9's phenotype commands; decisions on text heuristics, data bundles, keyed sources, open-by-default terms and the amended biomedical research scope.
- Examples: 20 to 40 real biomedical research and clinical questions from labels, papers and public issues, covering a syndrome from phenotype terms, the phenotypes of a hereditary cancer syndrome, a gene's phenotype annotations, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask Monarch and HPO directly for each example and record counts, speed, requests, identifiers, the fields each public tier carries, bundle sizes and update cadence where relevant, and terms, with use labels and groups. Keys are inherited by name only and never printed.
- Identity: record HPO, MONDO and gene identifiers and how they join to disease and gene, reusing the disease slice's kept annotations, without name matching.
- 0.9: measure 0.9's commands only behind verified network containment, otherwise mark them unmeasured; account for every command, option and section, and every closed 0.9 public issue touching this entity.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps under architecture 6.10, disclosure layers and measured token budget, sources and bundles kept and dropped, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
