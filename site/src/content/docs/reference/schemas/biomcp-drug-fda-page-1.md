---
title: Reply of biomcp search drug (biomcp.drug_fda_page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: a real
  example and the fields of schema biomcp.drug_fda_page/1."
goal: "Parse the biomcp.drug_fda_page/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_fda_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) replies with `schema` set to `biomcp.drug_fda_page/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search drug daraxonrasib --source drugsfda --limit 1
```

`biomcp schema biomcp.drug_fda_page/1` prints the full JSON Schema.

## Example reply

`biomcp search drug daraxonrasib --source drugsfda --limit 1` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.drug_fda_page/1",
  "status": "complete",
  "query": {
    "query": "daraxonrasib",
    "source": "drugsfda",
    "limit": 1,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "identity": {
          "id": "NDA220910",
          "source": "drugsfda",
          "identity_level": "fda_application"
        },
        "products": [
          {
            "product_number": "001",
            "brand_name": "RASONQUE",
            "active_ingredients": [
              {"name": "DARAXONRASIB", "strength": "100MG"}
            ],
            "dosage_form": "TABLET",
            "route": "ORAL",
            "marketing_status": "Prescription",
            "reference_drug": "Yes",
            "reference_standard": "No"
          },
          … 1 more
        ]
      }
    ],
    "total": 1,
    "source_updated": "2026-10-26"
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "drugsfda",
      "use": "open",
      "terms": "openFDA terms; original label rights retained",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok",
      "count": 1,
      "fetched": 1
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": [
        "get", "drug", "NDA220910", "--source", "drugsfda"
      ]
    }
  ]
}
```

## Fields

`result` holds [DrugFdaPage](#drugfdapage). `query` holds [DrugSearch](/reference/schemas/types/#drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### DrugFdaPage

A source-owned application page preserving complete product context and source order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [DrugFdaCandidate](#drugfdacandidate) | yes | Native applications and products; a phrase match never establishes equivalence. |
| `source_updated` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Native dataset last_updated date, independent of event dates and retrieval. |
| `total` | integer (uint64) | yes | Native application total, independent of product count. |

### DrugFdaCandidate

One candidate application and its independent source-listed products.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Exact native application identity; no product has been selected. |
| `products` | list of [DrugFdaProduct](/reference/schemas/types/#drugfdaproduct) | yes | Complete native product candidates within this application. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments.
