---
title: biomcp.adverse_event_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.adverse_event_page/2: its
  envelope fields, its result type AdverseEventPage and the commands that return
  it."
goal: Look up the fields of a biomcp.adverse_event_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.adverse_event_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.adverse_event_page/2`.

These commands return it: [`biomcp search adverse-event`](/reference/commands/search-adverse-event/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | AdverseEventSearch | yes | Parsed query. |
| `result` | AdverseEventPage or null | no | Answer when available. |
| `schema` | "biomcp.adverse_event_page/2" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: AdverseEventPage

Ordered report headers; totals count reports with no population denominator.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of ComponentLabel | yes | Applicable component terms. |
| `fetched` | PartialDate or null | no | Original retrieval date belonging to this reply, absent for undated legacy recordings. |
| `items` | array of AdverseEventHeader | yes | Native ordered report candidates. |
| `reaction_match` | AdverseEventReactionMatch or null | no | Explicit analyzed reaction selector semantics when a reaction was supplied. |
| `source_query` | string | yes | Exact literal provider expression; reaction selectors use phrase match. |
| `source_updated` | PartialDate or null | no | The publisher last_updated date, separate from retrieval. |
| `unit` | ReportUnit | yes | Explicit reports unit for envelope page totals. |

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

This page is generated from `biomcp schema biomcp.adverse_event_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
