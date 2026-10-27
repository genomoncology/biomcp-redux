---
title: biomcp.pathway_genes/4
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_genes/4: its envelope
  fields, its result type PathwayGenes and the commands that return it."
goal: Look up the fields of a biomcp.pathway_genes/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_genes/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_genes/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_genes/4`.

These commands return it: [`biomcp pathway genes`](/reference/commands/pathway-genes/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PathwayGenesQuery | yes | Parsed query. |
| `result` | PathwayGenes or null | no | Answer when available. |
| `schema` | "biomcp.pathway_genes/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PathwayGenes

Independent source-ordered distinct membership references and their observed counts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | PathwayHeader | yes | Exact source pathway owning this relation. |
| `items` | array of PathwayMember | yes | Distinct namespaced source references in first occurrence order. |
| `page` | PathwayPageInfo | yes | Distinct-reference total and continuation. |
| `physical_entities` | integer (uint64) | yes | All native physical entities, never described as gene counts. |
| `reference_occurrences` | integer (uint64) | yes | All reference occurrences across physical entities before distinct-reference grouping. |

## Query: PathwayGenesQuery

Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `details` | Details | no | Include every physical entity context for these references; never clips source text. |
| `id` | string | yes | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | no | Five complete relation rows by default, at most fifty. |
| `offset` | integer (uint32) | no | Offset within the retained selected relation. |
| `pool` | Text | no | Immutable relation snapshot. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pathway_genes/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
