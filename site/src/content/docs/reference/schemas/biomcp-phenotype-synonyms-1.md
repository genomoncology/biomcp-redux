---
title: Reply of biomcp get phenotype synonyms section (biomcp.phenotype-synonyms/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype
  synonyms: a real example and the fields of schema
  biomcp.phenotype-synonyms/1."
goal: "Parse the biomcp.phenotype-synonyms/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-synonyms/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype synonyms`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-synonyms/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0000001 synonyms
biomcp get phenotype HP:0001250 synonyms --source monarch --section-limit 3
```

`biomcp schema biomcp.phenotype-synonyms/1` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0000001 synonyms` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.phenotype-synonyms/1",
  "status": "absent",
  "query": {
    "id": "HP:0000001",
    "source": "hpo",
    "section": "synonyms",
    "section_offset": 0
  },
  "result": {
    "items": [],
    "components": [],
    "fetched": 1793140309
  },
  "page": {"offset": 0, "returned": 0, "total": 0},
  "sources": [
    {
      "source": "hpo",
      "use": "attribution",
      "terms": "HPO project custom license: attribution, version and integrity",
      "terms_url": "https://human-phenotype-ontology.github.io/license.html",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [PhenotypeSynonyms](#phenotypesynonyms). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeSynonyms

Source synonym rows preserving publisher classes.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable publisher component terms. |
| `fetched` | integer or null | no | Original source retrieval seconds, distinct from ontology release. |
| `items` | list of [PhenotypeSynonym](#phenotypesynonym) | yes | Native ordered rows; empty remains an empty collection. |

### PhenotypeSynonym

One native synonym; HPO values remain unclassified.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Publisher provenance. |
| `relationship` | [PhenotypeSynonymClass](#phenotypesynonymclass) or null | no | Native Monarch relationship class; absent for HPO. |
| `value` | string | yes | Whole source text. |

### PhenotypeSynonymClass

Publisher synonym relationship class.

| Value | Meaning |
| --- | --- |
| `exact` | Native exact synonym. |
| `broad` | Native broad synonym. |
| `narrow` | Native narrow synonym. |
| `related` | Native related synonym. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments.
