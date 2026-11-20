# 1097 — return-compact-trial-arms-for-an-intervention

Status: OPEN.

Milestone: 7

## Outcome

Trial search can return compact intervention-matched arm rows with the registry’s arm tag, label, intervention names and study purpose, and retain the descriptions needed to inspect a conflict.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get trial NCT06305754 NCT07895992 NCT05347134 NCT06132958 NCT06356311 NCT06670196 NCT06422143 NCT06074588 NCT06448312 NCT06459180 NCT06279364 NCT06312176 NCT07318558 NCT06952504 NCT06824467 NCT07419295 NCT06393374 NCT06711900 NCT07829783 NCT07216703 --section arms --fields=record,data.kind,data.label,data.intervention_names --limit 20` returned `"returned":7,"total":10` for NCT06952504. Two projected follow-up replies still totaled 70,044 bytes. A separate witness shows conflicting descriptions and tags in the registry itself.

## Proof

- Proof: Search a saved registry set for a named intervention. Check matched arms, original tags and labels, enough description to see a source conflict, and a smaller reply than the existing arm reads.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/detail/arms.rs` and `crates/biomcp-query/src/trial/get/page.rs`.

## Keeps

- Keeps: Full arm reads and the registry’s source wording stay available.

## Defers

- Defers: Judging which source tag is correct and changing pagination of existing arm reads.

## Changes

- Changes: Trial search and arm result rows, help and cases.
