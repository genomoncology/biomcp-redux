# 0890 — rare-disease-small-joins

Status: OPEN.

Milestone: 7

## Outcome

Three small joins the rare-disease agent spent calls on work in one: disease hierarchy rows carry parent labels; disease synonyms are not repeated; a `clinvar:` variant's population section uses the allele it already resolved.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`): `get disease MONDO:0016512 hierarchy` gave bare parent IDs and the agent made 3 more calls to name them (log.md:212-218); synonyms came back 7 with repeats (log.md:210); `get variant clinvar:5020581 population` said `absent` while the same allele by ClinGen allele ID had gnomAD counts (log.md:355).

## Proof

- Proof: recorded cases for each: parent labels from the recorded source reply, distinct synonyms, and the clinvar ID's population matching its allele ID's.

## Starts from

- Starts from: `crates/biomcp-adapters/src/mydisease/`, `crates/biomcp-query/src/variant/`.

## Keeps

- Keeps: source attribution per row.

## Defers

- Defers: transitive HPO ancestors.

## Changes

- Changes: `crates/biomcp-adapters/src/mydisease/`, `crates/biomcp-query/`, `tests/cases/`, `docs/`
