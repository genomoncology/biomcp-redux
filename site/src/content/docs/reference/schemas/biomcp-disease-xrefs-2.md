---
title: Reply of biomcp get disease xrefs section (biomcp.disease_xrefs/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease xrefs: a
  real example and the fields of schema biomcp.disease_xrefs/2."
goal: "Parse the biomcp.disease_xrefs/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_xrefs/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease xrefs`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_xrefs/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105 xrefs
biomcp get disease MONDO:0005105 xrefs --source mydisease
```

`biomcp schema biomcp.disease_xrefs/2` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105 xrefs` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.disease_xrefs/2",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "mydisease",
    "section": "xrefs",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "items": [
      {
        "subject": {"id": "MONDO:0005105", "label": "melanoma"},
        "predicate": "mydisease:mondo.xrefs.doid",
        "object": {"id": "DOID:1909"},
        "attribution": {
          "source": "mondo",
          "record": "MONDO:0005105",
          "url": "https://mydisease.info/v1/disease/MONDO:0005105"
        }
      },
      … 4 more
    ],
    "components": [
      {
        "component": "mondo",
        "terms": {
          "usage": "attribution",
          "name": "MONDO CC BY 4.0",
          "url": "https://mondo.monarchinitiative.org/pages/download/"
        }
      },
      … 1 more
    ]
  },
  "page": {"offset": 0, "returned": 5, "total": 19},
  "sources": [
    {
      "source": "mydisease",
      "use": "attribution",
      "terms": "MyDisease source-specific terms",
      "terms_url": "https://docs.mydisease.info/en/latest/doc/data.html",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "disease", "MONDO:0005105", "xrefs", "--section-limit", "5",
        "--section-offset", "5"
      ]
    }
  ]
}
```

## Fields

`result` holds [DiseaseXrefs](#diseasexrefs). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseXrefs

Native cross-reference predicates without equivalence promotion.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component use terms, independent of row provenance. |
| `items` | list of [DiseaseRelation](/reference/schemas/types/#diseaserelation) | yes | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments.
