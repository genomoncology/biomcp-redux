---
title: biomcp.drug_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.drug_page/3: its envelope
  fields, its result type DrugPage and the commands that return it."
goal: Look up the fields of a biomcp.drug_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.drug_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.drug_page/3`.

These commands return it: [`biomcp search drug`](/reference/commands/search-drug/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DrugSearch | yes | Parsed query. |
| `result` | DrugPage or null | no | Answer when available. |
| `schema` | "biomcp.drug_page/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DrugPage

A single source candidate page in native order, without selecting the first hit.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of DrugHeader | no | Native candidates, preserving their separate identity levels. |
| `total` | integer or null | no | Native total when supplied; absent remains unknown. |

## Query: DrugSearch

Literal single-source candidate discovery, without an implicit identity selection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native candidate rows per page. |
| `name_selector` | DrugNameSelector | no | Explicit ChEMBL native name selector; omitted means preferred name. |
| `offset` | integer (uint32) | no | Native rows to skip. |
| `query` | Text | yes | Literal source name selector, unchanged by the consumer. |
| `source` | DrugSource | no | Exactly one source; default MyChem never fans out. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.drug_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
