# 1116 — name-the-hpo-limit-in-phenotype-get-refusals

Status: COMPLETE.
Landed: 6e45a15093b8094994d0f709cc1744b93339762d

Milestone: 7

## Outcome

A phenotype get refusal says that HPO serves term records and which source can serve the requested section, and provides a copyable supported call.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get phenotype HP:0001083 diseases --source hpo` returned `a selected source does not support the requested filters`. `biomcp get phenotype HP:0001083 hierarchy --source hpo` and `biomcp get phenotype HP:0001083 genes --source hpo` give the same message for a section read.

## Proof

- Proof: Replay the three section requests. Each names the input source and section and offers an explicit Monarch call. Check that the corrected call reaches the right section.

## Starts from

- Starts from: `crates/biomcp-query/src/phenotype/sections.rs` and phenotype get source checks.

## Keeps

- Keeps: HPO term reads and Monarch association reads retain their current scope.

## Defers

- Defers: Release-pinned HPO intersections have a separate ticket.

## Changes

- Changes: Phenotype get validation, guidance and cases.
