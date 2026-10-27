---
title: biomcp.pathway_card/5
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_card/5: its envelope
  fields, its result type PathwayCard and the commands that return it."
goal: Look up the fields of a biomcp.pathway_card/5 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_card/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_card/5`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_card/5`.

These commands return it: [`biomcp get pathway`](/reference/commands/get-pathway/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | PathwayGet | yes | Parsed query. |
| `result` | PathwayCard or null | no | Answer when available. |
| `schema` | "biomcp.pathway_card/5" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: PathwayCard

One checked source pathway with supported disclosures and whole native text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description_truncated` | boolean or null | no | Whether the source itself shortened its description, rather than the client. |
| `events` | Section2 or null | no | Requested bounded event page, including unresolved integer references. |
| `genes` | Section or null | no | Requested bounded membership page, including its explicit source state. |
| `pathway` | Pathway | yes | Source-native identity, species, versions and requested description. |
| `release` | Text or null | no | Native Reactome database release, separate from pathway version. |
| `sections` | array of PathwaySection | yes | Source-supported named sections; WikiPathways excludes events. |

## Query: PathwayGet

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fresh` | boolean | no | Request a new source observation. |
| `id` | string | yes | Exact human Reactome or WikiPathways identity without version suffixes. |
| `pool` | Text | no | Retained snapshot, when continuing a source relation. |
| `section` | PathwaySection | no | Omit for the card; WikiPathways has summary and genes sections. |
| `section_limit` | integer (uint32) | no | Bounded initial section size; each all section has its own first page. |
| `section_offset` | integer (uint32) | no | Native relation offset for a selected genes or events section. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pathway_card/5` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
