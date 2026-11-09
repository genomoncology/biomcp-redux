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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug targets`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_targets/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug CHEMBL1229517 targets --source chembl
```

`biomcp schema biomcp.drug_targets/3` prints the full JSON Schema.

## Example reply

`biomcp get drug CHEMBL1229517 targets --source chembl` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      "data": {…}
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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugTargets](#drugtargets). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugTargets

Native target and mechanism relations, without a clinical indication inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |
| `targets` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugTargetRelation](#drugtargetrelation) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### Other types in this reply

2 more types appear only in this reply: [DrugNativeTarget](#drugnativetarget), [DrugTargetRelation](#drugtargetrelation). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### DrugNativeTarget

One source-native target identity, without inventing a gene label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Text](/reference/schemas/types/#text) | Exact published target identifier. |
| `label` | [Text](/reference/schemas/types/#text) or null | Native target label, only when supplied. Optional. |
| `source` | [Text](/reference/schemas/types/#text) | Source responsible for the native identifier. |
| `url` | [Text](/reference/schemas/types/#text) or null | Native target link, only when supplied. Optional. |

### DrugTargetRelation

A native target or mechanism relation, without an indication inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Native relation provenance. |
| `drug` | [DrugIdentity](/reference/schemas/types/#drugidentity) | Source drug identity. |
| `mechanism` | [Text](/reference/schemas/types/#text) or null | Native mechanism wording, only when supplied. Optional. |
| `native_target` | [DrugNativeTarget](#drugnativetarget) | Native target identity independent of a gene reference. Optional. |
| `references` | list of [DrugNativeReference](/reference/schemas/types/#drugnativereference) | Source-native supporting references. Optional. |
| `relation` | [Text](/reference/schemas/types/#text) | Native relationship label. |
| `target` | [GeneRef](/reference/schemas/types/#generef) | Source-stated target gene, without using mechanism text as a gene label. Optional. |

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
