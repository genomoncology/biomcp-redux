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
- 2026-10-31: commits, gate submissions or green proof claims that followed a failed earlier step in the same shell command since 2026-10-24 (assignments now run dependent steps under `sh -ec` or as separate calls whose success is checked). Baseline 4: landers j10138 and j10198, proofs 0051 j10204 and 0047 j10288. Target 0.
- 2026-10-31: full landing gates that failed on a help, lint, structural or ratchet check that the cheap checks would have caught, since 2026-10-24 (assignments now run the affected help and discovery cases, all-target Clippy and all nine ratchets on the final changed head before a full gate). Baseline 7: six in 0081 (j10458, j10486, j10489, j10493, j10498, j10502) and one in 0082 (help registration order). Target 0.
- 2026-10-31: Windows proof jobs that failed from a missing OS variable, a missing tool or the wrong executable in an isolated child environment, since 2026-10-24 (the Windows proof recipe now names required OS variables and selected executables and varies one suspected condition at a time). Baseline 3 jobs (j10477, j10592, j10606), 1,218.8 s elapsed. Target 0. The three j10615 TLS failures stay outside this count until their cause is proved.
- Proof: each line gains its measured count, the command or source that showed it, and keep or reopen.
