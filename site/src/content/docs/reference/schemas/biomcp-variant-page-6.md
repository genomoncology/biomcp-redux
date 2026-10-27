---
title: biomcp.variant_page/6
description: "The BioMCP 1.0.0 reply schema biomcp.variant_page/6: its envelope
  fields, its result type VariantPage and the commands that return it."
goal: Look up the fields of a biomcp.variant_page/6 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_page/6
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_page/6`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_page/6`.

These commands return it: [`biomcp search variant`](/reference/commands/search-variant/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | VariantSearch | yes | Parsed query. |
| `result` | VariantPage or null | no | Answer when available. |
| `schema` | "biomcp.variant_page/6" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VariantPage

One native MyVariant candidate page; its total and offset are in the shared envelope.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | Build or null | no | Assembly sent to the source, never inferred from a candidate label. |
| `filter_terms` | ComponentTerms or null | no | Independent publisher terms for named ClinVar filters, present only when requested. |
| `items` | array of VariantHeader | yes | Native source order, preserving every candidate including repeated source identities. |
| `selector_terms` | ComponentTerms or null | no | Independent publisher terms for the declared dbNSFP gene/protein selector. |
| `source_query` | string | yes | Exact query sent to the named source, without rewriting raw query text. |

## Query: VariantSearch

Discover source candidates using one declared selector, without interpreting free text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | Build | no | Optional caller assembly sent to MyVariant; regions require it explicitly. |
| `condition` | Text | no | Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. |
| `gene` | GeneSymbol | no | Exact checked gene symbol for the fixed dbNSFP gene/protein field query; obeys component terms switches. |
| `has` | VariantSearchField | no | Require the selected component's identifier field. Only source-proven mappings are offered. |
| `limit` | integer (uint32) | no | Native candidates per page, default 10, maximum 50; all candidates remain candidates. |
| `offset` | integer (uint32) | no | Native offset within this command's bounded result window; larger native totals remain visible. |
| `protein` | ProteinQuery | no | Explicit substitution using checked one-letter or three-letter amino-acid codes. |
| `query` | MyVariantQuery | no | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. |
| `region` | VariantRegion | no | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. |
| `review_status` | Text | no | Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. |
| `source` | VariantSearchSource | no | Explicit source language; required for a raw query. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_page/6` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
