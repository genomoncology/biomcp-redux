---
title: biomcp.pgx_diplotype/1
description: "The BioMCP 1.0.0 reply schema biomcp.pgx_diplotype/1: its envelope
  fields, its result type PgxDiplotype and the commands that return it."
goal: Look up the fields of a biomcp.pgx_diplotype/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_diplotype/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pgx_diplotype/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pgx_diplotype/1`.

These commands return it: [`biomcp get pgx`](/reference/commands/get-pgx/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PgxGet | yes | Parsed query. |
| `result` | PgxDiplotype or null | no | Answer when available. |
| `schema` | "biomcp.pgx_diplotype/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PgxDiplotype

Every source-returned mapping for the exact declared expression.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `diplotypes` | array of PgxDiplotypeResult | yes | Zero, one or multiple native mappings; never a selected patient phenotype. |
| `header` | PgxHeader | yes | Explicit gene and source. |

## Query: PgxGet

PGx card or section. No phenotype, phase or dose is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `diplotype` | PgxDiplotypeExpression or null | no | Unchanged declared expression required for the diplotype section only. |
| `drug_id` | Curie or null | no | Exact published CPIC drug key; recommendations only. |
| `gene` | GeneSymbol | yes | Explicit source gene symbol. |
| `kind` | PgxAnnotationKind or null | no | One ClinPGx collection; clinical is the default only on annotations. |
| `section` | PgxSection or null | no | One named section; omitted means the CPIC gene card. |
| `section_limit` | integer (uint32) | no | Rows per page: three clinical, five elsewhere. |
| `section_offset` | integer (uint32) | no | Offset within the selected section. |
| `snapshot` | Text or null | no | Retained collection for local continuation. |
| `source` | PgxSource | yes | Required source, checked against the section. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pgx_diplotype/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
