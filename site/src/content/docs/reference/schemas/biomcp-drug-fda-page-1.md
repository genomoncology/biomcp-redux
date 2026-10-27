---
title: biomcp.drug_fda_page/1
description: "The BioMCP 1.0.0 reply schema biomcp.drug_fda_page/1: its envelope
  fields, its result type DrugFdaPage and the commands that return it."
goal: Look up the fields of a biomcp.drug_fda_page/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_fda_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.drug_fda_page/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.drug_fda_page/1`.

This call returns it.

- `biomcp search drug daraxonrasib --source drugsfda --limit 1` ([`biomcp search drug`](/reference/commands/search-drug/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [DrugSearch](#query-drugsearch) | yes | Parsed query. |
| `result` | [DrugFdaPage](#result-drugfdapage) or null | no | Answer when available. |
| `schema` | "biomcp.drug_fda_page/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: DrugFdaPage

A source-owned application page preserving complete product context and source order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [DrugFdaCandidate](#drugfdacandidate) | yes | Native applications and products; a phrase match never establishes equivalence. |
| `source_updated` | [PartialDate](#partialdate) or null | no | Native dataset last_updated date, independent of event dates and retrieval. |
| `total` | integer (uint64) | yes | Native application total, independent of product count. |

## Query: DrugSearch

Literal single-source candidate discovery, without an implicit identity selection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native candidate rows per page. |
| `name_selector` | [DrugNameSelector](#drugnameselector) | no | Explicit ChEMBL native name selector; omitted means preferred name. |
| `offset` | integer (uint32) | no | Native rows to skip. |
| `query` | [Text](#text) | yes | Literal source name selector, unchanged by the consumer. |
| `source` | [DrugSource](#drugsource) | no | Exactly one source; default MyChem never fans out. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### DrugFdaCandidate

One candidate application and its independent source-listed products.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `identity` | [DrugHeader](#drugheader) | yes | Exact native application identity; no product has been selected. |
| `products` | array of [DrugFdaProduct](#drugfdaproduct) | yes | Complete native product candidates within this application. |

### DrugFdaIngredient

An unchanged native ingredient and strength, without local normalization.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Native ingredient name. |
| `strength` | [Text](#text) | no | Full source strength wording. |

### DrugFdaProduct

One explicit native product within the selected FDA application.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `active_ingredients` | array of [DrugFdaIngredient](#drugfdaingredient) | yes | Complete source-listed ingredient and strength pairs. |
| `brand_name` | [Text](#text) | yes | Source brand wording, without inferred ingredient equivalence. |
| `dosage_form` | [Text](#text) | yes | Native dosage form. |
| `marketing_status` | [Text](#text) | yes | Product marketing status, independent of submission history. |
| `product_number` | [Text](#text) | yes | Exact source product number, scoped by the response application. |
| `reference_drug` | [Text](#text) | yes | Original source reference-drug designation. |
| `reference_standard` | [Text](#text) or null | no | Original source reference-standard designation when supplied. |
| `route` | [Text](#text) or null | no | Native route; absent and null remain unavailable. |
| `te_code` | [Text](#text) or null | no | Native therapeutic-equivalence code when supplied. |

### DrugHeader

One source candidate, without authorization, targets, label text or interactions.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [DrugId](#drugid) | yes | Published native identifier. |
| `identity_level` | [DrugIdentityLevel](#drugidentitylevel) | yes | Level established only by the endpoint or record type. |
| `name` | [Text](#text) | no | Original source display name. |
| `source` | [DrugSource](#drugsource) | yes | The single source responsible for this header. |
| `type` | [Text](#text) or null | no | Native record type. |
| `url` | [Text](#text) or null | no | Source record URL. |
| `version` | [Text](#text) or null | no | Native source revision, omitted when the source supplies none. |

### DrugId

Type: string.

### DrugIdentityLevel

Identity level established by the source endpoint or record type, never inferred from ID shape.

Values: `ingredient`, `salt`, `clinical_formulation`, `branded_product`, `spl_set`, `spl_version`, `ndc_package`, `fda_application`, `fda_product`, `ema_authorization`, `ema_application`, `orphan_designation`, `unknown`.

### DrugNameSelector

One explicit ChEMBL name filter without fallback or synonym expansion.

Values: `preferred`, `synonym`.

### DrugSource

Values: `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter`.

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

This page is generated from `biomcp schema biomcp.drug_fda_page/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
