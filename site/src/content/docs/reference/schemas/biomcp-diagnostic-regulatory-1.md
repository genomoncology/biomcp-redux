---
title: biomcp.diagnostic_regulatory/1
description: "The BioMCP 1.0.0 reply schema biomcp.diagnostic_regulatory/1: its
  envelope fields, its result type DiagnosticRegulatory and the commands that
  return it."
goal: Look up the fields of a biomcp.diagnostic_regulatory/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_regulatory/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.diagnostic_regulatory/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.diagnostic_regulatory/1`.

This call returns it.

- `biomcp get diagnostic P110020 regulatory --source fda-pma` ([`biomcp get diagnostic`](/reference/commands/get-diagnostic/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [DiagnosticGet](#query-diagnosticget) | yes | Parsed query. |
| `result` | [DiagnosticRegulatory](#result-diagnosticregulatory) or null | no | Answer when available. |
| `schema` | "biomcp.diagnostic_regulatory/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: DiagnosticRegulatory

Exact selected submission regulatory text fragments.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | array of [DiagnosticRegulatoryFragment](#diagnosticregulatoryfragment) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

## Query: DiagnosticGet

Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `bundle_revision` | [Text](#text) or null | no | Immutable stored bundle revision retained by a continuation. |
| `id` | [Text](#text) | yes | Exact native identity, including version for GTR. |
| `section` | [DiagnosticSection](#diagnosticsection) or null | no | gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card. |
| `section_limit` | integer (uint32) | no | Maximum native rows; omitted means five rows, or one regulatory fragment. |
| `section_offset` | integer (uint32) | no | Zero-based native row or fragment offset. |
| `source` | [DiagnosticGetSource](#diagnosticgetsource) | yes | Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number. |
| `supplement` | [PmaSupplement](#pmasupplement) or null | no | Exact native PMA supplement; absent selects the base row. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### DiagnosticCountUnit

Meaning of the envelope's total for a source-owned diagnostic page.

Values: `product_identities`, `test_identities`, `native_relation_rows`, `native_method_entries`.

### DiagnosticGetSource

One explicit exact diagnostic record source.

Values: `gtr`, `who-ivd`, `fda-pma`, `fda-510k`.

### DiagnosticPmaHeader

Compact exact PMA row identity; its native empty base cell is explicit.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `applicant` | string or null | no | Native applicant when supplied, including explicit empty. |
| `decision_code` | string or null | no | Native decision_code when supplied, including explicit empty. |
| `decision_date` | string or null | no | Native decision_date when supplied, including explicit empty. |
| `pma_number` | [PmaNumber](#pmanumber) | yes | Native application. |
| `supplement_number` | [PmaSupplementCell](#pmasupplementcell) or null | no | Absent differs from explicit empty base cell. |
| `trade_name` | string or null | no | Native trade_name when supplied, including explicit empty. |
| `url` | [Text](#text) | yes | Exact source record URL. |

### DiagnosticPmaRegulatory

Dated metadata belonging only to the selected submission.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `advisory_committee` | string or null | no | Native advisory_committee when supplied, including empty. |
| `advisory_committee_description` | string or null | no | Native advisory_committee_description when supplied, including empty. |
| `date_received` | string or null | no | Native date_received when supplied, including empty. |
| `fed_reg_notice_date` | string or null | no | Native fed_reg_notice_date when supplied, including empty. |
| `product_code` | string or null | no | Native product_code when supplied, including empty. |
| `supplement_reason` | string or null | no | Native supplement_reason when supplied, including empty. |
| `supplement_type` | string or null | no | Native supplement_type when supplied, including empty. |

### DiagnosticRegulatoryField

Regulatory text available in this source tier.

Values: `ao_statement`.

### DiagnosticRegulatoryFragment

One bounded regulatory fragment from the exact selected native submission.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `field` | [DiagnosticRegulatoryField](#diagnosticregulatoryfield) | yes | Native text field being continued. |
| `metadata` | [DiagnosticPmaRegulatory](#diagnosticpmaregulatory) | yes | Native metadata for this exact record, without a duplicated whole statement. |
| `offset` | integer (uint32) | yes | Fragment offset, independent of characters and supplement rows. |
| `submission` | [DiagnosticPmaHeader](#diagnosticpmaheader) | yes | Exact selected native record and its dated regulatory metadata. |
| `text` | string | no | Complete unmodified text fragment; omitted when the source statement is absent. |
| `total` | integer (uint32) | yes | Total mechanical fragments. |

### DiagnosticSection

Source-supported diagnostic sections; no all section.

Values: `genes`, `conditions`, `methods`, `regulatory`, `supplements`.

### PageInfo

Search counts after filtering and deduplication.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Skipped results. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Optional pooled snapshot. |
| `returned` | integer (uint32) | yes | Results returned on this page. |
| `total` | integer (uint64) | yes | Total matching identities held by this snapshot. |
| `truncated` | boolean or null | no | Matching source count exceeded the retrieved window. |

### PmaNumber

Type: string.

### PmaSupplement

Type: string.

### PmaSupplementCell

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

This page is generated from `biomcp schema biomcp.diagnostic_regulatory/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
