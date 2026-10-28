# 0429 — Run the whole case suite against the installed release binary

Status: OPEN.

Milestone: 5

## Outcome

The release rehearsal runs the full `cases` target, YAML cases and Rust trials, against the installed candidate through `BIOMCP_TEST_EXECUTABLE`, on the Linux containers and every native target, and reports per-case results.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. The installed artifact runs 1 of 654 YAML cases today, and release builds differ from debug builds.
- Starts from: `release/smoke.py`, `release/gate.py`, `release/container_check.py`, the test launcher and the rehearsal recipe.
- Rules: no new framework; the smoke timeout is set from a measured run.
- Proof: a rehearsal job runs every case against the installed binary and fails when a planted wrong binary is installed.
- Changes: the release scripts and the rehearsal recipe.
- Defers: nothing.
