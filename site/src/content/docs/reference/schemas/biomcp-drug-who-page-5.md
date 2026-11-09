---
title: Reply of biomcp search drug (biomcp.drug-who-page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: the
  fields of schema biomcp.drug-who-page/5."
goal: "Parse the biomcp.drug-who-page/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-who-page/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.drug-who-page/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-who-page/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugWhoPage](#reply_drugwhopage). `query` holds [Reply_DrugSearch](/reference/schemas/types/#reply_drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugWhoPage

Source-ordered WHO matches; prequalification does not establish US/EU authorization.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_DrugWhoRow](/reference/schemas/types/#reply_drugwhorow) | Bounded native rows in export order. Optional. |
| `total` | integer (uint64) | Matching rows in the complete local snapshot. |

## Next

[`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
