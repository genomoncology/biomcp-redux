---
title: Reply of biomcp variant assertions (biomcp.variant-assertions/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp variant assertions:
  the fields of schema biomcp.variant-assertions/8."
goal: "Parse the biomcp.variant-assertions/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant-assertions/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp variant assertions`](/reference/commands/variant-assertions/) and [`biomcp variant`](/reference/commands/variant/) reply with `schema` set to `biomcp.variant-assertions/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.variant-assertions/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VariantAssertions](/reference/schemas/types/#reply_variantassertions), a shared type. `query` holds [Reply_VariantAssertionsGet](/reference/schemas/types/#reply_variantassertionsget), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[`biomcp variant assertions`](/reference/commands/variant-assertions/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
