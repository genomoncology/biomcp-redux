# 1222 — fda-biologics-center-products-read-absent

Status: OPEN.

Milestone: 10

## Outcome

For a product FDA's biologics center regulates (a vaccine, a cell or gene therapy, a blood product), `get drug <name> approvals`, `markets` and `label` say that Drugs@FDA does not cover it and point to the route that does hold it, instead of `absent` or `no-record`.

## Why

An FDA-licensed CAR-T therapy reads today as having no United States record.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp get drug Kymriah approvals` returns `status: absent` with Drugs@FDA `ok` and no notice. `biomcp get drug Yescarta markets` returns `complete` with the United States row `no-record`.
- Failing case 2: `biomcp get drug Kymriah label --label-section indications_and_usage` returns `absent`, yet the label is reachable by its set ID (`aad3ba54-dfd3-4cb3-9e2b-c5ef89559189`). The Shingrix lookup returns `absent`.
- The Purple Book source holds these licences; ticket 1223 makes its search find them.

## Proof

- Proof: outside-in cases on recorded replies: Kymriah `approvals` returns a notice naming the coverage limit and a `next` step to the Purple Book record; the `markets` United States row says not covered by Drugs@FDA; the name-to-label step finds the Kymriah label.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/markets/read.rs`, the approvals section in `crates/biomcp-query/src/drug/`, `crates/biomcp-adapters/src/drugsfda/`, `crates/biomcp-adapters/src/openfda_label/`.

## Keeps

- Keeps: products Drugs@FDA holds are unchanged; nothing is fetched from an FDA site outside the product's own source adapters.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-adapters/src/openfda_label/`, drug cases and replies.
