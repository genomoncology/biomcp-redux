---
title: biomcp.protein_function/2
description: "The BioMCP 1.0.0 reply schema biomcp.protein_function/2: its
  envelope fields, its result type ProteinFunctionResult and the commands that
  return it."
goal: Look up the fields of a biomcp.protein_function/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_function/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_function/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_function/2`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinGet | yes | Parsed query. |
| `result` | ProteinFunctionResult or null | no | Answer when available. |
| `schema` | "biomcp.protein_function/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinFunctionResult

Whole source function paragraphs and their own evidence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ProteinFunction | no | Native paragraphs in source order, without inferred interpretation. |

## Query: ProteinGet

Read source protein cards; a symbol returns candidates without choosing a protein.
Sections disclose native facts with independent row bounds; function keeps all paragraphs.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of ProteinInput | yes | Source accessions or gene symbols, preserving caller order. |
| `section` | array of ProteinSection | no | Select native sections or all bounded first pages; function keeps one whole text record. |
| `section_limit` | integer (uint32) | no | First-page row limit; function keeps one whole native text record and does not page its paragraphs. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_function/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
