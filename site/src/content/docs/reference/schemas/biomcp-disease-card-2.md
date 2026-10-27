---
title: Reply of biomcp get disease (biomcp.disease_card/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease: a real
  example and the fields of schema biomcp.disease_card/2."
goal: "Parse the biomcp.disease_card/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_card/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_card/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105
biomcp get disease MONDO:0005105 --source mydisease
```

`biomcp schema biomcp.disease_card/2` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.disease_card/2",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "mydisease",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "requested_id": "MONDO:0005105",
    "returned_id": "MONDO:0005105",
    "concept": {"id": "MONDO:0005105", "label": "melanoma"},
    "definition": "A malignant, usually aggressive tumor composed of atypical…",
    "replaced_by": null,
    "attribution": {
      "source": "mydisease",
      "record": "MONDO:0005105",
      "version": "1",
      "url": "https://mydisease.info/v1/disease/MONDO:0005105"
    },
    "sections": ["synonyms", "xrefs", "hierarchy"],
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
      "kind": "section",
      "args": [
        "get", "disease", "MONDO:0005105", "synonyms", "--section-limit", "5"
      ]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [DiseaseCard](#diseasecard). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseCard

Exact identity and available pages without hidden enrichment.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alias` | [DiseaseRelation](/reference/schemas/types/#diseaserelation) or null | no | Native relation proving a source-accepted alternate identifier. |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Publisher record and version. |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `concept` | [Concept](/reference/schemas/types/#concept) | yes | Native identity and complete primary label. |
| `consider` | list of [Curie](/reference/schemas/types/#curie) or null | no | Publisher alternatives, never an automatic replacement. |
| `definition` | [Text](/reference/schemas/types/#text) or null | no | Complete source definition when supplied. |
| `obsolete` | boolean or null | no | Publisher obsolete flag. |
| `replaced_by` | list of [Curie](/reference/schemas/types/#curie) or null | no | Publisher explicit replacement identifiers; null preserves no native replacement. |
| `requested_id` | [Curie](/reference/schemas/types/#curie) | yes | Exact requested identifier. |
| `returned_id` | [Curie](/reference/schemas/types/#curie) | yes | Actual returned identifier. |
| `sections` | list of [DiseaseSection](/reference/schemas/types/#diseasesection) | yes | Available named sections. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments.
