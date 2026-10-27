---
title: Reply of biomcp get drug (biomcp.drug_card/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: a real
  example and the fields of schema biomcp.drug_card/2."
goal: "Parse the biomcp.drug_card/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_card/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_card/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 38eea320-7e0c-485a-bc30-98c3c45e2763 --source openfda-label
```

`biomcp schema biomcp.drug_card/2` prints the full JSON Schema.

## Example reply

`biomcp get drug 38eea320-7e0c-485a-bc30-98c3c45e2763 --source openfda-label` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.drug_card/2",
  "status": "complete",
  "query": {
    "input": "38eea320-7e0c-485a-bc30-98c3c45e2763",
    "source": "openfda-label"
  },
  "result": {
    "header": {
      "id": "38eea320-7e0c-485a-bc30-98c3c45e2763",
      "source": "openfda-label",
      "name": "ZELBORAF",
      "identity_level": "spl_set",
      "version": "29",
      "url": "https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid=38eea32…"
    },
    "alternate_names": ["ZELBORAF", "VEMURAFENIB"],
    "version": "29",
    "updated": "2026-08-28",
    "retrieved": "2026-10-27",
    "available_sections": ["label"]
  },
  "sources": [
    {
      "source": "openfda-label",
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
      "kind": "section",
      "args": [
        "get", "drug", "38eea320-7e0c-485a-bc30-98c3c45e2763", "label",
        "--label-section", "dosage_and_administration", "--source",
        "openfda-label"
      ]
    },
    {
      "kind": "section",
      "args": [
        "get", "drug", "38eea320-7e0c-485a-bc30-98c3c45e2763", "label",
        "--label-section", "drug_interactions", "--source", "openfda-label"
      ]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [DrugCard](#drugcard). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugCard

Exact source identity and explicit native links; deeper facts require named sections.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alternate_names` | list of [Text](/reference/schemas/types/#text) | no | Source-native alternate names, without local alias expansion. |
| `available_sections` | list of [DrugSection](/reference/schemas/types/#drugsection) | no | Only disclosures available for this selected source identity. |
| `cross_references` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | no | Explicit source cross-references with their native relation and provenance. |
| `formulations` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | no | Explicit native formulation links. |
| `header` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Selected source-native identity. |
| `next` | list of [Step](/reference/schemas/types/#step) | no | Typed actions preserving exact source and identity. |
| `parents` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | no | Explicit native parent links. |
| `products` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | no | Explicit native product links. |
| `retrieved` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Actual retrieval date, separate from source publication or update. |
| `updated` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Source-stated update date. |
| `version` | [Text](/reference/schemas/types/#text) or null | no | Source record version. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments.
