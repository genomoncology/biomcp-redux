# 0900 — drugsfda-search-by-original-approval

Status: OPEN.

Milestone: 7

## Outcome

A company's newest original FDA approvals, newest first, come from one call: a Drugs@FDA applicant search can be ordered by original approval date and windowed on it, and returns a sponsor's whole set (a few hundred applications) in one reply or states its total and pages.

## Why

In reg-affairs round 2 (biomed-agent-bench `personas/reg-affairs/runs/2026-11-13-r2/log.md:64-96`, analysis beside it), the digest's first table still took five paged calls and a client-side sort. `--sort original_approval` was refused (`valid_values ["newest"]`), `--sort newest` orders by latest submission (often a labelling supplement; 0874), and `--limit 200` was refused (at most 50). Rows already carry `original_approval` since 0874. In round 1 the code-mode agent got this table from openFDA in one call.

## Proof

- Proof: a recorded case for `--applicant NOVARTIS --source drugsfda` with `--sort original-approval` returning all 204 applications, ordered by `original_approval.date` descending with rows lacking one last, total stated. A second step uses `--approved-from 2024-01-01`, returns only applications whose original approval falls in the window, and states the count. Both orders and counts must agree with the recorded openFDA reply.

## Starts from

- Starts from: 0874's `--sort newest` and `original_approval` in the Drugs@FDA search adapter (`crates/biomcp-adapters/src/` drugsfda, `crates/biomcp-query/src/drug/`). openFDA cannot sort on one element of `submissions`, so read the sponsor's whole set (openFDA allows `limit` up to 1000) and sort or window it in BioMCP, saying so in a notice.

## Keeps

- Keeps: `--sort newest` and its meaning; the 50-row page for other sources.

## Defers

- Defers: windows on other submission types.

## Changes

- Changes: the Drugs@FDA search adapter and query, `crates/biomcp-types/` (an added sort value and optional filters), `tests/cases/drug/`, `docs/`.
