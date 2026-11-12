# 0866 — drug-name-label-lookup-too-large

Status: OPEN.

Milestone: 7

## Outcome

`get drug metformin label`, and any other drug name with many openFDA labels, returns a label instead of failing as too large while the name lookup reads candidate labels.

## Why

0861's developer found that `get drug metformin label` exits 4 with the openfda-label source `state: failed`, `reason: too-large` on main (j38713) and on 0861's branch (j38722). The failure happens while the name lookup reads 50-row pages of full labels, before any label is chosen. Metformin is one of the most prescribed drugs, so a label read by its name must work. 0864 found the same cause in device recalls: a fixed reply limit smaller than real openFDA pages.

## Proof

- Proof: first measure, live, the size of the name lookup's label pages for metformin and a few other many-label generics (for example atorvastatin, lisinopril, amlodipine), and record which limit fails and by how much.
- Proof: a routine case with recorded replies reads a drug label by a many-label generic name and returns the chosen label with 0861's reason line; the same case fails on main.

## Starts from

- Starts from: 0861's name lookup and label choice (`crates/biomcp-query/src/drug/label_choice.rs`, `resolve.rs`), the openFDA label adapter's reply limit, and 0864's measured-limit pattern (`crates/biomcp-adapters/src/fda_device_recall/mod.rs`).
- PM decision: 2026-11-12. Prefer the simplest fix the measurement supports: ask openFDA for only the fields the lookup needs, or use smaller pages, before raising a limit. Raise a limit only with measured numbers in a comment, as 0864 did.

## Keeps

- Keeps: 0861's choice rule and reason line; the reported total and alternatives; label-ID reads unchanged; a page that is truly too large still fails visibly.

## Defers

- Defers: preferring the originator over repackagers (0861 review follow-up, its own ticket if a run shows it matters).

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-adapters/src/` (openFDA label), `tests/cases/drug/`
