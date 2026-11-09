---
title: Reply of biomcp get phenotype xrefs section (biomcp.phenotype-xrefs/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype xrefs:
  a real example and the fields of schema biomcp.phenotype-xrefs/1."
goal: "Parse the biomcp.phenotype-xrefs/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-xrefs/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype xrefs`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-xrefs/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0000001 xrefs
biomcp get phenotype HP:0001250 xrefs --source monarch --section-limit 3
```

`biomcp schema biomcp.phenotype-xrefs/1` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0000001 xrefs` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.phenotype-xrefs/1",
  "status": "absent",
  "query": {
    "id": "HP:0000001",
    "source": "hpo",
    "section": "xrefs",
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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PhenotypeXrefs](#phenotypexrefs). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeXrefs

Source reference rows without equivalence promotion.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable publisher component terms. |
| `fetched` | integer or null | Original source retrieval seconds, distinct from ontology release. Optional. |
| `items` | list of [PhenotypeXref](#phenotypexref) | Native ordered rows; empty remains an empty collection. |

### Other types in this reply

One more type appear only in this reply: [PhenotypeXref](#phenotypexref). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### PhenotypeXref

One exact native cross reference, without equivalence promotion.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Publisher provenance. |
| `id` | [Curie](/reference/schemas/types/#curie) | Exact publisher reference. |
| `predicate` | [Curie](/reference/schemas/types/#curie) or null | Native predicate when actually supplied. Optional. |

</details>

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
