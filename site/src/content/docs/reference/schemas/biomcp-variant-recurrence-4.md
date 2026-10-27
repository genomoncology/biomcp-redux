---
title: biomcp.variant_recurrence/4
description: "The BioMCP 1.0.0 reply schema biomcp.variant_recurrence/4: its
  envelope fields, its result type VariantRecurrence and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_recurrence/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_recurrence/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_recurrence/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_recurrence/4`.

These commands return it: [`biomcp variant recurrence`](/reference/commands/variant-recurrence/).

Calls that returned it:

- `biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3` ([`biomcp variant recurrence`](/reference/commands/variant-recurrence/))
- `biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3 --page 0` ([`biomcp variant recurrence`](/reference/commands/variant-recurrence/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [VariantRecurrenceQuery](#query-variantrecurrencequery) | yes | Parsed query. |
| `result` | [VariantRecurrence](#result-variantrecurrence) or null | no | Answer when available. |
| `schema` | "biomcp.variant_recurrence/4" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: VariantRecurrence

Exact-allele observations confined to one explicitly bounded source page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `all_pages_read` | boolean | yes | True only when this invocation read the whole gene query from its first page. |
| `entrez_gene_id` | [NcbiGeneId](#ncbigeneid) or null | no | HGNC-resolved gene query bound. |
| `examined_rows` | integer (uint32) | yes | Number of source mutation rows examined on this page. |
| `identity` | [Variant](#variant) or null | no | Source-qualified resolved identity; omitted when resolution failed. |
| `matching_granularity` | [RecurrenceGranularity](#recurrencegranularity) | no | Genomic allele equality only, never a protein, residue or gene match. |
| `more` | boolean | yes | Source has further rows or cannot yet establish its end. |
| `observed_sample_count` | integer or null | no | Distinct exact-matched samples on this source page only; absent on failure. |
| `selected_sample_count` | integer or null | no | Sample-list size as stated by that source; never an assayed denominator. |
| `study` | [RecurrenceStudy](#recurrencestudy) or null | no | Study title, citation and description kept as the source states them. |
| `terms` | [RecurrenceTerms](#recurrenceterms) | yes | Study-specific reuse restrictions require verification even for public data. |

## Query: VariantRecurrenceQuery

Observe exact-allele samples within one caller-selected source page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene` | [GeneSymbol](#genesymbol) | yes | HGNC symbol resolved to the source's Entrez gene identifier. |
| `input` | [VariantInput](#variantinput) | yes | Published variant identity; ambiguous identities are never selected silently. |
| `limit` | integer (uint32) | no | Source mutation rows per page, including excluded alleles. |
| `page` | integer (uint32) | no | Zero-based native source page. |
| `profile` | [Text](#text) | yes | Explicit mutation molecular profile. |
| `sample_set` | [Text](#text) | yes | Explicit sample list; selected size is never an allele denominator. |
| `source` | [RecurrenceSource](#recurrencesource) | yes | Explicit cohort source. |
| `study` | [Text](#text) | yes | Explicit study; there is no default cohort. |

## Types

Every other type this schema uses.

### Accession

Type: string.

### Allele

Source-stated interbase replacement; equality never claims normalized equivalence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alternate` | [Sequence](#sequence) | yes |  |
| `location` | [SequenceLocation](#sequencelocation) | yes |  |
| `reference` | [Sequence](#sequence) | yes |  |

### Build

The assembly stated by the source; no default is inferred.

Values: `GRCh37`, `GRCh38`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### EnsemblGeneId

Type: string.

### Expression

Source expression checked for its declared published form, without normalization.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `syntax` | [ExpressionSyntax](#expressionsyntax) | yes |  |
| `syntax_version` | [Text](#text) or null | no |  |
| `value` | [Text](#text) | yes |  |

### ExpressionSyntax

Published syntax label for a source-preserved expression.

Values: `hgvs.g`, `hgvs.c`, `hgvs.n`, `hgvs.r`, `hgvs.p`, `hgvs.m`, `hgvs.o`, `spdi`, `gnomad`, `vcf`, `iscn`.

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

### RecurrenceGranularity

Source fields required for a recurrence observation.

Values: `exact_genomic_allele`.

### RecurrenceSource

Source requiring explicit study, profile and sample set.

Values: `cbioportal`.

### RecurrenceStudy

Source-native descriptive metadata, without treating access groups as licenses.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `citation` | [Text](#text) or null | no | Source publication citation. |
| `description` | [Text](#text) or null | no | Complete source description, including any literal reuse notice. |
| `name` | [Text](#text) | yes | Source study title. |

### RecurrenceTerms

Public access grants no study-specific commercial reuse permission.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `notice` | string | yes | Source warning about possible commercial restrictions. |
| `unverified` | boolean | yes | Study-specific rights remain unverified. |
| `url` | string | yes | Publisher FAQ explaining study-specific terms. |

### Representation

One source's own expression, assembly and optional source-stated interbase allele.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele` | [Allele](#allele) or null | no | Source-supplied allele, without local normalization or equivalence inference. |
| `build` | [Build](#build) or null | no | Assembly stated by the source, omitted when absent. |
| `expression` | [Expression](#expression) | yes | Published notation kept unchanged. |
| `source` | [Text](#text) | yes | Source asserting this form. |

### Sequence

Type: string.

### SequenceLocation

An interbase interval on one versioned sequence accession.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `sequence` | [Accession](#accession) | yes | Versioned source sequence, without an inferred genome build. |
| `span` | [Span0](#span0) | yes | Zero-based half-open bounds. |

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

### Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | integer (uint32) | yes |  |
| `start` | integer (uint32) | yes |  |

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

### Variant

Variant entity with source-stated identities, forms and genes; no disclosure policy.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `genes` | array of [GeneRef](#generef) | no | Genes stated by the sources. |
| `ids` | array of [VariantId](#variantid) | no | All source identifiers, without inferred aliases. |
| `representations` | array of [Representation](#representation) | no | Every source-qualified representation. |

### VariantId

Type: string.

### VariantInput

Type: string.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_recurrence/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
