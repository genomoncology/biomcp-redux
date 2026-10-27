---
title: Reply of biomcp get disease genes section (biomcp.disease_genes/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease genes: a
  real example and the fields of schema biomcp.disease_genes/5."
goal: "Parse the biomcp.disease_genes/5 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_genes/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease genes`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_genes/5`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105 genes --source monarch --section-limit 3
```

`biomcp schema biomcp.disease_genes/5` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105 genes --source monarch --section-limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.disease_genes/5",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "monarch",
    "section": "genes",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "items": [
      {
        "association": {…},
        "components": […]
      },
      … 2 more
    ],
    "components": [
      {
        "component": "hpo",
        "terms": {…}
      },
      … 1 more
    ],
    "fetched": 1793140301
  },
  "page": {"offset": 0, "returned": 3, "total": 11},
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
      "args": ["get", "gene", "HGNC:950"]
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [DiseaseGenes](#diseasegenes). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseGenes

Native association rows, not a unique-gene count.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | no | Original response retrieval Unix seconds, independent of local delivery. |
| `items` | list of [DiseaseAssociationItem](/reference/schemas/types/#diseaseassociationitem) | yes | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments.
