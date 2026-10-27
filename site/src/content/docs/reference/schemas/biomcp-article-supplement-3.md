---
title: biomcp.article_supplement/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_supplement/3: its
  envelope fields, its result type ArticleSupplementResult and the commands that
  return it."
goal: Look up the fields of a biomcp.article_supplement/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_supplement/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_supplement/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_supplement/3`.

These commands return it: [`biomcp article supplement`](/reference/commands/article-supplement/).

Calls that returned it:

- `biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S` ([`biomcp article supplement`](/reference/commands/article-supplement/))
- `biomcp article supplement 28944238 MGG3-5-553-s002.xlsx` ([`biomcp article supplement`](/reference/commands/article-supplement/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ArticleSupplement](#query-articlesupplement) | yes | Parsed query. |
| `result` | [ArticleSupplementResult](#result-articlesupplementresult) or null | no | Answer when available. |
| `schema` | "biomcp.article_supplement/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ArticleSupplementResult

One supplement reader answer.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `content` | [Section](#section) | yes | Independently licensed supplement content. |
| `pmcid` | [Pmcid](#pmcid) or null | no | Resolved PMC identity. |

## Query: ArticleSupplement

Read a checked supplement file or page its original rows.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `file` | [Text](#text) | yes | Exact file name from the supplements section. |
| `find` | [Text](#text) | no | Literal case-insensitive needle. |
| `input` | [ArticleInput](#articleinput) | yes | PubMed or PMC identity. |
| `offset` | integer (uint32) | no | Matching-row or text-fragment offset. |
| `replace` | boolean | no | Replace an existing destination only when explicitly requested with save. |
| `rows` | [SheetRows](#sheetrows) | no | Inclusive original row range. |
| `save` | [Text](#text) | no | Save the whole original document to an explicit file or existing folder. |
| `sheet` | [Text](#text) | no | Sheet name for row selection. |
| `variant` | [Text](#text) | no | Explicit protein substitution. |

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

### CellFragment

A bounded portion of one original first-row or selected-row cell.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `column` | integer (uint32) | yes | Original one-based column. |
| `first_row` | boolean | yes | Whether the cell belongs to the first source row. |
| `offset` | integer (uint32) | yes | Unicode character offset within this cell. |
| `text` | string | yes | Original cell text at this offset. |

### FirstSheetRow

The first original row without interpreting its cells as column headings.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cells` | array of string | yes | Original cells, empty when represented by cell fragments. |
| `row` | integer (uint32) | yes | Original one-based row number. |

### License

License reported by the bucket metadata record.

Values: `cc_by`, `cc_by_nc`, `cc_by_sa`, `cc_by_nc_sa`, `cc_by_nd`, `cc_by_nc_nd`, `cc0`, `public_domain`, `tdm`, `other`.

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

### SheetInfo

Source sheet with its maximum original row number.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Sheet name. |
| `rows` | integer (uint32) | yes | Last original row number, preserving gaps. |

### SheetRow

One original sparse row with its numbered first source row.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cells` | array of string | yes | This row's cells, including empty cells before populated columns. |
| `first_row` | [FirstSheetRow](#firstsheetrow) | yes | First original row, including empty cells; never inferred column headings. |
| `fragment` | [CellFragment](#cellfragment) or null | no | Sparse source cell fragment when the complete row exceeds the output bound. |
| `row` | integer (uint32) | yes | Original one-based row number. |
| `sheet` | [Text](#text) | yes | Original sheet name. |

### SheetRows

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

### SupplementContent

Downloaded bounded file contents or an unsupported format's original URL.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `forms` | array or null | no | Explicit variant spellings searched. |
| `license` | [License](#license) | yes | Article license. |
| `reason` | [Reason](#reason) or null | no | Typed unsupported reason. |
| `rows` | array or null | no | Selected original rows. |
| `saved` | [ArticleSaved](#articlesaved) or null | no | Explicit original-byte save receipt. |
| `search_message` | string or null | no | Explicit absence of matching text from the returned search page. |
| `sha256` | [Text](#text) or null | no | Hash of the original downloaded bytes; absent when no download occurred. |
| `sheets` | array or null | no | Original sheet dimensions. |
| `url` | [Text](#text) or null | no | Unsupported file URL, returned without downloading it. |

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_supplement/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
