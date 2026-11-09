---
title: Reply of biomcp get trial (biomcp.nci-trial-cards/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get trial: the
  fields of schema biomcp.nci-trial-cards/8."
goal: "Parse the biomcp.nci-trial-cards/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci-trial-cards/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get trial`](/reference/commands/get-trial/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.nci-trial-cards/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.nci-trial-cards/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_NciTrialCards](#reply_ncitrialcards). `query` holds [Reply_TrialGet](/reference/schemas/types/#reply_trialget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_NciTrialCards

Independent exact NCI answers in caller order.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution. |
| `items` | list of [Reply_NciTrialItem](#reply_ncitrialitem) | One independent answer per requested identity. Optional. |

### Other types in this reply

19 more types appear only in this reply: [Reply_NciArm](#reply_nciarm), [Reply_NciArmRecord](#reply_nciarmrecord), [Reply_NciCriterion](#reply_ncicriterion), [Reply_NciDisease](#reply_ncidisease), [Reply_NciEligibilityPage](#reply_ncieligibilitypage), [Reply_NciEligibilityStructured](#reply_ncieligibilitystructured), [Reply_NciIntervention](#reply_nciintervention), [Reply_NciOutcome](#reply_ncioutcome), [Reply_NciTrialItem](#reply_ncitrialitem), [Reply_NciTrialSections](#reply_ncitrialsections), [Reply_Section_NciEligibilityPage](#reply_section_ncieligibilitypage), [Reply_Section_NciTextPage](#reply_section_ncitextpage), [Reply_Section_TrialRecords_NciArmRecord](#reply_section_trialrecords_nciarmrecord), [Reply_Section_TrialRecords_NciDisease](#reply_section_trialrecords_ncidisease), [Reply_Section_TrialRecords_NciOutcome](#reply_section_trialrecords_ncioutcome), [Reply_TrialRecords_NciArmRecord](#reply_trialrecords_nciarmrecord), [Reply_TrialRecords_NciCriterion](#reply_trialrecords_ncicriterion), [Reply_TrialRecords_NciDisease](#reply_trialrecords_ncidisease), [Reply_TrialRecords_NciOutcome](#reply_trialrecords_ncioutcome). Open the list below to read their fields.

<details>
<summary>Fields of the 19 other types</summary>

### Reply_NciArm

Source arm header; interventions follow as indexed records.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Whole original description. Optional. |
| `name` | string or null | Original arm name. Optional. |
| `type` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native arm type. Optional. |

### Reply_NciArmRecord

A record position preserves every arm/intervention relationship without invented identity.

One of these forms:

- `record` is `"arm"`: Original arm header, followed by its intervention rows. Fields: `arm_index` (integer (uint32)), `data` ([Reply_NciArm](#reply_nciarm)).
- `record` is `"intervention"`: Original intervention and its unchanged parent position. Fields: `arm_index` (integer (uint32)), `data` ([Reply_NciIntervention](#reply_nciintervention)), `index` (integer (uint32)).

### Reply_NciCriterion

One intact source criterion; the publisher supplies its classification.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Original source text, preserving every character. Optional. |
| `display_order` | integer or null | Original publisher display order. Optional. |
| `inclusion_indicator` | boolean or null | Native source inclusion marker. Optional. |

### Reply_NciDisease

One NCI disease concept; names never substitute for an NCIt identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `inclusion_indicator` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Publisher coding provenance, without local interpretation. Optional. |
| `is_lead_disease` | boolean or null | Native lead-disease marker. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source concept name. Optional. |
| `nci_thesaurus_concept_id` | [Reply_NcitId](/reference/schemas/types/#reply_ncitid) | Publisher NCIt identity. |
| `parents` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Original immediate ancestors. Optional. |
| `synonyms` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Original alternative labels. Optional. |
| `type` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Original concept categories. Optional. |

### Reply_NciEligibilityPage

Native eligibility fields and whole unclassified criteria.

| Field | Type | Meaning |
| --- | --- | --- |
| `structured` | [Reply_NciEligibilityStructured](#reply_ncieligibilitystructured) or null | Source structured bounds and sex, without interpretation. Optional. |
| `unstructured` | [Reply_TrialRecords_NciCriterion](#reply_trialrecords_ncicriterion) | Whole source criteria in original order. |

### Reply_NciEligibilityStructured

Native NCI structured fields; sentinel ages remain native values.

| Field | Type | Meaning |
| --- | --- | --- |
| `accepts_healthy_volunteers` | boolean or null | Native accepts healthy volunteers field. Optional. |
| `max_age` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native max age field. Optional. |
| `max_age_in_years` | [Reply_WithUnit_FiniteNumber_Text](/reference/schemas/types/#reply_withunit_finitenumber_text) or null | Native max age in years field. Optional. |
| `max_age_number` | [Reply_WithUnit_FiniteNumber_Text](/reference/schemas/types/#reply_withunit_finitenumber_text) or null | Native max age number field. Optional. |
| `min_age` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native min age field. Optional. |
| `min_age_in_years` | [Reply_WithUnit_FiniteNumber_Text](/reference/schemas/types/#reply_withunit_finitenumber_text) or null | Native min age in years field. Optional. |
| `min_age_number` | [Reply_WithUnit_FiniteNumber_Text](/reference/schemas/types/#reply_withunit_finitenumber_text) or null | Native min age number field. Optional. |
| `sex` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native sex field. Optional. |

### Reply_NciIntervention

One original NCI intervention attached to its source arm.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native category field. Optional. |
| `description` | string or null | Native description field. Optional. |
| `inclusion_indicator` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native inclusion indicator field. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native name field. Optional. |
| `nci_thesaurus_concept_id` | [Reply_NcitId](/reference/schemas/types/#reply_ncitid) | Native nci thesaurus concept id field. |
| `parents` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Native parents field. Optional. |
| `synonyms` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Native synonyms field. Optional. |
| `type` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native r#type field. Optional. |

### Reply_NciOutcome

One complete NCI outcome with its original vocabulary.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Original publisher description field. Optional. |
| `name` | string or null | Original publisher name field. Optional. |
| `timeframe` | string or null | Original publisher timeframe field. Optional. |
| `type_code` | string or null | Original publisher type_code field. Optional. |

### Reply_NciTrialItem

An exact NCI card and its source state.

| Field | Type | Meaning |
| --- | --- | --- |
| `card` | [Reply_NciTrial](/reference/schemas/types/#reply_ncitrial) or null | Native source card, absent after failure or confirmed absence. Optional. |
| `fetched` | integer or null | Original reply retrieval time. Optional. |
| `input` | [Reply_NctId](/reference/schemas/types/#reply_nctid) | Exact caller identity. |
| `more` | list of [Reply_TrialSection](/reference/schemas/types/#reply_trialsection) | Proven sections omitted from this request. Optional. |
| `sections` | [Reply_NciTrialSections](#reply_ncitrialsections) or null | Explicitly requested, source-native sections. Optional. |
| `status` | [Reply_Status](/reference/schemas/types/#reply_status) | Independent retrieval state. |
| `unsupported_sections` | list of [Reply_TrialSection](/reference/schemas/types/#reply_trialsection) | Requested sections without proven public-tier support. Optional. |

### Reply_NciTrialSections

Independently paged native collections from one immutable source record.

| Field | Type | Meaning |
| --- | --- | --- |
| `arms` | [Reply_Section_TrialRecords_NciArmRecord](#reply_section_trialrecords_nciarmrecord) or null | Arm headers and their attached intervention records. Optional. |
| `diseases` | [Reply_Section_TrialRecords_NciDisease](#reply_section_trialrecords_ncidisease) or null | Every source disease concept in unchanged order. Optional. |
| `eligibility` | [Reply_Section_NciEligibilityPage](#reply_section_ncieligibilitypage) or null | Source criteria and structured eligibility. Optional. |
| `outcomes` | [Reply_Section_TrialRecords_NciOutcome](#reply_section_trialrecords_ncioutcome) or null | Source outcomes without registry type coercion. Optional. |
| `summary` | [Reply_Section_NciTextPage](#reply_section_ncitextpage) or null | Complete native summary with character continuations. Optional. |

### Reply_Section_NciEligibilityPage

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_NciEligibilityPage](#reply_ncieligibilitypage)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_NciTextPage

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_NciTextPage](/reference/schemas/types/#reply_ncitextpage)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_NciArmRecord

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_NciArmRecord](#reply_trialrecords_nciarmrecord)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_NciDisease

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_NciDisease](#reply_trialrecords_ncidisease)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_NciOutcome

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_NciOutcome](#reply_trialrecords_ncioutcome)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_TrialRecords_NciArmRecord

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_NciArmRecord](#reply_nciarmrecord) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_NciCriterion

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_NciCriterion](#reply_ncicriterion) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_NciDisease

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_NciDisease](#reply_ncidisease) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_NciOutcome

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_NciOutcome](#reply_ncioutcome) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

</details>

## Next

[`biomcp get trial`](/reference/commands/get-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
