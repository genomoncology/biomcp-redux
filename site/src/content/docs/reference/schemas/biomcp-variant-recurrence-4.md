---
title: biomcp.variant_recurrence/4
description: "The BioMCP 1.0.0 reply schema biomcp.variant_recurrence/4: its
  envelope fields, its result type VariantRecurrence and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_recurrence/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_recurrence/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_recurrence/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_recurrence/4`.

These commands return it: [`biomcp variant recurrence`](/reference/commands/variant-recurrence/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | VariantRecurrenceQuery | yes | Parsed query. |
| `result` | VariantRecurrence or null | no | Answer when available. |
| `schema` | "biomcp.variant_recurrence/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VariantRecurrence

Exact-allele observations confined to one explicitly bounded source page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `all_pages_read` | boolean | yes | True only when this invocation read the whole gene query from its first page. |
| `entrez_gene_id` | NcbiGeneId or null | no | HGNC-resolved gene query bound. |
| `examined_rows` | integer (uint32) | yes | Number of source mutation rows examined on this page. |
| `identity` | Variant or null | no | Source-qualified resolved identity; omitted when resolution failed. |
| `matching_granularity` | RecurrenceGranularity | no | Genomic allele equality only, never a protein, residue or gene match. |
| `more` | boolean | yes | Source has further rows or cannot yet establish its end. |
| `observed_sample_count` | integer or null | no | Distinct exact-matched samples on this source page only; absent on failure. |
| `selected_sample_count` | integer or null | no | Sample-list size as stated by that source; never an assayed denominator. |
| `study` | RecurrenceStudy or null | no | Study title, citation and description kept as the source states them. |
| `terms` | RecurrenceTerms | yes | Study-specific reuse restrictions require verification even for public data. |

## Query: VariantRecurrenceQuery

Observe exact-allele samples within one caller-selected source page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene` | GeneSymbol | yes | HGNC symbol resolved to the source's Entrez gene identifier. |
| `input` | VariantInput | yes | Published variant identity; ambiguous identities are never selected silently. |
| `limit` | integer (uint32) | no | Source mutation rows per page, including excluded alleles. |
| `page` | integer (uint32) | no | Zero-based native source page. |
| `profile` | Text | yes | Explicit mutation molecular profile. |
| `sample_set` | Text | yes | Explicit sample list; selected size is never an allele denominator. |
| `source` | RecurrenceSource | yes | Explicit cohort source. |
| `study` | Text | yes | Explicit study; there is no default cohort. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_recurrence/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
