---
title: Reply of biomcp get protein (biomcp.protein-function/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get protein: the
  fields of schema biomcp.protein-function/4."
goal: "Parse the biomcp.protein-function/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein-function/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get protein`](/reference/commands/get-protein/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.protein-function/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.protein-function/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ProteinFunctionResult](/reference/schemas/types/#reply_proteinfunctionresult), a shared type. `query` holds [Reply_ProteinGet](/reference/schemas/types/#reply_proteinget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp get protein`](/reference/commands/get-protein/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
