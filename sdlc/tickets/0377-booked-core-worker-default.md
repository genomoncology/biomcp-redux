# 0377 — Default compiler jobs and test workers to the job's booked cores

Status: COMPLETE.
Landed: 7d98a3dc3bd2cda32765d715d2c397e3de991023

Milestone: 5

## Outcome

When `PM_JOB_CORES` is set, the repo's build and test targets default compiler jobs and test workers to it, including an explicit `--test-threads` for the libtest-mimic runners. A caller's own setting still wins.

- Why: split from 0344 so it can land before 0344's timing pair (coordinator m10791 ask 5, PM ruling 2026-10-27). Compilers and test runners default to every CPU, overloading shared build machines beyond each job's booking.
- Starts from: the Makefile targets, the coverage target and the libtest-mimic runners.
- Rules: one shared default; no new admission refusal or shared pm rule.
- Proof: a check shows the effective jobs and threads equal `PM_JOB_CORES` when set, equal the caller's value when given, and unchanged when neither is set; the focused review and one native batch routine pass.
- Changes: `Makefile` and runner argument forwarding.
- Defers: debug profile settings and the timing measurement, which stay in 0344.
