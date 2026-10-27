---
title: Reply of biomcp get disease synonyms section (biomcp.disease_synonyms/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease
  synonyms: a real example and the fields of schema biomcp.disease_synonyms/2."
goal: "Parse the biomcp.disease_synonyms/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_synonyms/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease synonyms`](/reference/commands/get-disease/) replies with `schema` set to `biomcp.disease_synonyms/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease MONDO:0005105 synonyms
biomcp get disease MONDO:0005105 synonyms --source mydisease
```

`biomcp schema biomcp.disease_synonyms/2` prints the full JSON Schema.

## Example reply

`biomcp get disease MONDO:0005105 synonyms` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.disease_synonyms/2",
  "status": "complete",
  "query": {
    "id": "MONDO:0005105",
    "source": "mydisease",
    "section": "synonyms",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "items": [
      {…},
      … 4 more
    ],
    "components": [
      {…},
      … 1 more
    ]
  },
  "page": {"offset": 0, "returned": 5, "total": 6},
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
        "get", "disease", "MONDO:0005105", "synonyms", "--section-limit", "5",
        "--section-offset", "5"
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiseaseSynonyms](#diseasesynonyms). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseSynonyms

One publisher synonym page with scope and language.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component use terms, independent of row provenance. |
| `items` | list of [DiseaseSynonym](#diseasesynonym) | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

### DiseaseSynonym

A publisher synonym with its scope and language.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Publisher record and version. |
| `language` | [Text](/reference/schemas/types/#text) or null | Source language tag. Optional. |
| `scope` | [Curie](/reference/schemas/types/#curie) or null | Publisher scope predicate. Optional. |
| `value` | [Text](/reference/schemas/types/#text) | Complete synonym text. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
