# 0117 — Investigate the disease slice and propose its design

Status: OPEN.

Milestone: 3

## Outcome

`sdlc/planning/slices/disease.md` holds the disease slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review and Ian's questions. No product code changes.

- Why: milestone 3 serves disease identity questions, and the trial slice left explicit MyDisease.info resolution for disease identity to a later slice.
- Starts from: `planning/slices.md`; `planning/slices/trial.md` and its MyDisease resolution section; `planning/slices/variant.md`; milestone 3 in `planning/milestones.md`; the disease sources in ROADMAP.md; 0.9's disease commands; decisions `2026-10-24-no-text-heuristics.md` and `2026-10-24-sources-are-open-by-default.md`.
- Examples: 20 to 40 real clinical disease questions from papers and public issues, covering a cancer type by name and by identifier, synonyms and cross-references such as MONDO, NCIt, OncoTree and ICD, subtypes and parents, genes associated with a cancer, phenotypes for a rare tumor syndrome, incidence and survival from SEER, an ambiguous name, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask each candidate source in ROADMAP.md directly for each example and record counts, speed, requests, returned identifiers, the fields its public tier carries, and terms. Each source marked investigate ends kept or dropped with a reason. UMLS stays uncalled unless a product smoke inherits a user key under an approved recipe.
- Identity: record how each source names the same disease, which identifier joins them, and how an ambiguous name returns candidates without a guess. Record how the trial slice's `--condition` could use a resolved disease identity explicitly.
- 0.9: measure 0.9's disease commands on the same examples in time, bytes and facts, find any filter mapped to the wrong field, and account for every command, option and section.
- Design: the delight questions, types and schema, a search that returns disease headers with drill-downs into synonyms, hierarchy, genes, phenotypes, statistics and other sections the examples need, disclosure layers and token budget against 0.9, sources kept and dropped, harvested case list, and the open questions for Ian with options and a recommendation.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- Defers: tickets, which the PM files from the reviewed design.
