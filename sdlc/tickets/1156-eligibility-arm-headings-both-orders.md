# 1156 — eligibility-arm-headings-both-orders

Status: COMPLETE.
Landed: 6ff8a5aa89c5eb0431261fa16f7b201ecd1569c9 (with 1154)

Milestone: 7

## Outcome

Trial eligibility keeps each arm's criteria under its own arm whether a heading reads "Alpha Arm" or "Arm B", and where an arm boundary is uncertain it returns the registry's raw text instead of a split.

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review (Astra, snapshot cf9a822889ce), reproduced by the reviewer under replay. In the `NCT04937062` eligibility case with headings `# Alpha Arm` and `# Arm B`, the reply held only `Alpha Arm`, its inclusion list held both arms' items, and its exclusion list held the literal `# Arm B` heading. The parser recognizes headings ending in "arm" only (`crates/biomcp-adapters/src/ctgov/criteria.rs:93`). The owner's contract is `sdlc/decisions/2026-10-27-eligibility-text-is-not-patient-matching.md`.

## Proof

- A replay with both heading orders returns two arms, each with only its own items.
- An unrecognized heading inside a list falls back to the raw text, never to a list holding a heading.

## Changes

- `crates/biomcp-adapters/src/ctgov/criteria.rs` and the trial eligibility cases.
