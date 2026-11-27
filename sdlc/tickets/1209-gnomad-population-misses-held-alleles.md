# 1209 — gnomad-population-misses-held-alleles

Status: OPEN.

Milestone: 10

## Outcome

`variant population` and `get variant ... population` return gnomAD's record for an allele gnomAD holds, whichever accepted form names it (VCF-style, rsID, `clinvar:N`, genomic or transcript HGVS), and say `unavailable` with a reason, never `absent`, when the form could not be turned into a gnomAD variant ID.

## Why

The same allele returns data by HGVS and `absent` by VCF-style ID today, because some forms are resolved through MyVariant.info's index, which lacks many gnomAD exome-only variants.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp variant population 17-7673174-A-T` returns `status: absent` (MyVariant `not-found`). `biomcp variant population 'NC_000017.11:g.7673174A>T'` returns gnomAD data for the same allele.
- Of 178 random gnomAD v4 SNVs from TP53, BRCA2 and EGFR, MyVariant's GRCh38 index lacked 30 (17%), all exome-only; six of six sampled misses behaved as above.
- `biomcp variant population rs80357906` and `biomcp variant population clinvar:17677` return `failed`, while BRCA1 `c.5266dup` by HGVS returns data.
- Ticket 0588 landed the route with "Defers: nothing".

## Proof

- Proof: outside-in cases on recorded replies: the VCF-style, rsID and ClinVar forms of one allele return the same gnomAD record as its HGVS form; a form that cannot be resolved returns `unavailable` with its reason.
- Proof: the gnomAD census in the reach census job draws a random sample of gnomAD variant IDs per release and holds the share returned as a floor.

## Starts from

- Starts from: `crates/biomcp-query/src/variant/population_identity.rs`, `crates/biomcp-query/src/variant/population.rs`, `crates/biomcp-adapters/src/gnomad/variant.rs`, `crates/biomcp-adapters/src/myvariant/population.rs`.

## Keeps

- Keeps: the exact-allele rule: no population record is returned for a neighbouring allele.

## Defers

- Defers: structural variants, mitochondrial variants and tandem repeats, which ticket 1221 covers.

## Changes

- Changes: `crates/biomcp-query/src/variant/population*.rs`, `crates/biomcp-adapters/src/gnomad/`, population cases and replies.
