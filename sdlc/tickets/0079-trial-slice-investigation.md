# 0079 — Investigate the trial slice and propose its design

Status: OPEN.

Milestone: 2

## Outcome

`sdlc/planning/slices/trial.md` holds the trial slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review and Ian's questions. No product code changes.

- Why: milestone 2 builds the trial slice, whose drill-downs set the progressive disclosure pattern for later entities.
- Starts from: `planning/slices.md`; `planning/slices/gene.md` and `article.md` as worked examples; milestone 2 in `planning/milestones.md`; the trial sources in ROADMAP.md; 0.9's trial commands; decisions `2026-10-24-no-text-heuristics.md` and `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md`.
- Examples: 20 to 40 real trial questions from papers and public issues, covering a condition with a biomarker, an intervention, recruiting status, phase, location and distance, eligibility, a single NCT ID, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask ClinicalTrials.gov directly for each example and record counts, speed, requests, returned identifiers, the fields its API carries and terms. Every filter gets a known-positive case whose count matches ClinicalTrials.gov's own count for the same query on the capture date, as milestone 2's done-when says. The NCI Clinical Trials Search API is checked from its public documentation and, only with a key the product smoke inherits, through one bounded job that never prints or records the key. MyDisease.info is checked for disease resolution.
- 0.9: measure 0.9's trial commands on the same examples in time, bytes and facts, find any filter mapped to the wrong field, and account for every command, option and section.
- Design: the delight questions, types and schema, a search that returns trial headers with drill-downs into locations, outcomes, eligibility and other sections, disclosure layers and token budget against 0.9, sources kept and dropped, harvested case list, and the open questions for Ian with options and a recommendation.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store.
- Defers: tickets, which the PM files from the reviewed design.
