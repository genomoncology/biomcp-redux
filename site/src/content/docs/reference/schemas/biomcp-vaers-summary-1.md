---
title: biomcp.vaers_summary/1
description: "The BioMCP 1.0.0 reply schema biomcp.vaers_summary/1: its envelope
  fields, its result type VaersSummary and the commands that return it."
goal: Look up the fields of a biomcp.vaers_summary/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.vaers_summary/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.vaers_summary/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.vaers_summary/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AdverseEventSearch | yes | Parsed query. |
| `result` | VaersSummary or null | no | Answer when available. |
| `schema` | "biomcp.vaers_summary/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VaersSummary

Bounded page of one independent native vaccine summary table.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `dataset` | map of string | yes | Native dataset attributes including processed-through vintage. |
| `footer` | VaersRow | yes | Whole native footer cells, never a sum calculated by this program. |
| `grouping` | map of string | yes | Native grouping attributes. |
| `interpretation` | string | yes | Reviewed interpretation boundary. |
| `measure_metadata` | array of map of string | yes | Response-specific measure metadata without denominator inference. |
| `measures` | array of map of string | yes | Source measure definitions, including original names and codes. |
| `notices` | array of VaersNotice | yes | Applied native messages, evaluated caveats and footnotes. |
| `report_total` | string | yes | Native report-total message, separate from the footer aggregation. |
| `rows` | array of VaersRow | yes | Native row order with local paging. |
| `scope` | VaersScope | yes | Native fixed State / Territory selection. |
| `vaccine` | map of string | yes | Native selected vaccine attributes. |

## Query: AdverseEventSearch

Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native report headers or VAERS rows per page; default five. |
| `offset` | integer (uint32) | no | Native row offset. VAERS continuation requires its retained snapshot. |
| `reaction` | Text or null | no | FAERS MedDRA term phrase match, not exact equality. |
| `received_from` | PartialDate or null | no | Inclusive first FDA receipt lower bound, YYYY-MM-DD. |
| `received_to` | PartialDate or null | no | Inclusive first FDA receipt upper bound, YYYY-MM-DD. |
| `section` | VaersSection or null | no | Independent VAERS table; omitted means reported symptoms. |
| `serious` | AdverseEventSerious or null | no | FAERS report flag: death or hospitalization; never a reaction outcome. |
| `snapshot` | Text or null | no | Retained VAERS table for provider-free continuation. |
| `source` | AdverseEventSource | no | FAERS by default; VAERS accepts only exact supported vaccine names. |
| `text` | Text | yes | Literal drug text, MAUDE device phrase or exact supported VAERS vaccine. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.vaers_summary/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
