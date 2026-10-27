---
title: biomcp.phenotype-diseases/3
description: "The BioMCP 1.0.0 reply schema biomcp.phenotype-diseases/3: its
  envelope fields, its result type PhenotypeDiseases and the commands that
  return it."
goal: Look up the fields of a biomcp.phenotype-diseases/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-diseases/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.phenotype-diseases/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.phenotype-diseases/3`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PhenotypeGet | yes | Parsed query. |
| `result` | PhenotypeDiseases or null | no | Answer when available. |
| `schema` | "biomcp.phenotype-diseases/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PhenotypeDiseases

Direct source disease association rows without clinical inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of ComponentLabel | yes | Applicable publisher component terms. |
| `direct` | boolean | yes | Publisher direct graph scope; this supplies no causal assertion. |
| `fetched` | integer or null | no | Original source retrieval time, distinct from dataset release. |
| `items` | array of DiseaseAssociationItem | yes | Native association rows in source order, without deduplication. |

## Query: PhenotypeGet

Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | HpId | yes | Exact HP identifier. |
| `section` | PhenotypeSection | no | Absent means the source identity card. |
| `section_limit` | integer (uint32) | no | Default five text/link rows or one complete association row. |
| `section_offset` | integer (uint32) | no | Native section row offset. |
| `snapshot` | Text | no | Retained exact source reply for local text and link pages; unsupported for diseases and genes. |
| `source` | PhenotypeSource | no | HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.phenotype-diseases/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
