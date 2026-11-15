# 0944 — gate-runs-tool-proofs

Status: OPEN.

Milestone: 7

## Outcome

The landing gate runs the case runner's tool-only proofs (`make proof`), so a proof that breaks fails the gate rather than only compiling.

## Why

0939's developer (2026-11-14): the gate and `make full-suite` only lint the tool-only tests (`lint-tools`); a broken proof such as `schema-discovery/starts-within-reads-raw-bytes` would pass the gate. 0939 ran it by hand (j40657).

## Proof

- Proof: a deliberately broken proof on a scratch branch fails `make landing-gate` in a job; the gate's time rises by no more than the measured cost, recorded in Progress.

## Starts from

- Starts from: `scripts/landing_gate.py` (`LANDING` stages), the `proof` target in the Makefile. Time a full `make proof` in a job first; if it is slow, run only proofs whose files changed, as 0914/0921 did for library tests.

## Keeps

- Keeps: gate output format; other stages.

## Defers

- Defers: nothing.

## Changes

- Changes: the landing gate script, Makefile, maybe the runner's proof selection.

## Progress

- Measured: the whole catalog (29 proofs, 1 ignored) runs in 292 s once built (j40931). The base gate on main took 588 s (j40893, 4e99fe4be). With the whole catalog added, the gate took 1,050 s (j40931, on an M5 under load 8 to 12, where cases took 455 s against 242 s on main), and the stage itself took 313 s. The first version ran the catalog through `scripts/trials.py`, which vendors every crate and rebuilds all dependencies against the vendored copy: that stage took 594 s, of which about 309 s was the build (j40909). Plain `cargo test` reuses the cases stage's build, so the `tools` binary builds in about 12 s.
- PM decision in review: 292 s is too much for every landing. Running the stage in parallel with cases was not taken: both share one cargo target, they would compete for CPU on a loaded M5, and the stage line format would change.
- Changed: the landing gate runs a new `changed-tool-proofs` stage after `cases` (`scripts/changed_tool_proofs.py`). Like changed-library-tests (0914/0921), it diffs `HEAD` against its merge base with `origin/main`. It runs every proof when the branch changes the runner (`crates/biomcp/tests/`, `crates/biomcp/Cargo.toml`), `crates/biomcp-test-support/`, the embedded schemas the schema proofs compile (`crates/biomcp-types/schemas/`), the workspace manifest, lock file, toolchain or `.cargo/`, or the selector itself. The proofs live in the runner and test it, so their own files are runner files. The HGNC recorded replies the cache-resilience proofs read (`tests/replies/trial/gene/hgnc/`) select only `cache-resilience/hgnc-`. Other changes run no proofs. With no base it runs every proof. The log names the base, the selection and each cargo command. `TOOL_PROOFS_BASE` overrides the base.
- `make full-suite` runs the whole catalog (`make tool-proofs`) before coverage. `make proof` without `PROOFS` runs the whole catalog; with `PROOFS` it runs those proofs as before.
- Tests: `scripts/test_gate_stages.py` checks the stage order, that a failing `changed-tool-proofs` stops the gate, and the selection against real Git with a stub cargo.
