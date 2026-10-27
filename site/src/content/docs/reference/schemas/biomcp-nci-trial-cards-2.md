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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get trial`](/reference/commands/get-trial/) replies with `schema` set to `biomcp.nci_trial_cards/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get trial NCT06887088 --source nci
```

`biomcp schema biomcp.nci_trial_cards/2` prints the full JSON Schema.

## Example reply

`biomcp get trial NCT06887088 --source nci` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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

`result` holds [NciTrialCards](#ncitrialcards). `query` holds [TrialGet](/reference/schemas/types/#trialget), a shared type. `query` repeats the request as BioMCP read it.

### NciTrialCards

Independent exact NCI answers in caller order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `items` | list of [NciTrialItem](#ncitrialitem) | yes | One independent answer per requested identity. |

### NciArm

Source arm header; interventions follow as indexed records.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | string or null | no | Whole original description. |
| `name` | string or null | no | Original arm name. |
| `type` | [Text](/reference/schemas/types/#text) or null | no | Native arm type. |

### NciArmRecord

A record position preserves every arm/intervention relationship without invented identity.

One of these forms:

- `record` is `"arm"`: Original arm header, followed by its intervention rows. Fields: `arm_index` (integer (uint32)), `data` ([NciArm](#nciarm)).
- `record` is `"intervention"`: Original intervention and its unchanged parent position. Fields: `arm_index` (integer (uint32)), `data` ([NciIntervention](#nciintervention)), `index` (integer (uint32)).

### NciCriterion

One intact source criterion; the publisher supplies its classification.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | string | yes | Original source text, preserving every character. |
| `display_order` | integer or null | no | Original publisher display order. |
| `inclusion_indicator` | boolean or null | no | Native source inclusion marker. |

### NciDisease

One NCI disease concept; names never substitute for an NCIt identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `inclusion_indicator` | [Text](/reference/schemas/types/#text) or null | no | Publisher coding provenance, without local interpretation. |
| `is_lead_disease` | boolean or null | no | Native lead-disease marker. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Source concept name. |
| `nci_thesaurus_concept_id` | [Text](/reference/schemas/types/#text) | yes | Publisher NCIt identity. |
| `parents` | list of [Text](/reference/schemas/types/#text) | no | Original immediate ancestors. |
| `synonyms` | list of [Text](/reference/schemas/types/#text) | no | Original alternative labels. |
| `type` | list of [Text](/reference/schemas/types/#text) | no | Original concept categories. |

### NciEligibilityPage

Native eligibility fields and whole unclassified criteria.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `structured` | [NciEligibilityStructured](#ncieligibilitystructured) or null | no | Source structured bounds and sex, without interpretation. |
| `unstructured` | [TrialRecords](/reference/schemas/types/#trialrecords) of [NciCriterion](#ncicriterion) | yes | Whole source criteria in original order. |

### NciEligibilityStructured

Native NCI structured fields; sentinel ages remain native values.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accepts_healthy_volunteers` | boolean or null | no | Native accepts healthy volunteers field. |
| `max_age` | [Text](/reference/schemas/types/#text) or null | no | Native max age field. |
| `max_age_in_years` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | no | Native max age in years field. |
| `max_age_number` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | no | Native max age number field. |
| `max_age_unit` | [Text](/reference/schemas/types/#text) or null | no | Native max age unit field. |
| `min_age` | [Text](/reference/schemas/types/#text) or null | no | Native min age field. |
| `min_age_in_years` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | no | Native min age in years field. |
| `min_age_number` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | no | Native min age number field. |
| `min_age_unit` | [Text](/reference/schemas/types/#text) or null | no | Native min age unit field. |
| `sex` | [Text](/reference/schemas/types/#text) or null | no | Native sex field. |

### NciIntervention

One original NCI intervention attached to its source arm.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `category` | [Text](/reference/schemas/types/#text) or null | no | Native category field. |
| `description` | string or null | no | Native description field. |
| `inclusion_indicator` | [Text](/reference/schemas/types/#text) or null | no | Native inclusion indicator field. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Native name field. |
| `nci_thesaurus_concept_id` | [Text](/reference/schemas/types/#text) | yes | Native nci thesaurus concept id field. |
| `parents` | list of [Text](/reference/schemas/types/#text) | yes | Native parents field. |
| `synonyms` | list of [Text](/reference/schemas/types/#text) | yes | Native synonyms field. |
| `type` | [Text](/reference/schemas/types/#text) or null | no | Native r#type field. |

### NciOutcome

One complete NCI outcome with its original vocabulary.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | string or null | no | Original publisher description field. |
| `name` | string or null | no | Original publisher name field. |
| `timeframe` | string or null | no | Original publisher timeframe field. |
| `type_code` | string or null | no | Original publisher type_code field. |

### NciTrialItem

An exact NCI card and its source state.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `card` | [NciTrial](/reference/schemas/types/#ncitrial) or null | no | Native source card, absent after failure or confirmed absence. |
| `fetched` | integer or null | no | Original reply retrieval time. |
| `input` | [NctId](/reference/schemas/types/#nctid) | yes | Exact caller identity. |
| `more` | list of [TrialSection](/reference/schemas/types/#trialsection) | no | Proven sections omitted from this request. |
| `sections` | [NciTrialSections](#ncitrialsections) or null | no | Explicitly requested, source-native sections. |
| `status` | [Status](/reference/schemas/types/#status) | yes | Independent retrieval state. |
| `unsupported_sections` | list of [TrialSection](/reference/schemas/types/#trialsection) | no | Requested sections without proven public-tier support. |

### NciTrialSections

Independently paged native collections from one immutable source record.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `arms` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [NciArmRecord](#nciarmrecord) or null | no | Arm headers and their attached intervention records. |
| `diseases` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [NciDisease](#ncidisease) or null | no | Every source disease concept in unchanged order. |
| `eligibility` | [Section](/reference/schemas/types/#section) of [NciEligibilityPage](#ncieligibilitypage) or null | no | Source criteria and structured eligibility. |
| `outcomes` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [NciOutcome](#ncioutcome) or null | no | Source outcomes without registry type coercion. |
| `summary` | [Section](/reference/schemas/types/#section) of [NciTextPage](/reference/schemas/types/#ncitextpage) or null | no | Complete native summary with character continuations. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get trial`](/reference/commands/get-trial/) gives the command's arguments.
