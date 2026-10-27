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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype hierarchy`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-hierarchy/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0001250 hierarchy --source monarch --section-limit 3
```

`biomcp schema biomcp.phenotype-hierarchy/1` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0001250 hierarchy --source monarch --section-limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…},
      … 2 more
    ],
    "components": [
      {…},
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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PhenotypeHierarchy](#phenotypehierarchy). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeHierarchy

Direct source hierarchy links without graph traversal.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable publisher component terms. |
| `fetched` | integer or null | Original source retrieval seconds, distinct from ontology release. Optional. |
| `items` | list of [PhenotypeHierarchyLink](#phenotypehierarchylink) | Native ordered rows; empty remains an empty collection. |

### PhenotypeHierarchyLink

One native direct hierarchy link, without traversal or inferred equivalence.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Publisher provenance. |
| `id` | [Curie](/reference/schemas/types/#curie) | Exact related term identifier; native parents can be outside HP. |
| `label` | string | Whole related term label. |
| `relation` | [PhenotypeHierarchyRelation](#phenotypehierarchyrelation) | Original source relation, such as super_classes or sub_classes. |

### PhenotypeHierarchyRelation

Native hierarchy collection that supplies the link.

| Value | Meaning |
| --- | --- |
| `super_classes` | A direct parent in the native super_classes collection. |
| `sub_classes` | A direct child in the native sub_classes collection. |

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
