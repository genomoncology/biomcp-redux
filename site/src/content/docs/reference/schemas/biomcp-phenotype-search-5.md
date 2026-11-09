---
title: Reply of biomcp search phenotype (biomcp.phenotype-search/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search phenotype:
  the fields of schema biomcp.phenotype-search/5."
goal: "Parse the biomcp.phenotype-search/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-search/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search phenotype`](/reference/commands/search-phenotype/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.phenotype-search/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.phenotype-search/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PhenotypePage](#reply-phenotypepage). `query` holds [Reply_PhenotypeSearch](#reply-phenotypesearch). `query` repeats the request as BioMCP read it.

### Reply_PhenotypePage

Native HP search header page, without local ranking.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable publisher component terms. Optional. |
| `fetched` | integer or null | Original source retrieval seconds, distinct from ontology release. Optional. |
| `items` | list of [Reply_PhenotypeHeader](#reply-phenotypeheader) | Native ordered rows; empty remains an empty collection. Optional. |

### Reply_PhenotypeSearch

Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum native headers, default five. Optional. |
| `offset` | integer (uint32) | Native source row offset. Optional. |
| `source` | [Reply_PhenotypeSource](/reference/schemas/types/#reply-phenotypesource) | Monarch only; HPO search is unsupported. Optional. |
| `text` | string | Unchanged literal term search text. |

### Other types in this reply

One more type appear only in this reply: [Reply_PhenotypeHeader](#reply-phenotypeheader). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_PhenotypeHeader

One native term search candidate, without a selected identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply-attribution) | Publisher identity and version when supplied. |
| `id` | [Reply_HpId](/reference/schemas/types/#reply-hpid) | Source HP identity. |
| `label` | string or null | Exact native label, including an empty value. Optional. |
| `match_score` | [Reply_FiniteNumber](/reference/schemas/types/#reply-finitenumber) or null | Native search match score with no local interpretation. Optional. |
| `match_type` | string or null | Native match type label. Optional. |
| `matched_field` | string or null | Native matched field name. Optional. |

</details>

## Next

[`biomcp search phenotype`](/reference/commands/search-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
