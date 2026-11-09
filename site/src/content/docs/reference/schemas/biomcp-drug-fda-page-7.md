---
title: Reply of biomcp search drug (biomcp.drug-fda-page/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: the
  fields of schema biomcp.drug-fda-page/7."
goal: "Parse the biomcp.drug-fda-page/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-fda-page/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.drug-fda-page/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-fda-page/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugFdaPage](#reply_drugfdapage). `query` holds [Reply_DrugSearch](/reference/schemas/types/#reply_drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugFdaPage

A source-owned application page preserving complete product context and source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_DrugFdaCandidate](#reply_drugfdacandidate) | Native applications and products; a phrase match never establishes equivalence. Optional. |
| `source_updated` | [Reply_SourceUpdated](/reference/schemas/types/#reply_sourceupdated) or null | Native dataset last_updated date, independent of event dates and retrieval. Optional. |
| `total` | integer (uint64) | Native application total, independent of product count. |

### Other types in this reply

One more type appear only in this reply: [Reply_DrugFdaCandidate](#reply_drugfdacandidate). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_DrugFdaCandidate

One candidate application and its independent source-listed products.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_DrugId](/reference/schemas/types/#reply_drugid) | Exact application identifier, shared with the retained application identity. |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply_drugheader) | Exact native application identity; no product has been selected. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact brand wording shared by every product; omitted for zero or differing brands. Optional. |
| `products` | list of [Reply_DrugFdaProduct](/reference/schemas/types/#reply_drugfdaproduct) | Complete native product candidates within this application. Optional. |

</details>

## Next

[`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
