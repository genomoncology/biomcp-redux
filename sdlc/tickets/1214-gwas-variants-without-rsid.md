# 1214 — gwas-variants-without-rsid

Status: COMPLETE.
Landed: b7c7b9f451fe7f0a6e5c44d109fb618713b9b849

Milestone: 10

## Outcome

GWAS association rows whose variant is not an rsID (a `chr:pos` identifier or an HLA allele name) return the variant identifier the catalog gives, with its type stated, on `get gwas` and `search gwas`.

## Why

An estimated 18.6% of the catalog's 1,206,717 associations name their variant by position; today those rows come back with no variant at all.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get gwas association:117751220` returns `status: complete` with problem `snp_allele.rs_id value.unknown` and no alleles. The catalog holds `chr4:6965877` for it.
- `biomcp search gwas --trait "Alzheimer disease" --limit 5` returns `partial`; its top row (p 9e-734, `chr19:44908684`) has no variant.
- The 18.6% share is from a random sample of 14,000 associations (1.16%) of data release 2026-11-17.

## Proof

- Proof: outside-in cases on recorded replies: association 117751220 returns `chr4:6965877` labelled as a positional identifier with no problem; the Alzheimer search page returns `complete`.
- Proof: the GWAS census, a scheduled job, samples 300 associations per release and holds the share returned with a variant identifier as a floor.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gwas/map.rs`.

## Keeps

- Keeps: rsID rows are unchanged; a positional identifier is never presented as an rsID or joined to other sources.

## Defers

- Defers: labelling multi-SNP haplotype and SNP-by-SNP interaction associations (about 0.03% and 0.3% of the sample) on the association row; today only the `loci` section says so.

## Changes

- Changes: `crates/biomcp-adapters/src/gwas/`, schemas, GWAS cases and replies.
