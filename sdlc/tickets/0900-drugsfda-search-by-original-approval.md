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

## Progress

- Measured live on 2026-11-13, without a key: drugsfda search accepts `limit` up to 1000 (1001 is refused: "Limit cannot exceed 1000 results for search requests"), and `skip` past 1000 works. `sponsor_name:NOVARTIS*&limit=1000` returns all 204 in one reply of 2.1 MB in about 1 s. Largest sponsor prefixes: WATSON* 1,053, TEVA* 798, AUROBINDO* 618, HIKMA* 693, SANDOZ* 515, MYLAN* 458, PFIZER* 210; WATSON* is the only one over 1,000. Bodies vary by row: 1,000 rows of NDA02* are 12.4 MB, past the 8 MiB body bound, and 500 BLAs are 5.3 MB. `sort=application_number:asc` works on its own.
- Changed: `search drug --source drugsfda` takes `--sort original-approval` and `--approved-from`/`--approved-to` (YYYY, YYYY-MM or YYYY-MM-DD, inclusive). For either, BioMCP reads the whole match set in requests of 500 ordered by application number (or by openFDA's newest order with `--sort newest`), keeps the window, orders by `original_approval` date newest first (ties by application number, rows without one last) and returns one page of it. The page total is the kept count; a notice says BioMCP read and ordered all N matches, and with a window how many it kept. At most 6 reads (3,000 applications): past that the source is failed as `too-large`, no rows come back, and the notice states the match count and how to narrow. Schemas only gain optional fields and a widened sort enum. `--sort newest` is unchanged.
- Kept the 50-row page for Drugs@FDA too: a 50-row page of these rows measures 48,828 bytes (job j39749), so all 204 Novartis rows would be about 200,000 characters, past the MCP 150,000-character result cap (`crates/biomcp/src/mcp/protocol.rs`). The whole set is ordered in one read and served as stated pages; the digest's first table (newest originals) is the first page.
- Proof: `tests/cases/drug/sponsor-original-approval.yaml`, replies recorded in job j39749. All 204 Novartis ids across five pages, the 50 original approval dates on page 1, the 2024-01-01 window (4 applications: NDA218436 2025-09-30, NDA219208 2025-04-02, NDA218466 2024-04-24, NDA218591 2024-04-12, matching the round 2 table) and the too-large refusal for the prefix A (4,953 matches) all agree with an order computed separately from the recorded body.
