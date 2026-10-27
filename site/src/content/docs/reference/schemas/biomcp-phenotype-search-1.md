---
title: Reply of biomcp search phenotype (biomcp.phenotype-search/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search phenotype: a
  real example and the fields of schema biomcp.phenotype-search/1."
goal: "Parse the biomcp.phenotype-search/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-search/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search phenotype`](/reference/commands/search-phenotype/) replies with `schema` set to `biomcp.phenotype-search/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search phenotype seizure --limit 3
biomcp search phenotype synthetic
```

`biomcp schema biomcp.phenotype-search/1` prints the full JSON Schema.

## Example reply

`biomcp search phenotype seizure --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.phenotype-search/1",
  "status": "complete",
  "query": {
    "text": "seizure",
    "source": "monarch",
    "limit": 3,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "id": "HP:0001250",
        "label": "Seizure",
        "match_score": 29.591873,
        "matched_field": "name",
        "match_type": "exact",
        "attribution": {
          "source": "monarch",
          "record": "HP:0001250",
          "url": "https://monarchinitiative.org/HP:0001250/"
        }
      },
      … 2 more
    ],
    "components": [
      {
        "component": "hpo",
        "terms": {
          "usage": "attribution",
          "name": "HPO project custom license: attribution, version and integ…",
          "url": "https://human-phenotype-ontology.github.io/license.html"
        }
      },
      … 1 more
    ],
    "fetched": 1793140310
  },
  "page": {"offset": 0, "returned": 3, "total": 355},
  "sources": [
    {
      "source": "monarch",
      "use": "attribution",
      "terms": "Monarch source-specific licensing",
      "terms_url": "https://monarch-app.monarchinitiative.org/Licensing/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "phenotype", "HP:0001250"]
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [PhenotypePage](#phenotypepage). `query` holds [PhenotypeSearch](#phenotypesearch). `query` repeats the request as BioMCP read it.

### PhenotypePage

Native HP search header page, without local ranking.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable publisher component terms. |
| `fetched` | integer or null | no | Original source retrieval seconds, distinct from ontology release. |
| `items` | list of [PhenotypeHeader](#phenotypeheader) | yes | Native ordered rows; empty remains an empty collection. |

### PhenotypeSearch

Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum native headers, default five. |
| `offset` | integer (uint32) | no | Native source row offset. |
| `source` | [PhenotypeSource](/reference/schemas/types/#phenotypesource) | no | Monarch only; HPO search is unsupported. |
| `text` | string | yes | Unchanged literal term search text. |

### PhenotypeHeader

One native term search candidate, without a selected identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Publisher identity and version when supplied. |
| `id` | [HpId](/reference/schemas/types/#hpid) | yes | Source HP identity. |
| `label` | string | yes | Exact native label, including an empty value. |
| `match_score` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | no | Native search match score with no local interpretation. |
| `match_type` | string or null | no | Native match type label. |
| `matched_field` | string or null | no | Native matched field name. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search phenotype`](/reference/commands/search-phenotype/) gives the command's arguments.
