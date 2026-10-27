---
title: biomcp.protein_function/2
description: "The BioMCP 1.0.0 reply schema biomcp.protein_function/2: its
  envelope fields, its result type ProteinFunctionResult and the commands that
  return it."
goal: Look up the fields of a biomcp.protein_function/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_function/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_function/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_function/2`.

`get protein ID function` returns the function text inside `biomcp.protein_card/3`, so no command returns this schema as a whole reply.

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ProteinGet](#query-proteinget) | yes | Parsed query. |
| `result` | [ProteinFunctionResult](#result-proteinfunctionresult) or null | no | Answer when available. |
| `schema` | "biomcp.protein_function/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ProteinFunctionResult

Whole source function paragraphs and their own evidence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinFunction](#proteinfunction) | no | Native paragraphs in source order, without inferred interpretation. |

## Query: ProteinGet

Read source protein cards; a symbol returns candidates without choosing a protein.
Sections disclose native facts with independent row bounds; function keeps all paragraphs.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of [ProteinInput](#proteininput) | yes | Source accessions or gene symbols, preserving caller order. |
| `section` | array of [ProteinSection](#proteinsection) | no | Select native sections or all bounded first pages; function keeps one whole text record. |
| `section_limit` | integer (uint32) | no | First-page row limit; function keeps one whole native text record and does not page its paragraphs. |

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

### ProteinFunction

One whole source function paragraph and its own evidence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `evidence` | array of [Text](#text) | no | All evidence references supplied for this text. |
| `molecule` | [Text](#text) or null | no | Native molecule qualifier; never attributed to another isoform. |
| `text` | [Text](#text) | yes | Whole native source text, without clinical interpretation. |

### ProteinInput

Exact source accession or a checked display gene symbol needing later resolution.
The schema describes the string surface; deserialization checks either identity grammar.

Type: string.

### ProteinSection

Named protein sections; all expands to bounded first pages.

Values: `features`, `domains`, `isoforms`, `complexes`, `structures`, `function`, `sequence`, `all`.

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

This page is generated from `biomcp schema biomcp.protein_function/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
