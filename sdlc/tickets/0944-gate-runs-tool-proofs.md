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

- Changed: the landing gate runs a new `changed-tool-proofs` stage after `cases` (`scripts/changed_tool_proofs.py`, `make changed-tool-proofs`). Like changed-library-tests (0914/0921), it reads `git diff -z --name-only` from `HEAD`'s merge base with `origin/main` (`TOOL_PROOFS_BASE` overrides it) and logs the base, the selection and each cargo command:
  - every proof, the slow ones included: a change to the case runner or proof code (`crates/biomcp/tests/`, `crates/biomcp/Cargo.toml`, `crates/biomcp/build.rs`, `crates/biomcp-test-support/`), the Makefile, the selector, the workspace manifest, lock file, toolchain or `.cargo/`; also when no base is found;
  - every proof except the slow ones: a change to product code every proof group runs, `crates/biomcp/src/`, `crates/biomcp-types/`, `crates/biomcp-query/`, `crates/biomcp-fetch/`, or adapter code outside a single source's module. Each group runs product code: mcp-http (early-close starts the product HTTP server), schema-discovery (`biomcp schema` and a real CIViC case through the runner, fixture evidence calling biomcp-fetch and biomcp-query, source terms reading biomcp-query), cache-resilience HGNC (spawns sync, calls fetch and query), article-semantic-sections (`article references --source semantic-scholar` on recorded replies), runner/report-parity (biomcp-types);
  - only the proofs that run it: the hgnc adapter or `tests/replies/trial/gene/hgnc/` (`cache-resilience/hgnc-`), the semantic_scholar or pubmed adapter or `tests/replies/article/edges/live/` (`article-semantic-sections/`), the civic adapter or `tests/fixtures/drug/civic/` (`schema-discovery/persistent-native-evidence`), and the files the binary's build script embeds, `skills/`, `tests/cases/`, `docs/reference/commands.md`, `docs/reference/source-pacing.md`, `docs/recipes.md` (`schema-discovery/catalog-and-help-reply-discovery`);
  - no proof: any other single-source adapter module, and every other path.
- The slow proofs, over 30 s alone, are named in the selector's `SLOW`: `schema-discovery/rejects-bad-cases` (264 s) and `schema-discovery/selects-and-lists-cases` (75 s) (j41018). Product changes skip them; they run on runner, proof or build changes and in `make proof` and `make full-suite`. PM decision: a follow-up ticket speeds them up; this ticket does not.
- `make full-suite` runs the whole catalog (`make tool-proofs`) before coverage. `make proof` without `PROOFS` runs the whole catalog; with `PROOFS` it runs those proofs as before. Both use plain `cargo test`, which reuses the cases stage's build: `scripts/trials.py` vendors every crate and rebuilt all dependencies, about 309 s (j40909), where the `tools` binary now builds in about 12 s.
- Tests: `scripts/test_gate_stages.py` checks the stage order, that a failing `changed-tool-proofs` stops the gate, and the selection against real Git with a stub cargo, a non-ASCII path included.
- Measured: the base gate on main took 588 s (j40893). The whole catalog takes about 279 s after one build (j40984). Alone after one build: mcp-http/ 21 s, schema-discovery/ 280 s, cache-resilience/hgnc- 10 s, article-semantic-sections/ 8 s, runner/report-parity 6 s, persistent-native-evidence 18 s, catalog-and-help-reply-discovery 12 s (j40984, j41018). Every other schema-discovery proof took 4 to 17 s alone, of which about 4 s is cargo start-up (j41018). A typical crates/biomcp/src change selects every proof but the slow ones: 27 proofs in 15 s, 21 s with cargo start-up, after the cases stage's build (j41030). A change that selects every proof adds the whole catalog: the stage took 308 s in this branch's gate (j41029, 871 s total against 588 s on main), because this branch changes the selector and Makefile.
- Proof: on the scratch branch `scratch/0944-broken-proof` (one wrong offset in `assertions/offsets.rs`), `make landing-gate` selected every proof (a runner file changed) and failed at `gate stage=changed-tool-proofs ... exit=2`, naming `schema-discovery/starts-within-reads-raw-bytes` ("starts_within read the wrong byte") (j40961). The scratch branch is deleted from origin.
