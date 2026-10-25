# 0196 — Investigate the cell line slice and propose its design

Status: OPEN.

Milestone: 4

## Outcome

`sdlc/planning/slices/cell-line.md` holds the cell line slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 4 delivers the remaining entities; 0.9 offered cell line lookups; this slice must first decide, under the standing care-scope ruling that skips bench research, whether any cell line question serves a cancer care decision. If no question serves care, the design proposes dropping the slice under that ruling with every 0.9 command accounted, and stops before types.
- Starts from: `planning/slices.md` with its quality-floor, capture-before-types and public-issue rules; the landed slice designs, especially those this entity joins; the shared biodata model; the cell line sources in ROADMAP.md; 0.9's cell line commands; decisions on text heuristics, data bundles, keyed sources, open-by-default terms and cancer-care scope.
- Examples: 20 to 40 real clinical questions from labels, papers and public issues, covering a cell line's identity and disease, its known driver mutations, a drug's screening results, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask Cellosaurus, ChEMBL and PharmacoDB directly for each example and record counts, speed, requests, identifiers, the fields each public tier carries, bundle sizes and update cadence where relevant, and terms, with use labels and groups. Keys are inherited by name only and never printed.
- Identity: record Cellosaurus accessions and how they join to disease, gene, variant and drug without name matching.
- 0.9: measure 0.9's commands only behind verified network containment, otherwise mark them unmeasured; account for every command, option and section, and every closed 0.9 public issue touching this entity.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps under architecture 6.10, disclosure layers and measured token budget, sources and bundles kept and dropped, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
