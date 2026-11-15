# 0960 — slow-schema-discovery-proofs-run-fast

Status: COMPLETE.
Landed: fd4dfb9c6ddbfe82d3b1abc5b35ae12902fefd5b

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

## Progress

- Measured where the time went (j41155, main at 75f65e4f6): `rejects-bad-cases` 282 s and `selects-and-lists-cases` 82 s alone. Each starts the `tools` runner once per probe case (about 58 and 13 starts), and every start took about 4.6 s, even `tools --proofs --list`, while the product call it checks took 0.04 s. The start-up cost was the runner compiling a JSON Schema validator for every embedded schema (`Registry::embedded()` in `discover` and `tool_trials`).
- Changed: the embedded registry checks each schema's identity at start-up and compiles each validator the first time a reply needs it (`crates/biomcp/tests/runner/schema.rs`). `Registry::new`, used by proofs with small fixture registries, still compiles everything at once, so an unregistered reference still fails at construction. Because nothing else proved every embedded schema compiles, `schema-discovery/resolves-only-registered-schema-references` now compiles all of them. The proofs' checks and names are unchanged.
- After (j41166): runner start-up 0.11 s (j41161); `rejects-bad-cases` 11.0 s, `selects-and-lists-cases` 3.9 s, `resolves-only-registered-schema-references` 4.6 s, each alone with cargo start-up.
- `SLOW` is gone from `scripts/changed_tool_proofs.py`: product code now selects every proof, like runner code. `scripts/test_gate_stages.py` shows `crates/biomcp/src/main.rs` and each other product path now running the whole catalog with no `--skip`. `docs/development.md` says the same.
- Gate time on a typical `crates/biomcp/src` change: the whole catalog took 15.3 s after the build (j41166), against 21 s for 0944's selection without the two slow proofs (j41030), so the change adds no time and catches what those two proofs check.
