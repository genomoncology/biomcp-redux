# 0278 — Clinical review of the author slice questions

Status: OPEN.

Milestone: 4

Depends on: 0270

## Outcome

A clinical reviewer reads the author slice question bank and the landed author command output and records, per question, whether the answer and its stated limits are acceptable for clinicians and researchers, in the review section of `sdlc/planning/slices/author.md`. ROADMAP.md's author clinical review and Slice complete boxes tick in the landing commit when the review accepts the slice, and any rejected answer becomes its own ticket.

- Why: the author block's clinical review and Slice complete boxes have no owner, and every other slice that has those boxes recorded a clinical review.
- Starts from: author.md's question bank, the 0270 docs and cases, and `skills/clinical-reviewer.md`.
- Rules: the reviewer judges what the output claims and omits, such as identity limits and the noncommercial label, and asks no source for new data. No licensed content enters the repo.
- Proof: the review section names each question with its verdict and reason, and a reviewer checks that every ticked box matches an accepted verdict.
- Changes: author.md and ROADMAP.md.
- Defers: nothing.
