---
title: biomcp.article_evidence/2
description: "The BioMCP 1.0.0 reply schema biomcp.article_evidence/2: its
  envelope fields, its result type ArticleEvidenceResult and the commands that
  return it."
goal: Look up the fields of a biomcp.article_evidence/2 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_evidence/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_evidence/2`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_evidence/2`.

These commands return it: [`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/).

Calls that returned it:

- `biomcp article citation-evidence 33123537 17611497` ([`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/))
- `biomcp article citation-evidence 33123537 17611497 --fulltext` ([`biomcp article citation-evidence`](/reference/commands/article-citation-evidence/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ArticleEvidence](#query-articleevidence) | yes | Parsed query. |
| `result` | [ArticleEvidenceResult](#result-articleevidenceresult) or null | no | Answer when available. |
| `schema` | "biomcp.article_evidence/2" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ArticleEvidenceResult

Independent graph and optional open-text evidence parts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fulltext` | [Section2](#section2) or null | no | The citing paper's own explicit reference paragraphs. |
| `semantic_scholar` | [Section](#section) | yes | Source-reported citation contexts. |

## Query: ArticleEvidence

Read every source context and optional explicit full-text citation paragraph.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cited` | [ArticleInput](#articleinput) | yes | Cited PMID, PMC identity or DOI. |
| `citing` | [ArticleInput](#articleinput) | yes | Citing PMID, PMC identity or DOI. |
| `fulltext` | boolean | no | Include the citing paper's own explicit reference paragraphs. |
| `offset` | integer (uint32) | no | Context fragment position returned by the advancing step. |
| `paragraph_offset` | integer (uint32) | no | Full-text fragment position returned by the advancing step. |
| `pool` | [Text](#text) | no | Stored checked source scan and original hit positions. |

## Types

Every other type this schema uses.

### ArticleInput

Type: string.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [EvidenceContext](#evidencecontext) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Capped2

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [EvidenceParagraph](#evidenceparagraph) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### CitationContexts

Source edge metadata and a bounded advancing context page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `complete` | boolean | yes | Completion means a terminal source page was reached. |
| `contexts` | [Capped](#capped) | yes | Every context remains accessible through the advancing step. |
| `influential` | boolean or null | no | Missing is distinct from false. |
| `intents` | array or null | no | Source-reported citation intents. |
| `notice` | [EvidenceNotice](#evidencenotice) or null | no | Successful empty-source explanation. |
| `source_cursor` | integer or null | no | Native next source offset when a scan is incomplete. |

### CitationIntent

Source-reported reason for citing a paper.

Values: `background`, `result`, `methodology`.

### CitationParagraphs

Exact JATS reference paragraphs independent of the graph result.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `license` | [License](#license) | yes | Original publisher license category. |
| `paragraphs` | [Capped2](#capped2) | yes | Original paragraph fragments and complete fragment count. |

### EvidenceContext

One advancing piece of an original source citation context.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `character_offset` | integer (uint32) | yes | Unicode character position within that context. |
| `position` | integer (uint32) | yes | One-based original source context position. |
| `text` | string | yes | Whole context, or an exact advancing fragment when the context exceeds a page. |

### EvidenceNotice

Explicit successful empty outcomes, never inferred from a source failure.

Values: `not_in_references`, `no_contexts`, `no_recommendations`.

### EvidenceParagraph

One source paragraph fragment with its own exact outline location.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `character_offset` | integer (uint32) | yes | Unicode character position within the original paragraph. |
| `markdown` | string (markdown) | yes | Preserved source Markdown. |
| `part` | integer (uint32) | yes | Exact semantic outline part. |
| `position` | integer (uint32) | yes | One-based original body paragraph position. |

### License

License reported by the bucket metadata record.

Values: `cc_by`, `cc_by_nc`, `cc_by_sa`, `cc_by_nc_sa`, `cc_by_nd`, `cc_by_nc_nd`, `cc0`, `public_domain`, `tdm`, `other`.

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

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_evidence/2` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
