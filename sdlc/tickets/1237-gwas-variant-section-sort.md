# 1237 — gwas-variant-section-sort

Status: COMPLETE.
Landed: 61672a365b965338aa52d5d44e080d1dc25d2a7c

Milestone: 10

## Outcome

`get variant <rsID> gwas` returns associations in the same order as `search gwas --variant <rsID>`, strongest p-value first, and the reply states the order.

## Why

The first page of the variant section today can omit the strongest associations, and the two routes disagree.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get variant rs2981582 gwas --section-limit 5` puts p = 1e-12 first and the strongest row (2e-76) fifth; `biomcp search gwas --variant rs2981582` sorts by p ascending. For rs7903146, with 353 rows, the section's first rows are 1e-11, 6e-50, 7e-15 while search starts at 3e-1315.
- Cause: the section's request sends no sort.

## Proof

- Proof: an outside-in case on recorded catalog replies: both routes return the same first five association IDs for rs2981582.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gwas/mod.rs`.

## Keeps

- Keeps: totals and row shapes are unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/gwas/`, variant cases.
