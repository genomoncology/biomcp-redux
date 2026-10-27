---
title: biomcp.nci_trial_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.nci_trial_page/3: its
  envelope fields, its result type NciTrialPage and the commands that return
  it."
goal: Look up the fields of a biomcp.nci_trial_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci_trial_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.nci_trial_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.nci_trial_page/3`.

This call returns it.

- `biomcp search trial --keyword melanoma --source nci --limit 1` ([`biomcp search trial`](/reference/commands/search-trial/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [TrialSearch](#query-trialsearch) | yes | Parsed query. |
| `result` | [NciTrialPage](#result-ncitrialpage) or null | no | Answer when available. |
| `schema` | "biomcp.nci_trial_page/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: NciTrialPage

NCI search page in native order with native count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `condition_resolution` | [TrialConditionResolution](#trialconditionresolution) or null | no | Exact condition identity and source-provided mapping, absent for literal text. |
| `items` | array of [NciTrial](#ncitrial) | yes | Source rows, never merged with another provider. |
| `request` | array of [TrialParameter](#trialparameter) | yes | Exact source parameters, excluding credentials. |

## Query: TrialSearch

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `age` | integer (uint32) | no | Age in years; unstated source bounds count as no limit. |
| `condition` | [Text](#text) | no | Registry condition text, or an NCIt code for NCI; use --keyword for NCI free text. |
| `condition_id` | [Curie](#curie) | no | Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. |
| `count_only` | boolean | no | Source total only; no returned study rows. |
| `criteria` | [Text](#text) | no | Search literal registry eligibility text for discovery only; no eligibility judgment. |
| `distance` | [Text](#text) | no | Source distance with explicit mi or km units. |
| `facility` | [Text](#text) | no | Exact registry facility field; cannot combine with radius. |
| `gene` | [GeneInput](#geneinput) | no | Resolve one checked gene for registry text discovery only; no eligibility judgment. |
| `has_results` | boolean | no | Require posted source results. |
| `id` | [NctId](#nctid) | no | Exact NCT identity in a conjunction with other filters. |
| `intervention` | [Text](#text) | no | Literal source intervention text. |
| `keyword` | [Text](#text) | no | NCI free text; explicit keyword mode never substitutes for a disease identity. |
| `lat` | [Text](#text) | no | Radius center latitude in degrees. |
| `limit` | integer (uint32) | no | Headers per page; NCI applies its own lower publisher limit. |
| `lon` | [Text](#text) | no | Radius center longitude in degrees. |
| `next_page` | [Text](#text) | no | Opaque registry continuation; keep filters unchanged. |
| `phase` | [PhaseFilter](#phasefilter) | no | Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3. |
| `query` | [Text](#text) | no | General registry discovery text. |
| `sex` | [TrialSex](#trialsex) | no | Registry sex facet. |
| `source` | [TrialSource](#trialsource) | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |
| `sponsor` | [Text](#text) | no | Exact lead sponsor field. |
| `sponsor_type` | [SponsorType](#sponsortype) | no | Registry sponsor funding facet. |
| `status` | [TrialStatus](#trialstatus) | no | Exact registry overall status. |
| `study_type` | [StudyType](#studytype) | no | Exact registry study type. |
| `updated_since` | [PartialDate](#partialdate) | no | First included registry update date. |
| `updated_until` | [PartialDate](#partialdate) | no | Last included registry update date. |
| `variant` | [Text](#text) | no | Literal variant text for registry discovery only; no eligibility judgment. |

## Types

Every other type this schema uses.

### Attribution

Provenance for one source record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `evaluated` | [PartialDate](#partialdate) or null | no | Source evaluation date. |
| `record` | [Text](#text) or null | no | Native assertion record. |
| `release` | [Text](#text) or null | no | Dataset release. |
| `source` | [Text](#text) | yes | Source responsible for the claim. |
| `submitter` | [Text](#text) or null | no | Source submitter or panel. |
| `url` | [Text](#text) or null | no | Source record link. |
| `version` | [Text](#text) or null | no | Record version. |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### ComponentLabel

Terms attached to a named carried component on a consumer response.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `component` | [Text](#text) | yes | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | yes | Canonical publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Publisher terms name. |
| `url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Publisher use category. |

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) or null | no | Source-stated concept identifier. |
| `label` | [Text](#text) | yes | Original concept label. |

### Curie

Type: string.

### DiseaseEndpoint

One native endpoint with an identifier and an optional source label.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) | yes | Exact source identifier. |
| `label` | [Text](#text) or null | no | Source label when supplied. |

### DiseaseRelation

A publisher relation with distinct endpoints and predicate.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Publisher record and version. |
| `description` | string or null | no | Unchanged native cross-reference description when supplied. |
| `object` | [DiseaseEndpoint](#diseaseendpoint) | yes | Native object. |
| `predicate` | [Curie](#curie) | yes | Publisher predicate, without equivalence promotion. |
| `subject` | [DiseaseEndpoint](#diseaseendpoint) | yes | Native subject. |

### GeneInput

Type: string.

### NciDiseaseRef

Compact source disease reference; the diseases section retains every coding field.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Native source name. |
| `nci_thesaurus_concept_id` | [Text](#text) | yes | Source NCIt identity. |

### NciTextPage

Exact source text fragment with an honest original character count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#trialget) or null | no | Checked request for the next original characters, absent when complete. |
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

### PhaseFilter

Explicit source phase selection, including the registry's combined phases.

Values: `early-1`, `1`, `2`, `3`, `4`, `not-applicable`, `1/2`, `2/3`.

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

### SourceState

State of a source actually asked by the command.

Values: `ok`, `stale`, `syncing`, `failed`, `skipped`.

### SponsorType

Registry funding category facet.

Values: `nih`, `fed`, `industry`, `other`.

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

### TrialConditionResolution

Exact condition identity used for one trial request, without eligibility inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alias` | [DiseaseRelation](#diseaserelation) or null | no | Native relation proving a requested alias, when required. |
| `attribution` | [Attribution](#attribution) or null | no | Resolver record provenance, independent of the trial provider. |
| `components` | array of [ComponentLabel](#componentlabel) | no | Admitted resolver components and their own terms. |
| `concept` | [Concept](#concept) or null | no | Returned concept with its unchanged native primary label. |
| `input` | [Curie](#curie) | yes | Caller-selected CURIE, never a source-name search. |
| `ncit_candidates` | array of [DiseaseRelation](#diseaserelation) | no | Native NCIt mapping rows; duplicate targets keep their independent provenance. |
| `resolver` | [Source](#source) or null | no | Named exact resolver; absent for explicit NCIt input. |
| `selected_ncit` | [Curie](#curie) or null | no | Explicit or unique source-provided NCIt identifier actually sent to NCI. |

### TrialGet

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of [NctId](#nctid) | yes | One to twenty exact NCT identifiers, in caller order. |
| `limit` | integer (uint32) | no | Whole-record count per source collection page; text fragments follow the text budget. |
| `offset` | integer (uint32) | no | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. |
| `pool` | [Text](#text) | no | Checked stored source record, supplied by a continuation. |
| `section` | array of [TrialSection](#trialsection) | no | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. |
| `source` | [TrialSource](#trialsource) | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |

### TrialParameter

One exact source request parameter, in canonical name order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | string | yes | Native parameter name. |
| `value` | string | yes | Exact unencoded parameter value. |

### TrialRecords

Whole-record page retaining collection order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#trialget) or null | no | Checked request for the next intact records, absent when complete. |
| `items` | array of [NciDiseaseRef](#ncidiseaseref) | yes | Complete records in source order. |
| `offset` | integer (uint32) | yes | Offset in the unchanged source collection. |
| `total` | integer (uint32) | yes | Original collection size. |

### TrialSection

Available source protocol sections.

Values: `eligibility`, `outcomes`, `arms`, `locations`, `contacts`, `references`, `documents`, `all`, `diseases`, `summary`.

### TrialSex

Source-native registry value.

Values: `ALL`, `FEMALE`, `MALE`.

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

This page is generated from `biomcp schema biomcp.nci_trial_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
