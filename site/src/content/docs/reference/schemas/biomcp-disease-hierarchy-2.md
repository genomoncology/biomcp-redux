---
title: Reply of biomcp get disease hierarchy section (biomcp.disease_hierarchy/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease
  hierarchy: a real example and the fields of schema
  biomcp.disease_hierarchy/2."
goal: "Parse the biomcp.disease_hierarchy/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_hierarchy/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease hierarchy`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_hierarchy/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105 hierarchy
biomcp get disease MONDO:0005105 hierarchy --source mydisease
```

`biomcp schema biomcp.disease_hierarchy/2` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105 hierarchy` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.disease_hierarchy/2",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "mydisease",
    "section": "hierarchy",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "items": [
      {
        "subject": {"id": "MONDO:0005105", "label": "melanoma"},
        "predicate": "mydisease:mondo.parents",
        "object": {"id": "MONDO:0021143"},
        "attribution": {…}
      },
      … 1 more
    ],
    "components": [
      {
        "component": "mondo",
        "terms": {…}
      },
      … 1 more
    ]
  },
  "page": {"offset": 0, "returned": 2, "total": 2},
  "sources": [
    {
      "source": "mydisease",
      "use": "attribution",
      "terms": "MyDisease source-specific terms",
      "terms_url": "https://docs.mydisease.info/en/latest/doc/data.html",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiseaseHierarchy](#diseasehierarchy). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseHierarchy

Native parent relations with both endpoints.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component use terms, independent of row provenance. |
| `items` | list of [DiseaseRelation](/reference/schemas/types/#diseaserelation) | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
