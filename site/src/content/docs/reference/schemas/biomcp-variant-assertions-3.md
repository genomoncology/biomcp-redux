---
title: biomcp.variant_assertions/3
description: "The BioMCP 1.0.0 reply schema biomcp.variant_assertions/3: its
  envelope fields, its result type VariantAssertions and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_assertions/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_assertions/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_assertions/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_assertions/3`.

These commands return it: [`biomcp variant assertions`](/reference/commands/variant-assertions/).

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | CacheProblem or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of Step | no | Available next commands. |
| `page` | PageInfo or null | no | Search page metadata. |
| `problems` | array of Problem | no | Rejected source fields. |
| `query` | VariantAssertionsGet | yes | Parsed query. |
| `result` | VariantAssertions or null | no | Answer when available. |
| `schema` | "biomcp.variant_assertions/3" | yes |  |
| `sources` | array of SourceOutcome | no | Sources actually asked. |
| `status` | Status | yes | Overall outcome. |

## Result: VariantAssertions

One source-ordered page, with native record and classification boundaries intact.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertions` | array of VariantAssertion | yes | Source assertion rows in native order. |
| `next` | VariantAssertionsGet or null | no | Typed continuation preserving source and selectors. |
| `page` | PageInfo | yes | Page bounds over native rows. |

## Query: VariantAssertionsGet

Page assertions from one explicitly selected evidence source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertion` | Text or null | no | Native assertion accession or document ID. |
| `input` | VariantInput | yes | Checked Variation ID for ClinVar, or CAid for ERepo. |
| `limit` | integer (uint32) | no | Assertions per page; omitted uses the native source default. |
| `offset` | integer (uint32) | no | Assertions to skip in source order. |
| `source` | AssertionSource | yes | One native source; no fallback or text search. |
| `version` | Text or null | no | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_assertions/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
