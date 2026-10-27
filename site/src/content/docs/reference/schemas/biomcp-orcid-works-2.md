---
title: biomcp.orcid_works/2
description: "The BioMCP 1.0.0 reply schema biomcp.orcid_works/2: its envelope
  fields, its result type OrcidWorks and the commands that return it."
goal: Look up the fields of a biomcp.orcid_works/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.orcid_works/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.orcid_works/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.orcid_works/2`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AuthorPapersQuery | yes | Parsed query. |
| `result` | OrcidWorks or null | no | Answer when available. |
| `schema` | "biomcp.orcid_works/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: OrcidWorks

Locally paged groups from one complete public ORCID works reply.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `author` | OrcidId | yes | Requested exact ORCID record. |
| `collection_scope` | OrcidWorksScope | yes | The whole source reply supplies every local page. |
| `groups` | array of OrcidWorkGroup | yes | Whole groups in native order, with every public assertion. |
| `observed_groups` | integer (uint64) | yes | Observed groups in the complete received reply, never a publication total. |

## Query: AuthorPapersQuery

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `full` | boolean | no | Legacy alias for details; normalized on construction. |
| `id` | AuthorId | yes | Exact qualified provider record. |
| `limit` | integer (uint32) | no | Bounded row count. |
| `offset` | integer (uint32) | no | Native or retained collection offset. |
| `pool` | Text | no | Retained collection identity. |
| `section` | AuthorPaperSection | no | Detailed disclosure when requested. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.orcid_works/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
