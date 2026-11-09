---
title: Reply of biomcp get gwas (biomcp.gwas-association/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: the fields
  of schema biomcp.gwas-association/6."
goal: "Parse the biomcp.gwas-association/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas-association/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.gwas-association/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gwas-association/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GwasAssociationCard](#reply_gwasassociationcard). `query` holds [Reply_GwasGet](/reference/schemas/types/#reply_gwasget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_GwasAssociationCard

Exact association card; no second association representation.

| Field | Type | Meaning |
| --- | --- | --- |
| `association` | [Reply_GwasAssociation](/reference/schemas/types/#reply_gwasassociation) | The shared source association record. |

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
