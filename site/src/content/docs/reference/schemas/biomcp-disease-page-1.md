---
title: Reply of biomcp search disease (biomcp.disease_page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search disease: a
  real example and the fields of schema biomcp.disease_page/1."
goal: "Parse the biomcp.disease_page/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search disease`](/reference/commands/search-disease/) replies with `schema` set to `biomcp.disease_page/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search disease melanoma --limit 3
```

`biomcp schema biomcp.disease_page/1` prints the full JSON Schema.

## Example reply

`biomcp search disease melanoma --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.disease_page/1",
  "status": "complete",
  "query": {
    "text": "melanoma",
    "source": "ols",
    "ontology": "mondo",
    "limit": 3,
    "offset": 0
  },
  "result": {
    "items": [
      {…},
      … 2 more
    ],
    "components": [
      {…}
    ]
  },
  "page": {"offset": 0, "returned": 3, "total": 158},
  "sources": [
    {
      "source": "ols",
      "use": "attribution",
      "terms": "EMBL-EBI Terms of Use; ontology-specific rights",
      "terms_url": "https://www.ebi.ac.uk/about/terms-of-use/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": [
        "get", "disease", "MONDO:0005105", "--section-limit", "5", "--source",
        "ols"
      ]
    },
    … 3 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiseasePage](#diseasepage). `query` holds [DiseaseSearch](/reference/schemas/types/#diseasesearch), a shared type. `query` repeats the request as BioMCP read it.

### DiseasePage

One source-ordered disease candidate page.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component use terms, independent of row provenance. |
| `items` | list of [DiseaseHeader](#diseaseheader) | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

### DiseaseHeader

One source candidate without a selected identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Source record and version. |
| `concept` | [Concept](/reference/schemas/types/#concept) | Native concept and optional exact ID. |
| `obsolete` | boolean or null | Publisher obsolete flag when supplied. Optional. |
| `ontology` | [DiseaseOntology](/reference/schemas/types/#diseaseontology) | The explicit queried ontology. |

## Next

[`biomcp search disease`](/reference/commands/search-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
