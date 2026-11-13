# 0874 — drugsfda-sponsor-approvals-newest-first

Status: COMPLETE.
Landed: 4604a9334868d2d54e1499e4de9808a75d9b958f

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

## Progress

- Measured live on 2026-11-13: openFDA accepts `sort=submissions.submission_status_date:desc,application_number:asc` together with the `sponsor_name:NOVARTIS*` prefix search. It sorts each application by its newest submission date, keeps the total at 204, breaks the 2026-10-15 tie (BLA125504 before BLA761349) by application number, and puts the 19 applications with no submissions last. `application_number.exact` has no sort mapping.
- Changed: `search drug --source drugsfda` takes `--sort newest` (Drugs@FDA only; other sources refuse it as a usage error). Every Drugs@FDA search row now carries `latest_submission` (date, type, number, status, class code), chosen by the same order the approvals section uses. A source notice states the order. Schemas only gain optional fields.
- Proof: `tests/cases/drug/sponsor-newest.yaml` with replies recorded by `biomcp --record` in job j38975. First page 5,082 bytes, second 5,499 bytes (job j38986); ceilings 6,000 and 6,400.
- Gate fixes: the Drugs@FDA search code moved from `crates/biomcp-adapters/src/drugsfda/mod.rs` into its own `search.rs`, because the adapter file passed the 300-line limit; the query-side notices moved into one helper to keep `search` under the function length limit.
- Review changes: the order notice and the `--sort` help now say openFDA orders every match, not just the page, and that the latest submission is often a labeling or efficacy supplement, not the original approval. Per the PM decision, each row also carries `original_approval` (date and submission number of the earliest ORIG submission with status AP, from the same reply; left out when there is none). The recorded case asserts it on both pages. The reply README now lists byte counts and SHA-256 of the original bodies (refetched the same day; parsed JSON identical to the recording).
- With original_approval and the longer notice, the pages measure 5,588 and 6,005 bytes (job j39118); ceilings 6,200 and 6,700.
