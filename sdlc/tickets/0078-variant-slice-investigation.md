# 0078 — Investigate the variant slice and propose its design

Status: COMPLETE.
Landed: 491747e60b22e5ad12bc45fa36148c71fb5b6834

Milestone: 2

## Outcome

`sdlc/planning/slices/variant.md` holds the variant slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review and Ian's questions. No product code changes.

- Why: milestone 2 starts with variants and trials, and every slice starts from its own investigation.
- Starts from: `planning/slices.md` (steps, rules learned from 0.9, progressive disclosure and budget); `planning/slices/gene.md` and `article.md` as worked examples; the variant sources in ROADMAP.md; 0.9's variant commands; decisions `2026-10-24-biodata-builds-its-own-parts.md`, `2026-10-24-no-text-heuristics.md`, `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` and `2026-10-24-article-keeps-every-source-one-option-away.md` (COSMIC only as MyVariant carries it).
- Examples: 20 to 40 real variant questions from papers and public issues, covering a common hotspot, a rare variant, HGVS in c., p. and g. forms, an rsID, a coordinate with an explicit build, an ambiguous protein change, an absent variant and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask each ROADMAP variant source directly for each example and record coverage, speed, requests, disagreements, returned identifiers, the build each coordinate states, the fields the public tier carries, and terms. Keyed sources are checked only from public documentation. Investigate-tagged sources end kept, deferred or dropped with a reason.
- Notation: BioMCP passes HGVS and other notation to sources as checked text. The design names which source resolves each form and which typed parses BioMCP needs, if any. A need for a full HGVS parser is a finding for the PM, never built here.
- 0.9: measure 0.9's variant commands on the same examples in time, bytes and facts, and account for every command, option and section.
- Design: the delight questions, types and schema, command tree with headers and drill-downs, disclosure layers and token budget against 0.9, sources kept and dropped, harvested case list, and the open questions for Ian with options and a recommendation.
- Proof: the PM reviews the design on its branch. Live captures run through pm job, keyless, with raw evidence on the evidence store and no licensed text in the repo.
- Defers: tickets, which the PM files from the reviewed design.
