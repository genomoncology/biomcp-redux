---
title: Reply of biomcp get disease survival section (biomcp.disease_survival/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease
  survival: a real example and the fields of schema biomcp.disease_survival/3."
goal: "Parse the biomcp.disease_survival/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_survival/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease survival`](/reference/commands/get-disease/) and [`biomcp get disease`](/reference/commands/get-disease/) reply with `schema` set to `biomcp.disease_survival/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 \
  --dataset 4 --interval 5 --section-limit 5
```

`biomcp schema biomcp.disease_survival/3` prints the full JSON Schema.

## Example reply

`biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 --dataset 4 --interval 5 --section-limit 5` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.disease_survival/3",
  "status": "partial",
  "query": {
    "id": "55",
    "source": "seer",
    "section": "survival",
    "section_limit": 5,
    "section_offset": 0,
    "sex": "3",
    "race": "1",
    "age_range": "1",
    "dataset": "4",
    "interval": "5"
  },
  "result": {
    "items": [
      {…},
      … 4 more
    ],
    "components": [],
    "fetched": 1793140303
  },
  "page": {"offset": 0, "returned": 5, "total": 48},
  "sources": [
    {
      "source": "seer",
      "use": "open",
      "terms": "SEER/NCI Using This Website",
      "terms_url": "https://seer.cancer.gov/about/using-website.html",
      "state": "ok"
    }
  ],
  "problems": [
    {"source": "seer", "field": "dataset_membership", "code": "value.unknown"},
    … 4 more
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "disease", "55", "survival", "--age-range", "1", "--dataset",
        "4",
        … 12 more
      ]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [DiseaseSurvival](#diseasesurvival). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseSurvival

Native annual survival rows with separate observed and modeled values.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | no | Original response retrieval time in Unix seconds, distinct from dataset publication. |
| `items` | list of [DiseaseStatistic](/reference/schemas/types/#diseasestatistic) | yes | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments.
