---
title: biomcp.study_card/2
description: "The BioMCP 1.0.0 reply schema biomcp.study_card/2: its envelope
  fields, its result type StudyCard and the commands that return it."
goal: Look up the fields of a biomcp.study_card/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_card/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.study_card/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.study_card/2`.

These commands return it: [`biomcp get study`](/reference/commands/get-study/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | StudyGet | yes | Parsed query. |
| `result` | StudyCard or null | no | Answer when available. |
| `schema` | "biomcp.study_card/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: StudyCard

Exact card with independent live section states and catalog provenance.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `files` | Section3 or null | no | Requested file-reference page, with no downloaded data. |
| `profiles` | Section or null | no | Requested molecular-profile page, absent when not requested. |
| `sample_lists` | Section2 or null | no | Requested sample-list metadata page, without member IDs. |
| `snapshot` | StudySnapshot | yes | Catalog revision and acquisition, apart from each live section's time. |
| `source_url` | Text | yes | Exact publisher study page for this checked source identity. |
| `study` | Study | yes | Native card metadata, preserving all source count distinctions. |

## Query: StudyGet

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | StudyId | yes | Exact source study ID; no default or repair. |
| `section` | StudySection or null | no | Omit for the card; all returns bounded first pages only. |
| `section_limit` | integer (uint32) | no | Maximum records in each selected section. |
| `section_offset` | integer (uint32) | no | Offset within one named section; all requires zero. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.study_card/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
