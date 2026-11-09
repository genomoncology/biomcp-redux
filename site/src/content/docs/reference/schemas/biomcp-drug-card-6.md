---
title: Reply of biomcp get drug (biomcp.drug-card/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-card/6."
goal: "Parse the biomcp.drug-card/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-card/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-card/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-card/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugCard](#reply-drugcard). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply-drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugCard

Exact source identity and explicit native links; deeper facts require named sections.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate_names` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Source-native alternate names, without local alias expansion. Optional. |
| `available_sections` | list of [Reply_DrugSection](/reference/schemas/types/#reply-drugsection) | Only disclosures available for this selected source identity. Optional. |
| `cross_references` | list of [Reply_DrugIdentityRelation](/reference/schemas/types/#reply-drugidentityrelation) | Explicit source cross-references with their native relation and provenance. Optional. |
| `formulations` | list of [Reply_DrugIdentityRelation](/reference/schemas/types/#reply-drugidentityrelation) | Explicit native formulation links. Optional. |
| `header` | [Reply_DrugHeader](/reference/schemas/types/#reply-drugheader) | Selected source-native identity. |
| `next` | list of [Reply_Step](/reference/schemas/types/#reply-step) | Typed actions preserving exact source and identity. Optional. |
| `official_names` | list of [Reply_Text](/reference/schemas/types/#reply-text) | The alternate names the source types as official: INN, USAN, BAN, JAN or FDA. Optional. |
| `parents` | list of [Reply_DrugIdentityRelation](/reference/schemas/types/#reply-drugidentityrelation) | Explicit native parent links. Optional. |
| `products` | list of [Reply_DrugIdentityRelation](/reference/schemas/types/#reply-drugidentityrelation) | Explicit native product links. Optional. |
| `retrieved` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Actual retrieval date, separate from source publication or update. Optional. |
| `updated` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Source-stated update date. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source record version. Optional. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
