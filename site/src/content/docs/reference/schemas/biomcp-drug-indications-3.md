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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug indications`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_indications/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug CHEMBL1229517 indications --source opentargets
```

`biomcp schema biomcp.drug_indications/3` prints the full JSON Schema.

## Example reply

`biomcp get drug CHEMBL1229517 indications --source opentargets` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      "data": {…}
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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugIndications](#drugindications). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugIndications

Source-native indication wording, populations and combinations.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |
| `indications` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugIndicationClaim](#drugindicationclaim) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### DrugDiseaseReference

One native disease identifier and its own source label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Text](/reference/schemas/types/#text) | Exact identifier spelling, without CURIE conversion. |
| `label` | [Text](/reference/schemas/types/#text) or null | Label attached to this identifier by the source. Optional. |
| `namespace` | [Text](/reference/schemas/types/#text) | Source namespace spelling, such as EFO or MeSH. |

### DrugIndicationClaim

A source indication retaining its full population and combination wording.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Native claim provenance. |
| `combination` | list of [DrugIdentity](/reference/schemas/types/#drugidentity) | Complete source-stated combination members, without local joins. Optional. |
| `disease_references` | list of [DrugDiseaseReference](#drugdiseasereference) | Independently labeled native disease identifiers, without cross-source normalization. Optional. |
| `drug` | [DrugIdentity](/reference/schemas/types/#drugidentity) | Source drug identity. |
| `indication` | [Text](/reference/schemas/types/#text) | Complete native indication wording. |
| `population` | [Text](/reference/schemas/types/#text) or null | Population wording explicitly supplied separately. Optional. |
| `references` | list of [DrugNativeReference](/reference/schemas/types/#drugnativereference) | Source-native supporting references. Optional. |
| `stage` | [DrugStage](#drugstage) or null | Source-stated development stage, separate from authorization events. Optional. |

### DrugStage

One source development-stage field and its unchanged value.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | string | Exact native field name. |
| `value` | string | Exact source string, without an authorization interpretation. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
