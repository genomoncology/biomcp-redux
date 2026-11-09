---
title: Reply of biomcp protein complexes (biomcp.protein-complexes/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein complexes:
  the fields of schema biomcp.protein-complexes/5."
goal: "Parse the biomcp.protein-complexes/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-complexes/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein complexes`](/reference/commands/protein-complexes/) and [`biomcp protein`](/reference/commands/protein/) reply with `schema` set to `biomcp.protein-complexes/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-complexes/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinComplexes](/reference/schemas/types/#reply-proteincomplexes), a shared type. `query` holds [Reply_ProteinRelationQuery](/reference/schemas/types/#reply-proteinrelationquery), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp protein complexes`](/reference/commands/protein-complexes/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
