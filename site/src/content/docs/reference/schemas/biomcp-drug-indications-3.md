---
title: Reply of biomcp get drug indications section (biomcp.drug_indications/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug
  indications: a real example and the fields of schema
  biomcp.drug_indications/3."
goal: "Parse the biomcp.drug_indications/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_indications/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug indications`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_indications/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug CHEMBL1229517 indications --source opentargets
```

`biomcp schema biomcp.drug_indications/3` prints the full JSON Schema.

## Example reply

`biomcp get drug CHEMBL1229517 indications --source opentargets` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.drug_indications/3",
  "status": "complete",
  "query": {
    "input": "CHEMBL1229517",
    "source": "opentargets",
    "section": ["indications"]
  },
  "result": {
    "identity": {
      "id": "CHEMBL1229517",
      "source": "opentargets",
      "name": "VEMURAFENIB",
      "identity_level": "unknown",
      "type": "drug",
      "url": "https://platform.opentargets.org/drug/CHEMBL1229517"
    },
    "indications": {
      "state": "data",
      "source": "opentargets",
      "data": {
        "records": [
          {
            "drug": {…},
            "indication": "digestive system neoplasm",
            "disease_references": […],
            "stage": {"field": "maxClinicalStage", "value": "PHASE_2"},
            "attribution": {"source": "opentargets", "record": "CHEMBL1229517"}
          },
          … 9 more
        ],
        "offset": 0,
        "total": 48,
        "retrieved": "2026-10-27"
      }
    }
  },
  "page": {"offset": 0, "returned": 10, "total": 48},
  "sources": [
    {
      "source": "opentargets",
      "use": "open",
      "terms": "Open Targets CC0; underlying source provenance retained",
      "terms_url": "https://platform-docs.opentargets.org/licence",
      "state": "ok",
      "count": 48,
      "fetched": 10
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "drug", "CHEMBL1229517", "indications", "--section-offset", "10",
        "--source", "opentargets"
      ]
    }
  ]
}
```

## Fields

`result` holds [DrugIndications](#drugindications). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugIndications

Source-native indication wording, populations and combinations.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Selected native identity, checked independently of section facts. |
| `indications` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugIndicationClaim](#drugindicationclaim) | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### DrugDiseaseReference

One native disease identifier and its own source label.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Text](/reference/schemas/types/#text) | yes | Exact identifier spelling, without CURIE conversion. |
| `label` | [Text](/reference/schemas/types/#text) or null | no | Label attached to this identifier by the source. |
| `namespace` | [Text](/reference/schemas/types/#text) | yes | Source namespace spelling, such as EFO or MeSH. |

### DrugIndicationClaim

A source indication retaining its full population and combination wording.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | yes | Native claim provenance. |
| `combination` | list of [DrugIdentity](/reference/schemas/types/#drugidentity) | no | Complete source-stated combination members, without local joins. |
| `disease_references` | list of [DrugDiseaseReference](#drugdiseasereference) | no | Independently labeled native disease identifiers, without cross-source normalization. |
| `drug` | [DrugIdentity](/reference/schemas/types/#drugidentity) | yes | Source drug identity. |
| `indication` | [Text](/reference/schemas/types/#text) | yes | Complete native indication wording. |
| `population` | [Text](/reference/schemas/types/#text) or null | no | Population wording explicitly supplied separately. |
| `references` | list of [DrugNativeReference](/reference/schemas/types/#drugnativereference) | no | Source-native supporting references. |
| `stage` | [DrugStage](#drugstage) or null | no | Source-stated development stage, separate from authorization events. |

### DrugStage

One source development-stage field and its unchanged value.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `field` | string | yes | Exact native field name. |
| `value` | string | yes | Exact source string, without an authorization interpretation. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments.
