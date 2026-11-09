---
title: Reply of biomcp protein isoforms (biomcp.protein-isoforms/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein isoforms:
  the fields of schema biomcp.protein-isoforms/4."
goal: "Parse the biomcp.protein-isoforms/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-isoforms/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein isoforms`](/reference/commands/protein-isoforms/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-isoforms/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-isoforms/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinIsoforms](#reply_proteinisoforms). `query` holds [Reply_ProteinRelationQuery](/reference/schemas/types/#reply_proteinrelationquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_ProteinIsoforms

Alternative products with every stated identifier and explicit direct sequence state.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinIsoform](/reference/schemas/types/#reply_proteinisoform) | Whole native rows in source order; totals apply only to this collection. Optional. |

## Next

[`biomcp protein isoforms`](/reference/commands/protein-isoforms/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
