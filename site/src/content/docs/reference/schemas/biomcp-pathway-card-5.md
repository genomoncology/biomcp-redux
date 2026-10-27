---
title: biomcp.pathway_card/5
description: "The BioMCP 1.0.0 reply schema biomcp.pathway_card/5: its envelope
  fields, its result type PathwayCard and the commands that return it."
goal: Look up the fields of a biomcp.pathway_card/5 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway_card/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.pathway_card/5`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.pathway_card/5`.

These commands return it: [`biomcp get pathway`](/reference/commands/get-pathway/).

Calls that returned it:

- `biomcp get pathway R-HSA-9652817 summary` ([`biomcp get pathway`](/reference/commands/get-pathway/))
- `biomcp get pathway R-HSA-9652817` ([`biomcp get pathway`](/reference/commands/get-pathway/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [PathwayGet](#query-pathwayget) | yes | Parsed query. |
| `result` | [PathwayCard](#result-pathwaycard) or null | no | Answer when available. |
| `schema` | "biomcp.pathway_card/5" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: PathwayCard

One checked source pathway with supported disclosures and whole native text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description_truncated` | boolean or null | no | Whether the source itself shortened its description, rather than the client. |
| `events` | [Section2](#section2) or null | no | Requested bounded event page, including unresolved integer references. |
| `genes` | [Section](#section) or null | no | Requested bounded membership page, including its explicit source state. |
| `pathway` | [Pathway](#pathway) | yes | Source-native identity, species, versions and requested description. |
| `release` | [Text](#text) or null | no | Native Reactome database release, separate from pathway version. |
| `sections` | array of [PathwaySection](#pathwaysection) | yes | Source-supported named sections; WikiPathways excludes events. |

## Query: PathwayGet

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fresh` | boolean | no | Request a new source observation. |
| `id` | string | yes | Exact human Reactome or WikiPathways identity without version suffixes. |
| `pool` | [Text](#text) | no | Retained snapshot, when continuing a source relation. |
| `section` | [PathwaySection](#pathwaysection) | no | Omit for the card; WikiPathways has summary and genes sections. |
| `section_limit` | integer (uint32) | no | Bounded initial section size; each all section has its own first page. |
| `section_offset` | integer (uint32) | no | Native relation offset for a selected genes or events section. |

## Types

Every other type this schema uses.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

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

### Pathway

Neutral pathway facts, without display sections or source calls.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cited_in` | string or null | no | WikiPathways catalog citedIn cell, including a native empty cell. |
| `description` | string or null | no | Complete native description when present; empty is distinct from absent. |
| `id` | [PathwayId](#pathwayid) | yes | Checked source-owned identity. |
| `name` | [Text](#text) | yes | Whole native pathway name. |
| `references` | array or null | no | Native pathway-level literature references, separate from each summation. |
| `revision` | [Text](#text) or null | no | WikiPathways native revision, separate from identity. |
| `species` | [PathwaySpecies](#pathwayspecies) | yes | Explicit source-stated human species. |
| `summations` | array or null | no | Native Reactome summation objects in source order; not concatenated. |
| `version` | [Text](#text) or null | no | Whole native Reactome stIdVersion, separate from stable identity. |

### PathwayEventRef

An event object or unresolved integer reference; never the parent's inferred ID.

One of:

- object: Native unresolved integer database reference.
- object: Native event object; fields are absent only when omitted by the source.

### PathwayEvents

Independent source-ordered page of whole native references.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | yes | Exact source pathway owning this relation. |
| `items` | array of [PathwayEventRef](#pathwayeventref) | yes | Whole native references; unresolved identities remain visible. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | yes | Relation-specific count provenance and continuation. |

### PathwayGenes

Independent source-ordered distinct membership references and their observed counts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | yes | Exact source pathway owning this relation. |
| `items` | array of [PathwayMember](#pathwaymember) | yes | Distinct namespaced source references in first occurrence order. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | yes | Distinct-reference total and continuation. |
| `physical_entities` | integer (uint64) | yes | All native physical entities, never described as gene counts. |
| `reference_occurrences` | integer (uint64) | yes | All reference occurrences across physical entities before distinct-reference grouping. |

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

### PathwayLiteratureRef

A native pathway-level literature reference; never inferred from prose.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `db_id` | integer (uint64) | yes | Native literature database reference. |
| `pubmed` | [Pmid](#pmid) or null | no | Native PubMed identifier when supplied. |
| `title` | [Text](#text) or null | no | Native title when supplied. |
| `url` | [Text](#text) or null | no | Source record URL when supplied. |

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

### PathwaySection

Supported source-owned pathway disclosures.

Values: `summary`, `genes`, `events`, `all`.

### PathwaySourceRef

An open source namespace paired with its complete native identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `namespace` | string or null | no | Native namespace; never discarded during a gene join. |
| `value` | [Text](#text) | yes | Complete namespace-local identifier, including native isoform suffixes. |

### PathwaySpecies

Source-stated human species; no species inference from a title.

Values: `Homo sapiens`.

### PathwaySummation

One native summation, with its own database identity and whole text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `db_id` | integer (uint64) | yes | Native summation database identity, not the pathway identity. |
| `text` | string | yes | Complete native text; no sentence clipping or reference reassignment. |

### PathwayTotalKind

Meaning of the collection total, independent of admitted rows on a page.

Values: `source`, `observed`, `distinct_references`, `data_nodes`.

### Pmid

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

### ReactomeId

Type: string.

### Reason

Why a source could not supply data.

Values: `not_found`, `not_open`, `not_pdf`, `unsupported`, `too_large`, `rate_limited`, `unavailable`, `tls`, `proxy`, `rejected`, `bad_reply`, `deadline_exceeded`, `offline`, `no_key`, `disabled`, `replay_miss`, `syncing`, `no_join_id`.

### Section

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section2

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

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

### SourceRecord

The source's own entry for an entity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Native source identifier. |
| `url` | [Text](#text) or null | no | Link to that entry. |

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

This page is generated from `biomcp schema biomcp.pathway_card/5` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
