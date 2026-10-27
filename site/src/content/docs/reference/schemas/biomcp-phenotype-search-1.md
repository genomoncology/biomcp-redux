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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search phenotype`](/reference/commands/search-phenotype/) replies with `schema` set to `biomcp.phenotype-search/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search phenotype seizure --limit 3
biomcp search phenotype synthetic
```

`biomcp schema biomcp.phenotype-search/1` prints the full JSON Schema.

## Example reply

`biomcp search phenotype seizure --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…},
      … 2 more
    ],
    "components": [
      {…},
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
    {"kind": "disambiguate", "args": ["get", "phenotype", "HP:0001250"]},
    … 3 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PhenotypePage](#phenotypepage). `query` holds [PhenotypeSearch](#phenotypesearch). `query` repeats the request as BioMCP read it.

### PhenotypePage

Native HP search header page, without local ranking.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable publisher component terms. |
| `fetched` | integer or null | Original source retrieval seconds, distinct from ontology release. Optional. |
| `items` | list of [PhenotypeHeader](#phenotypeheader) | Native ordered rows; empty remains an empty collection. |

### PhenotypeSearch

Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum native headers, default five. Optional. |
| `offset` | integer (uint32) | Native source row offset. Optional. |
| `source` | [PhenotypeSource](/reference/schemas/types/#phenotypesource) | Monarch only; HPO search is unsupported. Optional. |
| `text` | string | Unchanged literal term search text. |

### Other types in this reply

One more type appear only in this reply: [PhenotypeHeader](#phenotypeheader). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### PhenotypeHeader

One native term search candidate, without a selected identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Publisher identity and version when supplied. |
| `id` | [HpId](/reference/schemas/types/#hpid) | Source HP identity. |
| `label` | string | Exact native label, including an empty value. |
| `match_score` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | Native search match score with no local interpretation. Optional. |
| `match_type` | string or null | Native match type label. Optional. |
| `matched_field` | string or null | Native matched field name. Optional. |

</details>

## Next

[`biomcp search phenotype`](/reference/commands/search-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
