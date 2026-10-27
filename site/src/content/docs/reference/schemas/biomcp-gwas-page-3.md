---
title: biomcp.gwas_page/3
description: "The BioMCP 1.0.0 reply schema biomcp.gwas_page/3: its envelope
  fields, its result type GwasPage and the commands that return it."
goal: Look up the fields of a biomcp.gwas_page/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gwas_page/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gwas_page/3`.

These commands return it: [`biomcp search gwas`](/reference/commands/search-gwas/).

Calls that returned it:

- `biomcp search gwas --variant rs7903146 --limit 3` ([`biomcp search gwas`](/reference/commands/search-gwas/))
- `biomcp get gwas GCST000035 associations` ([`biomcp get gwas`](/reference/commands/get-gwas/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [GwasPageQuery](#query-gwaspagequery) | yes | Parsed query. |
| `result` | [GwasPage](#result-gwaspage) or null | no | Answer when available. |
| `schema` | "biomcp.gwas_page/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: GwasPage

One native association page, also used for a study associations section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `continuation` | [GwasPageQuery](#query-gwaspagequery) | no | Same-selector search continuation. |
| `items` | array of [GwasAssociation](#gwasassociation) | yes | Shared complete association records. |
| `page` | [PageInfo](#pageinfo) | yes | Native page scope and total. |
| `qualifying_total` | integer or null | no | Complete qualifying count, unknown until the threshold walk reaches its end. |

## Query: GwasPageQuery

Search and study-association pages echo their original checked request.

One of:

- object: Native association search.
- object: Exact study associations relation.

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### GeneSymbol

Type: string.

### GwasAssociation

Source association with unchanged trait and effect labels.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alleles` | array of [GwasEffectAllele](#gwaseffectallele) | yes | Every source effect allele paired with its rsID. |
| `author` | [Text](#text) or null | no | Source author attribution, without local publication inference. |
| `beta` | [Text](#text) or null | no | Source beta text, including units and direction. |
| `ci_lower` | [FiniteNumber](#finitenumber) or null | no | Source lower effect interval bound. |
| `ci_upper` | [FiniteNumber](#finitenumber) or null | no | Source upper effect interval bound. |
| `exact_p_value` | [ScientificPValue](#scientificpvalue) or null | no | Authoritative exact source probability, including double underflow. |
| `id` | integer (uint64) | yes | Native association identity. |
| `interval` | [Text](#text) or null | no | Source effect interval. |
| `locations` | array of [Text](#text) | no | Native mapping location strings; no assumed assembly or coordinate conversion. |
| `mapped_genes` | array of [Text](#text) | no | Original native mapped gene context; reported genes remain in loci. |
| `odds_ratio` | [FiniteNumber](#finitenumber) or null | no | Source odds ratio. |
| `ontology_traits` | array of [GwasTrait](#gwastrait) | no | Source mapped trait identifiers and whole labels, without clinical inference. |
| `p_value` | [FiniteNumber](#finitenumber) | yes | Reported p value. |
| `publication` | [Text](#text) or null | no | Source publication identifier. |
| `risk_frequency` | [Text](#text) or null | no | Source allele frequency text, without interpreting NR. |
| `study` | [Text](#text) | yes | Study accession. |
| `traits` | array of [Text](#text) | yes | Source reported traits. |

### GwasAssociationId

Type: string.

### GwasAssociationSection

Association-only section.

Values: `loci`.

### GwasEffectAllele

Catalog effect allele paired with its exact rsID.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele` | [Text](#text) | yes | Source allele text. |
| `rs_id` | [Text](#text) | yes | Source rsID. |

### GwasGet

Explicit resource sum; impossible section and identity pairings fail during decoding.

One of:

- object: Read an association.
- object: Read a study.

### GwasOrder

Native source order, echoed without a configurable sort command.

Values: `p_value,asc`.

### GwasSearch

Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `extended_geneset` | boolean | no | Source extended gene set; requires gene. |
| `gene` | [GeneSymbol](#genesymbol) | no | Exact native mapped gene; legacy positional query aliases this field. |
| `include_child_traits` | boolean | no | Source child-trait expansion; requires trait_id. |
| `limit` | integer (uint32) | no | Maximum association rows. |
| `offset` | integer (uint32) | no | Native association offset. |
| `order` | [GwasOrder](#gwasorder) | no | Source order; threshold requests use this same native order. |
| `p_value` | string | no | Inclusive positive probability in scientific notation, such as 1e-5. The qualifying total stays unknown until the ordered walk reaches its end. |
| `pool` | [Text](#text) | no | Retained threshold walk identity; requires a threshold. |
| `pubmed` | [Pmid](#pmid) | no | Exact source publication identity. |
| `study` | [GwasStudyId](#gwasstudyid) | no | Exact source study accession. |
| `trait` | [Text](#text) | no | Literal native efo_trait label; never searches reported_trait prose. |
| `trait_id` | [GwasTraitId](#gwastraitid) | no | Exact native ontology identity. |
| `variant` | [RsId](#rsid) | no | Exact source rsID. |

### GwasStudyId

Type: string.

### GwasStudySection

Study-only sections.

Values: `associations`, `ancestries`.

### GwasTrait

Source ontology mapping as supplied by GWAS Catalog.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Exact source ontology identifier spelling. |
| `label` | [Text](#text) | yes | Whole source trait label. |

### GwasTraitId

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

### Pmid

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

### RsId

Type: string.

### ScientificPValue

Exact normalized scientific p value; no floating-point conversion is used.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `exponent` | integer (int64) | yes | Source base-ten exponent; values below the double range remain exact. |
| `mantissa` | string | yes | Source mantissa spelling, including significant trailing zeros. |

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

This page is generated from `biomcp schema biomcp.gwas_page/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
