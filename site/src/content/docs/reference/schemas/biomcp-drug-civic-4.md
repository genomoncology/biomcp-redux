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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug civic`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_civic/4`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 2033 civic --source civic
```

`biomcp schema biomcp.drug_civic/4` prints the full JSON Schema.

## Example reply

`biomcp get drug 2033 civic --source civic` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
        "evidence": {
          "records": [],
          "page": {"returned": 0, "total": 0}
        },
        "assertions": {
          "records": [],
          "page": {"returned": 0, "total": 0}
        }
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

`result` holds [DrugCivic](#drugcivic). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugCivic

Complete CIViC therapy/profile context with independent native claim continuations.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `civic` | [Section](/reference/schemas/types/#section) of [DrugCivicStreams](#drugcivicstreams) | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | yes | Selected native identity, checked independently of section facts. |

### DrugCivicClaim

One native claim with its complete source therapy set and molecular profile.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `claim` | [CivicStatement](/reference/schemas/types/#civicstatement) | yes | Existing source claim mapping, including unmapped native facts. |
| `oncokb_lookup` | [DrugOncokbLookup](#drugoncokblookup) or null | no | Scoped lookup of a source-listed genomic description, with no evidence transfer. |
| `oncokb_missing` | list of [DrugOncokbMissing](#drugoncokbmissing) | no | Missing source context prevents a keyed variant annotation next command. |
| `profile` | [DrugCivicProfile](#drugcivicprofile) | yes | Source molecular profile context; its member expression is never collapsed. |
| `therapies` | list of [DrugHeader](/reference/schemas/types/#drugheader) | yes | Complete native therapy identities, without cross-source equivalence. |
| `therapy_interaction_type` | [Text](/reference/schemas/types/#text) or null | no | Native combination interaction label when supplied. |

### DrugCivicMember

A drug claim member with its source-owned build, without altering variant responses.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `feature` | [Text](/reference/schemas/types/#text) | yes | Native feature label, including fusion features. |
| `hgvs_descriptions` | list of [Text](/reference/schemas/types/#text) | no | Every native notation, without selecting a transcript or repairing a spelling. |
| `label` | [Text](/reference/schemas/types/#text) | yes | Whole source variant label. |
| `record` | [SourceRecord](/reference/schemas/types/#sourcerecord) | yes | Native variant identity and link. |
| `reference_build` | [Text](/reference/schemas/types/#text) or null | no | Native coordinates.referenceBuild spelling, including unsupported builds. |
| `subject` | [VariantRef](/reference/schemas/types/#variantref) or null | no | Source-qualified subject when supported by the common model. |

### DrugCivicPage

One native claim stream and its own continuation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `page` | [CivicClaimPage](/reference/schemas/types/#civicclaimpage) of [DrugGet](/reference/schemas/types/#drugget) | yes | Stream totals and continuation, independent of every other stream. |
| `records` | list of [DrugCivicClaim](#drugcivicclaim) | yes | Native claim order; each row carries the profile to which that claim belongs. |

### DrugCivicProfile

Profile context embedded in a therapy claim, with no invented profile totals.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `compound` | boolean | yes | Source complex-profile flag. |
| `label` | [Text](/reference/schemas/types/#text) | yes | Complete source expression. |
| `members` | list of [DrugCivicMember](#drugcivicmember) | yes | Every source member, including members without a common-model subject. |
| `record` | [SourceRecord](/reference/schemas/types/#sourcerecord) | yes | Native profile identity and link. |

### DrugCivicStreams

Independent accepted therapy claim collections, without profile count inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assertions` | [DrugCivicPage](#drugcivicpage) or null | no | Assertions filtered independently by the selected native therapy and optional profile. |
| `evidence` | [DrugCivicPage](#drugcivicpage) or null | no | Evidence items filtered by the selected native therapy and optional profile. |

### DrugOncokbLookup

Link one source member to a checked variant lookup in its claim's tumor context.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `label` | [Text](/reference/schemas/types/#text) | yes | Scope visible alongside the generated command. |
| `member` | [Text](/reference/schemas/types/#text) | yes | Native CIViC member ID to which the genomic description belongs. |
| `query` | [VariantGet](/reference/schemas/types/#variantget) | yes | Checked owning-entity request; input preserves the selected source HGVS. |

### DrugOncokbMissing

Context absent or insufficient for an exact source-owned annotation command.

Values: `gene`, `build`, `variant`, `tumor_type`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments.
