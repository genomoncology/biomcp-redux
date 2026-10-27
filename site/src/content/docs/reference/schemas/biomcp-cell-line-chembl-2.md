---
title: biomcp.cell_line_chembl/2
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_chembl/2: its
  envelope fields, its result type CellLineChembl and the commands that return
  it."
goal: Look up the fields of a biomcp.cell_line_chembl/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_chembl/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_chembl/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_chembl/2`.

This call returns it.

- `biomcp get cell-line CVCL_0132 chembl` ([`biomcp get cell-line`](/reference/commands/get-cell-line/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [CellLineGet](#query-celllineget) | yes | Parsed query. |
| `result` | [CellLineChembl](#result-celllinechembl) or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_chembl/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: CellLineChembl

Exact accession-linked metadata with its own native collection total.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Requested exact accession. |
| `fetched` | integer or null | no | Original retrieval seconds; never a source release date. |
| `items` | [Capped](#capped) | yes | Every selected whole metadata row and the native total. |
| `offset` | integer (uint32) | yes | Native metadata page offset. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independent ChEMBL release, distinct from retrieval time. |

## Query: CellLineGet

One exact identity and an optional bounded named section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [CellLineInput](#celllineinput) | yes | One exact primary accession or qualified source ID. |
| `section` | [CellLineSection](#celllinesection) | no | Omit for the default card; all returns bounded first pages without an assay count. |
| `section_limit` | integer (uint32) | no | Explicit row limit for each requested collection; native variation defaults to three. |
| `section_offset` | integer (uint32) | no | Offset within each independently requested collection. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [CellLineChemblRecord](#celllinechemblrecord) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### CellLineChemblRecord

One exact accession-linked ChEMBL cell identity; no assay request is implied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cell_id` | integer (uint64) | yes | Independent native integer cell record identity. |
| `cellosaurus_id` | [CellosaurusAccession](#cellosaurusaccession) | yes | Native exact accession; adapter verifies the join. |
| `clo` | [Text](#text) or null | no | Native CLO identifier; omission is source absence. |
| `description` | [Text](#text) or null | no | Native description. |
| `efo` | [Text](#text) or null | no | Native EFO identifier; omission is source absence. |
| `id` | [ChemblCellId](#chemblcellid) | yes | Native cell ChEMBL ID, never a compound or target. |
| `name` | [Text](#text) or null | no | Native display name without name-based joining. |
| `organism` | [Text](#text) or null | no | Native organism label. |
| `taxon` | integer or null | no | Native organism taxon. |
| `tissue` | [Text](#text) or null | no | Native tissue label. |

### CellLineInput

An exact accession or a database-qualified source identity; never a name lookup.

Type: string.

### CellLineRelease

Publisher release identity, fetched independently from record data.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `updated` | [PartialDate](#partialdate) | yes | Native release update date. |
| `version` | [Text](#text) | yes | Native release version. |

### CellLineSection

Named disclosure sections; all requests each bounded first page without an assay count.

Values: `names`, `identity`, `xrefs`, `variants`, `chembl`, `all`.

### CellosaurusAccession

Type: string.

### ChemblCellId

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

This page is generated from `biomcp schema biomcp.cell_line_chembl/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
