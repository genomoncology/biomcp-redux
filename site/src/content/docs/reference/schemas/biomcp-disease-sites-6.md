---
title: Reply of biomcp search disease (biomcp.disease-sites/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search disease: the
  fields of schema biomcp.disease-sites/6."
goal: "Parse the biomcp.disease-sites/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease-sites/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search disease`](/reference/commands/search-disease/), [`biomcp get disease`](/reference/commands/get-disease/), [`biomcp search`](/reference/commands/search/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.disease-sites/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.disease-sites/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiseaseSites](#reply-diseasesites). `query` holds [Reply_DiseaseSearch](/reference/schemas/types/#reply-diseasesearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiseaseSites

Publisher site catalog without an ontology-to-site mapping.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable component use terms, independent of row provenance. Optional. |
| `fetched` | integer or null | Original response retrieval time in Unix seconds, distinct from publisher version/date. Optional. |
| `items` | list of [Reply_DiseaseSite](#reply-diseasesite) | Rows as the source gives them. Totals in `page` count rows, not distinct records. Optional. |
| `selectors` | list of [Reply_StatisticSelector](/reference/schemas/types/#reply-statisticselector) | Publisher labels for required dataset, incidence-period and survival-interval choices. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_DiseaseSite](#reply-diseasesite). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_DiseaseSite

One publisher catalog site, independent of ontology identities.

| Field | Type | Meaning |
| --- | --- | --- |
| `active` | boolean or null | Publisher active-state flag. Optional. |
| `code` | [Reply_Text](/reference/schemas/types/#reply-text) | Native publisher site code. |
| `label` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Complete publisher site label. Optional. |

</details>

## Next

[`biomcp search disease`](/reference/commands/search-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
