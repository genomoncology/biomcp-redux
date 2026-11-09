---
title: Reply of biomcp search drug (biomcp.drug-recall-page/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: the
  fields of schema biomcp.drug-recall-page/4."
goal: "Parse the biomcp.drug-recall-page/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-recall-page/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.drug-recall-page/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-recall-page/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugRecallPage](#reply-drugrecallpage). `query` holds [Reply_DrugSearch](/reference/schemas/types/#reply-drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugRecallPage

A bounded page; totals count recall records.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [Reply_DrugRecallCountUnit](#reply-drugrecallcountunit) | Native count unit; no event deduplication or patient count. |
| `items` | list of [Reply_DrugRecallHeader](/reference/schemas/types/#reply-drugrecallheader) | Native source order, without event deduplication. Optional. |
| `total` | integer (uint64) | Number of native recall records. |

### Other types in this reply

One more type appear only in this reply: [Reply_DrugRecallCountUnit](#reply-drugrecallcountunit). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_DrugRecallCountUnit

The native unit counted by drug enforcement searches.

Type: string.

</details>

## Next

[`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
