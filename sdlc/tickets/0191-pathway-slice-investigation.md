# 0191 — Investigate the pathway slice and propose its design

Status: OPEN.

Milestone: 4

## Outcome

`sdlc/planning/slices/pathway.md` holds the pathway slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 4 delivers the remaining entities; clinicians and tumor boards ask which pathway a targetable gene belongs to and which other actionable genes share it.
- Starts from: `planning/slices.md` with its quality-floor, capture-before-types and public-issue rules; the landed slice designs, especially those this entity joins; the shared biodata model; the pathway sources in ROADMAP.md; 0.9's pathway commands; decisions on text heuristics, data bundles, keyed sources, open-by-default terms and the amended biomedical research scope.
- Examples: 20 to 40 real biomedical research and clinical questions from labels, papers and public issues, covering a pathway for a driver gene, the genes in a named pathway, a drug target's pathway context, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask Reactome, WikiPathways and g:Profiler (investigate); KEGG stays uncalled and dropped directly for each example and record counts, speed, requests, identifiers, the fields each public tier carries, bundle sizes and update cadence where relevant, and terms, with use labels and groups. Keys are inherited by name only and never printed.
- Identity: record pathway and gene identifiers and how they join to gene and drug without name matching.
- 0.9: measure 0.9's commands only behind verified network containment, otherwise mark them unmeasured; account for every command, option and section, and every closed 0.9 public issue touching this entity.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps under architecture 6.10, disclosure layers and measured token budget, sources and bundles kept and dropped, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
