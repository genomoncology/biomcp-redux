---
title: biomcp.pgx_recommendations/1
description: "The BioMCP 1.0.0 reply schema biomcp.pgx_recommendations/1: its
  envelope fields, its result type PgxRecommendations and the commands that
  return it."
goal: Look up the fields of a biomcp.pgx_recommendations/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_recommendations/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pgx_recommendations/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pgx_recommendations/1`.

This call returns it.

- `biomcp get pgx --gene DPYD --source cpic recommendations` ([`biomcp get pgx`](/reference/commands/get-pgx/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [PgxGet](#query-pgxget) | yes | Parsed query. |
| `result` | [PgxRecommendations](#result-pgxrecommendations) or null | no | Answer when available. |
| `schema` | "biomcp.pgx_recommendations/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: PgxRecommendations

Independent recommendation contracts preserve native table and view shapes.

One of:

- object: Gene-keyed recommendation_view with native names and no inferred drug ID.
- object: Exact drug selector uses the distinct native recommendation table.

## Query: PgxGet

PGx card or section. No phenotype, phase or dose is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `diplotype` | [PgxDiplotypeExpression](#pgxdiplotypeexpression) or null | no | Unchanged declared expression required for the diplotype section only. |
| `drug_id` | [Curie](#curie) or null | no | Exact published CPIC drug key; recommendations only. |
| `gene` | [GeneSymbol](#genesymbol) | yes | Explicit source gene symbol. |
| `kind` | [PgxAnnotationKind](#pgxannotationkind) or null | no | One ClinPGx collection; clinical is the default only on annotations. |
| `section` | [PgxSection](#pgxsection) or null | no | One named section; omitted means the CPIC gene card. |
| `section_limit` | integer (uint32) | no | Rows per page: three clinical, five elsewhere. |
| `section_offset` | integer (uint32) | no | Offset within the selected section. |
| `snapshot` | [Text](#text) or null | no | Retained collection for local continuation. |
| `source` | [PgxSource](#pgxsource) | yes | Required source, checked against the section. |

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

### PgxAnnotationKind

Independent ClinPGx collection kinds, never merged.

Values: `clinical`, `guideline`, `label`.

### PgxDiplotypeExpression

Type: string.

### PgxDrugRecommendation

Source-native PgxDrugRecommendation fields; no clinical interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `activityscore` | object | yes | Native `activityscore`; literal source value. |
| `allelestatus` | object | yes | Native `allelestatus`; literal source value. |
| `alternatedrugavailable` | boolean | yes | Native `alternatedrugavailable`; literal source value. |
| `classification` | string | yes | Native `classification`; literal source value. |
| `comments` | string | yes | Native `comments`; literal source value. |
| `dosinginformation` | boolean | yes | Native `dosinginformation`; literal source value. |
| `drugid` | [Curie](#curie) | yes | Native `drugid`; literal source value. |
| `drugrecommendation` | string | yes | Native `drugrecommendation`; literal source value. |
| `guidelineid` | integer (int64) | yes | Native `guidelineid`; literal source value. |
| `id` | integer (int64) | yes | Native `id`; literal source value. |
| `implications` | object | yes | Native `implications`; literal source value. |
| `lookupkey` | object | yes | Native `lookupkey`; literal source value. |
| `otherprescribingguidance` | boolean | yes | Native `otherprescribingguidance`; literal source value. |
| `phenotypes` | object | yes | Native `phenotypes`; literal source value. |
| `population` | string | yes | Native `population`; literal source value. |
| `version` | integer (int64) | yes | Native `version`; literal source value. |

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

### PgxRecommendation

Source-native PgxRecommendation fields; no clinical interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `activityscore` | object | yes | Native `activityscore`; literal source value. |
| `classification` | string | yes | Native `classification`; literal source value. |
| `comments` | string or null | no | Native `comments`; literal source value. |
| `drugname` | string | yes | Native `drugname`; literal source value. |
| `drugrecommendation` | string | yes | Native `drugrecommendation`; literal source value. |
| `guidelinename` | string | yes | Native `guidelinename`; literal source value. |
| `guidelineurl` | string | yes | Native `guidelineurl`; literal source value. |
| `implications` | object | yes | Native `implications`; literal source value. |
| `lookupkey` | object | yes | Native `lookupkey`; literal source value. |
| `phenotypes` | object | yes | Native `phenotypes`; literal source value. |
| `population` | string | yes | Native `population`; literal source value. |
| `recommendationid` | integer (int64) | yes | Native `recommendationid`; literal source value. |

### PgxSection

One independent named section.

Values: `recommendations`, `guidelines`, `diplotype`, `annotations`.

### PgxSource

Explicit independent source.

Values: `cpic`, `clinpgx`.

### PgxTotalKind

Meaning of the stated collection total.

Values: `source`, `observed`.

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

This page is generated from `biomcp schema biomcp.pgx_recommendations/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
