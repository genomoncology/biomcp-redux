# 0924 — version-verbose-leaves-schemas-out

Status: OPEN.

Milestone: 7

## Outcome

`biomcp version --verbose` prints the version, commit, target, config and cache, and no longer lists every schema version. The schema list is still available from the command that lists schemas.

## Why

The bench's help-read analysis of 2026-11-14 (bench `personas/_eval/help-reads-2026-11-14.md`): every persona run calls `biomcp version --verbose` at request 1, as the persona brief asks. About 7.6k of its 8k characters are the "Schemas:" list, about 2.4k tokens, which then stay in context to the end of the run: 4.9 to 5.8% of every run's input tokens. Any agent asked to report its BioMCP build pays the same.

## Proof

- Proof: the housekeeping case for `version --verbose` (`crates/biomcp/tests/runner/housekeeping.rs`) asserts commit and target are present and the schema list is not.

## Starts from

- Starts from: `crates/biomcp/src/housekeeping.rs:75`; the schema-listing command `biomcp schema`.

## Keeps

- Keeps: the version, commit and target lines that bench `run.md` files and release checks read; JSON output fields other than the schema list, unless the schema rules allow dropping it (if a JSON field must stay under the 1.0.0 baseline, keep it in JSON and drop it from the text only).

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/housekeeping.rs`, its case, generated docs.

## Progress

- `biomcp version --verbose` now prints version, commit, target, config and cache, and no schema list. Measured on the M5: 7,854 bytes on main 9330e44bc (j40195), 186 bytes on this branch (j40197). `biomcp schema` still lists the schemas.
- No reader depends on the list: release/gate.py and scripts/check_front_map.py read only the version and commit lines, scripts/test_reply_diff.py masks only build fields, and the bench reads only version and commit (`personas/_eval/make_cases.py`). `version` has no JSON form and no reply schema in release/schema-baseline.json, so no schema changed.
- Removed the now-unused `schema::versions()`. The `--verbose` help text drops "schema versions"; docs/reference/commands.md follows.
- Case: `crates/biomcp/tests/runner/housekeeping.rs` asserts commit and target are present and that no "Schemas: " line or schema ID appears. Gate `make --keep-going landing-gate docs-check schema-release-check` passed at ba41973a0 (j40196).
- Review fix: the `version` summary in `crates/biomcp/src/grammar/local.rs` now reads "Print version; --verbose adds build and path provenance. No source requests or key reads." `make docs-generate` (j40279) changed the four matching lines in docs/reference/commands.md. Gate passed at f091b9974 (j40283).
