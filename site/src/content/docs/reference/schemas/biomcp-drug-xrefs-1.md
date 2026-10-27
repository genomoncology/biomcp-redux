---
title: Reply of biomcp get drug xrefs section (biomcp.drug_xrefs/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug xrefs: a
  real example and the fields of schema biomcp.drug_xrefs/1."
goal: "Parse the biomcp.drug_xrefs/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_xrefs/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug xrefs`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_xrefs/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug CHEMBL1229517 xrefs --source chembl
```

`biomcp schema biomcp.drug_xrefs/1` prints the full JSON Schema.

## Example reply

`biomcp get drug CHEMBL1229517 xrefs --source chembl` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.drug_xrefs/1",
  "status": "complete",
  "query": {
    "input": "CHEMBL1229517",
    "source": "chembl",
    "section": ["xrefs"]
  },
  "result": {
    "identity": {
      "id": "CHEMBL1229517",
      "source": "chembl",
      "name": "VEMURAFENIB",
      "identity_level": "unknown",
      "type": "Small molecule",
      "url": "https://www.ebi.ac.uk/chembl/explore/compound/CHEMBL1229517"
    },
    "xrefs": {
      "state": "data",
      "source": "chembl",
      "data": {
        "records": [
          {
            "identity": {
              "id": "vemurafenib",
              "source": "DailyMed",
              "name": "vemurafenib",
              "identity_level": "unknown"
            },
            "relation": "cross_references",
            "attribution": {
              "source": "chembl",
              "record": "CHEMBL1229517",
              "url": "https://www.ebi.ac.uk/chembl/explore/compound/CHEMBL1229…"
            }
          },
          … 1 more
        ],
        "offset": 0,
        "total": 2,
        "retrieved": "2026-10-27"
      }
    }
  },
  "page": {"offset": 0, "returned": 2, "total": 2},
  "sources": [
    {
      "source": "chembl",
      "use": "attribution",
      "terms": "ChEMBL CC BY-SA 3.0",
      "terms_url": "https://chembl.github.io/chembl-licensing/",
      "state": "ok",
      "count": 2,
      "fetched": 2
    }
  ]
}
```

## Fields

`result` holds [DrugXrefs](#drugxrefs). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugXrefs

A mechanical page of the selected document's complete checked native links.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Exact source-selected identity, without choosing any linked product. |
| `xrefs` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | yes | Native links with an exact link count, original version and retrieval date. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments.
