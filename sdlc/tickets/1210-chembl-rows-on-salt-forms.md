# 1210 — chembl-rows-on-salt-forms

Status: OPEN.

Milestone: 10

## Outcome

`get drug <drug> targets --source chembl` and `indications --source chembl` return the ChEMBL mechanisms and indications recorded on any form of the drug (parent, salt or other child), each row stating the form it came from.

## Why

ChEMBL records many mechanisms on the salt; a lookup by the parent's ID reads as "no targets" today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get drug CHEMBL941 targets --source chembl` (imatinib) returns `status: absent`, `targets.state: empty`, source `ok`. ChEMBL holds four mechanisms for imatinib on the salt record CHEMBL1642. `biomcp get drug montelukast targets --source chembl` is `absent`; `"montelukast sodium"` returns one row.
- `biomcp get drug imatinib indications --source chembl` returns `complete` with 52 rows; ChEMBL holds 134 across parent and salt.
- Across ChEMBL's mechanism table, 1,626 of 7,561 rows (21.5%) sit on a non-parent record, and 862 of 5,594 parent drugs (15.4%) have mechanisms only on a salt or child.
- Cause: the adapter reads only the exact ChEMBL ID it was given.

## Proof

- Proof: outside-in cases on recorded replies: CHEMBL941 targets returns the four mechanisms with their source molecule; imatinib indications returns the rows of both forms without duplicates.
- Proof: a weekly census job reads ChEMBL's mechanism table (about 70 pages, roughly three minutes) and asserts that no parent with mechanisms on a child reads as `absent`.

## Starts from

- Starts from: `crates/biomcp-adapters/src/chembl/mod.rs` and ChEMBL's `molecule_form` and `molecule_hierarchy` data.

## Keeps

- Keeps: rows already returned keep their shape; a drug with no mechanism on any form stays `empty`.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/chembl/`, drug cases and replies.
