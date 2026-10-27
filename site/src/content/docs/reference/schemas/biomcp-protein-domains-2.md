---
title: biomcp.protein_domains/2
description: "The BioMCP 1.0.0 reply schema biomcp.protein_domains/2: its
  envelope fields, its result type ProteinDomains and the commands that return
  it."
goal: Look up the fields of a biomcp.protein_domains/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_domains/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_domains/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_domains/2`.

These commands return it: [`biomcp protein domains`](/reference/commands/protein-domains/).

Calls that returned it:

- `biomcp protein domains P15056` ([`biomcp protein domains`](/reference/commands/protein-domains/))
- `biomcp protein domains P15056 --limit 2` ([`biomcp protein domains`](/reference/commands/protein-domains/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ProteinDomainsQuery](#query-proteindomainsquery) | yes | Parsed query. |
| `result` | [ProteinDomains](#result-proteindomains) or null | no | Answer when available. |
| `schema` | "biomcp.protein_domains/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ProteinDomains

Whole native rows with the independent unfiltered source count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinDomainMatch](#proteindomainmatch) | yes | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | yes | Native source total before type selection or exact participant admission. |

## Query: ProteinDomainsQuery

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | [Text](#text) | no | Retained collection identity. |
| `type` | [Text](#text) | no | Exact native InterPro type, not a normalized domain label. |

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

### Bound

A bounded source range; equal min and max denote an exact boundary.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `max` | integer or null | no |  |
| `min` | integer or null | no |  |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### InterProId

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

### ProteinDomainFragment

One native match fragment after coordinate conversion.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `dc_status` | [Text](#text) or null | no | Literal discontinuity status supplied by InterPro. |
| `span` | [UncertainSpan0](#uncertainspan0) | yes | Converted interbase span with uncertainty retained. |

### ProteinDomainLocation

One native InterPro location group retaining all discontinuous fragments.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fragments` | array of [ProteinDomainFragment](#proteindomainfragment) | no | All native fragments; never reduced to a min/max hull. |

### ProteinDomainMatch

One native integrated InterPro match; not every entry type is a domain.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source record and release facts. |
| `entry_type` | [Text](#text) | yes | Literal source entry type, including family, site and repeat. |
| `id` | [InterProId](#interproid) | yes | Integrated entry accession, distinct from member signatures. |
| `locations` | array of [ProteinDomainLocation](#proteindomainlocation) | no | Every native location group in source order. |
| `member_signatures` | array of [ProteinMemberSignature](#proteinmembersignature) | no | Source member signatures remain distinct from integrated entries. |
| `name` | [Text](#text) | yes | Native integrated name. |
| `protein` | [UniProtIsoform](#uniprotisoform) | yes | Exact native protein identity. |
| `protein_length` | integer or null | no | Source protein length when supplied. |
| `taxon` | integer or null | no | Source taxon when supplied. |

### ProteinMemberSignature

A member database signature, not an integrated IPR entry.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [Text](#text) | yes | Native signature accession. |
| `database` | [Text](#text) | yes | Native member database name. |
| `name` | [Text](#text) or null | no | Native member signature name. |

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

### UncertainSpan0

Zero-based interbase span retaining uncertain boundary ranges.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | [Bound](#bound) | yes |  |
| `start` | [Bound](#bound) | yes |  |

### UniProtIsoform

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_domains/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
