---
title: biomcp.study_page/1
description: "The BioMCP 1.0.0 reply schema biomcp.study_page/1: its envelope
  fields, its result type StudyPage and the commands that return it."
goal: Look up the fields of a biomcp.study_page/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.study_page/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.study_page/1`.

These commands return it: [`biomcp search study`](/reference/commands/search-study/).

Calls that returned it:

- `biomcp search study melanoma --limit 2` ([`biomcp search study`](/reference/commands/search-study/))
- `biomcp search study Atlas` ([`biomcp search study`](/reference/commands/search-study/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [StudySearch](#query-studysearch) | yes | Parsed query. |
| `result` | [StudyPage](#result-studypage) or null | no | Answer when available. |
| `schema` | "biomcp.study_page/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: StudyPage

Source-ordered checked study headers from one retained catalog.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [StudyHeader](#studyheader) | yes | Whole source headers, without expanded descriptions. |
| `page` | [PageInfo](#pageinfo) | yes | Snapshot-scoped matching counts and offset. |
| `snapshot` | [StudySnapshot](#studysnapshot) | yes | Catalog acquisition provenance. |

## Query: StudySearch

Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cancer_type` | [CbioportalCancerTypeId](#cbioportalcancertypeid) | no | Exact cBioPortal cancer-type code; no OncoTree conversion. |
| `limit` | integer (uint32) | no | Maximum headers per page. |
| `offset` | integer (uint32) | no | Zero-based offset in the checked snapshot. |
| `pool` | [Text](#text) or null | no | Opaque continuation retaining the catalog revision and selectors. |
| `text` | [Text](#text) | no | Literal study ID, name or description text, without synonyms or typo repair. |

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

This page is generated from `biomcp schema biomcp.study_page/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
