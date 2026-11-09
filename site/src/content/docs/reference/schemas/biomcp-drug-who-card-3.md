---
title: Reply of biomcp get drug (biomcp.drug-who-card/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-who-card/3."
goal: "Parse the biomcp.drug-who-card/3 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-who-card/3
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-who-card/3`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-who-card/3` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugWhoCard](#reply_drugwhocard). `query` holds [Reply_DrugWhoGet](#reply_drugwhoget). `query` repeats the request as BioMCP read it.

### Reply_DrugWhoCard

Source-scoped native rows for one exact WHO reference.

| Field | Type | Meaning |
| --- | --- | --- |
| `native_rows` | list of [Reply_DrugWhoRow](/reference/schemas/types/#reply_drugwhorow) | Every native row for that reference, without invented fields. Optional. |
| `reference` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact requested and source-matched reference. |

### Reply_DrugWhoGet

WHO exact-reference selector keeps the shared DrugId contract unchanged.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact WHO reference number, including native spaces; no name or alias resolution. |
| `source` | [Reply_DrugSource](/reference/schemas/types/#reply_drugsource) | Explicit source who-pq; prequalification is source-scoped. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
