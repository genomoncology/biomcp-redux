# 0431 — Make the runner the single record of tests

Status: OPEN.

Milestone: 5

Depends on: 0430

## Outcome

The `cases` target holds only behavior cases and trials, each named by a listed feature. Proof, measurement and generation tools move to a test target that `cargo test` and coverage never run. Output budgets live only in YAML. Docs generation never edits a case. Trials that call library code in the harness process run the binary instead, or go.

- Why: `cases.rs` mixes behavior tests with parity, live-check, measure, native-proof and generator modes. Feature mapping lives in YAML fields and in `features.md` globs that drift. 13 JSON budget files duplicate YAML limits. `make docs-generate` rewrites cases.
- Starts from: `crates/biomcp/tests/cases.rs`, `crates/biomcp/tests/runner/`, the 51 runner files that import library crates, the 25 test files with `#[path]` includes, `scripts/check_docs.py`, `sdlc/features.md`, the JSON budget files under `tests/`, `Makefile`, `release/smoke.py`, and the in-process trial list from 0429.
- Rules: a new `crates/biomcp/tests/tools.rs` with `test = false` and `harness = false` takes parity, parity inventory, live check, measures, native proofs and generators, and shares `runner/launch.rs`. `release/smoke.py` calls moved modes from it. A Rust trial's name starts with `<feature>/`. `sdlc/features.md` keeps one `name: sentence` line per feature. The runner fails at discovery on an unlisted feature or a feature with no test. `covers` is built as architecture 10.2 says, or it goes with that paragraph.
- Proof: planted fixtures in `crates/biomcp/tests/ratchets/proof.rs` show that a case naming an unlisted feature, a listed feature with no test, and a trial named without a feature each fail discovery; a planted tool mode in `cases.rs` fails a ratchet; running docs generation in a temporary copy and finding a changed file under `tests/` fails a trial; the reply diff is empty.
- Changes: the files above and `crates/biomcp/Cargo.toml`.
- Defers: case content (0456).
