---
title: biomcp.protein_features/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_features/1: its
  envelope fields, its result type ProteinFeatures and the commands that return
  it."
goal: Look up the fields of a biomcp.protein_features/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_features/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_features/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_features/1`.

These commands return it: [`biomcp protein features`](/reference/commands/protein-features/).

Calls that returned it:

- `biomcp protein features P00533` ([`biomcp protein features`](/reference/commands/protein-features/))
- `biomcp protein features P00533 --limit 1` ([`biomcp protein features`](/reference/commands/protein-features/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ProteinFeaturesQuery](#query-proteinfeaturesquery) | yes | Parsed query. |
| `result` | [ProteinFeatures](#result-proteinfeatures) or null | no | Answer when available. |
| `schema` | "biomcp.protein_features/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ProteinFeatures

Native UniProt feature rows, preserving exact labels and uncertain bounds.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinFeature](#proteinfeature) | yes | Whole native rows in source order; totals apply only to this collection. |

## Query: ProteinFeaturesQuery

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | [Text](#text) | no | Retained collection identity. |
| `position` | integer (uint32) | no | Native one-based residue; zero fails during deserialization. |
| `type` | [Text](#text) | no | Exact UniProt native label, without an alias map. |

## Types

Every other type this schema uses.

### AcmgAmp2015Code

Checked AcmgAmp2015 codes.

Values: `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### AminoAcids

Type: string.

### AmpAscoCap2017LevelCode

Checked AmpAscoCap2017Level codes.

Values: `level_a`, `level_b`, `level_c`, `level_d`.

### AmpAscoCap2017TierCode

Checked AmpAscoCap2017Tier codes.

Values: `tier_i`, `tier_ii`, `tier_iii`, `tier_iv`.

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

### Bound

A bounded source range; equal min and max denote an exact boundary.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `max` | integer or null | no |  |
| `min` | integer or null | no |  |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

### ClingenCgcVicc2022Code

Checked ClingenCgcVicc2022 codes.

Values: `oncogenic`, `likely_oncogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### ClingenLowPenetrance2024Code

Checked ClingenLowPenetrance2024 codes.

Values: `pathogenic_low_penetrance`, `likely_pathogenic_low_penetrance`, `established_risk_allele`, `likely_risk_allele`, `uncertain_risk_allele`.

### Coding

A code in one explicitly named source framework.

One of:

- object: OncoKB oncogenicity vocabulary.
- object: OncoKB evidence levels.
- object: CIViC evidence level.
- object: acmg_amp_2015 vocabulary.
- object: clingen_low_penetrance_2024 vocabulary.
- object: amp_asco_cap_2017_tier vocabulary.
- object: amp_asco_cap_2017_level vocabulary.
- object: clingen_cgc_vicc_2022 vocabulary.

### OncokbLevelCode

Source-defined OncoKB vocabulary.

Values: `level_1`, `level_2`, `level_3a`, `level_3b`, `level_4`, `level_r1`, `level_r2`, `level_dx1`, `level_dx2`, `level_dx3`, `level_px1`, `level_px2`, `level_px3`.

### OncokbOncogenicityCode

Source-defined OncoKB vocabulary.

Values: `oncogenic`, `likely_oncogenic`, `likely_neutral`, `inconclusive`, `resistance`, `unknown`.

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

### ProteinCoordinateConvention

Coordinate convention on the native feature before conversion.

Values: `one_based_inclusive`.

### ProteinFeature

UniProt feature with source attribution and coordinate provenance.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source record and release facts. |
| `feature` | [UniProtFeature](#uniprotfeature) | yes | Shared feature, including native label and residues. |
| `source_coordinates` | [ProteinCoordinateConvention](#proteincoordinateconvention) | yes | Convention before mathematical conversion. |

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

### Term

A source label with optional explicitly supported coding.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `coding` | [Coding](#coding) or null | no | Documented framework coding. |
| `label` | [Text](#text) | yes | Original label. |

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### UncertainSpan0

Zero-based interbase span retaining uncertain boundary ranges.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | [Bound](#bound) | yes |  |
| `start` | [Bound](#bound) | yes |  |

### UniProtFeature

Shared native feature contract without source label remapping.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alternative` | array of [AminoAcids](#aminoacids) | no | Every stated alternative, without translation. |
| `description` | [Text](#text) or null | no | Complete source description. |
| `evidence` | array of [Text](#text) | no | All original source evidence references. |
| `id` | [Text](#text) or null | no | Native feature identifier. |
| `kind` | [Term](#term) | yes | Source feature label; coding only for documented exact feature types. |
| `original` | [AminoAcids](#aminoacids) or null | no | Source original residues, when supplied. |
| `protein` | [UniProtIsoform](#uniprotisoform) | yes | Exact source sequence identity. |
| `span` | [UncertainSpan0](#uncertainspan0) | yes | Zero-based interbase coordinates retaining uncertain ranges. |

### UniProtIsoform

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_features/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
