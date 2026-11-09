---
title: Reply of biomcp get cell-line (biomcp.cell-line-names/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get cell-line: the
  fields of schema biomcp.cell-line-names/5."
goal: "Parse the biomcp.cell-line-names/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell-line-names/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get cell-line`](/reference/commands/get-cell-line/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.cell-line-names/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.cell-line-names/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_CellLineNames](/reference/schemas/types/#reply-celllinenames), a shared type. `query` holds [Reply_CellLineGet](/reference/schemas/types/#reply-celllineget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp get cell-line`](/reference/commands/get-cell-line/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
