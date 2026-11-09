---
title: Reply of biomcp search disease (biomcp.disease-page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search disease: the
  fields of schema biomcp.disease-page/5."
goal: "Parse the biomcp.disease-page/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease-page/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search disease`](/reference/commands/search-disease/), [`biomcp get disease`](/reference/commands/get-disease/), [`biomcp search`](/reference/commands/search/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.disease-page/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.disease-page/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiseasePage](#reply_diseasepage). `query` holds [Reply_DiseaseSearch](/reference/schemas/types/#reply_diseasesearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiseasePage

One source-ordered disease candidate page.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component use terms, independent of row provenance. Optional. |
| `items` | list of [Reply_DiseaseHeader](#reply_diseaseheader) | Rows as the source gives them. Totals in `page` count rows, not distinct records. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_DiseaseHeader](#reply_diseaseheader). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_DiseaseHeader

One source candidate without a selected identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply_attribution) | Source record and version. |
| `concept` | [Reply_DiseaseConcept](/reference/schemas/types/#reply_diseaseconcept) | Native concept and optional exact ID. |
| `obsolete` | boolean or null | Publisher obsolete flag when supplied. Optional. |
| `ontology` | [Reply_DiseaseOntology](/reference/schemas/types/#reply_diseaseontology) | The explicit queried ontology. |

</details>

## Next

[`biomcp search disease`](/reference/commands/search-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
