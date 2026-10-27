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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_card/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 38eea320-7e0c-485a-bc30-98c3c45e2763 --source openfda-label
```

`biomcp schema biomcp.drug_card/2` prints the full JSON Schema.

## Example reply

`biomcp get drug 38eea320-7e0c-485a-bc30-98c3c45e2763 --source openfda-label` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugCard](#drugcard). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugCard

Exact source identity and explicit native links; deeper facts require named sections.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate_names` | list of [Text](/reference/schemas/types/#text) | Source-native alternate names, without local alias expansion. Optional. |
| `available_sections` | list of [DrugSection](/reference/schemas/types/#drugsection) | Only disclosures available for this selected source identity. Optional. |
| `cross_references` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | Explicit source cross-references with their native relation and provenance. Optional. |
| `formulations` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | Explicit native formulation links. Optional. |
| `header` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected source-native identity. |
| `next` | list of [Step](/reference/schemas/types/#step) | Typed actions preserving exact source and identity. Optional. |
| `parents` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | Explicit native parent links. Optional. |
| `products` | list of [DrugIdentityRelation](/reference/schemas/types/#drugidentityrelation) | Explicit native product links. Optional. |
| `retrieved` | [PartialDate](/reference/schemas/types/#partialdate) or null | Actual retrieval date, separate from source publication or update. Optional. |
| `updated` | [PartialDate](/reference/schemas/types/#partialdate) or null | Source-stated update date. Optional. |
| `version` | [Text](/reference/schemas/types/#text) or null | Source record version. Optional. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
