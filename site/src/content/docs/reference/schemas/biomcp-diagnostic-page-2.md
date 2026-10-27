---
title: biomcp.diagnostic_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.diagnostic_page/2: its
  envelope fields, its result type DiagnosticPage and the commands that return
  it."
goal: Look up the fields of a biomcp.diagnostic_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.diagnostic_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.diagnostic_page/2`.

These commands return it: [`biomcp search diagnostic`](/reference/commands/search-diagnostic/).

Calls that returned it:

- `biomcp search diagnostic --source gtr --gene BRAF --limit 2` ([`biomcp search diagnostic`](/reference/commands/search-diagnostic/))
- `biomcp search diagnostic --source who-ivd --limit 2` ([`biomcp search diagnostic`](/reference/commands/search-diagnostic/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [DiagnosticSearch](#query-diagnosticsearch) | yes | Parsed query. |
| `result` | [DiagnosticPage](#result-diagnosticpage) or null | no | Answer when available. |
| `schema` | "biomcp.diagnostic_page/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: DiagnosticPage

Source-ordered GTR test candidates or WHO product headers.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | array of [DiagnosticCandidate](#diagnosticcandidate) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

## Query: DiagnosticSearch

Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `biomarker` | [Text](#text) or null | no | Exact table-0 biomarker cell; unsupported in the group table. |
| `bundle_revision` | [Text](#text) or null | no | Immutable stored bundle revision retained by a continuation. |
| `condition_id` | [Text](#text) or null | no | Exact MedGen:CUI identifier only. |
| `device_id` | [DiagnosticDeviceId](#diagnosticdeviceid) or null | no | Exact PMA application, application/supplement or native K identity. |
| `drug_application` | [FdaDrugApplication](#fdadrugapplication) or null | no | Cell-owned typed NDA/BLA application. |
| `gene` | [GeneSymbol](#genesymbol) or null | no | Exact source gene_symbol field, without alias expansion. |
| `gene_id` | [NcbiGeneId](#ncbigeneid) or null | no | Exact source GeneID, exclusive with gene symbol. |
| `limit` | integer (uint32) | no | Five results by default; at most fifty source rows or product headers. |
| `manufacturer` | [Text](#text) or null | no | Literal whole-word phrase in WHO's Manufacturer name cell. |
| `marker` | [Text](#text) or null | no | Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. |
| `offset` | integer (uint32) | no | Zero-based source result offset. |
| `source` | [DiagnosticSearchSource](#diagnosticsearchsource) | yes | Explicit native GTR, WHO IVD or FDA companion discovery. |
| `type` | [GtrTestType](#gtrtesttype) or null | no | Exact native test_type value. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) or null | no | Source-stated concept identifier. |
| `label` | [Text](#text) | yes | Original concept label. |

### Curie

Type: string.

### DiagnosticCandidate

A conflict is visible once without a selected name or card.

One of:

- object: One matched WHO product with cells from its first matching native row.
- object: One resolved native identity.
- object: A native relation remains visible without a current card.
- object: Several incompatible native rows with the same exact accession/version.

### DiagnosticCountUnit

Meaning of the envelope's total for a source-owned diagnostic page.

Values: `product_identities`, `test_identities`, `native_relation_rows`, `native_method_entries`.

### DiagnosticDeviceId

Type: string.

### DiagnosticLaboratory

Compact native laboratory context without authorization interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `country` | [Text](#text) or null | no | Native country. |
| `id` | [Text](#text) | yes | Native laboratory identifier. |
| `institution` | [Text](#text) or null | no | Native institution name. |
| `name` | [Text](#text) or null | no | Source laboratory name, absent when the native cell is empty. |

### DiagnosticSearchSource

One explicit diagnostic discovery source.

Values: `gtr`, `fda-cdx`, `who-ivd`.

### EnsemblGeneId

Type: string.

### FdaDrugApplication

Type: string.

### GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ensembl_gene_id` | [EnsemblGeneId](#ensemblgeneid) or null | no |  |
| `hgnc_id` | [HgncId](#hgncid) or null | no |  |
| `label` | [Text](#text) | yes |  |
| `ncbi_gene_id` | [NcbiGeneId](#ncbigeneid) or null | no |  |

### GeneSymbol

Type: string.

### GtrConditionRelation

Native condition membership; SNOMED cells never become GeneID.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `condition` | [Concept](#concept) | yes | Shared MedGen concept with source label. |
| `mim_number` | string | yes | Native MIM cell, including empty. |
| `snomed_ct_id` | string | yes | Native SNOMED CT cell, including empty; never a gene ID. |
| `test_id` | [GtrTestId](#gtrtestid) | yes | Complete owning test accession/version. |
| `test_type` | [GtrTestType](#gtrtesttype) | yes | Native source test type. |

### GtrGeneRelation

Native gene relation; its numeric ID comes only from the gene row.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene` | [GeneRef](#generef) | yes | Shared source-given GeneID and gene_symbol; absent GeneID stays absent. |
| `mim_number` | string | yes | Native MIM cell, including empty. |
| `native_gene_or_snomed_ct_id` | string or null | no | Uninterpreted nonempty `gene_or_SNOMED_CT_ID` when it is not a checked NCBI Gene ID. |
| `native_medgen` | [Curie](#curie) | yes | Independent native MedGen identity from GTR_identifier. |
| `object_name` | string | yes | Full source object_name without interpretation. |
| `test_id` | [GtrTestId](#gtrtestid) | yes | Complete owning test accession/version. |
| `test_type` | [GtrTestType](#gtrtesttype) | yes | Native source test type. |

### GtrHeader

Small resolved GTR candidate; broad gene and condition arrays stay off the header.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [GtrTestId](#gtrtestid) | yes | Complete source accession and version. |
| `laboratory` | [DiagnosticLaboratory](#diagnosticlaboratory) | yes | Native laboratory context. |
| `name` | [Text](#text) or null | no | Native test name, absent when the cell is empty. |
| `url` | [Text](#text) | yes | Source record URL. |

### GtrMethod

One entry from one independent native method list.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `index` | integer (uint32) | yes | Zero-based native list position. |
| `list` | [GtrMethodList](#gtrmethodlist) | yes | Independent source list; lists are never positionally paired. |
| `test_id` | [GtrTestId](#gtrtestid) | yes | Complete owning test accession/version. |
| `value` | string | yes | Complete unmodified source entry. |

### GtrMethodList

Independent native GTR method lists.

Values: `method_categories`, `methods`, `platforms`.

### GtrTestId

Type: string.

### GtrTestRelation

Source-owned native membership or one independent method-list entry.

One of:

- object: Native gene relation row.
- object: Native condition relation row.
- object: One independent method-list entry.

### GtrTestType

Native GTR relation test category, not an offering or authorization judgment.

Values: `Clinical`, `Research`, `Clinical microbial`.

### HgncId

Type: string.

### NcbiGeneId

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

### WhoProductId

Type: string.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.diagnostic_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
