# 1207 — recurrence-counts-chrx-as-zero

Status: OPEN.

Milestone: 7

## Outcome

`variant recurrence` counts samples for a variant on chromosome X or Y or the mitochondrial genome as it does for an autosome, and when any study row could not be compared with the requested allele the reply is not `complete`.

## Why

cBioPortal writes chromosome X as `23`; today every chrX variant gets `observed_sample_count: 0` under `status: complete`, which reads as "never seen".

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp variant recurrence 'NC_000023.10:g.66943552A>G' --source cbioportal --gene AR --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --build GRCh37` returns `status: complete`, `observed_sample_count: 0`, `all_pages_read: true`. cBioPortal's mutations for that profile and gene hold 302 rows; six, in six distinct samples, are at position 66943552 A>G with `chr: "23"`.
- Cause, read from code: the row's chromosome fails to parse, the row is treated as "could not compare", such rows collapse into one de-duplicated `mutation.exact_allele value.unknown` problem, and that problem never changes the status.
- Control: BRAF V600E on the same study returns 295 samples. Every recurrence fixture uses chromosome 7.

## Proof

- Proof: an outside-in case on the recorded AR reply returns six samples; a case with a deliberately unreadable row returns `partial` and a count of rows that could not be compared.

## Starts from

- Starts from: `crates/biomcp-query/src/variant/recurrence/matching.rs`, `crates/biomcp-query/src/variant/recurrence.rs`.

## Keeps

- Keeps: autosomal counts and the exact-allele rule; no allele is matched by position alone.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/variant/recurrence/`, `crates/biomcp-query/src/variant/recurrence.rs`, recurrence cases and replies.
