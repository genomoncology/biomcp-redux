---
title: biomcp.variant_normalization/3
description: "The BioMCP 1.0.0 reply schema biomcp.variant_normalization/3: its
  envelope fields, its result type VariantNormalization and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_normalization/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_normalization/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_normalization/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_normalization/3`.

These commands return it: [`biomcp variant normalize`](/reference/commands/variant-normalize/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | VariantNormalize | yes | Parsed query. |
| `result` | VariantNormalization or null | no | Answer when available. |
| `schema` | "biomcp.variant_normalization/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VariantNormalization

Independent service results without local correction or transcript selection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `results` | array of NormalizationResult | yes | Service results in declared order. |

## Query: VariantNormalize

Send unchanged reference-qualified notation to explicitly named services.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | Build or null | no | Caller assembly; required when VariantValidator is selected. |
| `hgvs` | HgvsText | yes | Unchanged reference-qualified HGVS; no local molecular parse. |
| `limit` | integer (uint32) | no | Source mapping rows per service, with typed continuation for remaining rows. |
| `offset` | integer (uint32) | no | Source mapping rows to skip per service. |
| `source` | NormalizationSource | yes | Explicit service; all preserves independent results without consensus. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_normalization/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
