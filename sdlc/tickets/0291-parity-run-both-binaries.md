# 0291 — Run the harvested cases against 0.9.3 and 1.0 and compare asserted facts

Status: OPEN.

Milestone: 5

## Outcome

One parity run executes each harvested case's question against released 0.9.3 and the 1.0 candidate under the same recordings or a bounded live window, and reports for every asserted fact whether 0.9.3 got it right and whether 1.0 does. It also records each command's output size and elapsed time on both binaries, which fills the contained 0.9 size and timing comparison every slice deferred.

- Why: milestone 5 is done only when every asserted fact that 0.9 gets right is also right in 1.0, and every slice left its 0.9 size and timing figures unmeasured for this comparison.
- Starts from: milestones.md, the harvested cases under `tests/cases/`, the parity accounting tables that map 0.9.3 spellings to 1.0, the 0.9.3 release binary, and the nightly live check's bounded source pacing.
- Rules: a fact 0.9.3 got right and 1.0 gets wrong becomes its own bug ticket. A fact 0.9.3 got wrong is reported, not counted against 1.0. Live calls stay bounded and keyless or name-inherited, and no key value is recorded.
- Proof: the run's report lists every case with both verdicts and both sizes and times, and a reviewer checks a sample of verdicts against the raw outputs.
- Changes: a parity runner script and its make target, and the per-slice size and timing rows in ROADMAP.md and the slice docs.
- Defers: the outside judged agent comparison, which is 0098.
