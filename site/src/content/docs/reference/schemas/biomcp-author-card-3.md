---
title: biomcp.author_card/3
description: "The BioMCP 1.0.0 reply schema biomcp.author_card/3: its envelope
  fields, its result type AuthorCards and the commands that return it."
goal: Look up the fields of a biomcp.author_card/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.author_card/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.author_card/3`.

These commands return it: [`biomcp get author`](/reference/commands/get-author/).

Calls that returned it:

- `biomcp get author orcid:0000-0002-1825-0097` ([`biomcp get author`](/reference/commands/get-author/))
- `biomcp get author semantic-scholar:123 semantic-scholar:123` ([`biomcp get author`](/reference/commands/get-author/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [AuthorGet](#query-authorget) | yes | Parsed query. |
| `result` | [AuthorCards](#result-authorcards) or null | no | Answer when available. |
| `schema` | "biomcp.author_card/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: AuthorCards

Exact source cards in input order; outer source outcomes retain failures.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [AuthorCardItem](#authorcarditem) | yes | Exact returned source cards, with no cross-source links. |

## Query: AuthorGet

Exact qualified records in caller order, with no name reconciliation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ids` | array of [AuthorId](#authorid) | yes | One or more exact qualified IDs. |

## Types

Every other type this schema uses.

### AuthorCard

Exact source author metadata without identity reconciliation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | [AuthorHeader](#authorheader) | yes | Exact source candidate. |
| `url` | [Text](#text) or null | no | Provider record link. |

### AuthorCardItem

One exact input and its independent outcome.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `card` | [AuthorCard](#authorcard) or null | no | Exact checked source card when available. |
| `input` | [AuthorId](#authorid) | yes | Requested qualified provider identity. |
| `status` | [Status](#status) | yes | This exact input's outcome. |

### AuthorCountScope

Scope of source-reported bibliographic counts.

Values: `provider_record`.

### AuthorCounts

Counts describe only this provider record, never a global publication corpus.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `citation_count` | integer or null | no | Native citations to this record’s indexed papers. |
| `h_index` | integer or null | no | Native h-index for this record’s indexed papers. |
| `paper_count` | integer or null | no | Native indexed paper count. |
| `scope` | [AuthorCountScope](#authorcountscope) | yes | Counts apply only to this provider record. |

### AuthorHeader

One source candidate; absent affiliations differ from an empty source collection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `affiliations` | array or null | no | Native affiliation labels; no employment claim. |
| `counts` | [AuthorCounts](#authorcounts) or null | no | Source record counts when supplied. |
| `id` | [AuthorId](#authorid) | yes | Exact qualified provider record. |
| `name` | [Text](#text) | yes | Source display name, never a join key. |

### AuthorId

Type: string.

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

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.author_card/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
