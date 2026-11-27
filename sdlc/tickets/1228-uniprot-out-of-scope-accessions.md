# 1228 — uniprot-out-of-scope-accessions

Status: OPEN.

Milestone: 10

## Outcome

`get protein <accession>` for an accession outside BioMCP's scope (unreviewed, non-human, deleted) says it is out of scope and why, a secondary accession follows UniProt's redirect to its primary entry, and a protein-coding gene whose HGNC record has no UniProt ID still reaches its protein.

## Why

A scope rule reads as a source outage today, and 306,643 unreviewed human entries plus every non-human entry return `failed`, `bad-reply`.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp get protein P02340` (mouse p53) returns `status: failed`, source reason `bad-reply`, problem `reviewed_human value.unknown`. The same for `A0A0A0MS99` (unreviewed human) and `A0A008APQ8` (deleted).
- Failing case 2: `biomcp get protein Q13290` returns `failed`, `rejected`; UniProt redirects Q13290 to P42126.
- Failing case 3: `biomcp get gene HSD17B8 --section protein` says `not-applicable` and `get protein HSD17B8` says `absent`, while `search protein --gene HSD17B8` finds Q92506. Twenty protein-coding HGNC records have no UniProt ID.
- BioMCP reaches the 20,429 reviewed human entries; help says human and reviewed only.

## Proof

- Proof: outside-in cases on recorded UniProt replies: P02340 returns a usage-style refusal naming the scope; Q13290 returns P42126 with a notice; HSD17B8 returns Q92506.

## Starts from

- Starts from: `crates/biomcp-adapters/src/uniprot/map.rs`, `crates/biomcp-adapters/src/uniprot/entry/map.rs`, the gene protein section in `crates/biomcp-query/src/gene/`.

## Keeps

- Keeps: reviewed human entries are unchanged; scope is not widened by this ticket.

## Defers

- Defers: UniProt comment types other than function and subcellular location (disease comments exist on 5,350 reviewed human entries); file when a case needs them.

## Changes

- Changes: `crates/biomcp-adapters/src/uniprot/`, `crates/biomcp-query/src/gene/`, `crates/biomcp-query/src/protein/`, protein cases.
