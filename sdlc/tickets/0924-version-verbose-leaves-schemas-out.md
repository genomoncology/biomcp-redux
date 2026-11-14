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
