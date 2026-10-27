---
title: biomcp.protein_domains/2
description: "The BioMCP 1.0.0 reply schema biomcp.protein_domains/2: its
  envelope fields, its result type ProteinDomains and the commands that return
  it."
goal: Look up the fields of a biomcp.protein_domains/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_domains/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_domains/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_domains/2`.

These commands return it: [`biomcp protein domains`](/reference/commands/protein-domains/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | ProteinDomainsQuery | yes | Parsed query. |
| `result` | ProteinDomains or null | no | Answer when available. |
| `schema` | "biomcp.protein_domains/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: ProteinDomains

Whole native rows with the independent unfiltered source count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of ProteinDomainMatch | yes | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | yes | Native source total before type selection or exact participant admission. |

## Query: ProteinDomainsQuery

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | UniProtIsoform | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | Text | no | Retained collection identity. |
| `type` | Text | no | Exact native InterPro type, not a normalized domain label. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_domains/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
