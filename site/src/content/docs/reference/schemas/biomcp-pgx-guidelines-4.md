---
title: Reply of biomcp get pgx (biomcp.pgx-guidelines/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: the fields
  of schema biomcp.pgx-guidelines/4."
goal: "Parse the biomcp.pgx-guidelines/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-guidelines/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pgx-guidelines/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-guidelines/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxGuidelines](#reply-pgxguidelines). `query` holds [Reply_PgxGet](/reference/schemas/types/#reply-pgxget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PgxGuidelines

Independent source-ordered page of complete native records.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [Reply_PgxHeader](/reference/schemas/types/#reply-pgxheader) | Source and selected gene. |
| `items` | list of [Reply_PgxGuidelineReference](#reply-pgxguidelinereference) | Complete native rows without duplicate summaries. Optional. |
| `page` | [Reply_PgxPageInfo](/reference/schemas/types/#reply-pgxpageinfo) | Meaning and bounds of this collection. |

### Other types in this reply

2 more types appear only in this reply: [Reply_PgxGuidelineGene](#reply-pgxguidelinegene), [Reply_PgxGuidelineReference](#reply-pgxguidelinereference). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_PgxGuidelineGene

Source-native PgxGuidelineGene fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `symbol` | [Reply_GeneSymbol](/reference/schemas/types/#reply-genesymbol) | The source's own `symbol` value, unchanged. |
| `url` | string or null | The source's own `url` value, unchanged. Optional. |

### Reply_PgxGuidelineReference

Source-native PgxGuidelineReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `drugs` | list of string | The source's own `drugs` value, unchanged. Optional. |
| `genes` | list of [Reply_PgxGuidelineGene](#reply-pgxguidelinegene) | The source's own `genes` value, unchanged. Optional. |
| `guideline_name` | string | The source's own `guideline_name` value, unchanged. |
| `guideline_url` | string or null | The source's own `guideline_url` value, unchanged. Optional. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
