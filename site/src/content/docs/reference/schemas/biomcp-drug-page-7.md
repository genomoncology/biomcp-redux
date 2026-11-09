---
title: Reply of biomcp search drug (biomcp.drug-page/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: the
  fields of schema biomcp.drug-page/7."
goal: "Parse the biomcp.drug-page/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-page/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.drug-page/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-page/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugPage](#reply-drugpage). `query` holds [Reply_DrugSearch](/reference/schemas/types/#reply-drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugPage

A single source candidate page in native order, without selecting the first hit.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_DrugHeader](/reference/schemas/types/#reply-drugheader) | Native candidates, preserving their separate identity levels. Optional. |
| `total` | integer or null | Native total when supplied; absent remains unknown. Optional. |

## Next

[`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
