---
title: biomcp.adverse_event_faers_card/1
description: "The BioMCP 1.0.0 reply schema biomcp.adverse_event_faers_card/1:
  its envelope fields, its result type AdverseEventFaersCard and the commands
  that return it."
goal: Look up the fields of a biomcp.adverse_event_faers_card/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_faers_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.adverse_event_faers_card/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.adverse_event_faers_card/1`.

These commands return it: [`biomcp get adverse-event`](/reference/commands/get-adverse-event/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AdverseEventGet | yes | Parsed query. |
| `result` | AdverseEventFaersCard or null | no | Answer when available. |
| `schema` | "biomcp.adverse_event_faers_card/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: AdverseEventFaersCard

FAERS card with native report-level facts and independent collection lengths.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `available_sections` | array of AdverseEventSection | yes | Named sections available in this report. |
| `components` | array of ComponentLabel | yes | Applicable component terms. |
| `drug_count` | integer (uint32) | yes | Actual drug-array length, never count bucket. |
| `first_received_format` | string or null | no | Native first_received_format string; present empty values remain present. |
| `header` | AdverseEventHeader | yes | Exact selected report identity. |
| `latest_received` | string or null | no | Native latest_received string; present empty values remain present. |
| `latest_received_format` | string or null | no | Native latest_received_format string; present empty values remain present. |
| `patient` | FaersPatient or null | no | Fixed native patient strings. |
| `reaction_count` | integer (uint32) | yes | Actual reaction-array length, never causal pairs. |
| `report_url` | Text | yes | Exact successful source request URL, never guessed. |
| `reporter` | FaersReporter or null | no | Optional reporter facts. |
| `seriousness` | FaersSeriousness or null | no | Independent source report flags. |
| `snapshot` | Text | yes | Opaque checked retained-report snapshot identity. |
| `source_url` | Text | yes | Publisher overview, never a next step. |
| `transmitted` | string or null | no | Native transmitted string; present empty values remain present. |
| `transmitted_format` | string or null | no | Native transmitted_format string; present empty values remain present. |

## Query: AdverseEventGet

Exact native report identity and an optional independently paged section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `drug_index` | integer or null | no | Absolute source-array index; required only for identifier sections. |
| `id` | ReportId | yes | Published report ID with no source prefix. |
| `section` | AdverseEventSection or null | no | FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only. |
| `section_limit` | integer (uint32) | no | Whole source rows per section page. |
| `section_offset` | integer (uint32) | no | Offset within the selected native section. |
| `snapshot` | Text or null | no | Checked retained report snapshot for provider-free continuation. |
| `source` | "faers" or "maude" | no | FAERS by default; MAUDE is explicit. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.adverse_event_faers_card/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
