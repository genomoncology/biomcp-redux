# 1215 — wikipathways-genes-lost

Status: COMPLETE.
Landed: 3c493b1a4d075e4dc956e27370bbcbf26a088f2a

Milestone: 10

## Outcome

`pathway genes <WP id>` and `get pathway <WP id> genes` return the gene nodes WikiPathways lists for the pathway, including nodes identified by UniProt, RefSeq and other namespaces the source itself maps to genes; a reply that left gene nodes unmapped is `partial` and states the count; and a pathway file without a `Data-Source` attribute is read.

## Why

Ninety-four human pathways lose every gene today under `status: complete`, and six valid pathways read as a source failure.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp pathway genes WP4324` returns `status: complete`, `total: 0`, `gene_count.unmapped: 297`. WikiPathways' own gene file lists 62 genes for it. WP5098 returns 0 of 90 and WP5485 0 of 25.
- By the namespaces in the pathway files (not run per pathway), 57% of gene-bearing human pathways lose at least one node and 94 lose all.
- Failing case 2: `biomcp get pathway WP5639 genes` returns the section `unavailable`, reason `bad-reply`; the pathway has 28 gene nodes. The raw reader requires a `Data-Source` attribute that six current pathways lack. WP5423, which has no gene nodes, also fails where it should be `empty`.
- The pathway cases use well-known pathways such as WP382.

## Proof

- Proof: outside-in cases on recorded pathway files: WP4324 returns the 62 genes of the source's gene file; WP5639 returns its 28; a pathway with unmappable nodes is `partial` with the count.
- Proof: the WikiPathways census in the landing gate compares, for every human pathway in a pinned release, the genes returned with the release's own gene file and holds the share as a floor.

## Starts from

- Starts from: `crates/biomcp-adapters/src/wikipathways/gpml_raw.rs`, the node-to-gene mapping in `crates/biomcp-adapters/src/wikipathways/`, and WikiPathways' GMT gene file.

## Keeps

- Keeps: nodes mapped today keep their gene; a metabolite or nested-pathway node is never reported as a gene.

## Defers

- Defers: rows for metabolite (13,247), nested-pathway (3,279) and complex (236) nodes, and a gene-to-WikiPathways index (ticket 0224 defers it).

## Changes

- Changes: `crates/biomcp-adapters/src/wikipathways/`, pathway cases and replies.
