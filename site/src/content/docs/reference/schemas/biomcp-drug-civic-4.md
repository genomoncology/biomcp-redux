---
title: biomcp.drug_civic/4
description: "The BioMCP 1.0.0 reply schema biomcp.drug_civic/4: its envelope
  fields, its result type DrugCivic and the commands that return it."
goal: Look up the fields of a biomcp.drug_civic/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_civic/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.drug_civic/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.drug_civic/4`.

No help example returns this schema; the reply carries it in its `schema` field.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | DrugGet | yes | Parsed query. |
| `result` | DrugCivic or null | no | Answer when available. |
| `schema` | "biomcp.drug_civic/4" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: DrugCivic

Complete CIViC therapy/profile context with independent native claim continuations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic` | Section | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | DrugHeader | yes | Selected native identity, checked independently of section facts. |

## Query: DrugGet

Exact native ID retrieval with an explicitly selected source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic_cursor` | Text or null | no | Native continuation cursor for that exact drug, profile and claim collection. |
| `civic_kind` | CivicClaimKind or null | no | Independent native evidence or assertion collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected native CIViC molecular profile for a drug-scoped claim stream. |
| `input` | DrugId | yes | Published source-native identifier, never SOURCE:ID or a selected name. |
| `label_section` | Text or null | no | Exact native label heading, required only for the label section. |
| `product_number` | Text or null | no | Explicit native application product number, only for approvals. |
| `section` | array of DrugSection | no | Explicit named disclosures; empty means identity only. |
| `section_limit` | integer or null | no | Named section page size, refused on an identity-only card. |
| `section_offset` | integer or null | no | Named section continuation, refused on an identity-only card. |
| `source` | DrugSource | yes | Required source; there is no default exact-ID resolver. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.drug_civic/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
