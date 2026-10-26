# 0255 — Diagnose the WikiPathways concurrent catalog replacement failure

Status: OPEN.

Milestone: 1

## Outcome

A bounded diagnosis names the cause of the empty failure in `wikipathways/concurrent-readers-during-pair-replacement` from batch gate j14991, or states that the retained evidence cannot name it and what one added witness would. A witnessed product defect or test-isolation defect gets a fix in this ticket with a case that fails without it.

## Why

The batch gate failed with no message, and a passing diagnostic rerun (j14997) does not establish a cause.

## Proof

- Proof: the j14991 failed witness and the j14997 diagnostic with retained status and stdout are both cited. If a cause is found, a deterministic case that reproduces it fails before the fix and passes after it. If no cause is found, the case keeps its added status and stdout retention so the next failure carries its witness, and the ticket records that limit.

## Starts from

- Starts from: `concurrent()` in `crates/biomcp/tests/runner/wikipathways.rs`, the 0222 catalog pair replacement, the j14991 batch profile kept by the lander, and j14997.
- PM ruling 2026-10-26: requested by the coordinator in m7046 as the only unresolved test-isolation defect from the phenotype batch.

## Keeps

- Keeps: atomic catalog pair replacement, reader behavior during replacement and the existing case's assertions. No blind retry loop, timeout inflation or new framework.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/tests/runner/wikipathways.rs`, and the WikiPathways store code only if a product defect is witnessed.
