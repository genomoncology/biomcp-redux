# 0060 — PM count checks (PM-owned, no developer lane)

Status: OPEN.

Milestone: 1

## Outcome

The PM checks each count below on its date and records the result here. A count that missed its target reopens its analysis. This ticket needs no code and no lane; the PM closes it after the last check.

- 2026-10-26: process experiments from `decisions/2026-10-24-code-lands-on-recorded-proof.md`. Coordinator asks to the PM per day: baseline about 25 on 2026-10-24, target half or fewer. Lanes held by an outside source: baseline 1, target 0.
- 2026-10-30: landing-gate failures from a missing dependency license after 0007 landed on 2026-10-23. Baseline 2, target 0.
- 2026-10-31: handoffs reopened at an external boundary since 2026-10-24. Baseline 2, target 0.
- 2026-10-31: test failures from hand-built recording identity since architecture 5.5's recipe change (a73111a). Baseline 2, target 0.
- One week after 0067 lands: schema generation failures from unsupported unit-enum forms or unsupported schema keywords since that landing. Baseline 4 failed jobs from 3 mistakes, target 0.
- 2026-10-31: lander commits or gate submissions that followed a failed edit step since 2026-10-24 (the landing assignment now uses python3, checks the edit succeeded, and stages and commits in a separate call). Baseline 2 (j10138, j10198), target 0.
- Proof: each line gains its measured count, the command or source that showed it, and keep or reopen.
