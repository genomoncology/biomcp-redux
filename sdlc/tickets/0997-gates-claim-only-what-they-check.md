# 0997 — gates-claim-only-what-they-check

Status: COMPLETE.
Landed: 2d266f71739d850d23e5852409920d54b234ed32

Milestone: 7

## Outcome

Every check in the landing gate runs what it claims to run, and nothing in the repo looks like coverage when no gate runs it.

## Why

These findings from `sdlc/records/2026-11-14-adversarial-review.md` were still open on main 3f14da1a6, a PM triage on 2026-11-16:

- **D3:** `scripts/landing_gate.py:11-12` leaves out the schema contract check. Every landing so far has passed it only because the PM adds `schema-release-check` to the command by hand.
- **D4:** nine `tests/tool-cases/` files are run by no stage. The runner reads only `tests/cases` (`runner/case_main.rs:61`).
- **D5:** `manifest.json` tool names are checked only by the full suite and the release.
- **D8:** `scripts/changed_library_tests.py:23` doesn't treat the `Makefile` as a library-test input.
- **F2:** the success line at `scripts/check_docs.py:336` claims a case and feature comparison that lives in the Rust runner.

## Proof

- Proof: the landing gate runs the schema contract check when `crates/biomcp-types/` or `release/schema-reviews.json` changes. A test of the selection shows it.
- Proof: each `tests/tool-cases/` file is either moved into `tests/cases/`, so the routine runner runs it, or deleted, with the reason in Progress.
- Proof: `check_docs.py` checks the manifest's tool names against the typed catalog's, and a test plants a wrong name.
- Proof: a Makefile change selects the library tests, and `check_docs.py`'s success line names only what it checks.

## Starts from

- Starts from: the review's lane D findings 3, 4, 5 and 8, and finding F2.

## Keeps

- Keeps: gate time. Each new stage runs only on the paths it covers.

## Defers

- Defers: nothing.

## Changes

- Changes: `scripts/`, the Makefile, `tests/tool-cases/`, and the tests of those scripts.

## Progress

- D3: `scripts/schema_release.py --changed` (Makefile `changed-schema-release-check`) is the landing gate's last stage. It compares the reply schemas with the release baseline only when the branch changed `crates/biomcp-types/`, `release/schema-*`, the script itself, or the workspace manifest, lock file, toolchain, `.cargo/` or Makefile; otherwise it prints `inputs=none` and runs no cargo. With no base it compares. The full suite now always runs `schema-release-check`. `scripts/test_schema_release.py` tests the selection against a fake cargo.
- D4: all nine `tests/tool-cases/` files moved into `tests/cases/`, so the routine runner runs them. Seven went to `tests/cases/mcp/typed/`, prefixed by entity. The two that send deliberately invalid arguments to test local refusal went to `tests/cases/article/typed-text-default.yaml` and `typed-selector-errors.yaml`, because the mcp-parity description contract requires every step under `mcp/typed/` to pass the advertised schema (gate j41712 failed there on `part: "abstract"`; everything before the cases stage passed). None was deleted: each holds the typed-tool form of a claim, and `article-part-range` is the only case that reads its replay (`tests/replies/article/fulltext/part-range`). `tests/tool-cases/` is gone. Cheap job j41707 ran them: seven passed and two had rotted, as the review warned. `article-part-range` and `drug-source-aliases` expected follow-up calls with `get.input`, but the server's suggested next calls now use `get.id`, the documented typed key, so those two expectations now name `id`. Everything else in j41707 passed: schema and docs regeneration changed nothing, check-cheap, gate-stage-check, `test_docs.py`, `test_schema_release.py`, and `schema_release.py --changed`, which saw the Makefile and script change and compared.
- D5: `check_docs.py`, which pre-job runs, now calls `release/tools.py`'s check. The manifest names the default tool list (`biomcp`, `search`, `fetch`), not the typed catalog, so it is compared with the server's declared default tools, as the release does. `scripts/test_docs.py` plants `fetch_record` and expects the failure.
- D8: `Makefile` is a workspace file in `changed_library_tests.py`; `test_gate_stages.py` expects every crate on a Makefile change.
- F2: the `check_docs.py` success line now names what it checks; it no longer claims a case and feature comparison (that lives in the Rust runner).
- Gate j41720 on e35f1fd09 (main 82360be61 included) passed: `make --keep-going landing-gate docs-check schema-release-check`, landing gate 495 s. The new stage took 16 s; it compared because this branch changes the Makefile and the script. On a branch that changes none of its inputs it runs no cargo.
