---
title: Reply of biomcp get disease incidence section (biomcp.disease_incidence/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get disease
  incidence: a real example and the fields of schema
  biomcp.disease_incidence/3."
goal: "Parse the biomcp.disease_incidence/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_incidence/3
kind: reference
verified: 2026-10-28
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get disease incidence`](/reference/commands/get-disease/) and [`biomcp get disease`](/reference/commands/get-disease/) reply with `schema` set to `biomcp.disease_incidence/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get disease 47 incidence --source seer --dataset 4 --period 5 \
  --section-limit 1
```

`biomcp schema biomcp.disease_incidence/3` prints the full JSON Schema.

## Example reply

`biomcp get disease 47 incidence --source seer --dataset 4 --period 5 --section-limit 1` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.disease_incidence/3",
  "status": "partial",
  "query": {
    "id": "47",
    "source": "seer",
    "section": "incidence",
    "section_limit": 1,
    "section_offset": 0,
    "sex": "1",
    "race": "1",
    "age_range": "1",
    "dataset": "4",
    "period": "5"
  },
  "result": {
    "items": [
      {…}
    ],
    "components": [],
    "fetched": 1793140302
  },
  "page": {"offset": 0, "returned": 1, "total": 5},
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
    … 3 more
  ],
  "next": [
    {
      "kind": "page",
      "args": [
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiseaseIncidence](#diseaseincidence). `query` holds [DiseaseGet](/reference/schemas/types/#diseaseget), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseIncidence

Native incidence rows preserving requested and returned dimensions.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | Original response retrieval time in Unix seconds, distinct from dataset publication. Optional. |
| `items` | list of [DiseaseStatistic](/reference/schemas/types/#diseasestatistic) | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

## Next

[`biomcp get disease`](/reference/commands/get-disease/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
