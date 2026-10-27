---
title: biomcp.pgx_annotation/1
description: "The BioMCP 1.0.0 reply schema biomcp.pgx_annotation/1: its
  envelope fields, its result type PgxAnnotation and the commands that return
  it."
goal: Look up the fields of a biomcp.pgx_annotation/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_annotation/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pgx_annotation/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pgx_annotation/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PgxAnnotationGet | yes | Parsed query. |
| `result` | PgxAnnotation or null | no | Answer when available. |
| `schema` | "biomcp.pgx_annotation/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PgxAnnotation

Complete proved base-tier annotation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `annotation` | PgxAnnotationDetail | yes | Complete native base record, including history and genotype text. |
| `header` | PgxAnnotationIdentity | yes | Exact selected source identity. |

## Query: PgxAnnotationGet

Exact single clinical annotation get, with no inferred source or annotation kind.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | ClinPgxAccession | yes | Checked ClinPGx accession, never a shared PaId. |
| `source` | PgxSource | yes | Required ClinPGx source. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pgx_annotation/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
