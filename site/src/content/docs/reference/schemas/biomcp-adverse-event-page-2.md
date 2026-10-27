---
title: biomcp.adverse_event_page/2
description: "The BioMCP 1.0.0 reply schema biomcp.adverse_event_page/2: its
  envelope fields, its result type AdverseEventPage and the commands that return
  it."
goal: Look up the fields of a biomcp.adverse_event_page/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.adverse_event_page/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.adverse_event_page/2`.

These commands return it: [`biomcp search adverse-event`](/reference/commands/search-adverse-event/).

Calls that returned it:

- `biomcp search adverse-event vemurafenib --limit 3` ([`biomcp search adverse-event`](/reference/commands/search-adverse-event/))
- `biomcp search adverse-event pacemaker --source maude --limit 2` ([`biomcp search adverse-event`](/reference/commands/search-adverse-event/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [AdverseEventSearch](#query-adverseeventsearch) | yes | Parsed query. |
| `result` | [AdverseEventPage](#result-adverseeventpage) or null | no | Answer when available. |
| `schema` | "biomcp.adverse_event_page/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: AdverseEventPage

Ordered report headers; totals count reports with no population denominator.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [ComponentLabel](#componentlabel) | yes | Applicable component terms. |
| `fetched` | [PartialDate](#partialdate) or null | no | Original retrieval date belonging to this reply, absent for undated legacy recordings. |
| `items` | array of [AdverseEventHeader](#adverseeventheader) | yes | Native ordered report candidates. |
| `reaction_match` | [AdverseEventReactionMatch](#adverseeventreactionmatch) or null | no | Explicit analyzed reaction selector semantics when a reaction was supplied. |
| `source_query` | string | yes | Exact literal provider expression; reaction selectors use phrase match. |
| `source_updated` | [PartialDate](#partialdate) or null | no | The publisher last_updated date, separate from retrieval. |
| `unit` | [ReportUnit](#reportunit) | yes | Explicit reports unit for envelope page totals. |

## Query: AdverseEventSearch

Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native report headers or VAERS rows per page; default five. |
| `offset` | integer (uint32) | no | Native row offset. VAERS continuation requires its retained snapshot. |
| `reaction` | [Text](#text) or null | no | FAERS MedDRA term phrase match, not exact equality. |
| `received_from` | [PartialDate](#partialdate) or null | no | Inclusive first FDA receipt lower bound, YYYY-MM-DD. |
| `received_to` | [PartialDate](#partialdate) or null | no | Inclusive first FDA receipt upper bound, YYYY-MM-DD. |
| `section` | [VaersSection](#vaerssection) or null | no | Independent VAERS table; omitted means reported symptoms. |
| `serious` | [AdverseEventSerious](#adverseeventserious) or null | no | FAERS report flag: death or hospitalization; never a reaction outcome. |
| `snapshot` | [Text](#text) or null | no | Retained VAERS table for provider-free continuation. |
| `source` | [AdverseEventSource](#adverseeventsource) | no | FAERS by default; VAERS accepts only exact supported vaccine names. |
| `text` | [Text](#text) | yes | Literal drug text, MAUDE device phrase or exact supported VAERS vaccine. |

## Types

Every other type this schema uses.

### AdverseEventHeader

Small source report header without a drug title, flattened lists or count buckets.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `event_type` | string or null | no | Native event_type string; present empty values remain present. |
| `first_received` | string or null | no | Native first_received string; present empty values remain present. |
| `id` | [ReportId](#reportid) | yes | Exact published source identity. |
| `serious` | string or null | no | Native serious string; present empty values remain present. |
| `source` | [AdverseEventSource](#adverseeventsource) | yes | Explicit report family. |
| `version` | string or null | no | Native version string; present empty values remain present. |

### AdverseEventReactionMatch

Source reaction selector matching semantics.

Values: `phrase_match`.

### AdverseEventSerious

Report-level seriousness filter, independent of reaction outcome.

Values: `death`, `hospitalization`.

### AdverseEventSource

An explicit source; never probe the other family.

Values: `faers`, `maude`, `vaers`.

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

### ReportId

Type: string.

### ReportUnit

The count describes reports, never exposed people or causal events.

Values: `reports`.

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

### VaersSection

One independent CDC WONDER grouping.

Values: `symptoms`, `seriousness`, `age`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.adverse_event_page/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
