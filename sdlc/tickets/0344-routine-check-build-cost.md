# 0344 — Measure the landing gate's builds and keep only the ones it needs

Status: OPEN.

Milestone: 5

## Outcome

One `make landing-gate` run through pm job records `cargo build --timings` output and the size of each build folder it creates. Any separate full build the landing gate does not need, such as a sanitizer or minimum-version build, moves to the release or long suite. The root `Cargo.toml` sets `debug = "line-tables-only"` for the dev and test profiles and `debug = false` for dependencies, so containers, the Windows test machine and outside contributors get the same debug settings as the build machines. The landing gate runs `fmt` and `lint` before `docs-check` and the other checks, so mechanical failures surface first.

- Why: the coordinator's RCA (m10448, 2026-10-27) found three jobs that spent time on behavior checks before Clippy caught a function-length failure. Jarvis's build speed review (m10175, 2026-10-27) found that routine checks build the code several times in separate build folders, and that the shared Cargo settings reach only our own build machines.
- Starts from: the Makefile's `landing-gate`, `coverage` and related targets, `sdlc/pm.json`, and the root `Cargo.toml`.
- Rules: do not copy sccache, mold, incremental or other shared machine settings into the repo. Keep incremental builds off. The release profile is unchanged.
- Proof: the ticket records the measured run's job ID, per-folder sizes and wall time before and after; the landing gate still runs format, lint, docs, tensor proto, coverage and audit checks; any moved build runs in the release or long suite.
- Changes: `Makefile`, `Cargo.toml` and, if needed, `sdlc/pm.json`.
- Defers: the booked-core worker default, which moved to 0377 (m10791). Seeding ticket build folders and the M5 compile cache size, which belong to pm and Jarvis.
