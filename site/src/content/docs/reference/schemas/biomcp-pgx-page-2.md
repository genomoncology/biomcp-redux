---
title: biomcp.pgx_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.pgx_page/2: its envelope
  fields, its result type PgxPage and the commands that return it."
goal: Look up the fields of a biomcp.pgx_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pgx_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pgx_page/2`.

These commands return it: [`biomcp search pgx`](/reference/commands/search-pgx/).

Calls that returned it:

- `biomcp search pgx --gene CYP2C19 --source cpic --limit 50` ([`biomcp search pgx`](/reference/commands/search-pgx/))
- `biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C` ([`biomcp search pgx`](/reference/commands/search-pgx/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [PgxSearch](#query-pgxsearch) | yes | Parsed query. |
| `result` | [PgxPage](#result-pgxpage) or null | no | Answer when available. |
| `schema` | "biomcp.pgx_page/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: PgxPage

Independent source-ordered page of complete native records.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | [PgxHeader](#pgxheader) | yes | Source and selected gene. |
| `items` | array of [PgxPair](#pgxpair) | yes | Complete native rows without duplicate summaries. |
| `page` | [PgxPageInfo](#pgxpageinfo) | yes | Meaning and bounds of this collection. |

## Query: PgxSearch

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cpic_level` | [Text](#text) or null | no | Literal native level, including compound levels such as B/C. |
| `gene` | [GeneSymbol](#genesymbol) | yes | Explicit source gene symbol. |
| `limit` | integer (uint32) | no | Rows per native page. |
| `offset` | integer (uint32) | no | Native source offset. |
| `source` | "cpic" | yes | Required CPIC source; ClinPGx is not supported for search. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Curie

Type: string.

### GeneSymbol

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

### PgxHeader

Source and declared gene context; no chosen pair or patient result.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene` | [GeneSymbol](#genesymbol) | yes | Declared source symbol. |
| `source` | [PgxSource](#pgxsource) | yes | Explicit source. |

### PgxPageInfo

Independent collection scope; observed counts never imply native source totals.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Offset in this collection. |
| `returned` | integer (uint32) | yes | Number of whole returned rows. |
| `total` | integer (uint64) | yes | Exact source total or received bounded collection length. |
| `total_kind` | [PgxTotalKind](#pgxtotalkind) | yes | Provenance of total. |

### PgxPair

Source-native PgxPair fields; no clinical interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `clinpgxlevel` | string or null | no | Native `clinpgxlevel`; literal source value. |
| `cpiclevel` | string | yes | Native `cpiclevel`; literal source value. |
| `drugid` | [Curie](#curie) | yes | Native `drugid`; literal source value. |
| `drugname` | string | yes | Native `drugname`; literal source value. |
| `genesymbol` | [GeneSymbol](#genesymbol) | yes | Native `genesymbol`; literal source value. |
| `guidelinename` | string or null | no | Native `guidelinename`; literal source value. |
| `guidelineurl` | string or null | no | Native `guidelineurl`; literal source value. |
| `pairid` | integer (int64) | yes | Native `pairid`; literal source value. |
| `pgxtesting` | string or null | no | Native `pgxtesting`; literal source value. |
| `pmids` | array or null | no | Native `pmids`; literal source value. |
| `provisional` | boolean | yes | Native `provisional`; literal source value. |
| `usedforrecommendation` | string | yes | Native `usedforrecommendation`; literal source value. |

### PgxSource

Explicit independent source.

Values: `cpic`, `clinpgx`.

### PgxTotalKind

Meaning of the stated collection total.

Values: `source`, `observed`.

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

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pgx_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
