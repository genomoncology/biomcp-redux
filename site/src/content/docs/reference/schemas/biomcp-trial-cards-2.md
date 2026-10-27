---
title: Reply of biomcp get trial eligibility section (biomcp.trial_cards/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get trial
  eligibility: a real example and the fields of schema biomcp.trial_cards/2."
goal: "Parse the biomcp.trial_cards/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial_cards/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get trial eligibility`](/reference/commands/get-trial/) and [`biomcp get trial`](/reference/commands/get-trial/) reply with `schema` set to `biomcp.trial_cards/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get trial NCT06625320 eligibility
biomcp get trial NCT06887088
```

`biomcp schema biomcp.trial_cards/2` prints the full JSON Schema.

## Example reply

`biomcp get trial NCT06625320 eligibility` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.trial_cards/2",
  "status": "complete",
  "query": {
    "input": ["NCT06625320"],
    "section": ["eligibility"],
    "limit": 10,
    "offset": 0
  },
  "result": {
    "items": [
      {…}
    ],
    "source_updated": "2026-10-27T09:00:04",
    "attribution": "ClinicalTrials.gov; registry protocol text, no eligibility…"
  },
  "sources": [
    {
      "source": "ctgov",
      "use": "restricted",
      "terms": "ClinicalTrials.gov attribution and source processing date; pu…",
      "terms_url": "https://clinicaltrials.gov/about-site/terms-conditions",
      "state": "ok"
    }
  ],
  "next": [
    {"kind": "section", "args": ["get", "trial", "NCT06625320", "outcomes"]},
    … 4 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [TrialCards](#trialcards). `query` holds [TrialGet](/reference/schemas/types/#trialget), a shared type. `query` repeats the request as BioMCP read it.

### TrialCards

Trial answers in original input order, retaining independent states.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution and scope. |
| `items` | list of [TrialCardItem](#trialcarditem) | One answer per requested input, including absence and failure. |
| `source_updated` | [Text](/reference/schemas/types/#text) or null | Registry processing timestamp, distinct from each study's update and fetch times. Optional. |

### TrialAgeBound

Source age limit; absence is explicit and never a fabricated number.

One of these forms:

- `state` is `"no_limit"`: Registry supplied no limit.
- `state` is `"source"`: Unchanged source value; no local age conversion. Fields: `raw` (string).

### TrialArm

Registry arm with source intervention relationships.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Markdown](/reference/schemas/types/#markdown) or null | Original arm description. Optional. |
| `intervention_names` | list of string | Source intervention names, including their native prefixes. |
| `kind` | [TrialArmKind](#trialarmkind) or null | Native arm type. Optional. |
| `label` | string | Exact arm label. |

### TrialArmKind

Native registry arm group type from the publisher enum metadata.

Values: `EXPERIMENTAL`, `ACTIVE_COMPARATOR`, `PLACEBO_COMPARATOR`, `SHAM_COMPARATOR`, `NO_INTERVENTION`, `OTHER`.

### TrialArmRecord

Source arm rows followed by source intervention rows, each in its native order.

One of these forms:

- `record` is `"arm"`: Original arm with its intervention names. Fields: `data` ([TrialArm](#trialarm)).
- `record` is `"intervention"`: Original intervention with aliases and arm labels. Fields: `data` ([TrialIntervention](#trialintervention)).

### TrialCard

Complete default registry card.

| Field | Type | Meaning |
| --- | --- | --- |
| `brief_summary` | [Markdown](/reference/schemas/types/#markdown) or null | Whole brief summary, preserving whitespace. Optional. |
| `completion` | [TrialDate](#trialdate) or null | Source completion date, when present. Optional. |
| `conditions` | list of string | All registered condition names. |
| `interventions` | list of string | All registered intervention names. |
| `nct_id` | [NctId](/reference/schemas/types/#nctid) | Exact registry identity. |
| `phases` | list of [TrialPhase](/reference/schemas/types/#trialphase) | Source phases without inferred defaults. |
| `primary_completion` | [TrialDate](#trialdate) or null | Source primary completion date, when present. Optional. |
| `sponsor` | string or null | Lead sponsor as registered. Optional. |
| `start` | [TrialDate](#trialdate) or null | Source start date, when present. Optional. |
| `status` | [TrialStatus](/reference/schemas/types/#trialstatus) | Source overall status. |
| `study_type` | [StudyType](/reference/schemas/types/#studytype) | Native study type. |
| `title` | string | Complete brief title. |
| `updated` | [TrialDate](#trialdate) | Last posted registry update. |
| `url` | string | Canonical publisher record URL. |

### TrialCardItem

Result for one exact requested NCT identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `card` | [TrialCard](#trialcard) or null | Source card, absent after confirmed 404 or failed lookup. Optional. |
| `fetched` | integer or null | Original record retrieval timestamp. Optional. |
| `input` | [NctId](/reference/schemas/types/#nctid) | Exact requested identity. |
| `more` | list of [TrialSection](/reference/schemas/types/#trialsection) | Available sections omitted from this request. Optional. |
| `sections` | [TrialSections](#trialsections) or null | Explicitly requested source sections. Optional. |
| `status` | [Status](/reference/schemas/types/#status) | Independent resolution state. |

### TrialContact

A registry contact with its source role and original contact details.

| Field | Type | Meaning |
| --- | --- | --- |
| `email` | string or null | Source email address. Optional. |
| `name` | string or null | Source name. Optional. |
| `phone` | string or null | Source telephone number, without normalization. Optional. |
| `phone_ext` | string or null | Source extension, kept separate from the number. Optional. |
| `role` | [TrialContactRole](#trialcontactrole) or null | Source contact role, absent if the registry omitted it. Optional. |

### TrialContactRecord

Distinct registry-wide contact collections; site contacts stay on locations.

One of these forms:

- `record` is `"central"`: A central contact for this trial. Fields: `data` ([TrialContact](#trialcontact)).
- `record` is `"official"`: An overall official for this trial. Fields: `data` ([TrialOfficial](#trialofficial)).

### TrialContactRole

Native contact role.

Values: `CONTACT`, `STUDY_CHAIR`, `STUDY_DIRECTOR`, `PRINCIPAL_INVESTIGATOR`, `SUB_INVESTIGATOR`.

### TrialCoordinates

Checked geographic coordinates in degrees; no genomic coordinate type is involved.

| Field | Type | Meaning |
| --- | --- | --- |
| `latitude` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Latitude in degrees. |
| `longitude` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Longitude in degrees. |

### TrialDate

A registry date retains its actual or estimated qualifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [PartialDate](/reference/schemas/types/#partialdate) | Checked source date at its original precision. |
| `kind` | [TrialDateKind](#trialdatekind) or null | Source qualifier, absent only if the registry omitted it. Optional. |

### TrialDateKind

Registry date qualifier, without local interpretation.

Values: `ACTUAL`, `ESTIMATED`.

### TrialEligibilityPage

Character paging preserves source whitespace and counts Unicode scalar values.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [TrialGet](/reference/schemas/types/#trialget) or null | Checked request for the next intact fragment, absent when complete. Optional. |
| `criteria` | string | Exact original text fragment, including whitespace. |
| `criterion_continues` | boolean | The page ends within a criterion whose lines exceed the page budget. Optional. |
| `healthy_volunteers` | boolean or null | Source healthy-volunteer field. Optional. |
| `maximum_age` | [TrialAgeBound](#trialagebound) | Source maximum age. |
| `minimum_age` | [TrialAgeBound](#trialagebound) | Source minimum age. |
| `offset` | integer (uint32) | Original character offset. |
| `sex` | [TrialSex](/reference/schemas/types/#trialsex) or null | Source sex field. Optional. |
| `total_characters` | integer (uint32) | Complete original text length in Unicode scalar values. |

### TrialIntervention

Registry intervention, retaining aliases and arm labels.

| Field | Type | Meaning |
| --- | --- | --- |
| `arm_group_labels` | list of string | Source arm relationships. |
| `description` | [Markdown](/reference/schemas/types/#markdown) or null | Original description. Optional. |
| `kind` | [TrialInterventionKind](#trialinterventionkind) or null | Native intervention type. Optional. |
| `name` | string | Exact source name. |
| `other_names` | list of string | Source aliases, without invented expansion. |

### TrialInterventionKind

Native registry intervention type from the publisher enum metadata.

Values: `BEHAVIORAL`, `BIOLOGICAL`, `COMBINATION_PRODUCT`, `DEVICE`, `DIAGNOSTIC_TEST`, `DIETARY_SUPPLEMENT`, `DRUG`, `GENETIC`, `PROCEDURE`, `RADIATION`, `OTHER`.

### TrialLocation

One source site, kept whole during pagination.

| Field | Type | Meaning |
| --- | --- | --- |
| `city` | string or null | Source city. Optional. |
| `contacts` | list of [TrialContact](#trialcontact) | Contacts belong only to this site, in source order. |
| `coordinates` | [TrialCoordinates](#trialcoordinates) or null | Registry coordinates without a derived distance. Optional. |
| `country` | string or null | Source country. Optional. |
| `facility` | string or null | Source facility name, absent only when not supplied. Optional. |
| `state` | string or null | Source state or region. Optional. |
| `status` | [TrialSiteStatus](#trialsitestatus) or null | This site's native status; never copied from the overall trial status. Optional. |
| `zip` | string or null | Source postal code. Optional. |

### TrialOfficial

An overall official, distinct from a central or site contact.

| Field | Type | Meaning |
| --- | --- | --- |
| `affiliation` | string or null | Source organization. Optional. |
| `name` | string or null | Registered name. Optional. |
| `role` | [TrialOfficialRole](#trialofficialrole) or null | Source official role. Optional. |

### TrialOfficialRole

Native overall official role.

Values: `SUB_INVESTIGATOR`, `PRINCIPAL_INVESTIGATOR`, `STUDY_DIRECTOR`, `STUDY_CHAIR`.

### TrialOutcome

One whole registered outcome with its attached time frame.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Markdown](/reference/schemas/types/#markdown) or null | Original description. Optional. |
| `kind` | [TrialOutcomeKind](#trialoutcomekind) | Source collection. |
| `measure` | string | Original measure text. |
| `time_frame` | string or null | Original time frame. Optional. |

### TrialOutcomeKind

Registry outcome collection.

Values: `primary`, `secondary`, `other`.

### TrialReference

An unchanged registry reference in source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `citation` | string | Original citation text. |
| `kind` | string | Native reference classification, such as RESULT or BACKGROUND. |
| `pmid` | [Pmid](/reference/schemas/types/#pmid) or null | Registry-supplied PubMed identity, when present. Optional. |

### TrialSections

Independently bounded sections from the same source protocol snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `arms` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialArmRecord](#trialarmrecord) or null | Whole source arm and intervention records. Optional. |
| `contacts` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialContactRecord](#trialcontactrecord) or null | Central contacts followed by overall officials, each in source order. Optional. |
| `documents` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialDocument](/reference/schemas/types/#trialdocument) or null | Posted document manifest, without downloading its files. Optional. |
| `eligibility` | [Section](/reference/schemas/types/#section) of [TrialEligibilityPage](#trialeligibilitypage) or null | Source eligibility fields and exact text fragment. Optional. |
| `locations` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialLocation](#triallocation) or null | Whole sites with their original contacts attached. Optional. |
| `outcomes` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialOutcome](#trialoutcome) or null | Whole registered outcomes. Optional. |
| `references` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialReference](#trialreference) or null | Source publication records, without inferred roles. Optional. |

### TrialSiteStatus

Native site recruitment status, distinct from overall trial status.

Values: `NOT_YET_RECRUITING`, `RECRUITING`, `ENROLLING_BY_INVITATION`, `ACTIVE_NOT_RECRUITING`, `SUSPENDED`, `TERMINATED`, `COMPLETED`, `WITHDRAWN`, `AVAILABLE`.

## Next

[`biomcp get trial`](/reference/commands/get-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
