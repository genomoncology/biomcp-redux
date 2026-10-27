---
title: Reply of biomcp search gwas (biomcp.gwas_page/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search gwas: a real
  example and the fields of schema biomcp.gwas_page/3."
goal: "Parse the biomcp.gwas_page/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search gwas`](/reference/commands/search-gwas/) and [`biomcp get gwas associations`](/reference/commands/get-gwas/) reply with `schema` set to `biomcp.gwas_page/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search gwas --variant rs7903146 --limit 3
biomcp get gwas GCST000035 associations
```

`biomcp schema biomcp.gwas_page/3` prints the full JSON Schema.

## Example reply

`biomcp search gwas --variant rs7903146 --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.gwas_page/3",
  "status": "complete",
  "query": {
    "operation": "search",
    "query": {
      "variant": "rs7903146",
      "include_child_traits": false,
      "extended_geneset": false,
      "order": "p_value,asc",
      "limit": 3,
      "offset": 0
    }
  },
  "result": {
    "items": [
      {…},
      … 2 more
    ],
    "page": {"offset": 0, "returned": 3, "total": 353},
    "qualifying_total": null,
    "continuation": {
      "operation": "search",
      "query": {…}
    }
  },
  "page": {"offset": 0, "returned": 3, "total": 353},
  "sources": [
    {
      "source": "gwas",
      "use": "restricted",
      "terms": "GWAS Catalog terms and dataset rights",
      "terms_url": "https://www.ebi.ac.uk/about/terms-of-use/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": ["search", "gwas", "--offset", "3", "--variant", "rs7903146"]
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [GwasPage](#gwaspage). `query` holds [GwasPageQuery](#gwaspagequery). `query` repeats the request as BioMCP read it.

### GwasPage

One native association page, also used for a study associations section.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [GwasPageQuery](#gwaspagequery) | Same-selector search continuation. Optional. |
| `items` | list of [GwasAssociation](/reference/schemas/types/#gwasassociation) | Shared complete association records. |
| `page` | [PageInfo](/reference/schemas/types/#pageinfo) | Native page scope and total. |
| `qualifying_total` | integer or null | Complete qualifying count, unknown until the threshold walk reaches its end. Optional. |

### GwasPageQuery

Search and study-association pages echo their original checked request.

One of these forms:

- `operation` is `"search"`: Native association search. Fields: `query` ([GwasSearch](#gwassearch)).
- `operation` is `"get"`: Exact study associations relation. Fields: `query` ([GwasGet](/reference/schemas/types/#gwasget)).

### Other types in this reply

4 more types appear only in this reply: [GwasOrder](#gwasorder), [GwasSearch](#gwassearch), [GwasTraitId](#gwastraitid), [RsId](#rsid). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### GwasOrder

Native source order, echoed without a configurable sort command.

| Value | Meaning |
| --- | --- |
| `p_value,asc` | Ascending source p value. |

### GwasSearch

Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

| Field | Type | Meaning |
| --- | --- | --- |
| `extended_geneset` | boolean | Source extended gene set; requires gene. Optional. |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | Exact native mapped gene; legacy positional query aliases this field. Optional. |
| `include_child_traits` | boolean | Source child-trait expansion; requires trait_id. Optional. |
| `limit` | integer (uint32) | Maximum association rows. Optional. |
| `offset` | integer (uint32) | Native association offset. Optional. |
| `order` | [GwasOrder](#gwasorder) | Source order; threshold requests use this same native order. Optional. |
| `p_value` | string | Inclusive positive probability in scientific notation, such as 1e-5. The qualifying total stays unknown until the ordered walk reaches its end. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Retained threshold walk identity; requires a threshold. Optional. |
| `pubmed` | [Pmid](/reference/schemas/types/#pmid) | Exact source publication identity. Optional. |
| `study` | [GwasStudyId](/reference/schemas/types/#gwasstudyid) | Exact source study accession. Optional. |
| `trait` | [Text](/reference/schemas/types/#text) | Literal native efo_trait label; never searches reported_trait prose. Optional. |
| `trait_id` | [GwasTraitId](#gwastraitid) | Exact native ontology identity. Optional. |
| `variant` | [RsId](#rsid) | Exact source rsID. Optional. |

### GwasTraitId

Type: string.

### RsId

Type: string.

</details>

## Next

[`biomcp search gwas`](/reference/commands/search-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
