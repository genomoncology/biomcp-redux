---
title: biomcp.protein_sequence/1
description: "The BioMCP 1.0.0 reply schema biomcp.protein_sequence/1: its
  envelope fields, its result type ProteinSequenceResult and the commands that
  return it."
goal: Look up the fields of a biomcp.protein_sequence/1 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_sequence/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_sequence/1`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_sequence/1`.

These commands return it: [`biomcp protein sequence`](/reference/commands/protein-sequence/).

Calls that returned it:

- `biomcp protein sequence P00533` ([`biomcp protein sequence`](/reference/commands/protein-sequence/))
- `biomcp protein sequence P00533 --fresh` ([`biomcp protein sequence`](/reference/commands/protein-sequence/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ProteinSequenceQuery](#query-proteinsequencequery) | yes | Parsed query. |
| `result` | [ProteinSequenceResult](#result-proteinsequenceresult) or null | no | Answer when available. |
| `schema` | "biomcp.protein_sequence/1" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ProteinSequenceResult

One directly returned source sequence, with exact identity and audit metadata.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `sequence` | [ProteinSequence](#proteinsequence) | yes | Native sequence record; never constructed from another entry. |

## Query: ProteinSequenceQuery

Read one complete native sequence for an exact source accession.
No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `isoform` | [UniProtIsoform](#uniprotisoform) | yes | Exact isoform or base accession; no canonical suffix is synthesized. |

## Types

Every other type this schema uses.

### AminoAcids

Type: string.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

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

### ProteinAudit

Native source audit metadata, independent from local retrieval metadata.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `entry_version` | integer or null | no | Native entry version, never an isoform version. |
| `first_public` | [PartialDate](#partialdate) or null | no | Native first-public date. |
| `last_annotation_update` | [PartialDate](#partialdate) or null | no | Native annotation update date. |
| `last_sequence_update` | [PartialDate](#partialdate) or null | no | Native sequence update date. |
| `release` | [Text](#text) or null | no | Native release when supplied. |
| `sequence_version` | integer or null | no | Native sequence version when supplied. |

### ProteinSequence

A directly returned sequence with exact returned identity and audit facts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | yes | Exact returned accession; never replaced by the requested base entry. |
| `audit` | [ProteinAudit](#proteinaudit) | yes | Native audit facts for this record. |
| `crc64` | [Text](#text) or null | no | Native checksum when supplied. |
| `length` | integer (uint32) | yes | Native sequence length. |
| `md5` | [Text](#text) or null | no | Native checksum when supplied. |
| `sequence` | [AminoAcids](#aminoacids) | yes | Source residues, never constructed from another entry. |

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

### UniProtIsoform

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_sequence/1` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
