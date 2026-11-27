# 1211 — opentargets-drug-without-mechanism-fails

Status: OPEN.

Milestone: 7

## Outcome

`get drug <drug> indications --source opentargets` and the other Open Targets sections return what Open Targets holds for a drug that has no mechanism-of-action rows, with `targets` reported as `empty`.

## Why

Open Targets sections fail outright today for any drug without a mechanism, and `search drug` lists those drugs, so search leads to a failing get.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get drug CHEMBL3301579 indications --source opentargets` returns `status: failed`, source reason `bad-reply`, problem `record value.unknown`. Open Targets holds an approved indication for that drug.
- Fourteen of 26 sampled drugs that Open Targets holds failed this way.

## Proof

- Proof: an outside-in case on the recorded Open Targets reply for CHEMBL3301579 returns its indication rows with status `complete`; `targets` for the same drug is `empty`, not `failed`.

## Starts from

- Starts from: `crates/biomcp-adapters/src/opentargets/raw.rs`, `crates/biomcp-adapters/src/opentargets/map.rs`.

## Keeps

- Keeps: drugs with mechanisms return the same rows as today.

## Defers

- Defers: Open Targets pharmacovigilance and tractability, which have no route; file when a case needs them.

## Changes

- Changes: `crates/biomcp-adapters/src/opentargets/`, drug cases and replies.
