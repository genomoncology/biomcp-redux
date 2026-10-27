---
title: biomcp.variant_population/4
description: "The BioMCP 1.0.0 reply schema biomcp.variant_population/4: its
  envelope fields, its result type VariantPopulationResult and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_population/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_population/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_population/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_population/4`.

These commands return it: [`biomcp variant population`](/reference/commands/variant-population/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | VariantPopulationQuery | yes | Parsed query. |
| `result` | VariantPopulationResult or null | no | Answer when available. |
| `schema` | "biomcp.variant_population/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VariantPopulationResult

Population summary and one explicitly requested ancestry-detail page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `groups` | array of GnomadCounts | no | Selected native groups, qualified by sequencing subset. |
| `population` | Section | yes | Source population state; missing datasets stay absent within its summary. |

## Query: VariantPopulationQuery

Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | Build or null | no | Explicit source assembly when required by identity input. |
| `detail` | boolean | no | Read source ancestry groups, preserving overlapping groups individually. |
| `input` | VariantInput | yes | Exact published allele identity; ambiguous candidates must first be selected. |
| `limit` | integer (uint32) | no | Maximum rows per page. |
| `offset` | integer (uint32) | no | Native rows to skip. |
| `resolver` | VariantResolver | no | Resolver for reference-qualified input. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_population/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
