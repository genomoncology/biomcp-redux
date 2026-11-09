---
title: Reply of biomcp get diagnostic (biomcp.diagnostic-methods/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic: the
  fields of schema biomcp.diagnostic-methods/4."
goal: "Parse the biomcp.diagnostic-methods/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-methods/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.diagnostic-methods/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-methods/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticMethods](#reply-diagnosticmethods). `query` holds [Reply_DiagnosticGet](/reference/schemas/types/#reply-diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticMethods

Independent native method entries without positional pairing.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [Reply_DiagnosticCountUnit](/reference/schemas/types/#reply-diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [Reply_GtrMethod](/reference/schemas/types/#reply-gtrmethod) | Complete source-owned rows in native order; envelope totals count native rows. Optional. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
