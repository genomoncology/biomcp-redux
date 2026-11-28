# 1243 — clinvar-records-by-gene

Status: OPEN.

Milestone: 10

Depends on: 1232

## Outcome

`search variant --gene <symbol> --source clinvar` lists the gene's ClinVar variation records, paged, with optional `--classification` and `--review-status` filters, and each row carries its VCV accession so `get variant` can open it.

## Why

1232 deferred this list route until the PM chose its shape. PM ruling 2026-11-28: it is a source option on the existing `search variant` command, not a new command, because `search variant` is already the variant list route and other commands pick a source with `--source`. MyVariant stays the default source. Large copy-number and range records (about 1.9% of a sample of 1,123) have no small-variant HGVS spelling, so a gene listing is the only way to find them.

## Evidence

- [Source reach census](../records/2026-11-27-source-reach-census.md): `search variant` reads MyVariant only, and `get gene <symbol> clinvar` gives counts only, so no route lists a gene's ClinVar records.

## Proof

- Proof: outside-in cases on recorded ClinVar replies: a gene listing returns records including a range variant with no HGVS spelling, a classification filter narrows the list, and the listed total matches ClinVar's count for that query.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clinvar/`, `search variant` in `crates/biomcp-query/src/variant/`.

## Keeps

- Keeps: `search variant` without `--source` is unchanged.

## Changes

- Changes: the ClinVar adapter, `search variant` grammar and help, cases.
