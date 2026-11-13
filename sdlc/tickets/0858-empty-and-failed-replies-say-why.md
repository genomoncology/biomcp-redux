# 0858 — empty-and-failed-replies-say-why

Status: OPEN.

Milestone: 7

## Outcome

A reply never comes back empty, partial or failed without saying why: a label section absent from the label says so (not just empty); a section the chosen source lacks is a guided error, as other sources already give; a batched trial get names the reason for each missing section; a failed reply always names its cause in problems; an empty CPIC recommendation list for a provisional pair says the pair is provisional.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: life-sciences-ci log.md:40-43 (boxed warning empty, no reason); rare-disease log.md:87 vs 89 (silent 0 rows vs guided error); trial-ops log.md:46 (4 of 20 sections null); msl log.md:112 and pt-committee log.md:82 (`failed` with the only source `ok`, no problems); tumor-board log.md:265-270 (provisional CPIC pair, zero rows).

## Proof

- Proof: one routine case per behaviour above, each failing on the base commit.

## Starts from

- Starts from: the reply builders for drug label sections, disease sections, trial batched gets, article pdf and CPIC recommendations.

## Keeps

- Keeps: status rules; native totals; existing error schema unless a new code is needed (then the schema rule applies).

## Defers

- Defers: new sources; the fixes for the underlying data gaps.

## Amendment, 2026-11-12

From bench 0040's rerun, reproduced on main 59150d16d (`sdlc/records/2026-11-12-persona-rerun-failures.md`):

- Article PDF fails with no reason while its only source reports ok (failure #21, msl log :95-96).

## Changes

- Changes: `crates/biomcp-query/src/`, `crates/biomcp-adapters/src/cpic/`, `crates/biomcp-types/src/`, `tests/cases/`

## Progress

- Measured on main 6f7f0abb0 (pm job j38870). Still held: label section absent from the label (bare empty), batched trial get (failed items carried only input and status; the source said `deadline-exceeded` once), article PDF 24256730 (`failed`, source `ok`, no result; the publisher answered HTML, so the hidden reason was `not-pdf`), approvals document 68968 (`failed`, source `ok`; the document is a Label, not a Letter), and the provisional CPIC pair (complete, zero rows, no reason). No longer held: disease genes and phenotypes without `--source` give the guided `--source monarch` error for both MONDO:0016512 and MONDO:0007843. Document 68973 now names FDA's refusal (0868).
- A label heading the label lacks keeps `label` empty and fills the existing `sections` and `hint` fields: the hint says the label has no such section.
- A batched trial get names each unread trial with its reason in a `request-guidance` notice and puts a `fresh` step for just those trials first in `next`. Single gets are unchanged; their source reason already names the cause.
- A failed PDF save keeps its file section when the section is unavailable or skipped, so `result.file.reason` names the cause (`not-pdf`, `not-open`). Status and exit stay `failed` and 4.
- An approvals document ID the application does not list, or that is not a Letter, leaves `approvals` unavailable (`not-found` or `unsupported`, status `partial`, as an unlisted product number already did) with a notice naming the document's type and the application's letter IDs. Any other Drugs@FDA failure after an ok read now marks the source failed with its reason.
- An empty CPIC recommendation table for a gene and drug reads CPIC's pair row once and says, in a notice, whether the pair is provisional, has no guideline, or is not a CPIC pair.
- No schema shape changed: only the descriptions of the label `sections` and `hint` fields, and existing notice kinds, step kinds and section reasons.
