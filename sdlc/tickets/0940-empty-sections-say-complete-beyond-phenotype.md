# 0940 — empty-sections-say-complete-beyond-phenotype

Status: OPEN.

Milestone: 7

## Outcome

An entity section asked for an identity the source holds but whose section is empty comes back `complete` with an empty list, not `absent`, for disease sections read from Monarch and pathway relation pages, as 0929 did for phenotypes. `absent` stays for an identity the source doesn't hold (`crates/biomcp-types/src/response/status.rs:8-9`).

## Why

0929's developer (2026-11-14) found the same mix-up outside phenotypes: `crates/biomcp-query/src/disease/monarch.rs:49` and `crates/biomcp-query/src/pathway/relation.rs:218`, `:240` call an empty page `absent`. `crates/biomcp-query/src/drug/who.rs:36` may be a third case, or may be how it decides whether the drug exists; check before changing it. An agent then can't tell "no rows" from "no such disease or pathway".

## Proof

- Proof: recorded cases for a real disease and a real pathway with an empty section show `complete` and an empty list; an unknown ID still shows `absent`.

## Starts from

- Starts from: 0929's change (`git log origin/main --grep 0929`), its use of a term lookup when the source answers an empty page the same way for both, and the lines above.

## Keeps

- Keeps: `partial` for refused rows; the 1.0.0 schema rules (status values unchanged).

## Defers

- Defers: other entities; list any further drift in Progress.

## Changes

- Changes: the disease, pathway (and if confirmed, WHO drug) section status code and cases.
