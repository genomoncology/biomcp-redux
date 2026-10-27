---
title: biomcp.pathway_events/2
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_events/2: its
  envelope fields, its result type PathwayEvents and the commands that return
  it."
goal: Look up the fields of a biomcp.pathway_events/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_events/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_events/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_events/2`.

These commands return it: [`biomcp pathway events`](/reference/commands/pathway-events/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PathwayEventsQuery | yes | Parsed query. |
| `result` | PathwayEvents or null | no | Answer when available. |
| `schema` | "biomcp.pathway_events/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PathwayEvents

Independent source-ordered page of whole native references.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | PathwayHeader | yes | Exact source pathway owning this relation. |
| `items` | array of PathwayEventRef | yes | Whole native references; unresolved identities remain visible. |
| `page` | PathwayPageInfo | yes | Relation-specific count provenance and continuation. |

## Query: PathwayEventsQuery

Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | string | yes | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | no | Five complete relation rows by default, at most fifty. |
| `offset` | integer (uint32) | no | Offset within the retained selected relation. |
| `pool` | Text | no | Immutable relation snapshot. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pathway_events/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
