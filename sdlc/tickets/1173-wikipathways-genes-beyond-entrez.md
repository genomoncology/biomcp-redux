# 1173 — wikipathways-genes-beyond-entrez

Status: COMPLETE.
Landed: 12ca09aa0ac9775f24e9504611d393fd46efd109

Milestone: 7

## Outcome

`biomcp pathway genes WP437` lists the pathway's genes, not only the data nodes that carry an Entrez Gene reference. Gene products referenced by Ensembl, UniProt or HGNC map to genes too, and the reply's counts agree with its items.

## Why

On 1.0.0-87f56ceabae5, `pathway genes WP437` (EGF/EGFR signaling) returned `total: 2` (NOS3, COX2) while the same reply reports `physical_entities: 195`. EGFR itself is not listed. Found by the docs writers on 2026-11-24.

## Proof

- Proof: a replay of WP437 lists EGFR and the pathway's other gene products with their HGNC IDs; the total matches the distinct genes; a node with no mappable reference is counted and named as unmapped, not dropped silently.

## Changes

- Changes: the WikiPathways gene extraction and a replay case.
