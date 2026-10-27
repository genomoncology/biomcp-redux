---
title: biomcp.study_card/2
description: "The BioMCP 1.0.0 reply schema biomcp.study_card/2: its envelope
  fields, its result type StudyCard and the commands that return it."
goal: Look up the fields of a biomcp.study_card/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_card/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.study_card/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.study_card/2`.

These commands return it: [`biomcp get study`](/reference/commands/get-study/).

Calls that returned it:

- `biomcp get study synthetic_0` ([`biomcp get study`](/reference/commands/get-study/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [StudyGet](#query-studyget) | yes | Parsed query. |
| `result` | [StudyCard](#result-studycard) or null | no | Answer when available. |
| `schema` | "biomcp.study_card/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: StudyCard

Exact card with independent live section states and catalog provenance.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `files` | [Section3](#section3) or null | no | Requested file-reference page, with no downloaded data. |
| `profiles` | [Section](#section) or null | no | Requested molecular-profile page, absent when not requested. |
| `sample_lists` | [Section2](#section2) or null | no | Requested sample-list metadata page, without member IDs. |
| `snapshot` | [StudySnapshot](#studysnapshot) | yes | Catalog revision and acquisition, apart from each live section's time. |
| `source_url` | [Text](#text) | yes | Exact publisher study page for this checked source identity. |
| `study` | [Study](#study) | yes | Native card metadata, preserving all source count distinctions. |

## Query: StudyGet

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [StudyId](#studyid) | yes | Exact source study ID; no default or repair. |
| `section` | [StudySection](#studysection) or null | no | Omit for the card; all returns bounded first pages only. |
| `section_limit` | integer (uint32) | no | Maximum records in each selected section. |
| `section_offset` | integer (uint32) | no | Offset within one named section; all requires zero. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### CbioportalCancerTypeId

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

### Study

Source-specific study card metadata; no inferred denominator or gene aggregate.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cnaSampleCount` | integer (uint64) | yes | Native cnaSampleCount. |
| `description` | string | yes | Native description. |
| `groups` | string | yes | Native access groups, possibly empty; not a reuse license. |
| `header` | [StudyHeader](#studyheader) | yes | Compact study identity and publication references. |
| `importDate` | [StudyImportTime](#studyimporttime) | yes | Native timezone-unstated import time. |
| `patientCount` | integer or null | no | Only an explicitly stated card patientCount; captured cards omit it, distinct from zero. |
| `publicStudy` | boolean | yes | Native public-study access metadata. |
| `readPermission` | boolean | yes | Native access metadata; not a reuse license. |
| `resourceCounts` | array of [StudyResourceCount](#studyresourcecount) | yes | Independent resource-scoped counts, never summed into study totals. |
| `sequencedSampleCount` | integer (uint64) | yes | Native sequencedSampleCount, not gene coverage. |
| `structuralVariantCount` | integer (uint64) | yes | Native structuralVariantCount; its name does not imply sample units. |
| `treatmentCount` | integer (uint64) | yes | Native treatmentCount without a unit conversion. |

### StudyBlobSha

Type: string.

### StudyFile

Publisher file reference; repository blob bytes never mean data-content bytes.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `downloadUrl` | [Text](#text) or null | no | Native download link, absent for directory entries. |
| `htmlUrl` | [Text](#text) | yes | Publisher browser link. |
| `kind` | [StudyFileKind](#studyfilekind) | yes | Native file or directory kind. |
| `name` | [Text](#text) | yes | Native repository entry name. |
| `path` | [Text](#text) | yes | Exact native repository path. |
| `repositoryBlobSize` | integer (uint64) | yes | GitHub size in repository blob bytes; LFS content size is unknown. |
| `sha` | [StudyBlobSha](#studyblobsha) | yes | Repository blob or tree revision, not a data-content checksum. |

### StudyFileKind

Native GitHub contents entry kind.

Values: `file`, `dir`.

### StudyFiles

One native repository directory page; blob sizes do not describe data content.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Directory acquisition wall time in Unix seconds. |
| `items` | array of [StudyFile](#studyfile) | yes | Source-ordered repository file references. |
| `license_notice` | [Text](#text) or null | no | Native LICENSE publisher link, independent of the current file page. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds within the complete received directory. |
| `study_id` | [StudyId](#studyid) | yes | Exact study directory scope. |

### StudyHeader

Compact source study identity; source codes and counts retain their own scope.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allSampleCount` | integer (uint64) | yes | Native allSampleCount, never a patient or profiled-gene denominator. |
| `cancerTypeId` | [CbioportalCancerTypeId](#cbioportalcancertypeid) | yes | cBioPortal namespace only; no ontology inference. |
| `citation` | string or null | no | Native citation, absent for unpublished records. |
| `name` | [Text](#text) | yes | Native study name. |
| `pmids` | array or null | no | Every validated source PMID in native order, including repetitions; absent remains absent. |
| `referenceGenome` | [Text](#text) | yes | Native study metadata label, never a coordinate assembly. |
| `studyId` | [StudyId](#studyid) | yes | Exact case-sensitive source identity. |

### StudyId

Type: string.

### StudyImportTime

Type: string.

### StudyProfile

Molecular profile reference with exact study membership.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `datatype` | [Text](#text) | yes | Extensible native data-type label. |
| `description` | string | yes | Source description. |
| `genericAssayType` | [Text](#text) or null | no | Native generic assay type, absent outside those profiles. |
| `molecularAlterationType` | [Text](#text) | yes | Extensible source alteration-type label. |
| `molecularProfileId` | [StudyProfileId](#studyprofileid) | yes | Exact native profile ID; never constructed from a study ID. |
| `name` | string | yes | Source profile name. |
| `patientLevel` | boolean | yes | Native patient-level flag, not a unit inferred for other records. |
| `showProfileInAnalysisTab` | boolean | yes | Native profile display setting. |
| `studyId` | [StudyId](#studyid) | yes | Native profile study scope. |

### StudyProfileId

Type: string.

### StudyProfiles

One independently acquired, bounded native metadata collection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. |
| `items` | array of [StudyProfile](#studyprofile) | yes | Complete ordered metadata records, never member sample IDs. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds and source-stated total for this collection. |
| `study_id` | [StudyId](#studyid) | yes | Exact study scope of this collection. |

### StudyResourceCount

One native resource scope; counts remain attached to its study and resource.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cancerStudyIdentifier` | [StudyId](#studyid) | yes | Exact resource study scope. |
| `description` | string | yes | Native resource description. |
| `displayName` | string | yes | Native resource display name. |
| `openByDefault` | boolean | yes | Native access/display setting, not a reuse license. |
| `patientCount` | integer (uint64) | yes | Native patient count within this resource only. |
| `priority` | [Text](#text) | yes | Native priority string, without numeric reinterpretation. |
| `resourceId` | [StudyResourceId](#studyresourceid) | yes | Native resource ID. |
| `resourceType` | [Text](#text) | yes | Extensible native resource type, not a derived count unit. |
| `sampleCount` | integer (uint64) | yes | Native count within this resource only. |

### StudyResourceId

Type: string.

### StudySampleList

Sample-list metadata without a representation for member sample IDs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `category` | [Text](#text) | yes | Extensible native list category. |
| `description` | string | yes | Native list description. |
| `name` | string | yes | Native list name. |
| `sampleCount` | integer or null | no | Stated source sampleCount when supplied, never computed from member IDs. |
| `sampleListId` | [StudySampleListId](#studysamplelistid) | yes | Exact native list ID. |
| `studyId` | [StudyId](#studyid) | yes | Native study scope. |

### StudySampleListId

Type: string.

### StudySampleLists

One independently acquired, bounded native metadata collection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. |
| `items` | array of [StudySampleList](#studysamplelist) | yes | Complete ordered metadata records, never member sample IDs. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds and source-stated total for this collection. |
| `study_id` | [StudyId](#studyid) | yes | Exact study scope of this collection. |

### StudySection

Supported bounded study metadata sections; gene counts are deferred.

Values: `profiles`, `sample-lists`, `files`, `all`.

### StudySnapshot

Metadata catalog revision and acquisition time; never a biological release date.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Catalog acquisition wall time in Unix seconds. |
| `revision` | [Text](#text) | yes | Immutable bundle revision. |

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.study_card/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
