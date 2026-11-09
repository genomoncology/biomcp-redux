---
title: Reply of biomcp protein structures (biomcp.protein-structures/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein structures:
  the fields of schema biomcp.protein-structures/5."
goal: "Parse the biomcp.protein-structures/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-structures/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein structures`](/reference/commands/protein-structures/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-structures/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-structures/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinStructures](#reply-proteinstructures). `query` holds [Reply_ProteinRelationQuery](/reference/schemas/types/#reply-proteinrelationquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_ProteinStructures

Source structure links and native properties; no residue coverage claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinStructureRef](/reference/schemas/types/#reply-proteinstructureref) | Whole native rows in source order; totals apply only to this collection. Optional. |

## Next

[`biomcp protein structures`](/reference/commands/protein-structures/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
