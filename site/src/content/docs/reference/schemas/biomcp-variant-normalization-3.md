---
title: biomcp.variant_normalization/3
description: "The BioMCP 1.0.0 reply schema biomcp.variant_normalization/3: its
  envelope fields, its result type VariantNormalization and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_normalization/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_normalization/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_normalization/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_normalization/3`.

These commands return it: [`biomcp variant normalize`](/reference/commands/variant-normalize/).

Calls that returned it:

- `biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'` ([`biomcp variant normalize`](/reference/commands/variant-normalize/))
- `biomcp variant normalize --source variantvalidator --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38` ([`biomcp variant normalize`](/reference/commands/variant-normalize/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [VariantNormalize](#query-variantnormalize) | yes | Parsed query. |
| `result` | [VariantNormalization](#result-variantnormalization) or null | no | Answer when available. |
| `schema` | "biomcp.variant_normalization/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: VariantNormalization

Independent service results without local correction or transcript selection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `results` | array of [NormalizationResult](#normalizationresult) | yes | Service results in declared order. |

## Query: VariantNormalize

Send unchanged reference-qualified notation to explicitly named services.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Caller assembly; required when VariantValidator is selected. |
| `hgvs` | [HgvsText](#hgvstext) | yes | Unchanged reference-qualified HGVS; no local molecular parse. |
| `limit` | integer (uint32) | no | Source mapping rows per service, with typed continuation for remaining rows. |
| `offset` | integer (uint32) | no | Source mapping rows to skip per service. |
| `source` | [NormalizationSource](#normalizationsource) | yes | Explicit service; all preserves independent results without consensus. |

## Types

Every other type this schema uses.

### Build

The assembly stated by the source; no default is inferred.

Values: `GRCh37`, `GRCh38`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### HgvsText

Type: string.

### NormalizationField

A provider field name and its unchanged text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `field` | string | yes | Native field path or assembly label. |
| `value` | string | yes | Unchanged source text. |

### NormalizationMessage

Provider warning, error or informational message.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `category` | string | yes | Native message category. |
| `code` | string or null | no | Native code when supplied. |
| `text` | string | yes | Unchanged message body, including whitespace. |

### NormalizationResult

One provider's own descriptions, messages and version metadata.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `corrected` | string or null | no | Source corrected description, without local edits. |
| `mappings` | array of [NormalizationField](#normalizationfield) | yes | Mapping rows in native service order, labeled by their provider field. |
| `messages` | array of [NormalizationMessage](#normalizationmessage) | yes | Provider messages with original codes, text and category. |
| `normalized` | string or null | no | Source normalized description, without local edits. |
| `outcome` | [VariantResolution](#variantresolution) | yes | Provider outcome; warning rejection remains distinct from transport failure. |
| `source` | [Source](#source) | yes | Responsible service. |
| `total_mappings` | integer (uint32) | yes | Count before mapping pagination. |
| `total_messages` | integer (uint32) | no | Count before provider message pagination. |
| `version` | array of [NormalizationField](#normalizationfield) | yes | Version fields exactly as returned; absent when the service states none. |

### NormalizationSource

Explicit normalization service selection.

Values: `car`, `mutalyzer`, `variantvalidator`, `all`.

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

### VariantResolution

Identity resolution preserves candidates and source validation outcomes.

Values: `unresolved`, `resolved`, `candidates`, `absent`, `rejected`, `unavailable`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_normalization/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
