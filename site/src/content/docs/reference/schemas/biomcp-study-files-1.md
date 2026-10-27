---
title: biomcp.study_files/1
description: "The BioMCP 1.0.0 reply schema biomcp.study_files/1: its envelope
  fields, its result type StudyFiles and the commands that return it."
goal: Look up the fields of a biomcp.study_files/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study_files/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.study_files/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.study_files/1`.

This call returns it.

- `biomcp get study synthetic_0 files` ([`biomcp get study`](/reference/commands/get-study/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [StudyGet](#query-studyget) | yes | Parsed query. |
| `result` | [StudyFiles](#result-studyfiles) or null | no | Answer when available. |
| `schema` | "biomcp.study_files/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: StudyFiles

One native repository directory page; blob sizes do not describe data content.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Directory acquisition wall time in Unix seconds. |
| `items` | array of [StudyFile](#studyfile) | yes | Source-ordered repository file references. |
| `license_notice` | [Text](#text) or null | no | Native LICENSE publisher link, independent of the current file page. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds within the complete received directory. |
| `study_id` | [StudyId](#studyid) | yes | Exact study directory scope. |

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

### StudyId

Type: string.

### StudySection

Supported bounded study metadata sections; gene counts are deferred.

Values: `profiles`, `sample-lists`, `files`, `all`.

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.study_files/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
