---
title: biomcp.health/2
description: "The BioMCP 1.0.0 reply schema biomcp.health/2: its envelope
  fields, its result type Health and the commands that return it."
goal: Look up the fields of a biomcp.health/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.health/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.health/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.health/2`.

These commands return it: [`biomcp health`](/reference/commands/health/).

Calls that returned it:

- `biomcp health` ([`biomcp health`](/reference/commands/health/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | null | yes | Parsed query. |
| `result` | [Health](#result-health) or null | no | Answer when available. |
| `schema` | "biomcp.health/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: Health

Source reachability and publisher terms, independent of local HTTP service health.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `disable_groups` | array of [DisableGroup](#disablegroup) | no | Sources and carried components excluded by each supported terms group. |
| `sources` | array of [SourceHealth](#sourcehealth) | yes | Every implemented adapter, in declaration order. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### ComponentHealth

Policy labels for one carried source component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `disabled` | boolean | yes | Explicit source or component group policy prevents this component. |
| `name` | [Text](#text) | yes | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | yes | Independently reviewed publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Publisher terms name. |
| `url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Publisher use category. |

### DisableGroup

The consequences of one source-disable selector, regardless of current configuration.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [DisabledComponent](#disabledcomponent) | yes | Carried fields stopped while their aggregators remain available. |
| `group` | [Use](#use) | yes | Terms category accepted by disable_sources. |
| `sources` | array of [Source](#source) | yes | Whole sources stopped by this group. |

### DisabledComponent

One independently labeled carried field excluded by a terms group.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | string | yes | Registry-owned component name. |
| `source` | [Source](#source) | yes | Aggregator carrying the field. |

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

### SourceHealth

One bounded source probe, without credentials or provider error text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [ComponentHealth](#componenthealth) | no | Independently licensed carried components, without separate network probes. |
| `disabled` | boolean | yes | Whether source policy prevents all requests and cached reads. |
| `latency_ms` | integer or null | no | Probe duration in milliseconds; omitted when policy skipped the probe. |
| `reason` | [Reason](#reason) or null | no | Typed failure or skip reason. |
| `source` | [Source](#source) | yes | Implemented source identity. |
| `state` | [SourceState](#sourcestate) | yes | Probe outcome; disabled and missing credentials remain skipped. |
| `terms` | [Text](#text) | yes | Publisher license or terms name. |
| `terms_url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Reviewed source category, not blanket permission for each record. |

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

This page is generated from `biomcp schema biomcp.health/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
