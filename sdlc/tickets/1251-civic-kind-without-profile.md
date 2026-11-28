# 1251 — civic-kind-without-profile

Status: OPEN.

Milestone: 10

## Outcome

`civic_kind` (CLI `--civic-kind`) works without `civic_profile` when the input already names one CIViC profile, by profile ID or exact profile name. A `civic_profile` that does not belong to the input is an error naming both, never a complete empty reply. No next step or error offers a literal `civic_profile: 1` or any other example value.

## Why

Found in a run of all 100 items of the public CIViC MCP benchmark (Schimmelpfennig et al., Bioinformatics Advances 2026, vbag209) through BioMCP MCP tools only, GPT 6 Luna, 2026-11-28: 88 of 126 positive labels right. A reading of the 31 missed items found BioMCP usability faults behind 27 of them; reach is complete. About 22 of the 27 trace to this: the reply to `civic_kind` without a profile offered `civic_profile: 1` as the next step, models copied it, the call queried profile 1 (BCR::ABL1) instead of the input, and BioMCP answered `complete` and empty, which the model read as No Evidence.

## Evidence

- Benchmark items k075 (SRSF2 MUTATION, Acute Myeloid Leukemia, no therapy) and k013 (BRCA1 M1775K, no disease or therapy): the evidence-kind call with the copied `civic_profile: 1` returned `complete` and no rows.

## Proof

- Proof: outside-in cases on recorded CIViC replies: SRSF2 MUTATION with an evidence kind and no profile returns that profile's evidence; a profile ID that does not belong to the input is an error naming both; the reply to a call missing a needed profile offers the input's real profile ID or a call that needs none.

## Starts from

- Starts from: `crates/biomcp-adapters/src/civic/variant.rs` and `drug_map.rs`, the `civic_kind` and `civic_profile` grammar and their next steps.

## Keeps

- Keeps: calls that pass a matching `civic_profile` unchanged.
