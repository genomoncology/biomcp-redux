# 0946 — clinvar-counts-finish-without-a-key

Status: OPEN.

Milestone: 7

## Outcome

`get gene GENE clinvar` (0932) completes without an NCBI API key on the persona machines, pacing its requests to NCBI's keyless limit and honouring the retry time NCBI's 429 reply gives, so a keyless run gets the counts or a clear partial reply naming what's missing.

## Why

dx-lab-director round 2: the new ClinVar section came back rate-limited all 4 times the agent asked for BRCA1 (calls 54-56 and 62); persona runs have no NCBI key. The analyst found about half of keyless reads fail on the M5. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05). 0932's review measured 16 sequential requests per gene.

## Proof

- Proof: a job on the M5 with no NCBI key runs `get gene BRCA1 clinvar` three times and each completes (4,752 P/LP on build 2026/11/09, or the current build); a recorded 429 with a retry time is honoured in a case.

## Starts from

- Starts from: 0932's section (`crates/biomcp-adapters/src/clinvar/gene.rs`), the NCBI pacing settings, 0932's 429 handling in `crates/biomcp-fetch/src/request.rs:352-361`.

## Keeps

- Keeps: the global NCBI rate for keyed use; 0932's key-echo guard.

## Defers

- Defers: nothing.

## Changes

- Changes: the ClinVar gene section's request pacing and retry, cases.

## Progress

- 2026-11-14 measured on the M5 with no NCBI key (the job environment sets `NCBI_API_KEY`; each run unset it and used an empty config). On main (da7c978f5), 0 of 5 `get gene BRCA1 clinvar --fresh` reads completed; each took 14-17 s and ended `rate-limited` with "retry after 2 s" (j40834). NCBI's keyless 429 carries `Retry-After: 2`, `x-ratelimit-limit: 3`, `x-ratelimit-remaining: 0` and a body naming the caller's IP address as `api-key`, with `"count":"4","limit":"3"`. Back-to-back curl searches went three accepted, two refused, three accepted; one search per second or every half second never drew a 429 (j40861).
- Cause: the shared keyless `ncbi-eutils` pacing, evenly spaced at exactly three per second, lets NCBI count a fourth request inside a second. A read's 17 sequential ClinVar requests drew 2 to 5 429s at that rate, each costing 2 s, and a request that met three in a row failed the section. With only the throttle wait below, 2 of 5 reads still failed at three per second (j40858). Paced at five per two seconds, 5 of 5 completed in about 7.3 s with no 429; at two per second, 5 of 5 in about 8.9 s (j40861).
- Changed: the keyless `ncbi-eutils` default and ceiling is now five requests per two seconds; the keyed rate stays ten per second. The ClinVar gene section's einfo, esearch and esummary endpoints declare `wait_throttle`: a 429 that names its retry time is waited out and retried for as long as the budget allows rather than three attempts, because other clients on the same address share the keyless limit. A wait longer than the remaining budget still returns `rate-limited` with the retry time.
- Case: `tests/cases/gene/sections/clinvar-throttled.yaml` replays a keyless BRCA1 read recorded live in j40858 while a second client sent searches from the same address: NCBI refused the likely pathogenic count with `Retry-After: 2`, the section waited, and the read completed with 4,752 P/LP. The recorded 429 body's IP address is replaced by `2001:db8::1`.
