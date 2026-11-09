---
title: Reply of biomcp get gwas (biomcp.gwas-ancestries/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: the fields
  of schema biomcp.gwas-ancestries/5."
goal: "Parse the biomcp.gwas-ancestries/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas-ancestries/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.gwas-ancestries/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gwas-ancestries/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GwasAncestries](#reply-gwasancestries). `query` holds [Reply_GwasGet](/reference/schemas/types/#reply-gwasget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_GwasAncestries

Bounded retained ancestry relation for one exact study.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_GwasGet](/reference/schemas/types/#reply-gwasget) | Retained relation continuation. Optional. |
| `items` | list of [Reply_GwasAncestry](#reply-gwasancestry) | Independent source rows. Optional. |
| `study` | [Reply_GwasStudyId](/reference/schemas/types/#reply-gwasstudyid) | Exact owning study. |

### Other types in this reply

2 more types appear only in this reply: [Reply_GwasAncestry](#reply-gwasancestry), [Reply_GwasCountry](#reply-gwascountry). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_GwasAncestry

One independent source ancestry row; counts are never summed.

| Field | Type | Meaning |
| --- | --- | --- |
| `ancestry_groups` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Original ancestry group labels. Optional. |
| `country_of_origin` | list of [Reply_GwasCountry](#reply-gwascountry) | Native origin metadata. Optional. |
| `country_of_recruitment` | list of [Reply_GwasCountry](#reply-gwascountry) | Native recruitment metadata. Optional. |
| `number_of_individuals` | integer or null | Source row count; no unique-person denominator is inferred. Optional. |
| `source_url` | [Reply_Text](/reference/schemas/types/#reply-text) | Native row self link, checked against its owning study by the adapter. |
| `stage` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native discovery or replication stage. Optional. |

### Reply_GwasCountry

Native country and region metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `country` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Original country label including NR. Optional. |
| `major_area` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native source major area when supplied. Optional. |
| `region` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Original region label when supplied. Optional. |

</details>

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
