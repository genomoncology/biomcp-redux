# 0095 — Show OncoKB variant annotation with the user's key

Status: COMPLETE.
Landed: b62e916cdc94ce80375ef1a880071087d25eaac2

Milestone: 2

Depends on: 0087
Depends on: 0077

## Outcome

The `oncokb` section returns OncoKB's variant annotation with explicit build and tumor type only when the user's key is configured, and is skipped with the key step and no request otherwise.

- Starts from: variant.md (OncoKB row); the rulings; 0032's keyed OncoKB gene pattern; key registration from 0077.
- Risks the proof must cover: no request without the key, including public endpoints; the key never enters output or recordings; no actionability inferred beyond OncoKB's own fields.
- PM ruling 2026-10-24 (m4600): oncogenicity and mutation effect for the queried alteration become Statements; treatment, diagnostic and prognostic implications whose alteration groups have no source profile record stay native rows with group, drugs, tumor type, level, citations and provenance and `statement_unmapped: true`. No invented profile ID, no split group claim, no query echo used as a subject.
- Proof: no-key case with the key step; synthetic keyed cases started from a real keyed capture made once through pm job with the inherited key, which the agent never inspects, prints, copies or stores; 401, 429 and the failure set; MCP parity. A missing keyed capture becomes a follow-up that gates the roadmap box and never holds landing. Then make landing-gate runs through pm job after current main.
