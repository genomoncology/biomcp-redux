# 0874 — drugsfda-sponsor-approvals-newest-first

Status: OPEN.

Milestone: 7

## Outcome

A regulatory digest's first table, a company's most recent FDA drug approvals newest first with application numbers and dates, comes from one BioMCP call.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:63-75 and 269-280: `search drug --source drugsfda --applicant NOVARTIS` returns 204 applications with no approval date on any row and no sort, so the digest needs one `get drug <id> approvals` per application (204 calls). openFDA's own API sorts by `submissions.submission_status_date` in one call.

## Proof

- Proof: a routine case where an applicant search returns rows carrying each application's latest action date and type, ordered newest first on request; the reply states the sort and the source total.

## Starts from

- Starts from: the Drugs@FDA search adapter and page (`crates/biomcp-adapters/src/` drugsfda, `crates/biomcp-query/src/drug/`); openFDA's `sort=` on drugsfda.

## Keeps

- Keeps: the applicant prefix match (204 vs openFDA's exact 6); the page total; existing row fields.

## Defers

- Defers: a portfolio entity per sponsor.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/drug/`, `crates/biomcp-types/src/`, `tests/cases/drug/`
