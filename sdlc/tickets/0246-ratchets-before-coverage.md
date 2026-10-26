# 0246 — Run the ratchets before coverage

Status: OPEN.

Milestone: 1

Depends on: none

## Outcome

`make coverage` and `make landing-gate` run the existing `ratchets` target before any case or contract runs, so a structural failure stops a job in under a second instead of after the whole suite. A read-only RCA found five development jobs across 0201, 0203 and 0204 (j14273, j14292, j14310, j14290, j14340) that ran full suites before a ratchet failed; in 0203, 1,110 cases ran before a 0.19 s ratchet found a 601-line runner file.

- Changes: `Makefile` only. `coverage` gains `ratchets` as a prerequisite. The later ratchet collection inside `scripts/coverage.sh` stays for now.
- Keeps: every existing target, check and assertion. No new framework, rule or refusal.
- Proof: a job with a deliberately oversized throwaway file on a scratch branch shows `make coverage` failing in the ratchets step before any case runs; the clean tree passes `make landing-gate` through pm job. The scratch branch is deleted.
- Order: land before the next full-suite batch where practical.
