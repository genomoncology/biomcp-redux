---
title: biomcp.protein_structures/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_structures/1: its
  envelope fields, its result type ProteinStructures and the commands that
  return it."
goal: Look up the fields of a biomcp.protein_structures/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_structures/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_structures/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_structures/1`.

These commands return it: [`biomcp protein structures`](/reference/commands/protein-structures/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinRelationQuery | yes | Parsed query. |
| `result` | ProteinStructures or null | no | Answer when available. |
| `schema` | "biomcp.protein_structures/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinStructures

Source structure links and native properties; no residue coverage claim.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ProteinStructureRef | yes | Whole native rows in source order; totals apply only to this collection. |

## Query: ProteinRelationQuery

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | UniProtIsoform | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | Text | no | Retained collection identity. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_structures/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
