---
title: biomcp.variant_population/4
description: "The BioMCP 1.0.0 reply schema biomcp.variant_population/4: its
  envelope fields, its result type VariantPopulationResult and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_population/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_population/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_population/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_population/4`.

These commands return it: [`biomcp variant population`](/reference/commands/variant-population/).

Calls that returned it:

- `biomcp variant population CA118786` ([`biomcp variant population`](/reference/commands/variant-population/))
- `biomcp variant population CA118786 --detail --limit 50` ([`biomcp variant population`](/reference/commands/variant-population/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [VariantPopulationQuery](#query-variantpopulationquery) | yes | Parsed query. |
| `result` | [VariantPopulationResult](#result-variantpopulationresult) or null | no | Answer when available. |
| `schema` | "biomcp.variant_population/4" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: VariantPopulationResult

Population summary and one explicitly requested ancestry-detail page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `groups` | array of [GnomadCounts](#gnomadcounts) | no | Selected native groups, qualified by sequencing subset. |
| `population` | [Section](#section) | yes | Source population state; missing datasets stay absent within its summary. |

## Query: VariantPopulationQuery

Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Explicit source assembly when required by identity input. |
| `detail` | boolean | no | Read source ancestry groups, preserving overlapping groups individually. |
| `input` | [VariantInput](#variantinput) | yes | Exact published allele identity; ambiguous candidates must first be selected. |
| `limit` | integer (uint32) | no | Maximum rows per page. |
| `offset` | integer (uint32) | no | Native rows to skip. |
| `resolver` | [VariantResolver](#variantresolver) | no | Resolver for reference-qualified input. |

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

### Build

The assembly stated by the source; no default is inferred.

Values: `GRCh37`, `GRCh38`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### EnsemblGeneId

Type: string.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ensembl_gene_id` | [EnsemblGeneId](#ensemblgeneid) or null | no |  |
| `hgnc_id` | [HgncId](#hgncid) or null | no |  |
| `label` | [Text](#text) | yes |  |
| `ncbi_gene_id` | [NcbiGeneId](#ncbigeneid) or null | no |  |

### GnomadCounts

Native gnomAD counts retained when a dataset release cannot be established.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ac` | integer (uint64) | yes | Native allele count, including reported zero. |
| `an` | integer (uint64) | yes | Native allele denominator. |
| `ancestry_group` | [Text](#text) or null | no | Source ancestry code decoded only by the documented source grammar. |
| `data_type` | [PopulationDataType](#populationdatatype) | yes | Source sequencing sample set. |
| `filters` | array of [Text](#text) | no | Native source filters, never converted to a classification. |
| `frequency` | [PopulationFrequency](#populationfrequency) or null | no | Checked total from a release supplied alongside matching source counts. |
| `group_id` | [Text](#text) or null | no | Complete native group ID, including its source sex suffix when present. |
| `sex` | [PopulationSex](#populationsex) or null | no | Source sex stratum decoded only by the documented source grammar. |

### HgncId

Type: string.

### HgvsText

Type: string.

### LeafRef

One of:

- object
- object

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

### PopulationDataType

Dataset sequencing sample set.

Values: `exomes`, `genomes`, `joint`.

### PopulationFrequency

One checked stratum of one source dataset release.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele_count` | integer or null | no |  |
| `allele_frequency` | [FiniteNumber](#finitenumber) or null | no |  |
| `allele_number` | integer or null | no |  |
| `ancestry_group` | [Text](#text) or null | no |  |
| `attribution` | [Attribution](#attribution) | yes |  |
| `build` | [Build](#build) | yes |  |
| `data_type` | [PopulationDataType](#populationdatatype) or null | no |  |
| `dataset` | [Text](#text) | yes |  |
| `dataset_version` | [Text](#text) | yes |  |
| `faf95` | [FiniteNumber](#finitenumber) or null | no |  |
| `filters` | array of [Text](#text) | no |  |
| `hemizygote_count` | integer or null | no |  |
| `homozygote_count` | integer or null | no |  |
| `sex` | [PopulationSex](#populationsex) or null | no |  |
| `subject` | [VariantRef](#variantref) | yes |  |
| `subset` | [Text](#text) or null | no |  |

### PopulationSex

Sex stratum stated by a population source.

Values: `xx`, `xy`.

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

### VariantId

Type: string.

### VariantInput

Type: string.

### VariantPopulation

Bounded population summary with independent optional sequencing datasets.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) | yes | Source dataset assembly. |
| `dataset` | [Text](#text) | yes | Source dataset selector. |
| `exome` | [GnomadCounts](#gnomadcounts) or null | no | Exome observation; omitted when the source dataset is null. |
| `genome` | [GnomadCounts](#gnomadcounts) or null | no | Genome observation; omitted when the source dataset is null. |
| `variant` | [Text](#text) | yes | Source allele identifier. |

### VariantRef

Checked source subject reference; at least one source identity or label is required.

One of:

- object: Exact allele identifier or reference-qualified notation.
- object: Protein-level assertion, without genomic conversion.
- object: Compound source profile, without an inferred allele.

### VariantResolver

Explicit source service for reference-qualified notation.

Values: `car`, `mutalyzer`, `variantvalidator`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_population/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
