# 0960 — slow-schema-discovery-proofs-run-fast

Status: OPEN.

Milestone: 7

## Outcome

The two tool proofs 0944 keeps out of product-change landings, `schema-discovery/rejects-bad-cases` (264 s alone) and `schema-discovery/selects-and-lists-cases` (75 s alone), each run in under about 15 s with the same checks, and 0944's `SLOW` list in `scripts/changed_tool_proofs.py` is emptied, so every proof runs when product code changes.

## Why

0944 (2026-11-14): every tool proof runs product code, so a product change could break any of them; the landing gate runs them on product changes, about 21 s, except these two, which would add about 4.5 minutes to nearly every landing. Until they are fast, a product change that breaks them is caught only by `make full-suite` before a release.

## Proof

- Proof: timings for each proof alone, before and after, in a job (0944's j41018 is the before); the selector test in `scripts/test_gate_stages.py` shows a product change now selects them; the gate's added time on a typical `crates/biomcp/src` change, measured.

## Starts from

- Starts from: 0944's ticket and `SLOW`; the two proofs in `crates/biomcp/tests/runner/` (0944's developer: rejects-bad-cases starts the runner once per bad case, a likely cost).

## Keeps

- Keeps: what each proof checks; the proof names.

## Defers

- Defers: nothing.

## Changes

- Changes: the two proofs and the runner code they use, `scripts/changed_tool_proofs.py`, its test.
