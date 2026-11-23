# 1160 — synthetic-posted-sap-fixture

Status: COMPLETE.
Landed: 6ff8a5aa89c5eb0431261fa16f7b201ecd1569c9 (with 1154)

Milestone: 7

## Outcome

Core's tests use a synthetic posted statistical analysis plan in place of the real PRESERVE trial PDF, and test the same behavior.

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review: ticket 1151 added an unchanged 508,539-byte PRESERVE statistical analysis plan (`tests/replies/trial/posted-sap/README.md`) to a public repo, with no ruling on redistributing it. Ian (via Jarvis, m40348): swap in a synthetic PDF with the same structure; "A public repo shouldn't carry a third-party document when a synthetic one tests the same thing."

## Proof

- Every case that read the PRESERVE PDF passes on a synthetic PDF built by a checked-in script, with the same page and text structure the cases assert.
- The PRESERVE file is gone from the tree, and no other third-party trial document remains in `tests/replies/trial/` (list any found).

## Changes

- `tests/replies/trial/posted-sap/`, the cases that use it, and a small generator script beside the other fixture generators.
