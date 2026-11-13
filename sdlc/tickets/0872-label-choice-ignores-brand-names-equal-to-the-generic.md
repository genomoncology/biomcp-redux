# 0872 — label-choice-ignores-brand-names-equal-to-the-generic

Status: OPEN.

Milestone: 7

## Outcome

When a drug name is looked up by its generic name, a label whose "brand name" is just that generic name (a generic maker filing as "Metformin") does not count as an exact brand match, so the choice falls to 0861's tiers: brand-name NDA/BLA labels first, then by numbered indications and revision.

## Why

0866's developer found that `get drug metformin label` picks a generic maker's label (`05999192-...`, brand name "Metformin") because 0861's first rule puts an exact brand match ahead of everything, and many generic labels set the brand name to the generic name. That defeats 0861's purpose, which was to give the brand holder's current label.

## Proof

- Proof: 0866's recorded metformin case shows the chosen label moving to the top brand-name NDA label (for example Glucophage's, if openFDA holds it) or, if none, the best NDA/BLA label by 0861's tiers, with the reason line saying why.
- Proof: a real brand name typed by the user (`get drug Xarelto label`) still picks the Xarelto label first.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/label_choice.rs` (0861) and 0866's case `tests/cases/drug/label-many-names.yaml`.

## Keeps

- Keeps: 0861's other tiers, the reason line, totals and alternatives.

## Defers

- Defers: preferring the originator over repackagers (0861's review follow-up).

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/drug/`
