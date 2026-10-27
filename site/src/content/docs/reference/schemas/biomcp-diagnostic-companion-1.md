---
title: biomcp.diagnostic_companion/1
description: "The BioMCP 1.0.0 reply schema biomcp.diagnostic_companion/1: its
  envelope fields, its result type DiagnosticCompanion and the commands that
  return it."
goal: Look up the fields of a biomcp.diagnostic_companion/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_companion/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.diagnostic_companion/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.diagnostic_companion/1`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DiagnosticSearch | yes | Parsed query. |
| `result` | DiagnosticCompanion or null | no | Answer when available. |
| `schema` | "biomcp.diagnostic_companion/1" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DiagnosticCompanion

Whole distinct physical companion-table rows with cell-owned identifiers.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count_unit` | DiagnosticCountUnit or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | array of CompanionDiagnosticRelation | yes | Complete source-owned rows in native order; envelope totals count native rows. |

## Query: DiagnosticSearch

Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `biomarker` | Text or null | no | Exact table-0 biomarker cell; unsupported in the group table. |
| `bundle_revision` | Text or null | no | Immutable stored bundle revision retained by a continuation. |
| `condition_id` | Text or null | no | Exact MedGen:CUI identifier only. |
| `device_id` | DiagnosticDeviceId or null | no | Exact PMA application, application/supplement or native K identity. |
| `drug_application` | FdaDrugApplication or null | no | Cell-owned typed NDA/BLA application. |
| `gene` | GeneSymbol or null | no | Exact source gene_symbol field, without alias expansion. |
| `gene_id` | NcbiGeneId or null | no | Exact source GeneID, exclusive with gene symbol. |
| `limit` | integer (uint32) | no | Five results by default; at most fifty source rows or product headers. |
| `manufacturer` | Text or null | no | Literal whole-word phrase in WHO's Manufacturer name cell. |
| `marker` | Text or null | no | Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. |
| `offset` | integer (uint32) | no | Zero-based source result offset. |
| `source` | DiagnosticSearchSource | yes | Explicit native GTR, WHO IVD or FDA companion discovery. |
| `type` | GtrTestType or null | no | Exact native test_type value. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.diagnostic_companion/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
