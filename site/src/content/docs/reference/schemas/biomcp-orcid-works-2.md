---
title: biomcp.orcid_works/2
description: "The BioMCP 1.0.0 reply schema biomcp.orcid_works/2: its envelope
  fields, its result type OrcidWorks and the commands that return it."
goal: Look up the fields of a biomcp.orcid_works/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.orcid_works/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.orcid_works/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.orcid_works/2`.

This call returns it.

- `biomcp author papers orcid:0000-0002-1825-0097` ([`biomcp author papers`](/reference/commands/author-papers/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [AuthorPapersQuery](#query-authorpapersquery) | yes | Parsed query. |
| `result` | [OrcidWorks](#result-orcidworks) or null | no | Answer when available. |
| `schema` | "biomcp.orcid_works/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: OrcidWorks

Locally paged groups from one complete public ORCID works reply.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `author` | [OrcidId](#orcidid) | yes | Requested exact ORCID record. |
| `collection_scope` | [OrcidWorksScope](#orcidworksscope) | yes | The whole source reply supplies every local page. |
| `groups` | array of [OrcidWorkGroup](#orcidworkgroup) | yes | Whole groups in native order, with every public assertion. |
| `observed_groups` | integer (uint64) | yes | Observed groups in the complete received reply, never a publication total. |

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

### AuthorId

Type: string.

### AuthorPaperSection

Indexed-paper disclosure level.

Values: `details`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### OrcidAssertionSource

Source and assertion origin remain distinct provider claims.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertion_origin_client_id` | [OrcidClientId](#orcidclientid) or null | no | Native origin client ID. |
| `assertion_origin_name` | [Text](#text) or null | no | Native origin display name. |
| `assertion_origin_orcid` | [OrcidId](#orcidid) or null | no | Native assertion origin ORCID iD. |
| `source_client_id` | [OrcidClientId](#orcidclientid) or null | no | Native ORCID client ID; no person identity. |
| `source_name` | [Text](#text) or null | no | Native source display name. |
| `source_orcid` | [OrcidId](#orcidid) or null | no | Native source ORCID iD. |

### OrcidClientId

Type: string.

### OrcidExternalId

Native ORCID external identifier before any article pivot.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | [Text](#text) | yes | Open native external-id-type. |
| `relationship` | [Text](#text) | yes | Open native relationship, including self and part-of. |
| `url` | [Text](#text) or null | no | Native identifier URL. |
| `value` | [Text](#text) | yes | Unchanged native identifier value. |

### OrcidId

Type: string.

### OrcidPublicVisibility

Only public source visibility is admitted; legacy uppercase is an alias.

Values: `public`, `PUBLIC`.

### OrcidWorkAssertion

One public summary, retained in native group order without representative selection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `display_index` | [Text](#text) or null | no | Native display index retained without sorting. |
| `external_ids` | array of [OrcidExternalId](#orcidexternalid) | yes | Every native assertion identifier in order. |
| `journal_title` | [Text](#text) or null | no | Native journal title. |
| `publication_date` | [PartialDate](#partialdate) or null | no | Native partial publication date. |
| `put_code` | integer (uint64) | yes | Source assertion identifier scoped to this ORCID record. |
| `source` | [OrcidAssertionSource](#orcidassertionsource) | yes | Native source and origin provenance. |
| `subtitle` | [Text](#text) or null | no | Native subtitle. |
| `title` | [Text](#text) or null | no | Native title; some summaries have none. |
| `translated_title` | [Text](#text) or null | no | Native translated title. |
| `url` | [Text](#text) or null | no | Native work URL. |
| `visibility` | [OrcidPublicVisibility](#orcidpublicvisibility) | yes | Only public assertions belong to this product. |
| `work_type` | [Text](#text) | yes | Open native work type. |

### OrcidWorkGroup

One native work group is one group, regardless of assertion multiplicity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertions` | array of [OrcidWorkAssertion](#orcidworkassertion) | yes | Every public summary in native source order. |
| `external_ids` | array of [OrcidExternalId](#orcidexternalid) | yes | Native group identifiers in source order. |

### OrcidWorksScope

Scope of one received ORCID collection.

Values: `whole_public_works_reply`.

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

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.orcid_works/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
