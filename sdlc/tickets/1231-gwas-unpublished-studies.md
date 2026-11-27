# 1231 — gwas-unpublished-studies

Status: OPEN.

Milestone: 10

## Outcome

`get gwas <GCST>` and `search gwas --study <GCST>` return the catalog's record for an unpublished study (its traits, sample description and summary-statistics location), labelled as unpublished.

## Why

42,583 of the catalog's studies (17.2% of all study records) are unpublished, and each reads as not found today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get gwas GCST91007236` returns `status: absent`, `not-found`; `biomcp search gwas --study GCST91007236` returns `complete` with 0 rows. The catalog serves the record at `/unpublished-studies/GCST91007236`, with 11,668,952 variants of summary statistics.
- In a sample of 200 published studies, 74.5% have no curated associations and 82% have summary statistics.

## Proof

- Proof: outside-in cases on the recorded catalog reply: both commands return GCST91007236 labelled unpublished with its summary-statistics link.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gwas/mod.rs` and the catalog's `unpublished-studies` endpoint.

## Keeps

- Keeps: published study replies are unchanged; an unpublished study is never shown with curated associations it does not have.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/gwas/`, `crates/biomcp-query/src/gwas/`, GWAS cases.
