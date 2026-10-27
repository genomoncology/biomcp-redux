---
title: biomcp.drug_who_card/1
description: "The BioMCP 1.0.0 reply schema biomcp.drug_who_card/1: its envelope
  fields, its result type DrugWhoCard and the commands that return it."
goal: Look up the fields of a biomcp.drug_who_card/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_who_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.drug_who_card/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.drug_who_card/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DrugWhoGet | yes | Parsed query. |
| `result` | DrugWhoCard or null | no | Answer when available. |
| `schema` | "biomcp.drug_who_card/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DrugWhoCard

Source-scoped native rows for one exact WHO reference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `native_rows` | array of DrugWhoRow | yes | Every native row for that reference, without invented fields. |
| `reference` | Text | yes | Exact requested and source-matched reference. |

## Query: DrugWhoGet

WHO exact-reference selector keeps the shared DrugId contract unchanged.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | Text | yes | Exact WHO reference number, including native spaces; no name or alias resolution. |
| `source` | DrugSource | yes | Explicit source who-pq; prequalification is source-scoped. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.drug_who_card/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
