# 0253 — Keep null association evidence absent instead of an empty list

Status: COMPLETE.
Landed: 6b2f1774068ee91143e24173f3a34872c47216c1

Milestone: 4

Depends on: 0240

## Outcome

`DiseaseAssociation.evidence` is absent when Monarch sends no evidence or a null value, and it is an empty list only when Monarch sends an explicit empty list. Disease and phenotype association rows from Monarch show the same presence rule, and other sources that fill the field keep their current values.

## Why

The clinical reviewer accepted that null evidence means unavailable and never means no evidence, and today the required `evidence` array prints `[]` for a null reply, which reads as no evidence.

## Proof

- Proof: the existing 0240 synthetic direct-pages case, which prints `evidence: []` today, asserts the field is absent. A synthetic Monarch row with an explicit empty evidence list asserts `[]`, and a row with evidence terms asserts them unchanged. The affected focused disease Monarch cases assert absence where their retained replies carry null evidence. Versioned schema and contract tests cover the changed field. No source calls.

## Starts from

- Starts from: `crates/biodata/src/disease.rs` `DiseaseAssociation.evidence`, Monarch reference mapping in `crates/biomcp-adapters/src/monarch/`, `sdlc/planning/slices/phenotype.md` on null evidence and negation, and the 0240 reviewer finding in m6971.
- PM ruling 2026-10-26: use a presence-preserving field, following `negated` and `aggregator_knowledge_source`. Wording alone does not satisfy the clinical ruling.

## Keeps

- Keeps: `negated` omitted when null and explicit false kept, `evidence_count` separate from returned references, publications unchanged, and no evidence moved between rows.

## Defers

- Defers: nothing. Other association fields with the same null versus empty question become their own tickets when a reviewer finds them.

## Changes

- Changes: `crates/biodata/src/disease.rs`, the Monarch mapping, `crates/biomcp-types` schemas and contract tests, and the affected disease and phenotype focused cases.
