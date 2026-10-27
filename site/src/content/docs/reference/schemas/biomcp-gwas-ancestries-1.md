---
title: biomcp.gwas_ancestries/1
description: "The BioMCP 1.0.0 reply schema biomcp.gwas_ancestries/1: its
  envelope fields, its result type GwasAncestries and the commands that return
  it."
goal: Look up the fields of a biomcp.gwas_ancestries/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_ancestries/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gwas_ancestries/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gwas_ancestries/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | GwasGet | yes | Parsed query. |
| `result` | GwasAncestries or null | no | Answer when available. |
| `schema` | "biomcp.gwas_ancestries/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: GwasAncestries

Bounded retained ancestry relation for one exact study.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | GwasGet | no | Retained relation continuation. |
| `items` | array of GwasAncestry | yes | Independent source rows. |
| `study` | GwasStudyId | yes | Exact owning study. |

## Query: GwasGet

Explicit resource sum; impossible section and identity pairings fail during decoding.

One of:

- object: Read an association.
- object: Read a study.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.gwas_ancestries/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
