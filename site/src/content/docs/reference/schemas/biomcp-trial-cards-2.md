---
title: biomcp.trial_cards/2
description: "The BioMCP 1.0.0 reply schema biomcp.trial_cards/2: its envelope
  fields, its result type TrialCards and the commands that return it."
goal: Look up the fields of a biomcp.trial_cards/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial_cards/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.trial_cards/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.trial_cards/2`.

These commands return it: [`biomcp get trial`](/reference/commands/get-trial/).

Calls that returned it:

- `biomcp get trial NCT06625320 eligibility` ([`biomcp get trial`](/reference/commands/get-trial/))
- `biomcp get trial NCT06887088` ([`biomcp get trial`](/reference/commands/get-trial/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [TrialGet](#query-trialget) | yes | Parsed query. |
| `result` | [TrialCards](#result-trialcards) or null | no | Answer when available. |
| `schema` | "biomcp.trial_cards/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: TrialCards

Trial answers in original input order, retaining independent states.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution and scope. |
| `items` | array of [TrialCardItem](#trialcarditem) | yes | One answer per requested input, including absence and failure. |
| `source_updated` | [Text](#text) or null | no | Registry processing timestamp, distinct from each study's update and fetch times. |

## Query: TrialGet

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of [NctId](#nctid) | yes | One to twenty exact NCT identifiers, in caller order. |
| `limit` | integer (uint32) | no | Whole-record count per source collection page; text fragments follow the text budget. |
| `offset` | integer (uint32) | no | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. |
| `pool` | [Text](#text) | no | Checked stored source record, supplied by a continuation. |
| `section` | array of [TrialSection](#trialsection) | no | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. |
| `source` | [TrialSource](#trialsource) | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### License

License reported by the bucket metadata record.

Values: `cc_by`, `cc_by_nc`, `cc_by_sa`, `cc_by_nc_sa`, `cc_by_nd`, `cc_by_nc_nd`, `cc0`, `public_domain`, `tdm`, `other`.

### Markdown

Nonempty text with surrounding Unicode whitespace removed.

Type: string (markdown).

### NctId

Type: string.

### PageInfo

Search counts after filtering and deduplication.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Skipped results. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Optional pooled snapshot. |
| `returned` | integer (uint32) | yes | Results returned on this page. |
| `total` | integer (uint64) | yes | Total matching identities held by this snapshot. |
| `truncated` | boolean or null | no | Matching source count exceeded the retrieved window. |

### PartialDate

Type: string.

### Pmid

Type: string.

### PoolInfo

Fixed lifetime and size of a stored search snapshot.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `created` | integer (uint64) | yes | Publication wall time in Unix seconds. |
| `expires` | integer (uint64) | yes | Fixed expiry wall time in Unix seconds. |
| `id` | [Text](#text) | yes | Opaque snapshot identifier. |
| `size` | integer (uint32) | yes | Number of ordered identities. |

### Problem

A field that could not be accepted from a source record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `code` | [ProblemCode](#problemcode) | yes | Why the field value was not accepted. |
| `field` | string | yes | The biodata field name. |
| `record` | [Text](#text) or null | no | The record's canonical identifier, when known. |
| `source` | [Source](#source) | yes | The source that supplied the record. |

### ProblemCode

A stable code for a rejected field value.

Values: `value.unknown`, `collection.incomplete`, `identity.conflict`.

### Reason

Why a source could not supply data.

Values: `not_found`, `not_open`, `not_pdf`, `unsupported`, `too_large`, `rate_limited`, `unavailable`, `tls`, `proxy`, `rejected`, `bad_reply`, `deadline_exceeded`, `offline`, `no_key`, `disabled`, `replay_miss`, `syncing`, `no_join_id`.

### Section

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section2

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section3

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section4

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section5

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section6

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section7

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Source

A source of BioMCP data, identified by its canonical short name.

Values: `hgnc`, `mygene`, `reactome`, `wikipathways`, `hpa`, `uniprot`, `interpro`, `complex-portal`, `gnomad`, `clinvar`, `erepo`, `gwas`, `clingen`, `pubmed`, `europepmc`, `litvar`, `litsense`, `semantic-scholar`, `orcid`, `pmc-oa`, `icite`, `pubtator`, `ctgov`, `nci`, `civic`, `hotspots`, `alphagenome`, `oncokb`, `myvariant`, `car`, `mutalyzer`, `variantvalidator`, `mychem`, `chembl`, `opentargets`, `openfda-label`, `drugsfda`, `ema`, `who-pq`, `ddinter`, `drugbank-open`, `drugcentral`, `gtopdb`, `unii`, `ndc`, `openfda-xref`, `ols`, `mydisease`, `monarch`, `seer`, `mondo`, `doid`, `ncit`, `hpo`, `orphanet`, `omim`, `faers`, `vaers`, `maude`, `meddra`, `gmdn`, `gtr`, `who-ivd`, `fda-cdx`, `fda-pma`, `fda-510k`, `cpic`, `clinpgx`, `cbioportal`, `cellosaurus`.

### SourceNotice

A provider query interpretation notice.

One of:

- object: A checked provider answer survived a local cache failure.
- object: The source still has graph pages after the bounded fetch window.
- object: An empty LitVar lookup can be repeated with a caller-supplied rsID.
- object: PubMed supplied a named ESearch warning without changing its phrases.
- object: PubMed did not index these words.

### SourceOutcome

One source's outcome, without provider text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count` | integer or null | no | Provider matching count. |
| `fetched` | integer or null | no | Retrieved identity depth. |
| `notices` | array of [SourceNotice](#sourcenotice) | no | Provider query notices that do not indicate failure. |
| `reason` | [Reason](#reason) or null | no | Failure or stale-fallback reason. |
| `source` | [Source](#source) | yes | Asked source. |
| `state` | [SourceState](#sourcestate) | yes | Retained state. |
| `term` | [Text](#text) or null | no | Exact provider search term. |
| `terms` | string | yes | Publisher terms name from the source registry. |
| `terms_url` | string | yes | Publisher page documenting the terms. |
| `use` | [Use](#use) | yes | Reviewed source use category; publisher terms remain authoritative. |

### SourceRecord

The source's own entry for an entity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Native source identifier. |
| `url` | [Text](#text) or null | no | Link to that entry. |

### SourceState

State of a source actually asked by the command.

Values: `ok`, `stale`, `syncing`, `failed`, `skipped`.

### Status

Overall command outcome.

Values: `complete`, `partial`, `absent`, `failed`.

### Step

A command argument list spelled from a typed action.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `args` | array of string | yes | CLI and MCP arguments. |
| `kind` | [StepKind](#stepkind) | yes | Purpose of the action. |

### StepKind

Purpose of an available next command.

Values: `restart`, `section`, `sort`, `page`, `fresh`, `disambiguate`, `pivot`, `sync`, `key`, `configure`.

### StudyType

Source-native registry value.

Values: `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS`.

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### TrialAgeBound

Source age limit; absence is explicit and never a fabricated number.

One of:

- object: Registry supplied no limit.
- object: Unchanged source value; no local age conversion.

### TrialArm

Registry arm with source intervention relationships.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | [Markdown](#markdown) or null | no | Original arm description. |
| `intervention_names` | array of string | yes | Source intervention names, including their native prefixes. |
| `kind` | [TrialArmKind](#trialarmkind) or null | no | Native arm type. |
| `label` | string | yes | Exact arm label. |

### TrialArmKind

Native registry arm group type from the publisher enum metadata.

Values: `EXPERIMENTAL`, `ACTIVE_COMPARATOR`, `PLACEBO_COMPARATOR`, `SHAM_COMPARATOR`, `NO_INTERVENTION`, `OTHER`.

### TrialArmRecord

Source arm rows followed by source intervention rows, each in its native order.

One of:

- object: Original arm with its intervention names.
- object: Original intervention with aliases and arm labels.

### TrialCard

Complete default registry card.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `brief_summary` | [Markdown](#markdown) or null | no | Whole brief summary, preserving whitespace. |
| `completion` | [TrialDate](#trialdate) or null | no | Source completion date, when present. |
| `conditions` | array of string | yes | All registered condition names. |
| `interventions` | array of string | yes | All registered intervention names. |
| `nct_id` | [NctId](#nctid) | yes | Exact registry identity. |
| `phases` | array of [TrialPhase](#trialphase) | yes | Source phases without inferred defaults. |
| `primary_completion` | [TrialDate](#trialdate) or null | no | Source primary completion date, when present. |
| `sponsor` | string or null | no | Lead sponsor as registered. |
| `start` | [TrialDate](#trialdate) or null | no | Source start date, when present. |
| `status` | [TrialStatus](#trialstatus) | yes | Source overall status. |
| `study_type` | [StudyType](#studytype) | yes | Native study type. |
| `title` | string | yes | Complete brief title. |
| `updated` | [TrialDate](#trialdate) | yes | Last posted registry update. |
| `url` | string | yes | Canonical publisher record URL. |

### TrialCardItem

Result for one exact requested NCT identity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `card` | [TrialCard](#trialcard) or null | no | Source card, absent after confirmed 404 or failed lookup. |
| `fetched` | integer or null | no | Original record retrieval timestamp. |
| `input` | [NctId](#nctid) | yes | Exact requested identity. |
| `more` | array of [TrialSection](#trialsection) | no | Available sections omitted from this request. |
| `sections` | [TrialSections](#trialsections) or null | no | Explicitly requested source sections. |
| `status` | [Status](#status) | yes | Independent resolution state. |

### TrialContact

A registry contact with its source role and original contact details.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `email` | string or null | no | Source email address. |
| `name` | string or null | no | Source name. |
| `phone` | string or null | no | Source telephone number, without normalization. |
| `phone_ext` | string or null | no | Source extension, kept separate from the number. |
| `role` | [TrialContactRole](#trialcontactrole) or null | no | Source contact role, absent if the registry omitted it. |

### TrialContactRecord

Distinct registry-wide contact collections; site contacts stay on locations.

One of:

- object: A central contact for this trial.
- object: An overall official for this trial.

### TrialContactRole

Native contact role.

Values: `CONTACT`, `STUDY_CHAIR`, `STUDY_DIRECTOR`, `PRINCIPAL_INVESTIGATOR`, `SUB_INVESTIGATOR`.

### TrialCoordinates

Checked geographic coordinates in degrees; no genomic coordinate type is involved.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `latitude` | [FiniteNumber](#finitenumber) | yes | Latitude in degrees. |
| `longitude` | [FiniteNumber](#finitenumber) | yes | Longitude in degrees. |

### TrialDate

A registry date retains its actual or estimated qualifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `date` | [PartialDate](#partialdate) | yes | Checked source date at its original precision. |
| `kind` | [TrialDateKind](#trialdatekind) or null | no | Source qualifier, absent only if the registry omitted it. |

### TrialDateKind

Registry date qualifier, without local interpretation.

Values: `ACTUAL`, `ESTIMATED`.

### TrialDocument

A posted source document; a URL never comes from caller-supplied filename text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `date` | [PartialDate](#partialdate) or null | no | Source document date. |
| `filename` | [Text](#text) | yes | Exact advertised portable source filename. |
| `has_icf` | boolean or null | no | Source says this contains an informed consent form. |
| `has_protocol` | boolean or null | no | Source says this contains a protocol. |
| `has_sap` | boolean or null | no | Source says this contains a statistical analysis plan. |
| `kind` | string | yes | Native document type abbreviation. |
| `label` | string or null | no | Source label. |
| `license` | [License](#license) | yes | No blanket reuse license is inferred from registry availability. |
| `size` | integer or null | no | Source byte count, when supplied. |
| `uploaded` | string or null | no | Original upload timestamp, without guessing a timezone. |
| `url` | [Text](#text) | yes | Fixed registry CDN URL for this manifest entry. |

### TrialEligibilityPage

Character paging preserves source whitespace and counts Unicode scalar values.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact fragment, absent when complete. |
| `criteria` | string | yes | Exact original text fragment, including whitespace. |
| `criterion_continues` | boolean | no | The page ends within a criterion whose lines exceed the page budget. |
| `healthy_volunteers` | boolean or null | no | Source healthy-volunteer field. |
| `maximum_age` | [TrialAgeBound](#trialagebound) | yes | Source maximum age. |
| `minimum_age` | [TrialAgeBound](#trialagebound) | yes | Source minimum age. |
| `offset` | integer (uint32) | yes | Original character offset. |
| `sex` | [TrialSex](#trialsex) or null | no | Source sex field. |
| `total_characters` | integer (uint32) | yes | Complete original text length in Unicode scalar values. |

### TrialIntervention

Registry intervention, retaining aliases and arm labels.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `arm_group_labels` | array of string | yes | Source arm relationships. |
| `description` | [Markdown](#markdown) or null | no | Original description. |
| `kind` | [TrialInterventionKind](#trialinterventionkind) or null | no | Native intervention type. |
| `name` | string | yes | Exact source name. |
| `other_names` | array of string | yes | Source aliases, without invented expansion. |

### TrialInterventionKind

Native registry intervention type from the publisher enum metadata.

Values: `BEHAVIORAL`, `BIOLOGICAL`, `COMBINATION_PRODUCT`, `DEVICE`, `DIAGNOSTIC_TEST`, `DIETARY_SUPPLEMENT`, `DRUG`, `GENETIC`, `PROCEDURE`, `RADIATION`, `OTHER`.

### TrialLocation

One source site, kept whole during pagination.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `city` | string or null | no | Source city. |
| `contacts` | array of [TrialContact](#trialcontact) | yes | Contacts belong only to this site, in source order. |
| `coordinates` | [TrialCoordinates](#trialcoordinates) or null | no | Registry coordinates without a derived distance. |
| `country` | string or null | no | Source country. |
| `facility` | string or null | no | Source facility name, absent only when not supplied. |
| `state` | string or null | no | Source state or region. |
| `status` | [TrialSiteStatus](#trialsitestatus) or null | no | This site's native status; never copied from the overall trial status. |
| `zip` | string or null | no | Source postal code. |

### TrialOfficial

An overall official, distinct from a central or site contact.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `affiliation` | string or null | no | Source organization. |
| `name` | string or null | no | Registered name. |
| `role` | [TrialOfficialRole](#trialofficialrole) or null | no | Source official role. |

### TrialOfficialRole

Native overall official role.

Values: `SUB_INVESTIGATOR`, `PRINCIPAL_INVESTIGATOR`, `STUDY_DIRECTOR`, `STUDY_CHAIR`.

### TrialOutcome

One whole registered outcome with its attached time frame.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | [Markdown](#markdown) or null | no | Original description. |
| `kind` | [TrialOutcomeKind](#trialoutcomekind) | yes | Source collection. |
| `measure` | string | yes | Original measure text. |
| `time_frame` | string or null | no | Original time frame. |

### TrialOutcomeKind

Registry outcome collection.

Values: `primary`, `secondary`, `other`.

### TrialPhase

Source-native registry value.

Values: `EARLY_PHASE1`, `PHASE1`, `PHASE2`, `PHASE3`, `PHASE4`, `NA`.

### TrialRecords

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [TrialReference](#trialreference) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords2

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [TrialDocument](#trialdocument) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords3

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [TrialLocation](#triallocation) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords4

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [TrialContactRecord](#trialcontactrecord) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords5

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [TrialOutcome](#trialoutcome) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords6

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [TrialArmRecord](#trialarmrecord) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialReference

An unchanged registry reference in source order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `citation` | string | yes | Original citation text. |
| `kind` | string | yes | Native reference classification, such as RESULT or BACKGROUND. |
| `pmid` | [Pmid](#pmid) or null | no | Registry-supplied PubMed identity, when present. |

### TrialSection

Available source protocol sections.

Values: `eligibility`, `outcomes`, `arms`, `locations`, `contacts`, `references`, `documents`, `all`, `diseases`, `summary`.

### TrialSections

Independently bounded sections from the same source protocol snapshot.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `arms` | [Section7](#section7) or null | no | Whole source arm and intervention records. |
| `contacts` | [Section4](#section4) or null | no | Central contacts followed by overall officials, each in source order. |
| `documents` | [Section2](#section2) or null | no | Posted document manifest, without downloading its files. |
| `eligibility` | [Section5](#section5) or null | no | Source eligibility fields and exact text fragment. |
| `locations` | [Section3](#section3) or null | no | Whole sites with their original contacts attached. |
| `outcomes` | [Section6](#section6) or null | no | Whole registered outcomes. |
| `references` | [Section](#section) or null | no | Source publication records, without inferred roles. |

### TrialSex

Source-native registry value.

Values: `ALL`, `FEMALE`, `MALE`.

### TrialSiteStatus

Native site recruitment status, distinct from overall trial status.

Values: `NOT_YET_RECRUITING`, `RECRUITING`, `ENROLLING_BY_INVITATION`, `ACTIVE_NOT_RECRUITING`, `SUSPENDED`, `TERMINATED`, `COMPLETED`, `WITHDRAWN`, `AVAILABLE`.

### TrialSource

Independent trial providers; results are never folded or used as fallbacks.

Values: `ctgov`, `nci`.

### TrialStatus

Source-native registry value.

Values: `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN`.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.trial_cards/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
