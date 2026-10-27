---
title: biomcp.cell_line_card/3
description: "The BioMCP 1.0.0 reply schema biomcp.cell_line_card/3: its
  envelope fields, its result type CellLineCard and the commands that return
  it."
goal: Look up the fields of a biomcp.cell_line_card/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.cell_line_card/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.cell_line_card/3`.

These commands return it: [`biomcp get cell-line`](/reference/commands/get-cell-line/).

Calls that returned it:

- `biomcp get cell-line CVCL_0132` ([`biomcp get cell-line`](/reference/commands/get-cell-line/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [CellLineGet](#query-celllineget) | yes | Parsed query. |
| `result` | [CellLineCard](#result-celllinecard) or null | no | Answer when available. |
| `schema` | "biomcp.cell_line_card/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: CellLineCard

One requested cell identity with explicit ambiguity and requested section states.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `age` | [Text](#text) or null | no | Native age expression when supplied. |
| `candidates` | array of [CellLineHeader](#celllineheader) | yes | Candidate identities without first-hit selection. |
| `dates` | [CellLineEntryDates](#celllineentrydates) or null | no | Selected entry dates and version, separate from release metadata. |
| `fetched` | integer or null | no | Original retrieval seconds, distinct from entry and release dates. |
| `header` | [CellLineHeader](#celllineheader) or null | no | Selected verified identity only. |
| `input` | [CellLineInput](#celllineinput) | yes | Exact original caller identity. |
| `more` | array of [CellLineSection](#celllinesection) | yes | Supported disclosures not requested. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |
| `sections` | [CellLineSections](#celllinesections) | yes | Only explicitly requested sections carry a state. |
| `sex` | [Text](#text) or null | no | Native sex label when supplied. |
| `status` | [Status](#status) | yes | Resolved, absent, ambiguous or failed input outcome. |
| `url` | [Text](#text) or null | no | Publisher URL for the selected identity. |
| `warnings` | [Capped2](#capped2) or null | no | Selected card warnings, up to three whole rows; not a fetch state. |

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
| `items` | array of [CellLineXref](#celllinexref) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Capped2

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [CellLineWarning](#celllinewarning) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Capped3

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [CellLineName](#celllinename) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Capped4

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [CellLineVariation](#celllinevariation) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Capped5

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [CellLineChemblRecord](#celllinechemblrecord) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### CellLineChembl

Exact accession-linked metadata with its own native collection total.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Requested exact accession. |
| `fetched` | integer or null | no | Original retrieval seconds; never a source release date. |
| `items` | [Capped5](#capped5) | yes | Every selected whole metadata row and the native total. |
| `offset` | integer (uint32) | yes | Native metadata page offset. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independent ChEMBL release, distinct from retrieval time. |

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

### CellLineEntryDates

Entry dates and entry version are independent of dataset release and retrieval time.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `created` | [PartialDate](#partialdate) or null | no | Source entry creation date. |
| `updated` | [PartialDate](#partialdate) or null | no | Source entry update date. |
| `version` | integer or null | no | Source entry version, never a dataset release. |

### CellLineGene

A native variation gene member with only a source-supplied HGNC identity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `hgnc` | [HgncId](#hgncid) or null | no | Present only if this source member explicitly carries the HGNC identifier. |
| `reference` | [CellLineXref](#celllinexref) | yes | Complete original source gene cross-reference. |

### CellLineHeader

Compact source identity with independently capped species and diseases.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Exact primary accession. |
| `category` | [Text](#text) | yes | Native category. |
| `diseases` | [Capped](#capped) | yes | Up to three complete disease references and their independent total. |
| `name` | [Text](#text) | yes | Recommended display name. |
| `species` | [Capped](#capped) | yes | Up to three complete species references and their native total. |

### CellLineIdentityPage

Species, diseases and warnings page independently; each total belongs to its own collection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `diseases` | [Capped](#capped) | yes | Whole disease rows and their independent source total. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `offset` | integer (uint32) | yes | Requested offset applied independently to each collection. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |
| `species` | [Capped](#capped) | yes | Whole species rows and their source total. |
| `warnings` | [Capped2](#capped2) | yes | Whole scoped warning rows and their independent source total. |

### CellLineInput

An exact accession or a database-qualified source identity; never a name lookup.

Type: string.

### CellLineName

One native display name entry, never an identity key.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | [Text](#text) | yes | Native name type, preserving identifier and synonym labels. |
| `value` | [Text](#text) | yes | Native name spelling and punctuation. |

### CellLineNames

Native display names with collection-local totals and offset.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `items` | [Capped3](#capped3) | yes | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | yes | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |

### CellLineProvenance

One complete native source row; reference, institution and database provenance stay independent.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `institution` | [Text](#text) or null | no | Native institution name if supplied. |
| `reference` | [CellLineReference](#celllinereference) or null | no | Native publication reference if supplied. |
| `value` | [Text](#text) or null | no | Native institution value if supplied. |
| `xref` | [CellLineXref](#celllinexref) or null | no | Complete native database provenance if supplied. |

### CellLineReference

One native internal publication reference, without requiring PubMed.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `resource_internal_ref` | [Text](#text) | yes | Complete native resource reference, for example PubMed=9305600. |

### CellLineRelease

Publisher release identity, fetched independently from record data.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `updated` | [PartialDate](#partialdate) | yes | Native release update date. |
| `version` | [Text](#text) | yes | Native release version. |

### CellLineSection

Named disclosure sections; all requests each bounded first page without an assay count.

Values: `names`, `identity`, `xrefs`, `variants`, `chembl`, `all`.

### CellLineSections

Requested sections carry explicit states; absent members were not requested.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `chembl` | [Section5](#section5) or null | no | Requested chembl section state. |
| `identity` | [Section2](#section2) or null | no | Requested identity section state. |
| `names` | [Section](#section) or null | no | Requested names section state. |
| `variants` | [Section4](#section4) or null | no | Requested variants section state. |
| `xrefs` | [Section3](#section3) or null | no | Requested xrefs section state. |

### CellLineVariants

Whole native variation rows with collection-local totals and offset.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `items` | [Capped4](#capped4) | yes | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | yes | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |

### CellLineVariation

One complete native variation row; descriptions do not become normalized alleles.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | [Text](#text) or null | no | Complete native mutation description including all coordinate spellings. |
| `genes` | array of [CellLineGene](#celllinegene) | yes | Every native gene member, including both fusion members. |
| `mutation_type` | [Text](#text) or null | no | Native open mutation label. |
| `note` | [Text](#text) or null | no | Complete native variation note. |
| `provenance` | array of [CellLineProvenance](#celllineprovenance) | yes | Every native provenance row in source order; no PMID is required. |
| `references` | array of [CellLineXref](#celllinexref) | yes | Every other variation cross-reference in native order. |
| `variation_type` | [Text](#text) | yes | Native open variation label. |
| `zygosity` | [Text](#text) or null | no | Native zygosity label; absence is not inferred. |

### CellLineWarning

Complete source warning with its native scope; no stock classifier is derived.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `scope` | [Text](#text) | yes | Native warning category, such as Problematic cell line. |
| `text` | [Text](#text) | yes | Complete source text including historical and stock qualifications. |

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

### CellLineXrefs

Native source cross-references with collection-local totals and offset.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `items` | [Capped](#capped) | yes | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | yes | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |

### CellosaurusAccession

Type: string.

### ChemblCellId

Type: string.

### HgncId

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

### Section

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section2

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section3

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section4

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section5

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

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.cell_line_card/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
