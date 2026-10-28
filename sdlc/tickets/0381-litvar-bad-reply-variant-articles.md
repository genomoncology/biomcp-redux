# 0381 — Fix the LitVar bad reply for NAA10 E100K and match 0.9.3 variant article recall

Status: OPEN.

Milestone: 5

Depends on: 0388

## Outcome

The cause of LitVar returning `bad_reply` for `search article --gene NAA10 --variant E100K --source litvar` is found and fixed. Variant article search for NAA10 E100K, by gene and protein change and by its rsID rs2065171629, accounts for every PMID that 0.9.3's `variant articles rs2065171629 --limit 25` returned: each is found, reached by a next step, or recorded as not mentioning the variant.

- Why: regression against 0.9.3 found by experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only. 1.0 PubMed with gene and variant found only the original paper. Ian's ruling that 1.0 ships every known fix.
- Starts from: the LitVar adapter's reply admission, 0334's PubMed retries, 0335's recovery steps and 0.9.3's variant article path in the parity inventory.
- Rules: at most three bounded live requests per attempt, no key. No guessed aliases or rsIDs.
- Proof: a replay case of the LitVar reply that failed now maps or names its real problem; cases for NAA10 E100K by gene and variant and by rs2065171629 show the accounting for the witness set below.

## Evidence

Experiment 499, public PMIDs only. 0.9.3 `variant articles rs2065171629 --limit 25` returned 29558889, 30842225, 31174490, 32698785, 32973342, 33255974, 33335012, 35039925, 38574881, 38713657, 40204117, 40233338, 40304357, 40475252, 40558489, 41384780, 41548501, 41973310, 42165278, 42262012, 42587014, 42749089, 42749183. 1.0 at ff2e9b64 `search article --gene NAA10 --variant E100K --limit 25` returned only 33335012; with `--source pubtator,europepmc` it returned 33335012, 35039925 and 41973310. The 1.0 LitVar reply was `failed` with `bad_reply` with and without `--fresh`; the raw LitVar body was not preserved, only BioMCP's output. 0.9.3 ranked weak PubMed rows into its list, so some of its PMIDs may not mention the variant; the accounting above decides each.
- Changes: the LitVar adapter, article search and cases.
- Defers: nothing.
