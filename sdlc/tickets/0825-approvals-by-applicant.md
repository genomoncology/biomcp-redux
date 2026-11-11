# 0825 — approvals-by-applicant

Status: OPEN.

Milestone: 8

## Outcome

`search drug --source drugsfda --applicant NAME` returns the applications that sponsor holds, paged, with totals; the same filter works in MCP.

## Why

Regulatory affairs asked what a company has had approved; raw openFDA answers in four requests and BioMCP cannot, though it already reads the sponsor and shows it as `submitter`.

## Proof

- Proof: a routine case under `tests/cases/drug/` with a recorded reply asserts a known sponsor's rows and total; an unknown sponsor answers empty, not an error.

## Starts from

- Starts from: `crates/biomcp-adapters/src/drugsfda/mod.rs`, which searches brand and ingredient names only, and `raw.rs`, which reads `sponsor_name`; `--firm` on recall search as the filter pattern.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. Matching is the source's own on `sponsor_name`; no company-family or subsidiary inference.

## Defers

- Defers: a per-company digest across approvals, labels and recalls waits for the composite commands.

## Changes

- Changes: `crates/biomcp-adapters/src/drugsfda/`, `crates/biomcp-types/src/query/`, `tests/cases/drug/`
