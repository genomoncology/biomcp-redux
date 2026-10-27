---
title: biomcp.protein_complex_members/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_complex_members/1:
  its envelope fields, its result type ProteinComplexMembers and the commands
  that return it."
goal: Look up the fields of a biomcp.protein_complex_members/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_complex_members/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_complex_members/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_complex_members/1`.

These commands return it: [`biomcp protein complex-members`](/reference/commands/protein-complex-members/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinComplexMembersQuery | yes | Parsed query. |
| `result` | ProteinComplexMembers or null | no | Answer when available. |
| `schema` | "biomcp.protein_complex_members/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinComplexMembers

Native participants of one exact containing complex, including non-protein source facts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ProteinComplexMember | yes | Whole native rows in source order; totals apply only to this collection. |

## Query: ProteinComplexMembersQuery

Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | UniProtIsoform | yes | Exact source protein sequence identity. |
| `complex` | ComplexPortalId | yes | Exact complex whose native participants contain the anchor. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | Text | no | Retained collection identity. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_complex_members/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
