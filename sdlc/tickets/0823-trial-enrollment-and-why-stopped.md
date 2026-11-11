# 0823 — trial-enrollment-and-why-stopped

Status: COMPLETE.
Landed: 18a0e2cd5263fcb2fabdaeb3ee7b8c588154b3bc

Milestone: 8

## Outcome

`get trial NCT...` from ClinicalTrials.gov shows enrollment (count and whether anticipated or actual) and, for a stopped trial, the registry's why-stopped text, in CLI and MCP.

## Why

Trial planners calibrate on these two fields; the persona run got them from the registry API by curl and not from BioMCP (`biomed-agent-bench/personas/trial-ops`, questions 6 and 7).

## Proof

- Proof: a routine case under `tests/cases/trial/` with a recorded reply for one completed and one terminated trial asserts both fields; it fails on the base commit.

## Starts from

- Starts from: `Status` and `Design` in `crates/biomcp-adapters/src/ctgov/detail.rs` and `detail/dates.rs`, which read status, dates, study type and phases only; the trial card type in `crates/biomcp-types`. The registry's `designModule.enrollmentInfo` and `statusModule.whyStopped` are already in the fetched record.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. A trial with neither field shows neither, with no placeholder.

## Defers

- Defers: enrollment on search rows goes to 0826's follow-up if asked; NCI's enrollment fields wait for a request.

## Changes

- Changes: `crates/biomcp-adapters/src/ctgov/detail.rs`, `crates/biomcp-adapters/src/ctgov/detail/`, `crates/biomcp-types/src/`, `tests/cases/trial/`
