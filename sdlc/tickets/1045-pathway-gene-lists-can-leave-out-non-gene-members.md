# 1045 — pathway-gene-lists-can-leave-out-non-gene-members

Status: OPEN.

Milestone: 7

## Outcome

`get pathway R-HSA-… genes` can return gene products only: an option keeps the rows whose reference is a gene product (Reactome's UniProt, and Ensembl or NCBI Gene where present) and leaves out small molecules (ChEBI) and other non-gene references. The reply says how many rows it left out and of which namespaces, and its total counts the kept rows. Without the option the reply is unchanged.

## Why

The comp-bio persona built gene sets from pathway "genes" replies in its 2026-11-10 and 2026-11-12 runs, and small molecules came along: 6 of 75 rows in one pathway were ChEBI compounds (biomed-agent-bench `personas/rollup.md`, comp-bio; `personas/comp-bio/runs/2026-11-12/log.md:102`; `records/2026-11-12-persona-review-findings.md`, Waiting). A gene set with compounds in it gives wrong enrichment and overlap answers. Ian's `decisions/2026-10-27-1-0-ships-every-known-fix.md` puts every known gap in 1.0.

## Proof

- Proof: a replayed case reads the genes of a Reactome pathway with ChEBI participants using the option and expects no ChEBI row, the left-out count by namespace and a total that matches the kept rows. It fails on the base commit.

## Starts from

- Starts from: `map_members` in `crates/biomcp-adapters/src/reactome/relations.rs`, which already splits each reference's `st_id` into namespace and identifier, and `PathwayGenesQuery` in `crates/biomcp-types/src/query/pathway/relation.rs`. WikiPathways already reads GeneProduct nodes only; check that and say so in the help.

## Keeps

- Keeps: the default genes reply, its paging and pool, and the details layer.

## Defers

- Defers: nothing.

## Changes

- Changes: the pathway genes query and Reactome mapping, a case and its recording, help and one release-notes line.
