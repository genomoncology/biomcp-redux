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
- Proof: on the scratch branch `scratch/0944-broken-proof` (this branch plus one wrong offset in `assertions/offsets.rs`), `make landing-gate` selected every proof (a runner file changed) and failed at `gate stage=changed-tool-proofs ... exit=2`, naming `schema-discovery/starts-within-reads-raw-bytes` ("starts_within read the wrong byte") (j40961). The scratch branch is deleted from origin. This branch's own gate passed with every proof selected, because it changes the selector (j40960; the M5 was under load 14, so cases took 889 s).
- Gate time: a branch that changes no runner, schema or build file adds one `git diff` and selects no proofs, which takes well under a second (changed-library-tests' identical step takes 0.12 s). A branch that changes the runner adds the whole catalog, 297 s in j40961.
- PM review of fedbf0705: product-only changes must select the proofs that run product code. Each proof group was checked in `tool_trials()`: mcp-http (early-close starts the product HTTP server), schema-discovery (runs `biomcp schema` and real cases through the runner; fixture evidence calls biomcp-fetch and biomcp-query directly; source terms reads biomcp-query), cache-resilience HGNC (spawns sync, calls Fetch and query), article-semantic-sections (runs `article references --source semantic-scholar` on the recorded `article/edges/live` replies), runner/report-parity (biomcp-types). Since every group runs crates/biomcp/src, biomcp-types, biomcp-query or biomcp-fetch code, a change to any of them selects every proof. Shared adapter code does too; one source's adapter module or recorded data selects only the proofs that run it, and the files the binary embeds select only the help discovery proof. The selector reads `git diff -z` and treats the Makefile as a run-all input.
- Measured each subset alone after one build (j40984, 2026-11-15): mcp-http/ 21 s, schema-discovery/ 280 s, cache-resilience/hgnc- 10 s, article-semantic-sections/ 8 s, runner/report-parity 6 s, persistent-native-evidence 18 s, catalog-and-help-reply-discovery 11 s, whole catalog 279 s. Each figure includes about 5 s of cargo start-up. A typical crates/biomcp/src change therefore adds about 280 s, past the PM's two-minute line, so the work stopped here for a PM decision. The schema-discovery group sets the time: the two terms-subtraction proofs finished last in both the group and the whole run.
