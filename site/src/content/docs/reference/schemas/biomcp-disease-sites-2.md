---
title: Reply of biomcp search disease (biomcp.disease_sites/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search disease: a
  real example and the fields of schema biomcp.disease_sites/2."
goal: "Parse the biomcp.disease_sites/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_sites/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search disease`](/reference/commands/search-disease/) replies with `schema` set to `biomcp.disease_sites/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search disease --source seer --limit 3
biomcp search disease --source seer --limit 1 --offset 1
```

`biomcp schema biomcp.disease_sites/2` prints the full JSON Schema.

## Example reply

`biomcp search disease --source seer --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.disease_sites/2",
  "status": "partial",
  "query": {"source": "seer", "limit": 3, "offset": 0},
  "result": {
    "items": [
      {
        "code": "1",
        "label": "All Cancer Sites Combined",
        "active": true
      },
      {
        "code": "34",
        "label": "Anus, Anal Canal & Anorectum",
        "active": true
      },
      … 1 more
    ],
    "components": [],
    "fetched": 1793140302
  },
  "page": {"offset": 0, "returned": 3, "total": 73},
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
    {
      "source": "seer",
      "field": "catalog.publisher_date",
      "code": "value.unknown"
    },
    {
      "source": "seer",
      "field": "catalog.publisher_version",
      "code": "value.unknown"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "search", "disease", "--limit", "3", "--offset", "3", "--source", "seer"
      ]
    }
  ]
}
```

## Fields

`result` holds [DiseaseSites](#diseasesites). `query` holds [DiseaseSearch](/reference/schemas/types/#diseasesearch), a shared type. `query` repeats the request as BioMCP read it.

### DiseaseSites

Publisher site catalog without an ontology-to-site mapping.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | no | Original response retrieval time in Unix seconds, distinct from publisher version/date. |
| `items` | list of [DiseaseSite](#diseasesite) | yes | Rows as the source gives them. Totals in `page` count rows, not distinct records. |

### DiseaseSite

One publisher catalog site, independent of ontology identities.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `active` | boolean | yes | Publisher active-state flag. |
| `code` | [Text](/reference/schemas/types/#text) | yes | Native publisher site code. |
| `label` | [Text](/reference/schemas/types/#text) | yes | Complete publisher site label. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search disease`](/reference/commands/search-disease/) gives the command's arguments.
