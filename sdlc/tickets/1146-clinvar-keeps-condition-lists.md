# 1146 — clinvar-keeps-condition-lists

Status: COMPLETE.
Landed: 4916c66a1f2dcc606eae5d1564a03e28c3097bfc

Milestone: 7

## Outcome

A ClinVar variant read returns every condition a submitted record names, including records that name several, so no RCV appears without its conditions.

## Why

Found 2026-11-22 by scouts exploring BioMCP for multi-entity questions (about 760 CLI calls on build b7def4651). `biomcp get variant clinvar:546089 clinvar --limit 50` returned RCV000763038.11 and RCV007091897.2 with no `statement.condition`, while ClinVar's own VCV000546089.47 XML lists three and five `ClassifiedCondition` values for them. ATP1A3 RCV000415180.10 and RCV000763431.10 showed the same. The rows are counted but their conditions are missing, which reads as a record about nothing.

## Changes

- The ClinVar adapter keeps every `ClassifiedCondition` of a multi-condition record in the reply's condition list, with its identifiers.
- Records whose conditions truly are absent are marked so, not left empty.

## Proof

- Fixture from the VCV XML for a variant with multi-condition RCVs: each RCV's conditions match the XML, in CLI and typed MCP.
