---
title: Reply of biomcp get trial (biomcp.trial-cards/9)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get trial: the
  fields of schema biomcp.trial-cards/9."
goal: "Parse the biomcp.trial-cards/9 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial-cards/9
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get trial`](/reference/commands/get-trial/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.trial-cards/9`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.trial-cards/9` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_TrialCards](#reply_trialcards). `query` holds [Reply_TrialGet](/reference/schemas/types/#reply_trialget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_TrialCards

Trial answers in original input order, retaining independent states.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution and scope. |
| `items` | list of [Reply_TrialCardItem](#reply_trialcarditem) | One answer per requested input, including absence and failure. Optional. |
| `source_updated` | [Reply_SourceUpdated](/reference/schemas/types/#reply_sourceupdated) or null | Registry processing timestamp, distinct from each study's update and fetch times. Optional. |

### Other types in this reply

52 more types appear only in this reply: [Reply_Section_TrialEligibilityPage](#reply_section_trialeligibilitypage), [Reply_Section_TrialRecords_TrialArmRecord](#reply_section_trialrecords_trialarmrecord), [Reply_Section_TrialRecords_TrialContactRecord](#reply_section_trialrecords_trialcontactrecord), [Reply_Section_TrialRecords_TrialDocument](#reply_section_trialrecords_trialdocument), [Reply_Section_TrialRecords_TrialLocation](#reply_section_trialrecords_triallocation), [Reply_Section_TrialRecords_TrialOutcome](#reply_section_trialrecords_trialoutcome), [Reply_Section_TrialRecords_TrialReference](#reply_section_trialrecords_trialreference), [Reply_Section_TrialRecords_TrialResultRecord](#reply_section_trialrecords_trialresultrecord), [Reply_TrialAdverseEvents](#reply_trialadverseevents), [Reply_TrialAgeBound](#reply_trialagebound), [Reply_TrialArm](#reply_trialarm), [Reply_TrialArmKind](#reply_trialarmkind), [Reply_TrialArmRecord](#reply_trialarmrecord), [Reply_TrialCardItem](#reply_trialcarditem), [Reply_TrialContact](#reply_trialcontact), [Reply_TrialContactRecord](#reply_trialcontactrecord), [Reply_TrialContactRole](#reply_trialcontactrole), [Reply_TrialCoordinates](#reply_trialcoordinates), [Reply_TrialDate](#reply_trialdate), [Reply_TrialDateKind](#reply_trialdatekind), [Reply_TrialEligibilityPage](#reply_trialeligibilitypage), [Reply_TrialEvent](#reply_trialevent), [Reply_TrialEventGroup](#reply_trialeventgroup), [Reply_TrialEventStats](#reply_trialeventstats), [Reply_TrialIntervention](#reply_trialintervention), [Reply_TrialInterventionKind](#reply_trialinterventionkind), [Reply_TrialLocation](#reply_triallocation), [Reply_TrialOfficial](#reply_trialofficial), [Reply_TrialOfficialRole](#reply_trialofficialrole), [Reply_TrialOutcome](#reply_trialoutcome), [Reply_TrialOutcomeKind](#reply_trialoutcomekind), [Reply_TrialPostedCard](#reply_trialpostedcard), [Reply_TrialPostedOutcome](#reply_trialpostedoutcome), [Reply_TrialRecords_TrialArmRecord](#reply_trialrecords_trialarmrecord), [Reply_TrialRecords_TrialContactRecord](#reply_trialrecords_trialcontactrecord), [Reply_TrialRecords_TrialDocument](#reply_trialrecords_trialdocument), [Reply_TrialRecords_TrialLocation](#reply_trialrecords_triallocation), [Reply_TrialRecords_TrialOutcome](#reply_trialrecords_trialoutcome), [Reply_TrialRecords_TrialReference](#reply_trialrecords_trialreference), [Reply_TrialRecords_TrialResultRecord](#reply_trialrecords_trialresultrecord), [Reply_TrialReference](#reply_trialreference), [Reply_TrialReferenceKind](#reply_trialreferencekind), [Reply_TrialResultAnalysis](#reply_trialresultanalysis), [Reply_TrialResultCategory](#reply_trialresultcategory), [Reply_TrialResultClass](#reply_trialresultclass), [Reply_TrialResultCount](#reply_trialresultcount), [Reply_TrialResultDenominator](#reply_trialresultdenominator), [Reply_TrialResultGroup](#reply_trialresultgroup), [Reply_TrialResultMeasurement](#reply_trialresultmeasurement), [Reply_TrialResultRecord](#reply_trialresultrecord), [Reply_TrialSections](#reply_trialsections), [Reply_TrialSiteStatus](#reply_trialsitestatus). Open the list below to read their fields.

<details>
<summary>Fields of the 52 other types</summary>

### Reply_Section_TrialEligibilityPage

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialEligibilityPage](#reply_trialeligibilitypage)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialArmRecord

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialArmRecord](#reply_trialrecords_trialarmrecord)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialContactRecord

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialContactRecord](#reply_trialrecords_trialcontactrecord)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialDocument

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialDocument](#reply_trialrecords_trialdocument)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialLocation

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialLocation](#reply_trialrecords_triallocation)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialOutcome

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialOutcome](#reply_trialrecords_trialoutcome)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialReference

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialReference](#reply_trialrecords_trialreference)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_TrialRecords_TrialResultRecord

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_TrialRecords_TrialResultRecord](#reply_trialrecords_trialresultrecord)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_TrialAdverseEvents

Native event reporting scope, repeated nowhere else.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Native `description` field. Optional. |
| `frequencyThreshold` | string or null | Native `frequency_threshold` field. Optional. |
| `timeFrame` | string or null | Native `time_frame` field. Optional. |

### Reply_TrialAgeBound

Source age limit; absence is explicit and never a fabricated number.

One of these forms:

- `state` is `"no_limit"`: Registry supplied no limit.
- `state` is `"source"`: Unchanged source value; no local age conversion. Fields: `raw` (string).

### Reply_TrialArm

Registry arm with source intervention relationships.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Reply_Markdown](/reference/schemas/types/#reply_markdown) or null | Original arm description. Optional. |
| `intervention_names` | list of string | Source intervention names, including their native prefixes. Optional. |
| `kind` | [Reply_TrialArmKind](#reply_trialarmkind) or null | Native arm type. Optional. |
| `label` | string or null | Exact arm label. Optional. |

### Reply_TrialArmKind

Native registry arm group type from the publisher enum metadata.

Type: string.

### Reply_TrialArmRecord

Source arm rows followed by source intervention rows, each in its native order.

One of these forms:

- `record` is `"arm"`: Original arm with its intervention names. Fields: `data` ([Reply_TrialArm](#reply_trialarm)).
- `record` is `"intervention"`: Original intervention with aliases and arm labels. Fields: `data` ([Reply_TrialIntervention](#reply_trialintervention)).

### Reply_TrialCardItem

Result for one exact requested NCT identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `card` | [Reply_TrialPostedCard](#reply_trialpostedcard) or null | Source card, absent after confirmed 404 or failed lookup. Optional. |
| `fetched` | integer or null | Original record retrieval timestamp. Optional. |
| `input` | [Reply_NctId](/reference/schemas/types/#reply_nctid) | Exact requested identity. |
| `more` | list of [Reply_TrialSection](/reference/schemas/types/#reply_trialsection) | Available sections omitted from this request. Optional. |
| `sections` | [Reply_TrialSections](#reply_trialsections) or null | Explicitly requested source sections. Optional. |
| `status` | [Reply_Status](/reference/schemas/types/#reply_status) | Independent resolution state. |

### Reply_TrialContact

A registry contact with its source role and original contact details.

| Field | Type | Meaning |
| --- | --- | --- |
| `email` | string or null | Source email address. Optional. |
| `name` | string or null | Source name. Optional. |
| `phone` | string or null | Source telephone number, without normalization. Optional. |
| `phone_ext` | string or null | Source extension, kept separate from the number. Optional. |
| `role` | [Reply_TrialContactRole](#reply_trialcontactrole) or null | Source contact role, absent if the registry omitted it. Optional. |

### Reply_TrialContactRecord

Distinct registry-wide contact collections; site contacts stay on locations.

One of these forms:

- `record` is `"central"`: A central contact for this trial. Fields: `data` ([Reply_TrialContact](#reply_trialcontact)).
- `record` is `"official"`: An overall official for this trial. Fields: `data` ([Reply_TrialOfficial](#reply_trialofficial)).

### Reply_TrialContactRole

Native contact role.

Values: `CONTACT`, `STUDY_CHAIR`, `STUDY_DIRECTOR`, `PRINCIPAL_INVESTIGATOR`, `SUB_INVESTIGATOR`.

### Reply_TrialCoordinates

Checked geographic coordinates in degrees; no genomic coordinate type is involved.

| Field | Type | Meaning |
| --- | --- | --- |
| `latitude` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) | Latitude in degrees. |
| `longitude` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) | Longitude in degrees. |

### Reply_TrialDate

A registry date retains its actual or estimated qualifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) | Checked source date at its original precision. |
| `kind` | [Reply_TrialDateKind](#reply_trialdatekind) or null | Source qualifier, absent only if the registry omitted it. Optional. |

### Reply_TrialDateKind

Registry date qualifier, without local interpretation.

Values: `ACTUAL`, `ESTIMATED`.

### Reply_TrialEligibilityPage

Character paging preserves source whitespace and counts Unicode scalar values.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | Checked request for the next intact fragment, absent when complete. Optional. |
| `criteria` | string | Exact original text fragment, including whitespace. |
| `criterion_continues` | boolean | The page ends within a criterion whose lines exceed the page budget. Optional. |
| `healthy_volunteers` | boolean or null | Source healthy-volunteer field. Optional. |
| `maximum_age` | [Reply_TrialAgeBound](#reply_trialagebound) or null | Source maximum age. Optional. |
| `minimum_age` | [Reply_TrialAgeBound](#reply_trialagebound) or null | Source minimum age. Optional. |
| `offset` | integer (uint32) | Original character offset. |
| `sex` | [Reply_TrialSex](/reference/schemas/types/#reply_trialsex) or null | Source sex field. Optional. |
| `total_characters` | integer (uint32) | Complete original text length in Unicode scalar values. |

### Reply_TrialEvent

Native event term and its group statistics.

| Field | Type | Meaning |
| --- | --- | --- |
| `assessmentType` | string or null | Native `assessment_type` field. Optional. |
| `notes` | string or null | Native `notes` field. Optional. |
| `organSystem` | string or null | Native `organ_system` field. Optional. |
| `sourceVocabulary` | string or null | Native `source_vocabulary` field. Optional. |
| `stats` | list of [Reply_TrialEventStats](#reply_trialeventstats) | Native `stats` field. Optional. |
| `term` | string or null | Native `term` field. Optional. |

### Reply_TrialEventGroup

Native event group and source counts; no rates are derived.

| Field | Type | Meaning |
| --- | --- | --- |
| `deathsNumAffected` | integer or null | Native `deaths_num_affected` field. Optional. |
| `deathsNumAtRisk` | integer or null | Native `deaths_num_at_risk` field. Optional. |
| `description` | string or null | Native `description` field. Optional. |
| `id` | string or null | Native `id` field. Optional. |
| `otherNumAffected` | integer or null | Native `other_num_affected` field. Optional. |
| `otherNumAtRisk` | integer or null | Native `other_num_at_risk` field. Optional. |
| `seriousNumAffected` | integer or null | Native `serious_num_affected` field. Optional. |
| `seriousNumAtRisk` | integer or null | Native `serious_num_at_risk` field. Optional. |
| `title` | string or null | Native `title` field. Optional. |

### Reply_TrialEventStats

Native adverse-event group statistics.

| Field | Type | Meaning |
| --- | --- | --- |
| `groupId` | string or null | Native `group_id` field. Optional. |
| `numAffected` | integer or null | Native `num_affected` field. Optional. |
| `numAtRisk` | integer or null | Native `num_at_risk` field. Optional. |
| `numEvents` | integer or null | Native `num_events` field. Optional. |

### Reply_TrialIntervention

Registry intervention, retaining aliases and arm labels.

| Field | Type | Meaning |
| --- | --- | --- |
| `arm_group_labels` | list of string | Source arm relationships. Optional. |
| `description` | [Reply_Markdown](/reference/schemas/types/#reply_markdown) or null | Original description. Optional. |
| `kind` | [Reply_TrialInterventionKind](#reply_trialinterventionkind) or null | Native intervention type. Optional. |
| `name` | string or null | Exact source name. Optional. |
| `other_names` | list of string | Source aliases, without invented expansion. Optional. |

### Reply_TrialInterventionKind

Native registry intervention type from the publisher enum metadata.

Type: string.

### Reply_TrialLocation

One source site, kept whole during pagination.

| Field | Type | Meaning |
| --- | --- | --- |
| `city` | string or null | Source city. Optional. |
| `contacts` | list of [Reply_TrialContact](#reply_trialcontact) | Contacts belong only to this site, in source order. Optional. |
| `coordinates` | [Reply_TrialCoordinates](#reply_trialcoordinates) or null | Registry coordinates without a derived distance. Optional. |
| `country` | string or null | Source country. Optional. |
| `facility` | string or null | Source facility name, absent only when not supplied. Optional. |
| `state` | string or null | Source state or region. Optional. |
| `status` | [Reply_TrialSiteStatus](#reply_trialsitestatus) or null | This site's native status; never copied from the overall trial status. Optional. |
| `zip` | string or null | Source postal code. Optional. |

### Reply_TrialOfficial

An overall official, distinct from a central or site contact.

| Field | Type | Meaning |
| --- | --- | --- |
| `affiliation` | string or null | Source organization. Optional. |
| `name` | string or null | Registered name. Optional. |
| `role` | [Reply_TrialOfficialRole](#reply_trialofficialrole) or null | Source official role. Optional. |

### Reply_TrialOfficialRole

Native overall official role.

Values: `SUB_INVESTIGATOR`, `PRINCIPAL_INVESTIGATOR`, `STUDY_DIRECTOR`, `STUDY_CHAIR`.

### Reply_TrialOutcome

One whole registered outcome with its attached time frame.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Reply_Markdown](/reference/schemas/types/#reply_markdown) or null | Original description. Optional. |
| `kind` | [Reply_TrialOutcomeKind](#reply_trialoutcomekind) | Source collection. |
| `measure` | string or null | Original measure text. Optional. |
| `time_frame` | string or null | Original time frame. Optional. |

### Reply_TrialOutcomeKind

Registry outcome collection.

Values: `primary`, `secondary`, `other`.

### Reply_TrialPostedCard

Original trial card plus the posted results date from the same record.

| Field | Type | Meaning |
| --- | --- | --- |
| `brief_summary` | [Reply_Markdown](/reference/schemas/types/#reply_markdown) or null | Whole brief summary, preserving whitespace. Optional. |
| `completion` | [Reply_TrialDate](#reply_trialdate) or null | Source completion date, when present. Optional. |
| `conditions` | list of string | All registered condition names. Optional. |
| `interventions` | list of string | All registered intervention names. Optional. |
| `nct_id` | [Reply_NctId](/reference/schemas/types/#reply_nctid) | Exact registry identity. |
| `phases` | list of [Reply_TrialPhase](/reference/schemas/types/#reply_trialphase) | Source phases without inferred defaults. Optional. |
| `primary_completion` | [Reply_TrialDate](#reply_trialdate) or null | Source primary completion date, when present. Optional. |
| `results_first_posted` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | First public posting date, distinct from submission and update dates. Optional. |
| `sponsor` | string or null | Lead sponsor as registered. Optional. |
| `start` | [Reply_TrialDate](#reply_trialdate) or null | Source start date, when present. Optional. |
| `status` | [Reply_TrialStatus](/reference/schemas/types/#reply_trialstatus) or null | Source overall status, absent when missing or rejected. Optional. |
| `study_type` | [Reply_StudyType](/reference/schemas/types/#reply_studytype) or null | Native study type, absent when missing or rejected. Optional. |
| `title` | string or null | Source brief title, absent when missing or rejected. Optional. |
| `updated` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | Last posted registry update, absent when missing or rejected. Optional. |
| `updated_kind` | [Reply_TrialDateKind](#reply_trialdatekind) or null | Native qualifier for the posted update date, absent when the registry omitted it. Optional. |
| `url` | string (uri) | Canonical publisher record URL. |

### Reply_TrialPostedOutcome

Posted outcome measure.

| Field | Type | Meaning |
| --- | --- | --- |
| `analyses` | list of [Reply_TrialResultAnalysis](#reply_trialresultanalysis) | Native `analyses` field. Optional. |
| `calculatePct` | boolean or null | Native `calculate_pct` field. Optional. |
| `classes` | list of [Reply_TrialResultClass](#reply_trialresultclass) | Native `classes` field. Optional. |
| `denomUnits` | string or null | Native `denom_units` field. Optional. |
| `denoms` | list of [Reply_TrialResultDenominator](#reply_trialresultdenominator) | Native `denoms` field. Optional. |
| `description` | string or null | Native `description` field. Optional. |
| `dispersionType` | string or null | Native `dispersion_type` field. Optional. |
| `groups` | list of [Reply_TrialResultGroup](#reply_trialresultgroup) | Native `groups` field. Optional. |
| `paramType` | string or null | Native `param_type` field. Optional. |
| `populationDescription` | string or null | Native `population_description` field. Optional. |
| `reportingStatus` | string or null | Native `reporting_status` field. Optional. |
| `timeFrame` | string or null | Native `time_frame` field. Optional. |
| `title` | string or null | Native `title` field. Optional. |
| `type` | string or null | Native `type` field. Optional. |
| `typeUnits` | string or null | Native `type_units` field. Optional. |
| `unitOfMeasure` | string or null | Native `unit_of_measure` field. Optional. |

### Reply_TrialRecords_TrialArmRecord

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialArmRecord](#reply_trialarmrecord) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_TrialContactRecord

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialContactRecord](#reply_trialcontactrecord) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_TrialDocument

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialDocument](/reference/schemas/types/#reply_trialdocument) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_TrialLocation

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialLocation](#reply_triallocation) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_TrialOutcome

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialOutcome](#reply_trialoutcome) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_TrialReference

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialReference](#reply_trialreference) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialRecords_TrialResultRecord

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](/reference/schemas/types/#reply_trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_TrialResultRecord](#reply_trialresultrecord) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialReference

An unchanged registry reference in source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `citation` | string or null | Original citation text. Optional. |
| `kind` | [Reply_TrialReferenceKind](#reply_trialreferencekind) or null | Native reference classification, from the registry vocabulary. Optional. |
| `pmid` | [Reply_Pmid](/reference/schemas/types/#reply_pmid) or null | Registry-supplied PubMed identity, when present. Optional. |

### Reply_TrialReferenceKind

Native ClinicalTrials.gov reference classification.

Values: `BACKGROUND`, `RESULT`, `DERIVED`.

### Reply_TrialResultAnalysis

Source statistical analysis and comparison identities.

| Field | Type | Meaning |
| --- | --- | --- |
| `ciLowerLimit` | string or null | Native `ci_lower_limit` field. Optional. |
| `ciLowerLimitComment` | string or null | Native `ci_lower_limit_comment` field. Optional. |
| `ciNumSides` | string or null | Native `ci_num_sides` field. Optional. |
| `ciPctValue` | string or null | Native `ci_pct_value` field. Optional. |
| `ciUpperLimit` | string or null | Native `ci_upper_limit` field. Optional. |
| `ciUpperLimitComment` | string or null | Native `ci_upper_limit_comment` field. Optional. |
| `dispersionType` | string or null | Native `dispersion_type` field. Optional. |
| `dispersionValue` | string or null | Native `dispersion_value` field. Optional. |
| `estimateComment` | string or null | Native `estimate_comment` field. Optional. |
| `groupDescription` | string or null | Native `group_description` field. Optional. |
| `groupIds` | list of string | Native `group_ids` field. Optional. |
| `nonInferiorityComment` | string or null | Native `non_inferiority_comment` field. Optional. |
| `nonInferiorityType` | string or null | Native `non_inferiority_type` field. Optional. |
| `otherAnalysisDescription` | string or null | Native `other_analysis_description` field. Optional. |
| `pValue` | string or null | Native `p_value` field. Optional. |
| `pValueComment` | string or null | Native `p_value_comment` field. Optional. |
| `paramType` | string or null | Native `param_type` field. Optional. |
| `paramValue` | string or null | Native `param_value` field. Optional. |
| `statisticalComment` | string or null | Native `statistical_comment` field. Optional. |
| `statisticalMethod` | string or null | Native `statistical_method` field. Optional. |

### Reply_TrialResultCategory

Native category containing reported measurements.

| Field | Type | Meaning |
| --- | --- | --- |
| `measurements` | list of [Reply_TrialResultMeasurement](#reply_trialresultmeasurement) | Native `measurements` field. Optional. |
| `title` | string or null | Native `title` field. Optional. |

### Reply_TrialResultClass

Native outcome class; no regrouping.

| Field | Type | Meaning |
| --- | --- | --- |
| `categories` | list of [Reply_TrialResultCategory](#reply_trialresultcategory) | Native `categories` field. Optional. |
| `denoms` | list of [Reply_TrialResultDenominator](#reply_trialresultdenominator) | Native `denoms` field. Optional. |
| `title` | string or null | Native `title` field. Optional. |

### Reply_TrialResultCount

Reported count, with source group identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `groupId` | string or null | Native `group_id` field. Optional. |
| `value` | string or null | Native `value` field. Optional. |

### Reply_TrialResultDenominator

Reported denominator with its original units.

| Field | Type | Meaning |
| --- | --- | --- |
| `counts` | list of [Reply_TrialResultCount](#reply_trialresultcount) | Native `counts` field. Optional. |
| `units` | string or null | Native `units` field. Optional. |

### Reply_TrialResultGroup

Native outcome group.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string or null | Native `description` field. Optional. |
| `id` | string or null | Native `id` field. Optional. |
| `title` | string or null | Native `title` field. Optional. |

### Reply_TrialResultMeasurement

Reported value and dispersion; strings are never recomputed.

| Field | Type | Meaning |
| --- | --- | --- |
| `comment` | string or null | Native `comment` field. Optional. |
| `groupId` | string or null | Native `group_id` field. Optional. |
| `lowerLimit` | string or null | Native `lower_limit` field. Optional. |
| `spread` | string or null | Native `spread` field. Optional. |
| `upperLimit` | string or null | Native `upper_limit` field. Optional. |
| `value` | string or null | Native `value` field. Optional. |

### Reply_TrialResultRecord

Whole native records in outcome, adverse-event metadata, group, serious and other order.

One of these forms:

- `record` is `"outcome"`: Posted measure, including group denominators, values and analyses. Fields: `data` ([Reply_TrialPostedOutcome](#reply_trialpostedoutcome)).
- `record` is `"adverse-events"`: Native adverse-event collection scope. Fields: `data` ([Reply_TrialAdverseEvents](#reply_trialadverseevents)).
- `record` is `"adverse-event-group"`: Native adverse-event group and reported counts. Fields: `data` ([Reply_TrialEventGroup](#reply_trialeventgroup)).
- `record` is `"serious-event"`: One serious adverse-event term with group statistics. Fields: `data` ([Reply_TrialEvent](#reply_trialevent)).
- `record` is `"other-event"`: One other adverse-event term with group statistics. Fields: `data` ([Reply_TrialEvent](#reply_trialevent)).

### Reply_TrialSections

Independently bounded sections from the same source protocol snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `arms` | [Reply_Section_TrialRecords_TrialArmRecord](#reply_section_trialrecords_trialarmrecord) or null | Whole source arm and intervention records. Optional. |
| `contacts` | [Reply_Section_TrialRecords_TrialContactRecord](#reply_section_trialrecords_trialcontactrecord) or null | Central contacts followed by overall officials, each in source order. Optional. |
| `documents` | [Reply_Section_TrialRecords_TrialDocument](#reply_section_trialrecords_trialdocument) or null | Posted document manifest, without downloading its files. Optional. |
| `eligibility` | [Reply_Section_TrialEligibilityPage](#reply_section_trialeligibilitypage) or null | Source eligibility fields and exact text fragment. Optional. |
| `locations` | [Reply_Section_TrialRecords_TrialLocation](#reply_section_trialrecords_triallocation) or null | Whole sites with their original contacts attached. Optional. |
| `outcomes` | [Reply_Section_TrialRecords_TrialOutcome](#reply_section_trialrecords_trialoutcome) or null | Whole registered outcomes. Optional. |
| `references` | [Reply_Section_TrialRecords_TrialReference](#reply_section_trialrecords_trialreference) or null | Source publication records, without inferred roles. Optional. |
| `results` | [Reply_Section_TrialRecords_TrialResultRecord](#reply_section_trialrecords_trialresultrecord) or null | Posted results, followed by adverse-event groups and terms, in source order. Optional. |

### Reply_TrialSiteStatus

Native site recruitment status, distinct from overall trial status.

Values: `NOT_YET_RECRUITING`, `RECRUITING`, `ENROLLING_BY_INVITATION`, `ACTIVE_NOT_RECRUITING`, `SUSPENDED`, `TERMINATED`, `COMPLETED`, `WITHDRAWN`, `AVAILABLE`.

</details>

## Next

[`biomcp get trial`](/reference/commands/get-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
