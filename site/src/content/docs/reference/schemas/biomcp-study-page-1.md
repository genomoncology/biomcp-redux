---
title: biomcp.study_page/1
description: "The BioMCP 1.0.0 reply schema biomcp.study_page/1: its envelope
  fields, its result type StudyPage and the commands that return it."
goal: Look up the fields of a biomcp.study_page/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.study_page/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.study_page/1`.

These commands return it: [`biomcp search study`](/reference/commands/search-study/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | StudySearch | yes | Parsed query. |
| `result` | StudyPage or null | no | Answer when available. |
| `schema` | "biomcp.study_page/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: StudyPage

Source-ordered checked study headers from one retained catalog.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of StudyHeader | yes | Whole source headers, without expanded descriptions. |
| `page` | PageInfo | yes | Snapshot-scoped matching counts and offset. |
| `snapshot` | StudySnapshot | yes | Catalog acquisition provenance. |

## Query: StudySearch

Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cancer_type` | CbioportalCancerTypeId | no | Exact cBioPortal cancer-type code; no OncoTree conversion. |
| `limit` | integer (uint32) | no | Maximum headers per page. |
| `offset` | integer (uint32) | no | Zero-based offset in the checked snapshot. |
| `pool` | Text or null | no | Opaque continuation retaining the catalog revision and selectors. |
| `text` | Text | no | Literal study ID, name or description text, without synonyms or typo repair. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.study_page/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
