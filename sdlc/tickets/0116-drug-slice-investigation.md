# 0116 — Investigate the drug slice and propose its design

Status: OPEN.

Milestone: 3

## Outcome

`sdlc/planning/slices/drug.md` holds the drug slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review and Ian's questions. No product code changes.

- Why: milestone 3 serves approval, label, safety and drug identity questions for clinicians, and every slice starts from its own investigation.
- Starts from: `planning/slices.md`; `planning/slices/variant.md` and `trial.md` as worked examples; milestone 3 in `planning/milestones.md`; the drug sources in ROADMAP.md; 0.9's drug commands; decisions `2026-10-24-no-text-heuristics.md`, `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` and `2026-10-24-sources-are-open-by-default.md`.
- Examples: 20 to 40 real clinical drug questions from papers and public issues, covering a brand or generic name, an approval status and date in the US and EU, a label section such as boxed warnings or dosing, a drug's targets for a cancer gene, a biomarker-linked therapy, an interaction, an orphan designation, an empty result and a failing source, each with its source and accepted answer. Bench chemistry and compound screening questions are out of scope. Clinical judgments go to a clinical reviewer.
- Sources: ask each candidate source in ROADMAP.md directly for each example and record counts, speed, requests, returned identifiers, the fields its public tier carries, and terms. Each bundle candidate records its file, size, update cadence and license. Each source marked investigate ends kept or dropped with a reason.
- Identity: record how each source names the same drug, which identifier joins them, and where they disagree. A changed drug identity answer goes to a clinical reviewer, as milestone 3's done-when says.
- 0.9: measure 0.9's drug commands on the same examples in time, bytes and facts, find any filter mapped to the wrong field, and account for every command, option and section.
- Design: the delight questions, types and schema, a search that returns drug headers with drill-downs into approvals, label, targets, interactions and other sections the examples need, disclosure layers and token budget against 0.9, sources and bundles kept and dropped, harvested case list, and the open questions for Ian with options and a recommendation.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- PM ruling 2026-10-24 after j11400: the old 0.9 binary bypassed the capture proxy, so its outputs have unknown network provenance and are excluded. The design is reviewed from the verified direct and bundle captures, with the 0.9 comparison marked unmeasured. A later 0.9 baseline runs only behind verified network containment, under a separate ticket.
- Defers: tickets, which the PM files from the reviewed design.
