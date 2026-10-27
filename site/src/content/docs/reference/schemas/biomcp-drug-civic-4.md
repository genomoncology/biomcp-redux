---
title: Reply of biomcp get drug civic section (biomcp.drug_civic/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug civic: a
  real example and the fields of schema biomcp.drug_civic/4."
goal: "Parse the biomcp.drug_civic/4 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_civic/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug civic`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_civic/4`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 2033 civic --source civic
```

`biomcp schema biomcp.drug_civic/4` prints the full JSON Schema.

## Example reply

`biomcp get drug 2033 civic --source civic` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_civic/4",
  "status": "complete",
  "query": {
    "input": "2033",
    "source": "civic",
    "section": ["civic"]
  },
  "result": {
    "identity": {
      "id": "2033",
      "source": "civic",
      "name": "Carbon C 14 Vemurafenib",
      "identity_level": "unknown",
      "type": "Therapy",
      "url": "https://civicdb.org/therapies/2033"
    },
    "civic": {
      "state": "data",
      "source": "civic",
      "data": {
        "evidence": {"records": [], "page": {"returned": 0, "total": 0}},
        "assertions": {"records": [], "page": {"returned": 0, "total": 0}}
      }
    }
  },
  "sources": [
    {
      "source": "civic",
      "use": "open",
      "terms": "CC0 1.0",
      "terms_url": "https://civicdb.org/releases/licensing",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugCivic](#drugcivic). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugCivic

Complete CIViC therapy/profile context with independent native claim continuations.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic` | [Section](/reference/schemas/types/#section) of [DrugCivicStreams](#drugcivicstreams) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |

### Other types in this reply

7 more types appear only in this reply: [DrugCivicClaim](#drugcivicclaim), [DrugCivicMember](#drugcivicmember), [DrugCivicPage](#drugcivicpage), [DrugCivicProfile](#drugcivicprofile), [DrugCivicStreams](#drugcivicstreams), [DrugOncokbLookup](#drugoncokblookup), [DrugOncokbMissing](#drugoncokbmissing). Open the list below to read their fields.

<details>
<summary>Fields of the 7 other types</summary>

### DrugCivicClaim

One native claim with its complete source therapy set and molecular profile.

| Field | Type | Meaning |
| --- | --- | --- |
| `claim` | [CivicStatement](/reference/schemas/types/#civicstatement) | Existing source claim mapping, including unmapped native facts. |
| `oncokb_lookup` | [DrugOncokbLookup](#drugoncokblookup) or null | Scoped lookup of a source-listed genomic description, with no evidence transfer. Optional. |
| `oncokb_missing` | list of [DrugOncokbMissing](#drugoncokbmissing) | Missing source context prevents a keyed variant annotation next command. Optional. |
| `profile` | [DrugCivicProfile](#drugcivicprofile) | Source molecular profile context; its member expression is never collapsed. |
| `therapies` | list of [DrugHeader](/reference/schemas/types/#drugheader) | Complete native therapy identities, without cross-source equivalence. |
| `therapy_interaction_type` | [Text](/reference/schemas/types/#text) or null | Native combination interaction label when supplied. Optional. |

### DrugCivicMember

A drug claim member with its source-owned build, without altering variant responses.

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | [Text](/reference/schemas/types/#text) | Native feature label, including fusion features. |
| `hgvs_descriptions` | list of [Text](/reference/schemas/types/#text) | Every native notation, without selecting a transcript or repairing a spelling. Optional. |
| `label` | [Text](/reference/schemas/types/#text) | Whole source variant label. |
| `record` | [SourceRecord](/reference/schemas/types/#sourcerecord) | Native variant identity and link. |
| `reference_build` | [Text](/reference/schemas/types/#text) or null | Native coordinates.referenceBuild spelling, including unsupported builds. Optional. |
| `subject` | [VariantRef](/reference/schemas/types/#variantref) or null | Source-qualified subject when supported by the common model. Optional. |

### DrugCivicPage

One native claim stream and its own continuation.

| Field | Type | Meaning |
| --- | --- | --- |
| `page` | [CivicClaimPage](/reference/schemas/types/#civicclaimpage) of [DrugGet](/reference/schemas/types/#drugget) | Stream totals and continuation, independent of every other stream. |
| `records` | list of [DrugCivicClaim](#drugcivicclaim) | Native claim order; each row carries the profile to which that claim belongs. |

### DrugCivicProfile

Profile context embedded in a therapy claim, with no invented profile totals.

| Field | Type | Meaning |
| --- | --- | --- |
| `compound` | boolean | Source complex-profile flag. |
| `label` | [Text](/reference/schemas/types/#text) | Complete source expression. |
| `members` | list of [DrugCivicMember](#drugcivicmember) | Every source member, including members without a common-model subject. |
| `record` | [SourceRecord](/reference/schemas/types/#sourcerecord) | Native profile identity and link. |

### DrugCivicStreams

Independent accepted therapy claim collections, without profile count inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertions` | [DrugCivicPage](#drugcivicpage) or null | Assertions filtered independently by the selected native therapy and optional profile. Optional. |
| `evidence` | [DrugCivicPage](#drugcivicpage) or null | Evidence items filtered by the selected native therapy and optional profile. Optional. |

### DrugOncokbLookup

Link one source member to a checked variant lookup in its claim's tumor context.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Text](/reference/schemas/types/#text) | Scope visible alongside the generated command. |
| `member` | [Text](/reference/schemas/types/#text) | Native CIViC member ID to which the genomic description belongs. |
| `query` | [VariantGet](/reference/schemas/types/#variantget) | Checked owning-entity request; input preserves the selected source HGVS. |

### DrugOncokbMissing

Context absent or insufficient for an exact source-owned annotation command.

Values: `gene`, `build`, `variant`, `tumor_type`.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
