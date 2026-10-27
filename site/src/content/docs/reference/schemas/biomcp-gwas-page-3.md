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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search gwas`](/reference/commands/search-gwas/) and [`biomcp get gwas associations`](/reference/commands/get-gwas/) reply with `schema` set to `biomcp.gwas_page/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search gwas --variant rs7903146 --limit 3
biomcp get gwas GCST000035 associations
```

`biomcp schema biomcp.gwas_page/3` prints the full JSON Schema.

## Example reply

`biomcp search gwas --variant rs7903146 --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [GwasPage](#gwaspage). `query` holds [GwasPageQuery](#gwaspagequery). `query` repeats the request as BioMCP read it.

### GwasPage

One native association page, also used for a study associations section.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `continuation` | [GwasPageQuery](#gwaspagequery) | no | Same-selector search continuation. |
| `items` | list of [GwasAssociation](/reference/schemas/types/#gwasassociation) | yes | Shared complete association records. |
| `page` | [PageInfo](/reference/schemas/types/#pageinfo) | yes | Native page scope and total. |
| `qualifying_total` | integer or null | no | Complete qualifying count, unknown until the threshold walk reaches its end. |

### GwasPageQuery

Search and study-association pages echo their original checked request.

One of these forms:

- `operation` is `"search"`: Native association search. Fields: `query` ([GwasSearch](#gwassearch)).
- `operation` is `"get"`: Exact study associations relation. Fields: `query` ([GwasGet](/reference/schemas/types/#gwasget)).

### GwasOrder

Native source order, echoed without a configurable sort command.

| Value | Meaning |
| --- | --- |
| `p_value,asc` | Ascending source p value. |

### GwasSearch

Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `extended_geneset` | boolean | no | Source extended gene set; requires gene. |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | no | Exact native mapped gene; legacy positional query aliases this field. |
| `include_child_traits` | boolean | no | Source child-trait expansion; requires trait_id. |
| `limit` | integer (uint32) | no | Maximum association rows. |
| `offset` | integer (uint32) | no | Native association offset. |
| `order` | [GwasOrder](#gwasorder) | no | Source order; threshold requests use this same native order. |
| `p_value` | string | no | Inclusive positive probability in scientific notation, such as 1e-5. The qualifying total stays unknown until the ordered walk reaches its end. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Retained threshold walk identity; requires a threshold. |
| `pubmed` | [Pmid](/reference/schemas/types/#pmid) | no | Exact source publication identity. |
| `study` | [GwasStudyId](/reference/schemas/types/#gwasstudyid) | no | Exact source study accession. |
| `trait` | [Text](/reference/schemas/types/#text) | no | Literal native efo_trait label; never searches reported_trait prose. |
| `trait_id` | [GwasTraitId](#gwastraitid) | no | Exact native ontology identity. |
| `variant` | [RsId](#rsid) | no | Exact source rsID. |

### GwasTraitId

Type: string.

### RsId

Type: string.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search gwas`](/reference/commands/search-gwas/) gives the command's arguments.
