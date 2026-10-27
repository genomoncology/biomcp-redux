---
title: biomcp.cell_line_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_page/2: its
  envelope fields, its result type CellLinePage and the commands that return
  it."
goal: Look up the fields of a biomcp.cell_line_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_page/2`.

These commands return it: [`biomcp search cell-line`](/reference/commands/search-cell-line/).

Calls that returned it:

- `biomcp search cell-line A-375 --limit 20` ([`biomcp search cell-line`](/reference/commands/search-cell-line/))
- `biomcp search cell-line MOLM13` ([`biomcp search cell-line`](/reference/commands/search-cell-line/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [CellLineSearch](#query-celllinesearch) | yes | Parsed query. |
| `result` | [CellLinePage](#result-celllinepage) or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_page/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: CellLinePage

Source-ordered cell line search headers; totals remain unknown.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fetched` | integer or null | no | Original retrieval seconds; not a source release date. |
| `items` | array of [CellLineHeader](#celllineheader) | yes | Native result order, without reranking or selected identity. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release, never a result total. |

## Query: CellLineSearch

One literal cell line search, with native order and no total claim.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native page size. |
| `offset` | integer (uint32) | no | Native start offset. |
| `query` | [Text](#text) | yes | Literal source query; positional spelling and --query use the same field. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [CellLineXref](#celllinexref) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### CellLineHeader

Compact source identity with independently capped species and diseases.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Exact primary accession. |
| `category` | [Text](#text) | yes | Native category. |
| `diseases` | [Capped](#capped) | yes | Up to three complete disease references and their independent total. |
| `name` | [Text](#text) | yes | Recommended display name. |
| `species` | [Capped](#capped) | yes | Up to three complete species references and their native total. |

### CellLineRelease

Publisher release identity, fetched independently from record data.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `updated` | [PartialDate](#partialdate) | yes | Native release update date. |
| `version` | [Text](#text) | yes | Native release version. |

### CellLineXref

One complete native cross-reference; database and accession form the source identity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [Text](#text) | yes | Native accession, including any internal prefix. |
| `category` | [Text](#text) or null | no | Native source category when supplied. |
| `database` | [Text](#text) | yes | Native database name; no fixed vocabulary. |
| `discontinued` | [Text](#text) or null | no | Native discontinuation qualifier, including values such as probable. |
| `iri` | [Text](#text) or null | no | Native IRI when supplied. |
| `label` | [Text](#text) or null | no | Native display label when supplied. |
| `url` | [Text](#text) or null | no | Native source URL when supplied. |

### CellosaurusAccession

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

This page is generated from `biomcp schema biomcp.cell_line_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
