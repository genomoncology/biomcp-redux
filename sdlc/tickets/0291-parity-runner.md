# 0291 — Build a parity runner that compares asserted facts on 0.9.3 and 1.0

Status: OPEN.

Milestone: 5

## Outcome

A parity runner and its make target execute each harvested case's question against released 0.9.3 and a 1.0 binary under the same recordings or a bounded live window. Its report says for every asserted fact whether 0.9.3 got it right and whether 1.0 does, and records each command's output size and elapsed time on both binaries.

- Why: milestone 5 is done only when every asserted fact that 0.9 gets right is also right in 1.0, and every slice left its 0.9 size and timing figures for this comparison. PM ruling 2026-10-26 (m8126 ask 27) splits the whole comparison run into 0297, so this code lands on scoped proof.
- Starts from: milestones.md, the harvested cases under `tests/cases/`, the parity accounting tables that map 0.9.3 spellings to 1.0, the 0.9.3 release binary, and the nightly live check's bounded source pacing.
- Rules: released 0.9.3 is the baseline. Live calls stay bounded and keyless or name-inherited, and no key value is recorded.
- Proof: the runner on a small fixed sample of cases under recordings produces a report with both verdicts, sizes and times for each case, including one fact 0.9.3 got wrong and one spelling mapped through the parity tables, and a reviewer checks those verdicts against the raw outputs.
- Changes: a parity runner script, its make target and its sample fixtures.
- Defers: the whole comparison run and the size and timing rows, which are 0297, and the outside judged agent comparison, which is 0098.
