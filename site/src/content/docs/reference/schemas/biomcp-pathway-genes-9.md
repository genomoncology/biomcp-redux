---
title: Reply of biomcp pathway genes (biomcp.pathway-genes/9)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp pathway genes: the
  fields of schema biomcp.pathway-genes/9."
goal: "Parse the biomcp.pathway-genes/9 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway-genes/9
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp pathway genes`](/reference/commands/pathway-genes/) and [`biomcp pathway`](/reference/commands/pathway/) reply with `schema` set to `biomcp.pathway-genes/9`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pathway-genes/9` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PathwayGenes](/reference/schemas/types/#reply_pathwaygenes), a shared type. `query` holds [Reply_PathwayGenesQuery](#reply_pathwaygenesquery). `query` repeats the request as BioMCP read it.

### Reply_PathwayGenesQuery

Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `details` | [Reply_Details](#reply_details) | Adds each reference's Reactome `PhysicalEntity` contexts, with their source text whole. Optional. |
| `id` | string | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | Five complete relation rows by default, at most fifty. Optional. |
| `offset` | integer (uint32) | Offset within the retained selected relation. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) | Immutable relation snapshot. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_Details](#reply_details). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_Details

Explicit full physical-context disclosure; omission keeps membership compact.

Values: `details`.

</details>

## Next

[`biomcp pathway genes`](/reference/commands/pathway-genes/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
