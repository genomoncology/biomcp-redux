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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get trial eligibility`](/reference/commands/get-trial/) and [`biomcp get trial`](/reference/commands/get-trial/) reply with `schema` set to `biomcp.trial_cards/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get trial NCT06625320 eligibility
biomcp get trial NCT06887088
```

`biomcp schema biomcp.trial_cards/2` prints the full JSON Schema.

## Example reply

`biomcp get trial NCT06625320 eligibility` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "input": "NCT06625320",
        "status": "complete",
        "card": {…},
        "fetched": 1793140313,
        "sections": {…},
        "more": [
          "outcomes", "arms", "locations", "contacts", "references", "documents"
        ]
      }
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
    {
      "kind": "section",
      "args": ["get", "trial", "NCT06625320", "outcomes"]
    },
    … 4 more
  ]
}
```

## Fields

`result` holds [TrialCards](#trialcards). `query` holds [TrialGet](/reference/schemas/types/#trialget), a shared type. `query` repeats the request as BioMCP read it.

### TrialCards

Trial answers in original input order, retaining independent states.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution and scope. |
| `items` | list of [TrialCardItem](#trialcarditem) | yes | One answer per requested input, including absence and failure. |
| `source_updated` | [Text](/reference/schemas/types/#text) or null | no | Registry processing timestamp, distinct from each study's update and fetch times. |

### TrialAgeBound

Source age limit; absence is explicit and never a fabricated number.

One of these forms:

- `state` is `"no_limit"`: Registry supplied no limit.
- `state` is `"source"`: Unchanged source value; no local age conversion. Fields: `raw` (string).

### TrialArm

Registry arm with source intervention relationships.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | [Markdown](/reference/schemas/types/#markdown) or null | no | Original arm description. |
| `intervention_names` | list of string | yes | Source intervention names, including their native prefixes. |
| `kind` | [TrialArmKind](#trialarmkind) or null | no | Native arm type. |
| `label` | string | yes | Exact arm label. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `brief_summary` | [Markdown](/reference/schemas/types/#markdown) or null | no | Whole brief summary, preserving whitespace. |
| `completion` | [TrialDate](#trialdate) or null | no | Source completion date, when present. |
| `conditions` | list of string | yes | All registered condition names. |
| `interventions` | list of string | yes | All registered intervention names. |
| `nct_id` | [NctId](/reference/schemas/types/#nctid) | yes | Exact registry identity. |
| `phases` | list of [TrialPhase](/reference/schemas/types/#trialphase) | yes | Source phases without inferred defaults. |
| `primary_completion` | [TrialDate](#trialdate) or null | no | Source primary completion date, when present. |
| `sponsor` | string or null | no | Lead sponsor as registered. |
| `start` | [TrialDate](#trialdate) or null | no | Source start date, when present. |
| `status` | [TrialStatus](/reference/schemas/types/#trialstatus) | yes | Source overall status. |
| `study_type` | [StudyType](/reference/schemas/types/#studytype) | yes | Native study type. |
| `title` | string | yes | Complete brief title. |
| `updated` | [TrialDate](#trialdate) | yes | Last posted registry update. |
| `url` | string | yes | Canonical publisher record URL. |

### TrialCardItem

Result for one exact requested NCT identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `card` | [TrialCard](#trialcard) or null | no | Source card, absent after confirmed 404 or failed lookup. |
| `fetched` | integer or null | no | Original record retrieval timestamp. |
| `input` | [NctId](/reference/schemas/types/#nctid) | yes | Exact requested identity. |
| `more` | list of [TrialSection](/reference/schemas/types/#trialsection) | no | Available sections omitted from this request. |
| `sections` | [TrialSections](#trialsections) or null | no | Explicitly requested source sections. |
| `status` | [Status](/reference/schemas/types/#status) | yes | Independent resolution state. |

### TrialContact

A registry contact with its source role and original contact details.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `email` | string or null | no | Source email address. |
| `name` | string or null | no | Source name. |
| `phone` | string or null | no | Source telephone number, without normalization. |
| `phone_ext` | string or null | no | Source extension, kept separate from the number. |
| `role` | [TrialContactRole](#trialcontactrole) or null | no | Source contact role, absent if the registry omitted it. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `latitude` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Latitude in degrees. |
| `longitude` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Longitude in degrees. |

### TrialDate

A registry date retains its actual or estimated qualifier.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `date` | [PartialDate](/reference/schemas/types/#partialdate) | yes | Checked source date at its original precision. |
| `kind` | [TrialDateKind](#trialdatekind) or null | no | Source qualifier, absent only if the registry omitted it. |

### TrialDateKind

Registry date qualifier, without local interpretation.

Values: `ACTUAL`, `ESTIMATED`.

### TrialEligibilityPage

Character paging preserves source whitespace and counts Unicode scalar values.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](/reference/schemas/types/#trialget) or null | no | Checked request for the next intact fragment, absent when complete. |
| `criteria` | string | yes | Exact original text fragment, including whitespace. |
| `criterion_continues` | boolean | no | The page ends within a criterion whose lines exceed the page budget. |
| `healthy_volunteers` | boolean or null | no | Source healthy-volunteer field. |
| `maximum_age` | [TrialAgeBound](#trialagebound) | yes | Source maximum age. |
| `minimum_age` | [TrialAgeBound](#trialagebound) | yes | Source minimum age. |
| `offset` | integer (uint32) | yes | Original character offset. |
| `sex` | [TrialSex](/reference/schemas/types/#trialsex) or null | no | Source sex field. |
| `total_characters` | integer (uint32) | yes | Complete original text length in Unicode scalar values. |

### TrialIntervention

Registry intervention, retaining aliases and arm labels.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `arm_group_labels` | list of string | yes | Source arm relationships. |
| `description` | [Markdown](/reference/schemas/types/#markdown) or null | no | Original description. |
| `kind` | [TrialInterventionKind](#trialinterventionkind) or null | no | Native intervention type. |
| `name` | string | yes | Exact source name. |
| `other_names` | list of string | yes | Source aliases, without invented expansion. |

### TrialInterventionKind

Native registry intervention type from the publisher enum metadata.

Values: `BEHAVIORAL`, `BIOLOGICAL`, `COMBINATION_PRODUCT`, `DEVICE`, `DIAGNOSTIC_TEST`, `DIETARY_SUPPLEMENT`, `DRUG`, `GENETIC`, `PROCEDURE`, `RADIATION`, `OTHER`.

### TrialLocation

One source site, kept whole during pagination.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `city` | string or null | no | Source city. |
| `contacts` | list of [TrialContact](#trialcontact) | yes | Contacts belong only to this site, in source order. |
| `coordinates` | [TrialCoordinates](#trialcoordinates) or null | no | Registry coordinates without a derived distance. |
| `country` | string or null | no | Source country. |
| `facility` | string or null | no | Source facility name, absent only when not supplied. |
| `state` | string or null | no | Source state or region. |
| `status` | [TrialSiteStatus](#trialsitestatus) or null | no | This site's native status; never copied from the overall trial status. |
| `zip` | string or null | no | Source postal code. |

### TrialOfficial

An overall official, distinct from a central or site contact.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `affiliation` | string or null | no | Source organization. |
| `name` | string or null | no | Registered name. |
| `role` | [TrialOfficialRole](#trialofficialrole) or null | no | Source official role. |

### TrialOfficialRole

Native overall official role.

Values: `SUB_INVESTIGATOR`, `PRINCIPAL_INVESTIGATOR`, `STUDY_DIRECTOR`, `STUDY_CHAIR`.

### TrialOutcome

One whole registered outcome with its attached time frame.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | [Markdown](/reference/schemas/types/#markdown) or null | no | Original description. |
| `kind` | [TrialOutcomeKind](#trialoutcomekind) | yes | Source collection. |
| `measure` | string | yes | Original measure text. |
| `time_frame` | string or null | no | Original time frame. |

### TrialOutcomeKind

Registry outcome collection.

Values: `primary`, `secondary`, `other`.

### TrialReference

An unchanged registry reference in source order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `citation` | string | yes | Original citation text. |
| `kind` | string | yes | Native reference classification, such as RESULT or BACKGROUND. |
| `pmid` | [Pmid](/reference/schemas/types/#pmid) or null | no | Registry-supplied PubMed identity, when present. |

### TrialSections

Independently bounded sections from the same source protocol snapshot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `arms` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialArmRecord](#trialarmrecord) or null | no | Whole source arm and intervention records. |
| `contacts` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialContactRecord](#trialcontactrecord) or null | no | Central contacts followed by overall officials, each in source order. |
| `documents` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialDocument](/reference/schemas/types/#trialdocument) or null | no | Posted document manifest, without downloading its files. |
| `eligibility` | [Section](/reference/schemas/types/#section) of [TrialEligibilityPage](#trialeligibilitypage) or null | no | Source eligibility fields and exact text fragment. |
| `locations` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialLocation](#triallocation) or null | no | Whole sites with their original contacts attached. |
| `outcomes` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialOutcome](#trialoutcome) or null | no | Whole registered outcomes. |
| `references` | [Section](/reference/schemas/types/#section) of [TrialRecords](/reference/schemas/types/#trialrecords) of [TrialReference](#trialreference) or null | no | Source publication records, without inferred roles. |

### TrialSiteStatus

Native site recruitment status, distinct from overall trial status.

Values: `NOT_YET_RECRUITING`, `RECRUITING`, `ENROLLING_BY_INVITATION`, `ACTIVE_NOT_RECRUITING`, `SUSPENDED`, `TERMINATED`, `COMPLETED`, `WITHDRAWN`, `AVAILABLE`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get trial`](/reference/commands/get-trial/) gives the command's arguments.
