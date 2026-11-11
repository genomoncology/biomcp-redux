# 0830 — drug-names-widen-by-default-everywhere

Status: OPEN.

Milestone: 8

## Outcome

Every search that takes a drug name widens it to the drug's known names by default, says in the reply which names it searched and which it could not, and searches the literal text only when given `--exact-term`. This covers FAERS, openFDA labels, Drugs@FDA, EMA, drug recalls, MAUDE and VAERS, as trial and article search already do.

## Why

The same drug gives different counts by spelling: the persona run reported 292 fatal reports for the generic name, 414 for the brand and 703 for both. Trial and article search already widen by default, so the difference between commands is itself a trap.

## Proof

- Proof: for each source above, a routine case with a recorded reply shows a generic-name search returning the union with the searched names listed, and `--exact-term` returning the literal count; continuation pages keep the first page's names.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs` and `aliases.rs`; the opt-in `--expand-drug-names` paths landed by 0812, 0815 and 0816; trial search's default widening as the pattern.
- PM decision: 2026-11-11. This overrides the "explicit opt-in only, literal default" line in 0812, 0815 and 0816. One rule for every command beats a safer default that differs by command; `--expand-drug-names` stays accepted as a no-op for one release.
- PM decision: 2026-11-11. This ticket gets a design review before hand-over, because it changes default results. Reproduce the three counts on the base commit first and put the real numbers here.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. Bounded name lists, frozen across pages, native totals never summed per name, resolver failure reported and not hidden, source exclusions honored.

## Defers

- Defers: phrase and spelling variants for conditions and protein changes are a separate ticket in the composite batch; NCI trial search stays literal with its notice.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-adapters/src/openfda_faers/`, `crates/biomcp-adapters/src/openfda_label/`, `crates/biomcp-adapters/src/drugsfda/`, `tests/cases/drug/`, `tests/cases/adverse-event/`
