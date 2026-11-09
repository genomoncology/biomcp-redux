---
title: Reply of biomcp get diagnostic (biomcp.diagnostic-genes/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic: the
  fields of schema biomcp.diagnostic-genes/5."
goal: "Parse the biomcp.diagnostic-genes/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-genes/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.diagnostic-genes/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-genes/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticGenes](#reply_diagnosticgenes). `query` holds [Reply_DiagnosticGet](/reference/schemas/types/#reply_diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticGenes

Native GTR gene relation rows without variant-coverage inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [Reply_DiagnosticCountUnit](/reference/schemas/types/#reply_diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [Reply_GtrGeneRelation](/reference/schemas/types/#reply_gtrgenerelation) | Complete source-owned rows in native order; envelope totals count native rows. Optional. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
