---
title: biomcp.diagnostic_regulatory/1
description: "The BioMCP 1.0.0 reply schema biomcp.diagnostic_regulatory/1: its
  envelope fields, its result type DiagnosticRegulatory and the commands that
  return it."
goal: Look up the fields of a biomcp.diagnostic_regulatory/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_regulatory/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.diagnostic_regulatory/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.diagnostic_regulatory/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DiagnosticGet | yes | Parsed query. |
| `result` | DiagnosticRegulatory or null | no | Answer when available. |
| `schema` | "biomcp.diagnostic_regulatory/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DiagnosticRegulatory

Exact selected submission regulatory text fragments.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count_unit` | DiagnosticCountUnit or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | array of DiagnosticRegulatoryFragment | yes | Complete source-owned rows in native order; envelope totals count native rows. |

## Query: DiagnosticGet

Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `bundle_revision` | Text or null | no | Immutable stored bundle revision retained by a continuation. |
| `id` | Text | yes | Exact native identity, including version for GTR. |
| `section` | DiagnosticSection or null | no | gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card. |
| `section_limit` | integer (uint32) | no | Maximum native rows; omitted means five rows, or one regulatory fragment. |
| `section_offset` | integer (uint32) | no | Zero-based native row or fragment offset. |
| `source` | DiagnosticGetSource | yes | Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number. |
| `supplement` | PmaSupplement or null | no | Exact native PMA supplement; absent selects the base row. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.diagnostic_regulatory/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
