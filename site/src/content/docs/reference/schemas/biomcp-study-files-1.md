---
title: biomcp.study_files/1
description: "The BioMCP 1.0.0 reply schema biomcp.study_files/1: its envelope
  fields, its result type StudyFiles and the commands that return it."
goal: Look up the fields of a biomcp.study_files/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_files/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.study_files/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.study_files/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | StudyGet | yes | Parsed query. |
| `result` | StudyFiles or null | no | Answer when available. |
| `schema` | "biomcp.study_files/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: StudyFiles

One native repository directory page; blob sizes do not describe data content.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Directory acquisition wall time in Unix seconds. |
| `items` | array of StudyFile | yes | Source-ordered repository file references. |
| `license_notice` | Text or null | no | Native LICENSE publisher link, independent of the current file page. |
| `page` | PageInfo | yes | Bounds within the complete received directory. |
| `study_id` | StudyId | yes | Exact study directory scope. |

## Query: StudyGet

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | StudyId | yes | Exact source study ID; no default or repair. |
| `section` | StudySection or null | no | Omit for the card; all returns bounded first pages only. |
| `section_limit` | integer (uint32) | no | Maximum records in each selected section. |
| `section_offset` | integer (uint32) | no | Offset within one named section; all requires zero. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.study_files/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
