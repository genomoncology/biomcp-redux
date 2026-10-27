---
title: biomcp.disease_survival/3
description: "The BioMCP 1.0.0 reply schema biomcp.disease_survival/3: its
  envelope fields, its result type DiseaseSurvival and the commands that return
  it."
goal: Look up the fields of a biomcp.disease_survival/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.disease_survival/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.disease_survival/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.disease_survival/3`.

These commands return it: [`biomcp get disease`](/reference/commands/get-disease/).

Calls that returned it:

- `biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 --dataset 4 --interval 5 --section-limit 5` ([`biomcp get disease`](/reference/commands/get-disease/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [DiseaseGet](#query-diseaseget) | yes | Parsed query. |
| `result` | [DiseaseSurvival](#result-diseasesurvival) or null | no | Answer when available. |
| `schema` | "biomcp.disease_survival/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: DiseaseSurvival

Native annual survival rows with separate observed and modeled values.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [ComponentLabel](#componentlabel) | yes | Applicable component use terms, independent of row provenance. |
| `fetched` | integer or null | no | Original response retrieval time in Unix seconds, distinct from dataset publication. |
| `items` | array of [DiseaseStatistic](#diseasestatistic) | yes | Native source rows; totals in the envelope count rows, not distinct entities. |

## Query: DiseaseGet

Exact concept or native SEER site with independently paged section.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `age_range` | [Text](#text) or null | no | Native age-range code. |
| `dataset` | [Text](#text) or null | no | Requested native dataset selector, not proof of returned membership. |
| `id` | [Text](#text) | yes | Exact ontology CURIE or an explicit SEER catalog site ID. |
| `interval` | [Text](#text) or null | no | Survival-only interval code. |
| `ontology` | [DiseaseOntology](#diseaseontology) | no | One OLS ontology. |
| `period` | [Text](#text) or null | no | Incidence-only native period code. |
| `race` | [Text](#text) or null | no | Native race code. |
| `section` | [DiseaseSection](#diseasesection) or null | no | One named section; absent means identity card only. |
| `section_limit` | integer (uint32) | no | Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. |
| `section_offset` | integer (uint32) | no | Native row offset within the selected section. |
| `sex` | [Text](#text) or null | no | Native sex code. |
| `source` | [DiseaseSource](#diseasesource) | no | Default MyDisease; no automatic fallback. |

## Types

Every other type this schema uses.

### Attribution

Provenance for one source record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `evaluated` | [PartialDate](#partialdate) or null | no | Source evaluation date. |
| `record` | [Text](#text) or null | no | Native assertion record. |
| `release` | [Text](#text) or null | no | Dataset release. |
| `source` | [Text](#text) | yes | Source responsible for the claim. |
| `submitter` | [Text](#text) or null | no | Source submitter or panel. |
| `url` | [Text](#text) or null | no | Source record link. |
| `version` | [Text](#text) or null | no | Record version. |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### ComponentLabel

Terms attached to a named carried component on a consumer response.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `component` | [Text](#text) | yes | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | yes | Canonical publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Publisher terms name. |
| `url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Publisher use category. |

### DiseaseOntology

One explicit OLS ontology.

Values: `mondo`, `doid`, `ncit`.

### DiseaseSection

One named disease page.

Values: `synonyms`, `xrefs`, `hierarchy`, `genes`, `phenotypes`, `incidence`, `survival`.

### DiseaseSource

An explicit disease source without fallback.

Values: `ols`, `mydisease`, `monarch`, `seer`.

### DiseaseStatistic

One native statistics row with requested and returned dimensions.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Publisher record and dataset version when supplied. |
| `measure` | [Text](#text) | yes | Native measure name. |
| `modeled` | [StatisticEstimate](#statisticestimate) or null | no | Modeled estimate kept separately. |
| `observed` | [StatisticEstimate](#statisticestimate) or null | no | Observed estimate; missing stays absent. |
| `requested_selectors` | array of [StatisticSelector](#statisticselector) | yes | Settings sent to the publisher. |
| `requested_site` | [Text](#text) | yes | Explicit selected native site ID. |
| `returned_selectors` | array of [StatisticSelector](#statisticselector) | yes | Settings actually returned, never filled from request. |
| `year` | integer or null | no | Native annual row year. |

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

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

### StatisticEstimate

Source estimate and uncertainty without computed quantities.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count` | integer or null | no | Native count; no inferred denominator. |
| `lower_ci` | [FiniteNumber](#finitenumber) or null | no | Native lower confidence bound. |
| `standard_error` | [FiniteNumber](#finitenumber) or null | no | Native standard error. |
| `upper_ci` | [FiniteNumber](#finitenumber) or null | no | Native upper confidence bound. |
| `value` | [FiniteNumber](#finitenumber) or null | no | Native estimate. |

### StatisticSelector

One source-native selector without a local interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `code` | [Text](#text) | yes | Source selector code. |
| `label` | [Text](#text) or null | no | Publisher label when supplied. |
| `name` | [Text](#text) | yes | Source field name. |

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

This page is generated from `biomcp schema biomcp.disease_survival/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
