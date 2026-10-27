---
title: Reply of biomcp get trial (biomcp.nci_trial_cards/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get trial: a real
  example and the fields of schema biomcp.nci_trial_cards/2."
goal: "Parse the biomcp.nci_trial_cards/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci_trial_cards/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get trial`](/reference/commands/get-trial/) replies with `schema` set to `biomcp.nci_trial_cards/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get trial NCT06887088 --source nci
```

`biomcp schema biomcp.nci_trial_cards/2` prints the full JSON Schema.

## Example reply

`biomcp get trial NCT06887088 --source nci` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.nci_trial_cards/2",
  "status": "failed",
  "query": {
    "input": ["NCT06887088"],
    "limit": 10,
    "offset": 0,
    "source": "nci"
  },
  "result": {
    "items": [{"input": "NCT06887088", "status": "failed"}],
    "attribution": "NCI Clinical Trials Search; source-native fields, no eligi…"
  },
  "sources": [
    {
      "source": "nci",
      "use": "restricted",
      "terms": "NCI Clinical Trials Search publisher terms; user's API key re…",
      "terms_url": "https://www.cancer.gov/syndication/api",
      "state": "skipped",
      "reason": "no_key"
    }
  ],
  "next": [{"kind": "key", "args": ["help", "key", "nci"]}]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [NciTrialCards](#ncitrialcards). `query` holds [TrialGet](/reference/schemas/types/#trialget), a shared type. `query` repeats the request as BioMCP read it.

### NciTrialCards

Independent exact NCI answers in caller order.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution. |
| `items` | list of [NciTrialItem](#ncitrialitem) | One independent answer per requested identity. |

### Other types in this reply

10 more types appear only in this reply: [NciArm](#nciarm), [NciArmRecord](#nciarmrecord), [NciCriterion](#ncicriterion), [NciDisease](#ncidisease), [NciEligibilityPage](#ncieligibilitypage), [NciEligibilityStructured](#ncieligibilitystructured), [NciIntervention](#nciintervention), [NciOutcome](#ncioutcome), [NciTrialItem](#ncitrialitem), [NciTrialSections](#ncitrialsections). Open the list below to read their fields.

<details>
<summary>Fields of the 10 other types</summary>

### NciArm

Source arm header; interventions follow as indexed records.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Whole original description. Optional. |
| `name` | string or null | Original arm name. Optional. |
| `type` | [Text](/reference/schemas/types/#text) or null | Native arm type. Optional. |

### NciArmRecord

A record position preserves every arm/intervention relationship without invented identity.

One of these forms:

- `record` is `"arm"`: Original arm header, followed by its intervention rows. Fields: `arm_index` (integer (uint32)), `data` ([NciArm](#nciarm)).
- `record` is `"intervention"`: Original intervention and its unchanged parent position. Fields: `arm_index` (integer (uint32)), `data` ([NciIntervention](#nciintervention)), `index` (integer (uint32)).

### NciCriterion

One intact source criterion; the publisher supplies its classification.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string | Original source text, preserving every character. |
| `display_order` | integer or null | Original publisher display order. Optional. |
| `inclusion_indicator` | boolean or null | Native source inclusion marker. Optional. |

### NciDisease

One NCI disease concept; names never substitute for an NCIt identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `inclusion_indicator` | [Text](/reference/schemas/types/#text) or null | Publisher coding provenance, without local interpretation. Optional. |
| `is_lead_disease` | boolean or null | Native lead-disease marker. Optional. |
| `name` | [Text](/reference/schemas/types/#text) | Source concept name. |
| `nci_thesaurus_concept_id` | [Text](/reference/schemas/types/#text) | Publisher NCIt identity. |
| `parents` | list of [Text](/reference/schemas/types/#text) | Original immediate ancestors. Optional. |
| `synonyms` | list of [Text](/reference/schemas/types/#text) | Original alternative labels. Optional. |
| `type` | list of [Text](/reference/schemas/types/#text) | Original concept categories. Optional. |

### NciEligibilityPage

Native eligibility fields and whole unclassified criteria.

| Field | Type | Meaning |
| --- | --- | --- |
| `structured` | [NciEligibilityStructured](#ncieligibilitystructured) or null | Source structured bounds and sex, without interpretation. Optional. |
| `unstructured` | [TrialRecords](/reference/schemas/types/#trialrecords) of [NciCriterion](#ncicriterion) | Whole source criteria in original order. |

### NciEligibilityStructured

Native NCI structured fields; sentinel ages remain native values.

| Field | Type | Meaning |
| --- | --- | --- |
| `accepts_healthy_volunteers` | boolean or null | Native accepts healthy volunteers field. Optional. |
| `max_age` | [Text](/reference/schemas/types/#text) or null | Native max age field. Optional. |
| `max_age_in_years` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | Native max age in years field. Optional. |
| `max_age_number` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | Native max age number field. Optional. |
| `max_age_unit` | [Text](/reference/schemas/types/#text) or null | Native max age unit field. Optional. |
| `min_age` | [Text](/reference/schemas/types/#text) or null | Native min age field. Optional. |
| `min_age_in_years` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | Native min age in years field. Optional. |
| `min_age_number` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | Native min age number field. Optional. |
| `min_age_unit` | [Text](/reference/schemas/types/#text) or null | Native min age unit field. Optional. |
| `sex` | [Text](/reference/schemas/types/#text) or null | Native sex field. Optional. |

### NciIntervention

One original NCI intervention attached to its source arm.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Text](/reference/schemas/types/#text) or null | Native category field. Optional. |
| `description` | string or null | Native description field. Optional. |
| `inclusion_indicator` | [Text](/reference/schemas/types/#text) or null | Native inclusion indicator field. Optional. |
| `name` | [Text](/reference/schemas/types/#text) | Native name field. |
| `nci_thesaurus_concept_id` | [Text](/reference/schemas/types/#text) | Native nci thesaurus concept id field. |
| `parents` | list of [Text](/reference/schemas/types/#text) | Native parents field. |
| `synonyms` | list of [Text](/reference/schemas/types/#text) | Native synonyms field. |
| `type` | [Text](/reference/schemas/types/#text) or null | Native r#type field. Optional. |

### NciOutcome

One complete NCI outcome with its original vocabulary.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Original publisher description field. Optional. |
| `name` | string or null | Original publisher name field. Optional. |
| `timeframe` | string or null | Original publisher timeframe field. Optional. |
| `type_code` | string or null | Original publisher type_code field. Optional. |

### NciTrialItem

An exact NCI card and its source state.

| Field | Type | Meaning |
| --- | --- | --- |
| `card` | [NciTrial](/reference/schemas/types/#ncitrial) or null | Native source card, absent after failure or confirmed absence. Optional. |
| `fetched` | integer or null | Original reply retrieval time. Optional. |
| `input` | [NctId](/reference/schemas/types/#nctid) | Exact caller identity. |
| `more` | list of [TrialSection](/reference/schemas/types/#trialsection) | Proven sections omitted from this request. Optional. |
| `sections` | [NciTrialSections](#ncitrialsections) or null | Explicitly requested, source-native sections. Optional. |
| `status` | [Status](/reference/schemas/types/#status) | Independent retrieval state. |
| `unsupported_sections` | list of [TrialSection](/reference/schemas/types/#trialsection) | Requested sections without proven public-tier support. Optional. |

### NciTrialSections

Independently paged native collections from one immutable source record.

| Field | Type | Meaning |
| --- | --- | --- |
| `arms` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [NciArmRecord](#nciarmrecord) or null | Arm headers and their attached intervention records. Optional. |
| `diseases` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [NciDisease](#ncidisease) or null | Every source disease concept in unchanged order. Optional. |
| `eligibility` | [Section](/reference/schemas/types/#section) of [NciEligibilityPage](#ncieligibilitypage) or null | Source criteria and structured eligibility. Optional. |
| `outcomes` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [NciOutcome](#ncioutcome) or null | Source outcomes without registry type coercion. Optional. |
| `summary` | [Section](/reference/schemas/types/#section) of [NciTextPage](/reference/schemas/types/#ncitextpage) or null | Complete native summary with character continuations. Optional. |

</details>

## Next

[`biomcp get trial`](/reference/commands/get-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
