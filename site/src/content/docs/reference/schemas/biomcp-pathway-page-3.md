---
title: biomcp.pathway_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_page/3: its envelope
  fields, its result type PathwayPage and the commands that return it."
goal: Look up the fields of a biomcp.pathway_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_page/3`.

These commands return it: [`biomcp search pathway`](/reference/commands/search-pathway/).

Calls that returned it:

- `biomcp search pathway MAPK --source wikipathways --limit 3` ([`biomcp search pathway`](/reference/commands/search-pathway/))
- `biomcp search pathway MAPK --limit 3` ([`biomcp search pathway`](/reference/commands/search-pathway/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [PathwaySearch](#query-pathwaysearch) | yes | Parsed query. |
| `result` | [PathwayPage](#result-pathwaypage) or null | no | Answer when available. |
| `schema` | "biomcp.pathway_page/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: PathwayPage

Source-ordered header page; class admission never changes the native total.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [PathwayHeader](#pathwayheader) | yes | Whole admitted pathway headers in native order. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | yes | Native position and total provenance. |

## Query: PathwaySearch

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fresh` | boolean | no | Request a new retained observation. |
| `gene` | [GeneInput](#geneinput) | no | Human gene input for exact resolution and UniProt mapping; Reactome only. |
| `limit` | integer (uint32) | no | Five rows by default, one to fifty. Reactome rows equals this limit without refill. |
| `offset` | integer (uint32) | no | Native Reactome Start row or retained WikiPathways catalog offset. |
| `pool` | [Text](#text) | no | Retained snapshot identity for deterministic continuation. |
| `query` | [Text](#text) | no | Literal source query; exclusive with gene and top-level discovery. |
| `source` | array of [PathwaySource](#pathwaysource) | no | Sources in caller order; repeated sources take turns without title joins. |
| `top_level` | boolean | no | Native human Reactome top-level discovery, exclusive with text and gene. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### GeneInput

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

### PathwayHeader

Small selectable native pathway identity, without its potentially long description.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [PathwayId](#pathwayid) | yes | Checked source-owned identity. |
| `name` | [Text](#text) | yes | Whole native name. |
| `revision` | [Text](#text) or null | no | WikiPathways revision, separate from identity. |
| `species` | [PathwaySpecies](#pathwayspecies) | yes | Explicit native human species. |
| `version` | [Text](#text) or null | no | Whole native Reactome stIdVersion, separate from stable identity. |

### PathwayId

Source-owned pathway identity; equal names never merge sources.

One of:

- object: Human Reactome stable identity without a version suffix.
- object: WikiPathways identity without a revision suffix.

### PathwayPageInfo

Native page position and explicit count provenance; a short admitted page can continue.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `next_offset` | integer or null | no | Next native position, independent of returned rows; absent only at the end. |
| `offset` | integer (uint32) | yes | Native source or retained relation offset. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Snapshot identity and lifetime when retained. |
| `returned` | integer (uint32) | yes | Whole admitted rows on this page. |
| `total` | integer (uint64) | yes | Source-reported or observed relation total, as qualified by total_kind. |
| `total_kind` | [PathwayTotalKind](#pathwaytotalkind) | yes | Whether the total comes from the source or this retained collection. |

### PathwaySource

One explicit pathway source, without fallback or cross-source title joins.

Values: `reactome`, `wikipathways`.

### PathwaySpecies

Source-stated human species; no species inference from a title.

Values: `Homo sapiens`.

### PathwayTotalKind

Meaning of the collection total, independent of admitted rows on a page.

Values: `source`, `observed`, `distinct_references`, `data_nodes`.

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

### ReactomeId

Type: string.

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

### WikiPathwaysId

Type: string.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pathway_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
