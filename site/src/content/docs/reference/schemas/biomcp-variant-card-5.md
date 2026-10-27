---
title: biomcp.variant_card/5
description: "The BioMCP 1.0.0 reply schema biomcp.variant_card/5: its envelope
  fields, its result type VariantCard and the commands that return it."
goal: Look up the fields of a biomcp.variant_card/5 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_card/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_card/5`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_card/5`.

These commands return it: [`biomcp get variant`](/reference/commands/get-variant/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | VariantGet | yes | Parsed query. |
| `result` | VariantCard or null | no | Answer when available. |
| `schema` | "biomcp.variant_card/5" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VariantCard

A compact identity card, including every provider candidate and explicit rejection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `candidates` | array of VariantHeader | yes | All retained source identities. |
| `rejection` | Text or null | no | Provider rejection text, when present, separate from unchanged query input. |
| `resolution` | VariantResolution | yes | Identity outcome, without selecting the first candidate. |
| `source` | Source | yes | Responsible identity source. |

## Query: VariantGet

Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. Use search variant --gene BRAF --protein V600E for shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | Build or null | no | Required for chromosome notation; no assembly is guessed. |
| `civic_cursor` | Text or null | no | Native CIViC cursor, retained unchanged. |
| `civic_kind` | CivicClaimKind or null | no | Native CIViC claim collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected CIViC molecular profile for native claim paging. |
| `input` | VariantInput | yes | Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS. |
| `limit` | integer (uint32) | no | Candidate rows per page; more remain reachable through typed next commands. |
| `offset` | integer (uint32) | no | Candidate rows to skip in source order. |
| `resolver` | VariantResolver | no | Resolver for reference-qualified notation; never repairs the input locally. |
| `section` | array of VariantSection | no | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. |
| `section_limit` | integer (uint32) | no | Rows per named source section page; omitted uses the native source default. |
| `section_offset` | integer (uint32) | no | Source rows to skip independently of identity candidate paging. |
| `tumor_type` | Text or null | no | Explicit tumor context for keyed OncoKB annotation. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_card/5` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
