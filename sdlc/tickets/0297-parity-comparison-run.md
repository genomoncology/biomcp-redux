# 0297 — Run the whole parity comparison and fill the size and timing rows

Status: OPEN.

Milestone: 5

Depends on: 0291, 0298, 0299, 0300, 0301, 0302, 0303, 0304

## Outcome

A build delegate runs the landed parity runner from main over every harvested case against released 0.9.3 and the 1.0 candidate, records the per-slice size and timing rows, and files a bug ticket for each fact 0.9.3 got right and 1.0 gets wrong.

- Why: the milestone 5 parity result and the size and timing comparison every slice deferred come from one whole run on landed code. PM ruling 2026-10-26 (m8126 ask 27).
- Starts from: the landed 0291 runner and make target, the 0.9.3 release binary and main at the time of the run.
- Rules: released 0.9.3 is the baseline. A fact 0.9.3 got right and 1.0 gets wrong becomes its own bug ticket. A fact 0.9.3 got wrong is reported, not counted against 1.0. Live calls stay bounded and keyless or name-inherited, and no key value is recorded.
- Proof: before the run, the dry inventory over every harvested case shows zero pending gaps and no unsafe plan; the run's report lists every case with both verdicts, sizes and times; a reviewer checks a sample of verdicts against the raw outputs; every regression has a ticket.
- Changes: the per-slice size and timing rows in ROADMAP.md and the slice docs, and new bug tickets.
- Defers: the outside judged agent comparison, which is 0098.
