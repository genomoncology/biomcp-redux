---
title: Reply of biomcp get disease phenotypes section (biomcp.disease_phenotypes/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease
  phenotypes: a real example and the fields of schema
  biomcp.disease_phenotypes/5."
goal: "Parse the biomcp.disease_phenotypes/5 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_phenotypes/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease phenotypes`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_phenotypes/5`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105 phenotypes --source monarch --section-limit 3
```

`biomcp schema biomcp.disease_phenotypes/5` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105 phenotypes --source monarch --section-limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.disease_phenotypes/5",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "monarch",
    "section": "phenotypes",
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
    "fetched": 1793140302
  },
  "page": {"offset": 0, "returned": 3, "total": 15},
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
        "get", "disease", "MONDO:0005105", "phenotypes", "--section-limit", "3",
        "--section-offset", "3",
        … 2 more
      ]
    }
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiseasePhenotypes](#diseasephenotypes). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseasePhenotypes

Native phenotype association rows without diagnostic inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | Original response retrieval Unix seconds, independent of local delivery. Optional. |
| `items` | list of [DiseaseAssociationItem](/reference/schemas/types/#diseaseassociationitem) | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
