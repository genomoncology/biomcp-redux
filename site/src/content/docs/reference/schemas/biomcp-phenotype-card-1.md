---
title: biomcp.phenotype-card/1
description: "The BioMCP 1.0.0 reply schema biomcp.phenotype-card/1: its
  envelope fields, its result type PhenotypeTermCard and the commands that
  return it."
goal: Look up the fields of a biomcp.phenotype-card/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.phenotype-card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.phenotype-card/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.phenotype-card/1`.

These commands return it: [`biomcp get phenotype`](/reference/commands/get-phenotype/).

Calls that returned it:

- `biomcp get phenotype HP:0001250 --source hpo` ([`biomcp get phenotype`](/reference/commands/get-phenotype/))
- `biomcp get phenotype HP:0000001` ([`biomcp get phenotype`](/reference/commands/get-phenotype/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [PhenotypeGet](#query-phenotypeget) | yes | Parsed query. |
| `result` | [PhenotypeTermCard](#result-phenotypetermcard) or null | no | Answer when available. |
| `schema` | "biomcp.phenotype-card/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: PhenotypeTermCard

Independent source identity card and its available sections.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [ComponentLabel](#componentlabel) | yes | Applicable publisher component terms. |
| `sections` | array of [PhenotypeSection](#phenotypesection) | yes | Sections available on this source record. |
| `term` | [PhenotypeCard](#phenotypecard) | yes | Complete source-specific native card. |

## Query: PhenotypeGet

Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [HpId](#hpid) | yes | Exact HP identifier. |
| `section` | [PhenotypeSection](#phenotypesection) | no | Absent means the source identity card. |
| `section_limit` | integer (uint32) | no | Default five text/link rows or one complete association row. |
| `section_offset` | integer (uint32) | no | Native section row offset. |
| `snapshot` | [Text](#text) | no | Retained exact source reply for local text and link pages; unsupported for diseases and genes. |
| `source` | [PhenotypeSource](#phenotypesource) | no | HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. |

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

### Curie

Type: string.

### HpId

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

### PhenotypeAssociationCount

Native collection counts, without summing scopes or counting patients.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `category` | [Curie](#curie) | yes | Native category. |
| `count` | integer (uint64) | yes | Native closure-expanded count. |
| `count_direct` | integer or null | no | Native direct count. |
| `count_with_orthologs` | integer or null | no | Native count including orthologs, only when supplied. |
| `key` | [Curie](#curie) | yes | Native category key. |
| `label` | string | yes | Native scope label. |

### PhenotypeCard

Native source card, with independent HPO and Monarch metadata.

One of:

- object: The direct HPO term answer.
- object: The independent Monarch term answer.

### PhenotypeSection

A named source term section.

Values: `synonyms`, `xrefs`, `hierarchy`, `diseases`, `genes`.

### PhenotypeSource

One independent term publisher.

Values: `hpo`, `monarch`.

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

This page is generated from `biomcp schema biomcp.phenotype-card/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
