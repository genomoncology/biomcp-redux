---
title: biomcp.article_text/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_text/3: its envelope
  fields, its result type ArticleTextResult and the commands that return it."
goal: Look up the fields of a biomcp.article_text/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_text/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_text/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_text/3`.

These commands return it: [`biomcp article text`](/reference/commands/article-text/).

Calls that returned it:

- `biomcp article text PMC3549296 --find V600E` ([`biomcp article text`](/reference/commands/article-text/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ArticleText](#query-articletext) | yes | Parsed query. |
| `result` | [ArticleTextResult](#result-articletextresult) or null | no | Answer when available. |
| `schema` | "biomcp.article_text/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ArticleTextResult

Body text response; the envelope supplies advancing reader steps.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `content` | [Section](#section) | yes | Explicit section outcome including not applicable and not open. |
| `pmcid` | [Pmcid](#pmcid) or null | no | Requested PMC record when known. |

## Query: ArticleText

Read one bounded body part, find matching rows, or save the complete Markdown document.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `find` | [Text](#text) | no | Case-insensitive literal text to find. |
| `input` | [ArticleInput](#articleinput) | yes | PubMed or PMC identity. DOI selectors are not accepted by the reader. |
| `offset` | integer (uint32) | no | Number of matching rows already returned. |
| `part` | integer (uint32) | no | One-based outline part number. |
| `replace` | boolean | no | Replace an existing destination only when explicitly requested with save. |
| `save` | [Text](#text) | no | Save the whole original document to an explicit file or existing folder. |
| `variant` | [Text](#text) | no | Explicit checked protein substitution. |

## Types

Every other type this schema uses.

### ArticleInput

Type: string.

### ArticleSaved

Explicit saved article file, owned by the caller.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `bytes` | integer (uint64) | yes | Exact byte count. |
| `parts` | integer or null | no | Number of ordered text parts; absent for original supplement bytes. |
| `path` | [Text](#text) | yes | Absolute caller destination. |
| `sha256` | [Text](#text) | yes | SHA-256 of the saved bytes. |
| `url` | [Text](#text) | yes | Source URL for the original material. |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### License

License reported by the bucket metadata record.

Values: `cc_by`, `cc_by_nc`, `cc_by_sa`, `cc_by_nc_sa`, `cc_by_nd`, `cc_by_nc_nd`, `cc0`, `public_domain`, `tdm`, `other`.

### Markdown

Nonempty text with surrounding Unicode whitespace removed.

Type: string (markdown).

### PageInfo

Search counts after filtering and deduplication.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Skipped results. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Optional pooled snapshot. |
| `returned` | integer (uint32) | yes | Results returned on this page. |
| `total` | integer (uint64) | yes | Total matching identities held by this snapshot. |
| `truncated` | boolean or null | no | Matching source count exceeded the retrieved window. |

### Pmcid

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

### TextContent

One bounded part or matching rows with the bucket license.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `forms` | array or null | no | Literal protein change spellings searched by an explicit variant selector. |
| `heading_path` | array or null | no | Section ancestry for a selected part. |
| `hits` | array or null | no | Matching source rows for a find selection. |
| `license` | [License](#license) | yes | Bucket license. |
| `manuscript` | boolean | no | Author manuscript flag. |
| `markdown` | [Markdown](#markdown) or null | no | Bounded body text for a direct part selection. |
| `part` | integer or null | no | Part number for a direct part selection. |
| `processing_version` | integer (uint32) | no | Selected processing version. |
| `saved` | [ArticleSaved](#articlesaved) or null | no | Explicit save receipt, absent for reader views. |
| `scope` | [Text](#text) | yes | Scope of returned source text. |
| `url` | [Text](#text) or null | no | Source URL for an explicit save, including refused destinations. |

### TextHit

One matching paragraph or table row, split when necessary to fit the text budget.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `heading_path` | array of [Text](#text) | yes | Section ancestry in source order. |
| `markdown` | [Markdown](#markdown) | yes | Whole bounded source row. |
| `part` | integer (uint32) | yes | Outline part containing this text. |

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_text/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
