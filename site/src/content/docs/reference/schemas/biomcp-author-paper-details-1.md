---
title: biomcp.author_paper_details/1
description: "The BioMCP 1.0.0 reply schema biomcp.author_paper_details/1: its
  envelope fields, its result type AuthorPaperDetails and the commands that
  return it."
goal: Look up the fields of a biomcp.author_paper_details/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_paper_details/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.author_paper_details/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.author_paper_details/1`.

These commands return it: [`biomcp author papers`](/reference/commands/author-papers/).

Calls that returned it:

- `biomcp author papers semantic-scholar:123 details` ([`biomcp author papers`](/reference/commands/author-papers/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [AuthorPapersQuery](#query-authorpapersquery) | yes | Parsed query. |
| `result` | [AuthorPaperDetails](#result-authorpaperdetails) or null | no | Answer when available. |
| `schema` | "biomcp.author_paper_details/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: AuthorPaperDetails

Detailed metadata for the same native indexed-paper window.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `author` | [AuthorId](#authorid) | yes | Requested provider record. |
| `items` | array of [AuthorPaperDetail](#authorpaperdetail) | yes | Original source order; no native corpus total is claimed. |

## Query: AuthorPapersQuery

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `full` | boolean | no | Legacy alias for details; normalized on construction. |
| `id` | [AuthorId](#authorid) | yes | Exact qualified provider record. |
| `limit` | integer (uint32) | no | Bounded row count. |
| `offset` | integer (uint32) | no | Native or retained collection offset. |
| `pool` | [Text](#text) | no | Retained collection identity. |
| `section` | [AuthorPaperSection](#authorpapersection) | no | Detailed disclosure when requested. |

## Types

Every other type this schema uses.

### AuthorByline

One source byline row; repeated display names remain separate.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [SemanticScholarAuthorId](#semanticscholarauthorid) or null | no | Source author ID when supplied. |
| `name` | [Text](#text) | yes | Native display name. |

### AuthorId

Type: string.

### AuthorPaperAccess

Native access metadata; access does not establish reuse rights.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `disclaimer` | [Text](#text) or null | no | Native source disclaimer. |
| `license` | [Text](#text) or null | no | Native license label. |
| `status` | [Text](#text) or null | no | Open native access status. |
| `url` | [Text](#text) or null | no | Native PDF URL. |

### AuthorPaperDetail

Detailed indexed paper metadata for the same native paper page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `abstract_text` | [Text](#text) or null | no | Source abstract when available. |
| `authors` | array or null | no | Native ordered byline; completeness is unknown. |
| `citation_count` | integer or null | no | Native citations to this paper. |
| `fields_of_study` | array or null | no | Native study labels. |
| `header` | [AuthorPaperHeader](#authorpaperheader) | yes | Same compact paper identity. |
| `influential_citation_count` | integer or null | no | Native influential citations to this paper. |
| `is_open_access` | boolean or null | no | Native open-access flag. |
| `open_access_pdf` | [AuthorPaperAccess](#authorpaperaccess) or null | no | Native access metadata. |
| `publication_date` | [PartialDate](#partialdate) or null | no | Native publication date. |
| `publication_types` | array or null | no | Native publication type labels. |
| `reference_count` | integer or null | no | Native references on this paper. |

### AuthorPaperExternalId

An open native identifier namespace and value; no identity inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) | yes | Native namespace and identifier. |

### AuthorPaperHeader

One indexed paper in source order; identifiers come only from that source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `doi` | [Doi](#doi) or null | no | Exact native DOI. |
| `external_ids` | array of [AuthorPaperExternalId](#authorpaperexternalid) | yes | Every other native paper identifier. |
| `paper_id` | [SemanticScholarId](#semanticscholarid) | yes | Native source paper ID. |
| `pmcid` | [Pmcid](#pmcid) or null | no | Exact native PMCID. |
| `pmid` | [Pmid](#pmid) or null | no | Exact native PMID. |
| `title` | [Text](#text) | yes | Whole source title. |
| `venue` | [Text](#text) or null | no | Source venue when supplied and nonempty. |
| `year` | integer or null | no | Source publication year. |

### AuthorPaperSection

Indexed-paper disclosure level.

Values: `details`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Curie

Type: string.

### Doi

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

### Pmcid

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

### SemanticScholarAuthorId

Type: string.

### SemanticScholarId

Type: string.

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

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.author_paper_details/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
