---
title: Reply of biomcp get phenotype hierarchy section (biomcp.phenotype-hierarchy/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype
  hierarchy: a real example and the fields of schema
  biomcp.phenotype-hierarchy/1."
goal: "Parse the biomcp.phenotype-hierarchy/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-hierarchy/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype hierarchy`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-hierarchy/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0001250 hierarchy --source monarch --section-limit 3
```

`biomcp schema biomcp.phenotype-hierarchy/1` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0001250 hierarchy --source monarch --section-limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.phenotype-hierarchy/1",
  "status": "complete",
  "query": {
    "id": "HP:0001250",
    "source": "monarch",
    "section": "hierarchy",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "items": [
      {
        "id": "HP:0012638",
        "label": "Abnormal nervous system physiology",
        "relation": "super_classes",
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
  "page": {"offset": 0, "returned": 3, "total": 13},
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
      "kind": "page",
      "args": [
        "get", "phenotype", "HP:0001250", "hierarchy", "--section-limit", "3",
        "--section-offset", "3",
        … 2 more
      ]
    }
  ]
}
```

## Fields

`result` holds [PhenotypeHierarchy](#phenotypehierarchy). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeHierarchy

Direct source hierarchy links without graph traversal.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable publisher component terms. |
| `fetched` | integer or null | no | Original source retrieval seconds, distinct from ontology release. |
| `items` | list of [PhenotypeHierarchyLink](#phenotypehierarchylink) | yes | Native ordered rows; empty remains an empty collection. |

### PhenotypeHierarchyLink

One native direct hierarchy link, without traversal or inferred equivalence.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Publisher provenance. |
| `id` | [Curie](/reference/schemas/types/#curie) | yes | Exact related term identifier; native parents can be outside HP. |
| `label` | string | yes | Whole related term label. |
| `relation` | [PhenotypeHierarchyRelation](#phenotypehierarchyrelation) | yes | Original source relation, such as super_classes or sub_classes. |

### PhenotypeHierarchyRelation

Native hierarchy collection that supplies the link.

| Value | Meaning |
| --- | --- |
| `super_classes` | A direct parent in the native super_classes collection. |
| `sub_classes` | A direct child in the native sub_classes collection. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments.
