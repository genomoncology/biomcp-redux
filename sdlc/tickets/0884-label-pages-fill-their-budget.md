# 0884 — label-pages-fill-their-budget

Status: OPEN.

Milestone: 7

## Outcome

A drug label section comes back in as few replies as its size allows: each reply fills its byte budget across openFDA's short pieces, and the budget is large enough that a typical indications section (Keytruda's included) is one call.

## Why

0882's measurements (its Progress, 2026-11-13): Keytruda's Indications and Usage is 23,683 bytes, Opdivo's 12,307, Zestril's 6,450, others 3-4 KB; a label section gets 8,192 bytes a reply in 6,144-byte fragments, and a reply returns one openFDA piece, so Zestril's indications take four calls although they would fit in one. The reg-affairs runs of 2026-11-13 showed the cost: BioMCP's agent read one fragment of Keytruda's indications and stopped, while the raw-tools agent read the whole section in one DailyMed call.

## Proof

- Proof: measure label section sizes across a spread of labels and record the share that fit in one reply at the chosen budget; routine cases show Zestril's indications in one reply and Keytruda's in one reply, with `remaining` absent.

## Starts from

- Starts from: 0882 (`remaining` before `text`), `crates/biomcp-adapters/src/openfda_label/page.rs`, `crates/biomcp-query/src/drug/label_bound.rs`, `sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`.
- PM decision: 2026-11-13. The label section reply budget may grow beyond half its current size, up to 32 KB of text, chosen from the measurement so most indications sections fit in one reply; the budget is a setting with that default. This ruling is the one the measured-caps decision asks for.

## Keeps

- Keeps: label text verbatim; paging and `remaining` for sections larger than the budget; other entities' budgets.

## Defers

- Defers: FDA PDF text pages (device summaries, approval letters), which 0882 also deferred.

## Changes

- Changes: `crates/biomcp-adapters/src/openfda_label/`, `crates/biomcp-query/src/drug/`, `crates/biomcp/src/` (setting), `tests/cases/drug/`, `docs/`
