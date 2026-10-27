---
title: Reply of biomcp get phenotype diseases section (biomcp.phenotype-diseases/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get phenotype
  diseases: a real example and the fields of schema
  biomcp.phenotype-diseases/3."
goal: "Parse the biomcp.phenotype-diseases/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-diseases/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get phenotype diseases`](/reference/commands/get-phenotype/) replies with `schema` set to `biomcp.phenotype-diseases/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get phenotype HP:0000001 diseases
biomcp get phenotype HP:0000001 diseases --source monarch
```

`biomcp schema biomcp.phenotype-diseases/3` prints the full JSON Schema.

## Example reply

`biomcp get phenotype HP:0000001 diseases --source monarch` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.phenotype-diseases/3",
  "status": "absent",
  "query": {
    "id": "HP:0000001",
    "source": "monarch",
    "section": "diseases",
    "section_offset": 0
  },
  "result": {
    "items": [],
    "direct": true,
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
  "page": {"offset": 0, "returned": 0, "total": 0},
  "sources": [
    {
      "source": "monarch",
      "use": "attribution",
      "terms": "Monarch source-specific licensing",
      "terms_url": "https://monarch-app.monarchinitiative.org/Licensing/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PhenotypeDiseases](#phenotypediseases). `query` holds [PhenotypeGet](/reference/schemas/types/#phenotypeget), a shared type. `query` repeats the request as BioMCP read it.

### PhenotypeDiseases

Direct source disease association rows without clinical inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable publisher component terms. |
| `direct` | boolean | Publisher direct graph scope; this supplies no causal assertion. |
| `fetched` | integer or null | Original source retrieval time, distinct from dataset release. Optional. |
| `items` | list of [DiseaseAssociationItem](/reference/schemas/types/#diseaseassociationitem) | Native association rows in source order, without deduplication. |

## Next

[`biomcp get phenotype`](/reference/commands/get-phenotype/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
