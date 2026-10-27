---
title: biomcp.nci_trial_cards/2
description: "The BioMCP 1.0.0 reply schema biomcp.nci_trial_cards/2: its
  envelope fields, its result type NciTrialCards and the commands that return
  it."
goal: Look up the fields of a biomcp.nci_trial_cards/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci_trial_cards/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.nci_trial_cards/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.nci_trial_cards/2`.

This call returns it.

- `biomcp get trial NCT06887088 --source nci` ([`biomcp get trial`](/reference/commands/get-trial/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [TrialGet](#query-trialget) | yes | Parsed query. |
| `result` | [NciTrialCards](#result-ncitrialcards) or null | no | Answer when available. |
| `schema` | "biomcp.nci_trial_cards/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: NciTrialCards

Independent exact NCI answers in caller order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `items` | array of [NciTrialItem](#ncitrialitem) | yes | One independent answer per requested identity. |

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

### NciArm

Source arm header; interventions follow as indexed records.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | string or null | no | Whole original description. |
| `name` | string or null | no | Original arm name. |
| `type` | [Text](#text) or null | no | Native arm type. |

### NciArmRecord

A record position preserves every arm/intervention relationship without invented identity.

One of:

- object: Original arm header, followed by its intervention rows.
- object: Original intervention and its unchanged parent position.

### NciCriterion

One intact source criterion; the publisher supplies its classification.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | string | yes | Original source text, preserving every character. |
| `display_order` | integer or null | no | Original publisher display order. |
| `inclusion_indicator` | boolean or null | no | Native source inclusion marker. |

### NciDisease

One NCI disease concept; names never substitute for an NCIt identity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `inclusion_indicator` | [Text](#text) or null | no | Publisher coding provenance, without local interpretation. |
| `is_lead_disease` | boolean or null | no | Native lead-disease marker. |
| `name` | [Text](#text) | yes | Source concept name. |
| `nci_thesaurus_concept_id` | [Text](#text) | yes | Publisher NCIt identity. |
| `parents` | array of [Text](#text) | no | Original immediate ancestors. |
| `synonyms` | array of [Text](#text) | no | Original alternative labels. |
| `type` | array of [Text](#text) | no | Original concept categories. |

### NciDiseaseRef

Compact source disease reference; the diseases section retains every coding field.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Native source name. |
| `nci_thesaurus_concept_id` | [Text](#text) | yes | Source NCIt identity. |

### NciEligibilityPage

Native eligibility fields and whole unclassified criteria.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `structured` | [NciEligibilityStructured](#ncieligibilitystructured) or null | no | Source structured bounds and sex, without interpretation. |
| `unstructured` | [TrialRecords2](#trialrecords2) | yes | Whole source criteria in original order. |

### NciEligibilityStructured

Native NCI structured fields; sentinel ages remain native values.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accepts_healthy_volunteers` | boolean or null | no | Native accepts healthy volunteers field. |
| `max_age` | [Text](#text) or null | no | Native max age field. |
| `max_age_in_years` | [FiniteNumber](#finitenumber) or null | no | Native max age in years field. |
| `max_age_number` | [FiniteNumber](#finitenumber) or null | no | Native max age number field. |
| `max_age_unit` | [Text](#text) or null | no | Native max age unit field. |
| `min_age` | [Text](#text) or null | no | Native min age field. |
| `min_age_in_years` | [FiniteNumber](#finitenumber) or null | no | Native min age in years field. |
| `min_age_number` | [FiniteNumber](#finitenumber) or null | no | Native min age number field. |
| `min_age_unit` | [Text](#text) or null | no | Native min age unit field. |
| `sex` | [Text](#text) or null | no | Native sex field. |

### NciIntervention

One original NCI intervention attached to its source arm.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `category` | [Text](#text) or null | no | Native category field. |
| `description` | string or null | no | Native description field. |
| `inclusion_indicator` | [Text](#text) or null | no | Native inclusion indicator field. |
| `name` | [Text](#text) | yes | Native name field. |
| `nci_thesaurus_concept_id` | [Text](#text) | yes | Native nci thesaurus concept id field. |
| `parents` | array of [Text](#text) | yes | Native parents field. |
| `synonyms` | array of [Text](#text) | yes | Native synonyms field. |
| `type` | [Text](#text) or null | no | Native r#type field. |

### NciOutcome

One complete NCI outcome with its original vocabulary.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | string or null | no | Original publisher description field. |
| `name` | string or null | no | Original publisher name field. |
| `timeframe` | string or null | no | Original publisher timeframe field. |
| `type_code` | string or null | no | Original publisher type_code field. |

### NciTextPage

Exact source text fragment with an honest original character count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next original characters, absent when complete. |
| `offset` | integer (uint32) | yes | Original Unicode scalar position. |
| `text` | string | yes | Original source fragment, including whitespace. |
| `total_characters` | integer (uint32) | yes | Complete source length in Unicode scalar values. |

### NciTrial

NCI trial card without registry status or phase coercion.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `brief_summary` | [NciTextPage](#ncitextpage) or null | no | Native short summary, only on exact retrieval. |
| `brief_title` | [Text](#text) | yes | Native title. |
| `current_trial_status` | [Text](#text) | yes | Native status, without a registry translation. |
| `diseases` | [TrialRecords](#trialrecords) | yes | Publisher-provided disease concepts in source order. |
| `nci_id` | [Text](#text) | yes | Publisher NCI identifier. |
| `nct_id` | [NctId](#nctid) | yes | Registry cross-reference supplied by NCI. |
| `phase` | [Text](#text) or null | no | Native phase. |

### NciTrialItem

An exact NCI card and its source state.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `card` | [NciTrial](#ncitrial) or null | no | Native source card, absent after failure or confirmed absence. |
| `fetched` | integer or null | no | Original reply retrieval time. |
| `input` | [NctId](#nctid) | yes | Exact caller identity. |
| `more` | array of [TrialSection](#trialsection) | no | Proven sections omitted from this request. |
| `sections` | [NciTrialSections](#ncitrialsections) or null | no | Explicitly requested, source-native sections. |
| `status` | [Status](#status) | yes | Independent retrieval state. |
| `unsupported_sections` | array of [TrialSection](#trialsection) | no | Requested sections without proven public-tier support. |

### NciTrialSections

Independently paged native collections from one immutable source record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `arms` | [Section4](#section4) or null | no | Arm headers and their attached intervention records. |
| `diseases` | [Section5](#section5) or null | no | Every source disease concept in unchanged order. |
| `eligibility` | [Section2](#section2) or null | no | Source criteria and structured eligibility. |
| `outcomes` | [Section3](#section3) or null | no | Source outcomes without registry type coercion. |
| `summary` | [Section](#section) or null | no | Complete native summary with character continuations. |

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

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### TrialRecords

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [NciDiseaseRef](#ncidiseaseref) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords2

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [NciCriterion](#ncicriterion) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords3

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [NciOutcome](#ncioutcome) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords4

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [NciArmRecord](#nciarmrecord) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialRecords5

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#query-trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [NciDisease](#ncidisease) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialSection

Available source protocol sections.

Values: `eligibility`, `outcomes`, `arms`, `locations`, `contacts`, `references`, `documents`, `all`, `diseases`, `summary`.

### TrialSource

Independent trial providers; results are never folded or used as fallbacks.

Values: `ctgov`, `nci`.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.nci_trial_cards/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
