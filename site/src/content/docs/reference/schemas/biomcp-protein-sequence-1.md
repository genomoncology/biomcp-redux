---
title: biomcp.protein_sequence/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_sequence/1: its
  envelope fields, its result type ProteinSequenceResult and the commands that
  return it."
goal: Look up the fields of a biomcp.protein_sequence/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_sequence/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_sequence/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_sequence/1`.

These commands return it: [`biomcp protein sequence`](/reference/commands/protein-sequence/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinSequenceQuery | yes | Parsed query. |
| `result` | ProteinSequenceResult or null | no | Answer when available. |
| `schema` | "biomcp.protein_sequence/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinSequenceResult

One directly returned source sequence, with exact identity and audit metadata.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `sequence` | ProteinSequence | yes | Native sequence record; never constructed from another entry. |

## Query: ProteinSequenceQuery

Read one complete native sequence for an exact source accession.
No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `isoform` | UniProtIsoform | yes | Exact isoform or base accession; no canonical suffix is synthesized. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_sequence/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
