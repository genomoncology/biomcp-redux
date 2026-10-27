---
title: Reply of biomcp get drug targets section (biomcp.drug_targets/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug targets: a
  real example and the fields of schema biomcp.drug_targets/3."
goal: "Parse the biomcp.drug_targets/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_targets/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug targets`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_targets/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug CHEMBL1229517 targets --source chembl
```

`biomcp schema biomcp.drug_targets/3` prints the full JSON Schema.

## Example reply

`biomcp get drug CHEMBL1229517 targets --source chembl` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.drug_targets/3",
  "status": "complete",
  "query": {
    "input": "CHEMBL1229517",
    "source": "chembl",
    "section": ["targets"]
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
    "targets": {
      "state": "data",
      "source": "chembl",
      "data": {
        "records": [
          {
            "drug": {…},
            "native_target": {"source": "chembl", "id": "CHEMBL5145"},
            "relation": "INHIBITOR",
            "mechanism": "Serine/threonine-protein kinase B-raf inhibitor",
            "references": […],
            "attribution": {"source": "chembl", "record": "995"}
          }
        ],
        "offset": 0,
        "total": 1,
        "retrieved": "2026-10-27"
      }
    }
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "chembl",
      "use": "attribution",
      "terms": "ChEMBL CC BY-SA 3.0",
      "terms_url": "https://chembl.github.io/chembl-licensing/",
      "state": "ok",
      "count": 1,
      "fetched": 1
    }
  ]
}
```

## Fields

`result` holds [DrugTargets](#drugtargets). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugTargets

Native target and mechanism relations, without a clinical indication inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Selected native identity, checked independently of section facts. |
| `targets` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugTargetRelation](#drugtargetrelation) | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### DrugNativeTarget

One source-native target identity, without inventing a gene label.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Text](/reference/schemas/types/#text) | yes | Exact published target identifier. |
| `label` | [Text](/reference/schemas/types/#text) or null | no | Native target label, only when supplied. |
| `source` | [Text](/reference/schemas/types/#text) | yes | Source responsible for the native identifier. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Native target link, only when supplied. |

### DrugTargetRelation

A native target or mechanism relation, without an indication inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Native relation provenance. |
| `drug` | [DrugIdentity](/reference/schemas/types/#drugidentity) | yes | Source drug identity. |
| `mechanism` | [Text](/reference/schemas/types/#text) or null | no | Native mechanism wording, only when supplied. |
| `native_target` | [DrugNativeTarget](#drugnativetarget) | no | Native target identity independent of a gene reference. |
| `references` | list of [DrugNativeReference](/reference/schemas/types/#drugnativereference) | no | Source-native supporting references. |
| `relation` | [Text](/reference/schemas/types/#text) | yes | Native relationship label. |
| `target` | [GeneRef](/reference/schemas/types/#generef) | no | Source-stated target gene, without using mechanism text as a gene label. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments.
