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
