---
title: Reply of biomcp protein domains (biomcp.protein-domains/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein domains: the
  fields of schema biomcp.protein-domains/7."
goal: "Parse the biomcp.protein-domains/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-domains/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein domains`](/reference/commands/protein-domains/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-domains/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-domains/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinDomains](/reference/schemas/types/#reply_proteindomains), a shared type. `query` holds [Reply_ProteinDomainsQuery](#reply_proteindomainsquery). `query` repeats the request as BioMCP read it.

### Reply_ProteinDomainsQuery

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtIsoform](/reference/schemas/types/#reply_uniprotisoform) | Exact source protein sequence identity. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |
| `type` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact native InterPro type, not a normalized domain label. Optional. |

## Next

[`biomcp protein domains`](/reference/commands/protein-domains/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
