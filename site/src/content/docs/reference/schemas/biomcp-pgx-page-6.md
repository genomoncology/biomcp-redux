---
title: Reply of biomcp search pgx (biomcp.pgx-page/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search pgx: the
  fields of schema biomcp.pgx-page/6."
goal: "Parse the biomcp.pgx-page/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-page/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search pgx`](/reference/commands/search-pgx/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.pgx-page/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-page/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxPage](#reply-pgxpage). `query` holds [Reply_PgxSearch](#reply-pgxsearch). `query` repeats the request as BioMCP read it.

### Reply_PgxPage

Independent source-ordered page of complete native records.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [Reply_PgxHeader](/reference/schemas/types/#reply-pgxheader) | Source and selected gene. |
| `items` | list of [Reply_PgxPair](#reply-pgxpair) | Complete native rows without duplicate summaries. Optional. |
| `page` | [Reply_PgxPageInfo](/reference/schemas/types/#reply-pgxpageinfo) | Meaning and bounds of this collection. |

### Reply_PgxSearch

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `cpic_level` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Literal native level, including compound levels such as B/C. Optional. |
| `gene` | [Reply_GeneSymbol](/reference/schemas/types/#reply-genesymbol) | Explicit source gene symbol. |
| `limit` | integer (uint32) | Rows per native page. Optional. |
| `offset` | integer (uint32) | Native source offset. Optional. |
| `source` | `"cpic"` | Required CPIC source; ClinPGx is not supported for search. |

### Other types in this reply

One more type appear only in this reply: [Reply_PgxPair](#reply-pgxpair). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_PgxPair

Source-native PgxPair fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `clinpgxlevel` | string or null | The source's own `clinpgxlevel` value, unchanged. Optional. |
| `cpiclevel` | string or null | The source's own `cpiclevel` value, unchanged. Optional. |
| `drugid` | [Reply_Curie](/reference/schemas/types/#reply-curie) or null | The source's own `drugid` value, unchanged. Optional. |
| `drugname` | string or null | The source's own `drugname` value, unchanged. Optional. |
| `genesymbol` | [Reply_GeneSymbol](/reference/schemas/types/#reply-genesymbol) | The source's own `genesymbol` value, unchanged. |
| `guidelinename` | string or null | The source's own `guidelinename` value, unchanged. Optional. |
| `guidelineurl` | string or null | The source's own `guidelineurl` value, unchanged. Optional. |
| `pairid` | [Reply_Identifier_int64](/reference/schemas/types/#reply-identifier-int64) | The source's own `pairid` value, unchanged. |
| `pgxtesting` | string or null | The source's own `pgxtesting` value, unchanged. Optional. |
| `pmids` | list of [Reply_Pmid](/reference/schemas/types/#reply-pmid) or null | The source's own `pmids` value, unchanged. Optional. |
| `provisional` | boolean or null | The source's own `provisional` value, unchanged. Optional. |
| `usedforrecommendation` | string or null | The source's own `usedforrecommendation` value, unchanged. Optional. |

</details>

## Next

[`biomcp search pgx`](/reference/commands/search-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
