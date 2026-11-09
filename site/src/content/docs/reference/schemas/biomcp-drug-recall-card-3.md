---
title: Reply of biomcp get drug (biomcp.drug-recall-card/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-recall-card/3."
goal: "Parse the biomcp.drug-recall-card/3 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-recall-card/3
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-recall-card/3`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-recall-card/3` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugRecallCard](#reply_drugrecallcard). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply_drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugRecallCard

Native detail fields from the selected recall record.

| Field | Type | Meaning |
| --- | --- | --- |
| `code_info` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published product code information. Optional. |
| `distribution_pattern` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published distribution pattern. Optional. |
| `header` | [Reply_DrugRecallHeader](/reference/schemas/types/#reply_drugrecallheader) | Native recall header. |
| `reason` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Published reason for recall. Optional. |
| `voluntary_mandated` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native voluntary or mandated status. Optional. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
