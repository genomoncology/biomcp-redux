# 1050 — Explain confirmed gnomAD misses in JSON

Status: OPEN.

Milestone: 7

## Outcome

When gnomAD confirms that the requested variant has no record, the JSON population reply tells the reader which dataset and exact allele were checked and that there is no measured allele frequency.

## Why

In the rare-disease comparison the actor read the correct scoped record and still made a second trip to gnomAD because the JSON section said empty.

## Proof

- Proof: Replay `variant population rs121917919` and the variant card for ClinVar 68552. Both JSON replies explain the checked no-record result for `gnomad_r4` and `2-165994236-A-G`. A source failure, mismatched allele and a returned record with zero counts stay distinguishable.

## Starts from

- Starts from: The qualified record added in 1047, `variant/population.rs`, the existing JSON source notices and the existing confirmed-miss and wrong-allele fixtures.

## Keeps

- Keeps: The exact publisher result, any native counts, and the existing Markdown explanation.

## Defers

- Defers: Measure whether actors need fewer verification calls separately on the next build.

## Changes

- Changes: Variant population query, affected JSON cases and command documentation.
