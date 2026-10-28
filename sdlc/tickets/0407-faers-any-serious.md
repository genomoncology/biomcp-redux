# 0407 — Restore the any-seriousness FAERS filter

Status: OPEN.

Milestone: 5

## Outcome

`search adverse-event --source faers --serious any` returns reports whose native report-level `serious` flag is 1, alongside the existing `--serious death` and `--serious hospitalization`. Help says `any` uses the source's own seriousness flag and that the specific flags can overlap.

- Why: the 0405 benchmark replay (coordinator m12774, 2026-10-28) found agents trained on 0.4.6 filter FAERS with `serious=true`, and 0.9.3 also had a bare `--serious`. The adverse-event slice dropped the bare form only because its harvest had no such question. The native flag needs no inference.
- Starts from: the adverse-event slice ruling rows at `sdlc/planning/slices/adverse-event.md` 316 and 424, the FAERS adapter and its cases.
- Rules: map `any` exactly to openFDA `serious:1`; no counts, sums or causal wording. Other retired seriousness values still fail before any request. One bounded live capture at most, within openFDA limits.
- Proof: a replay case shows the exact query and a report header with native serious code; retired values still refuse; the slice ruling row and upgrade table are updated.
- Changes: the FAERS filter grammar, adapter, help, the slice row, the generated upgrade table and cases.
- Defers: nothing.
