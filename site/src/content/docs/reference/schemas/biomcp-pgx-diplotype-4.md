---
title: Reply of biomcp get pgx (biomcp.pgx-diplotype/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: the fields
  of schema biomcp.pgx-diplotype/4."
goal: "Parse the biomcp.pgx-diplotype/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-diplotype/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pgx-diplotype/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-diplotype/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxDiplotype](#reply-pgxdiplotype). `query` holds [Reply_PgxGet](/reference/schemas/types/#reply-pgxget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PgxDiplotype

Every source-returned mapping for the exact declared expression.

| Field | Type | Meaning |
| --- | --- | --- |
| `diplotypes` | list of [Reply_PgxDiplotypeResult](#reply-pgxdiplotyperesult) | Zero, one or multiple native mappings; never a selected patient phenotype. Optional. |
| `header` | [Reply_PgxHeader](/reference/schemas/types/#reply-pgxheader) | Explicit gene and source. |

### Other types in this reply

One more type appear only in this reply: [Reply_PgxDiplotypeResult](#reply-pgxdiplotyperesult). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_PgxDiplotypeResult

Source-native PgxDiplotypeResult fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `activityvalue1` | string or null | The source's own `activityvalue1` value, unchanged. Optional. |
| `activityvalue2` | string or null | The source's own `activityvalue2` value, unchanged. Optional. |
| `consultationtext` | string or null | The source's own `consultationtext` value, unchanged. Optional. |
| `description` | string or null | The source's own `description` value, unchanged. Optional. |
| `diplotype` | string | The source's own `diplotype` value, unchanged. |
| `diplotypekey` | object or null | The source's own `diplotypekey` value, unchanged. Optional. |
| `ehrpriority` | string or null | The source's own `ehrpriority` value, unchanged. Optional. |
| `function1` | string or null | The source's own `function1` value, unchanged. Optional. |
| `function2` | string or null | The source's own `function2` value, unchanged. Optional. |
| `generesult` | string or null | The source's own `generesult` value, unchanged. Optional. |
| `genesymbol` | [Reply_GeneSymbol](/reference/schemas/types/#reply-genesymbol) | The source's own `genesymbol` value, unchanged. |
| `lookupkey` | object or null | The source's own `lookupkey` value, unchanged. Optional. |
| `totalactivityscore` | string or null | The source's own `totalactivityscore` value, unchanged. Optional. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
