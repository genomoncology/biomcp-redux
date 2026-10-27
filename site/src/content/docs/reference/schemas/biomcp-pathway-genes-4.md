---
title: biomcp.pathway_genes/4
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_genes/4: its envelope
  fields, its result type PathwayGenes and the commands that return it."
goal: Look up the fields of a biomcp.pathway_genes/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_genes/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_genes/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_genes/4`.

These commands return it: [`biomcp pathway genes`](/reference/commands/pathway-genes/).

Calls that returned it:

- `biomcp pathway genes R-HSA-5674135` ([`biomcp pathway genes`](/reference/commands/pathway-genes/))
- `biomcp pathway genes R-HSA-5673001 details` ([`biomcp pathway genes`](/reference/commands/pathway-genes/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [PathwayGenesQuery](#query-pathwaygenesquery) | yes | Parsed query. |
| `result` | [PathwayGenes](#result-pathwaygenes) or null | no | Answer when available. |
| `schema` | "biomcp.pathway_genes/4" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: PathwayGenes

Independent source-ordered distinct membership references and their observed counts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | yes | Exact source pathway owning this relation. |
| `items` | array of [PathwayMember](#pathwaymember) | yes | Distinct namespaced source references in first occurrence order. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | yes | Distinct-reference total and continuation. |
| `physical_entities` | integer (uint64) | yes | All native physical entities, never described as gene counts. |
| `reference_occurrences` | integer (uint64) | yes | All reference occurrences across physical entities before distinct-reference grouping. |

## Query: PathwayGenesQuery

Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `details` | [Details](#details) | no | Include every physical entity context for these references; never clips source text. |
| `id` | string | yes | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | no | Five complete relation rows by default, at most fifty. |
| `offset` | integer (uint32) | no | Offset within the retained selected relation. |
| `pool` | [Text](#text) | no | Immutable relation snapshot. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Details

Explicit full physical-context disclosure; omission keeps membership compact.

Values: `details`.

### HgncId

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

### PathwayHeader

Small selectable native pathway identity, without its potentially long description.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [PathwayId](#pathwayid) | yes | Checked source-owned identity. |
| `name` | [Text](#text) | yes | Whole native name. |
| `revision` | [Text](#text) or null | no | WikiPathways revision, separate from identity. |
| `species` | [PathwaySpecies](#pathwayspecies) | yes | Explicit native human species. |
| `version` | [Text](#text) or null | no | Whole native Reactome stIdVersion, separate from stable identity. |

### PathwayId

Source-owned pathway identity; equal names never merge sources.

One of:

- object: Human Reactome stable identity without a version suffix.
- object: WikiPathways identity without a revision suffix.

### PathwayMember

One source membership reference, without name-based gene inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene` | [HgncId](#hgncid) or null | no | Exact resolved gene identifier, absent when no checked join exists. |
| `name` | [Text](#text) or null | no | Native label, never interpreted as a gene symbol. |
| `node_id` | [Text](#text) or null | no | Native data-node identity; repeated references remain separate nodes. |
| `participants` | array or null | no | Complete physical contexts, disclosed only by the explicit details view. |
| `physical_context_count` | integer (uint64) | yes | Number of source physical entity contexts carrying this exact reference. |
| `reference` | [PathwaySourceRef](#pathwaysourceref) | yes | Namespaced native reference. |
| `schema_class` | [Text](#text) or null | no | Native reference schema class. |
| `source_url` | [Text](#text) or null | no | Native reference URL, including therapeutic references without a drug action. |

### PathwayPageInfo

Native page position and explicit count provenance; a short admitted page can continue.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `next_offset` | integer or null | no | Next native position, independent of returned rows; absent only at the end. |
| `offset` | integer (uint32) | yes | Native source or retained relation offset. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Snapshot identity and lifetime when retained. |
| `returned` | integer (uint32) | yes | Whole admitted rows on this page. |
| `total` | integer (uint64) | yes | Source-reported or observed relation total, as qualified by total_kind. |
| `total_kind` | [PathwayTotalKind](#pathwaytotalkind) | yes | Whether the total comes from the source or this retained collection. |

### PathwayParticipant

One native physical entity carrying a membership reference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `db_id` | integer (uint64) | yes | Native physical entity database identity, distinct from reference identity. |
| `name` | [Text](#text) | yes | Whole physical entity label, including source-stated modification context. |
| `schema_class` | [Text](#text) | yes | Native physical entity schema class, separate from reference schema class. |

### PathwaySourceRef

An open source namespace paired with its complete native identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `namespace` | string or null | no | Native namespace; never discarded during a gene join. |
| `value` | [Text](#text) | yes | Complete namespace-local identifier, including native isoform suffixes. |

### PathwaySpecies

Source-stated human species; no species inference from a title.

Values: `Homo sapiens`.

### PathwayTotalKind

Meaning of the collection total, independent of admitted rows on a page.

Values: `source`, `observed`, `distinct_references`, `data_nodes`.

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

### ReactomeId

Type: string.

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

### WikiPathwaysId

Type: string.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.pathway_genes/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
