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
- 2026-10-31: commits, gate submissions or green proof claims that followed a failed earlier step in the same shell command since 2026-10-24 (assignments now run dependent steps under `sh -ec` or as separate calls whose success is checked). Baseline 4: landers j10138 and j10198, proofs 0051 j10204 and 0047 j10288. Target 0. Since then: 2 (0089 pushed conflict markers 62b2f945 after a format failure, fixed c2994c2 before any gate; 0093 j10884 continued after a failed merge into a refused commit).
- 2026-10-31: full landing gates that failed on a help, lint, structural or ratchet check that the cheap checks would have caught, since 2026-10-24 (assignments now run the affected help and discovery cases, all-target Clippy and all nine ratchets on the final changed head before a full gate). Baseline 7: six in 0081 (j10458, j10486, j10489, j10493, j10498, j10502) and one in 0082 (help registration order). Target 0.
- 2026-10-31: Windows proof jobs that failed from a missing OS variable, a missing tool or the wrong executable in an isolated child environment, since 2026-10-24 (the Windows proof recipe now names required OS variables and selected executables and varies one suspected condition at a time). Baseline 3 jobs (j10477, j10592, j10606), 1,218.8 s elapsed. Target 0. The three j10615 TLS failures stay outside this count until their cause is proved.
- 2026-10-31: jobs or capture runs whose local certificate setup was built by hand instead of reusing `network_stub::identity` in tests or the retained 0094 capture recipe, and that failed or passed on a TLS failure, since 2026-10-24. Baseline 3 (0101, 0088, 0094 j10794), 174.0 s elapsed. Target 0. Invalid case segments are counted apart from failed jobs.
- 2026-10-31: jobs blocked by out-of-ticket code inherited from another unaccepted lane, since 2026-10-24 (shared dependencies now get one writer and fresh acceptance, and consumers start from accepted main). Baseline 4 jobs across 3 tickets from 2 inherited defects (0091 j10935 and 0095 j10902 predictor test placement; 0095 j10919 and 0096 j10946 Statement suppression), at least 85.4 s elapsed. Target 0.
- 2026-10-31: candidate gates that reached a source outside an isolated fixture's declared sources, through expanding all sources or duplicated source ordering, since 0090 landed (fixture exclusions now come from the canonical source registry and allow only the declared fixture sources). Baseline 3 jobs: candidate gates j11116 and j11158 and targeted repair j11168. Target 0.
- Proof: each line gains its measured count, the command or source that showed it, and keep or reopen.
