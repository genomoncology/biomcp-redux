---
title: Reply of biomcp get disease (biomcp.disease-synonyms/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease: the
  fields of schema biomcp.disease-synonyms/5."
goal: "Parse the biomcp.disease-synonyms/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease-synonyms/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease`](/reference/commands/get-disease/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.disease-synonyms/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.disease-synonyms/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiseaseSynonyms](#reply-diseasesynonyms). `query` holds [Reply_DiseaseGet](/reference/schemas/types/#reply-diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiseaseSynonyms

One publisher synonym page with scope and language.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable component use terms, independent of row provenance. Optional. |
| `items` | list of [Reply_DiseaseSynonym](#reply-diseasesynonym) | Rows as the source gives them. Totals in `page` count rows, not distinct records. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_DiseaseSynonym](#reply-diseasesynonym). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_DiseaseSynonym

A publisher synonym with its scope and language.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply-attribution) | Publisher record and version. |
| `language` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source language tag. Optional. |
| `scope` | [Reply_Curie](/reference/schemas/types/#reply-curie) or null | Publisher scope predicate. Optional. |
| `value` | [Reply_Text](/reference/schemas/types/#reply-text) | Complete synonym text. |

</details>

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
