# 0381 — Fix the LitVar bad reply for NAA10 E100K and match 0.9.3 variant article recall

Status: OPEN.

Milestone: 5

## Outcome

The cause of LitVar returning `bad_reply` for NAA10 E100K is found and fixed. Variant article search with `--gene NAA10 --variant E100K` finds at least the papers 0.9.3's variant articles found for the same input, or the reply's next steps reach them.

- Why: regression against 0.9.3 found by experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only. 1.0 PubMed with gene and variant found only the original paper. Ian's ruling that 1.0 ships every known fix.
- Starts from: the LitVar adapter's reply admission, 0334's PubMed retries, 0335's recovery steps and 0.9.3's variant article path in the parity inventory.
- Rules: at most three bounded live requests per attempt, no key. No guessed aliases or rsIDs.
- Proof: a replay case of the LitVar reply that failed now maps or names its real problem; a case for NAA10 E100K returns or reaches 0.9.3's papers, listed by PMID in the ticket.
- Changes: the LitVar adapter, article search and cases.
- Defers: nothing.
