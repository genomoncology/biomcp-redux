---
title: biomcp.article_recommendations/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_recommendations/3:
  its envelope fields, its result type ArticleRecommendations and the commands
  that return it."
goal: Look up the fields of a biomcp.article_recommendations/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_recommendations/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_recommendations/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_recommendations/3`.

These commands return it: [`biomcp article recommendations`](/reference/commands/article-recommendations/).

Calls that returned it:

- `biomcp article recommendations 33123537 --limit 2` ([`biomcp article recommendations`](/reference/commands/article-recommendations/))
- `biomcp article recommendations 34096690 32955176 --negative 33433946 --limit 5` ([`biomcp article recommendations`](/reference/commands/article-recommendations/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ArticleRecommend](#query-articlerecommend) | yes | Parsed query. |
| `result` | [ArticleRecommendations](#result-articlerecommendations) or null | no | Answer when available. |
| `schema` | "biomcp.article_recommendations/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ArticleRecommendations

Source-native recommendation page.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ArticleItem](#articleitem) | yes | Papers in provider order, without a local topic filter. |
| `notice` | [EvidenceNotice](#evidencenotice) or null | no | Successful zero-result explanation. |
| `scope` | string | yes | Provider's default pool, stated without a clinical relevance claim. |

## Query: ArticleRecommend

Recommendations in the provider's recent-paper pool order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum recommendations requested, in native order. |
| `negative` | array of [ArticleInput](#articleinput) | no | Up to ten negative paper identities. |
| `offset` | integer (uint32) | no | Number of stored recommendations to skip. |
| `pool` | [Text](#text) | no | Fixed recommendation pool. |
| `seeds` | array of [ArticleInput](#articleinput) | yes | One to ten positive paper identities. |

## Types

Every other type this schema uses.

### ArticleHeader

Citation metadata from PubMed or Europe PMC.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `author_count` | integer (uint32) | yes | Whole source author count. |
| `date` | [PartialDate](#partialdate) or null | no | Publication date at source precision. |
| `doi` | [Doi](#doi) or null | no | Checked DOI, when supplied. |
| `electronic_date` | [PartialDate](#partialdate) or null | no | Electronic publication date when different from the publication date. |
| `europepmc_id` | [EuropePmcId](#europepmcid) or null | no | Europe PMC source-local identity. |
| `first_author` | [Text](#text) or null | no | First author in source order. |
| `first_author_id` | [SemanticScholarAuthorId](#semanticscholarauthorid) or null | no | Exact Semantic Scholar ID for the displayed native first author, when supplied. |
| `journal` | [Text](#text) or null | no | Journal name. |
| `pmcid` | [Pmcid](#pmcid) or null | no | Checked PMC identity, when supplied. |
| `pmid` | [Pmid](#pmid) | no | Checked PubMed identity. |
| `title` | [Text](#text) | yes | Whole source title. |
| `types` | array of [PublicationType](#publicationtype) | yes | Clinical publication types. |
| `types_unknown` | boolean | no | The provider does not supply checked publication types. |

### ArticleInput

Type: string.

### ArticleItem

One original pool position, including a summary hole.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cited_by` | integer or null | no | iCite citing-paper count, only under citation ordering. |
| `contexts` | [Capped](#capped) or null | no | First citation context and the source's complete context count. |
| `found_by` | object | no | Native one-based source positions, disclosed only for multiple sources. |
| `header` | [ArticleHeader](#articleheader) or null | no | Citation metadata when accepted. |
| `influential` | boolean or null | no | Provider's influential flag; absent is distinct from false. |
| `intents` | array or null | no | Citation intents in provider order; absent means not reported. |
| `key` | [ArticleKey](#articlekey) | no | Stable checked paper identity when the provider supplies one. |
| `pmid` | [Pmid](#pmid) | no | The selected PMID, retained when summary mapping fails. |
| `sentence` | [ArticleSentence](#articlesentence) or null | no | First matching sentence in LitSense2 provider order. |

### ArticleKey

Stable identity independent of whether citation metadata was accepted.

One of:

- object: PubMed identity.
- object: PMC identity when PubMed is absent.
- object: DOI when both indexed numeric identities are absent.
- object: Europe PMC local source identity.
- object: Semantic Scholar local identity when no indexed identifier exists.

### ArticleSentence

Source-provided sentence resemblance, without a claim of clinical relevance.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `section` | [Text](#text) or null | no | Source section, when present. |
| `text` | [Text](#text) | yes | Source sentence text. |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [Text](#text) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### CitationIntent

Source-reported reason for citing a paper.

Values: `background`, `result`, `methodology`.

### Doi

Type: string.

### EuropePmcId

Type: string.

### EvidenceNotice

Explicit successful empty outcomes, never inferred from a source failure.

Values: `not_in_references`, `no_contexts`, `no_recommendations`.

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

### Pmcid

Type: string.

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

### PublicationType

Clinically meaningful PubMed publication types.

Values: `randomized_controlled_trial`, `clinical_trial`, `clinical_trial_phase_1`, `clinical_trial_phase_2`, `clinical_trial_phase_3`, `clinical_trial_phase_4`, `meta_analysis`, `systematic_review`, `review`, `practice_guideline`, `guideline`, `case_report`, `observational_study`, `multicenter_study`, `letter`, `comment`, `editorial`, `preprint`, `retracted_publication`, `retraction_notice`, `erratum`.

### Reason

Why a source could not supply data.

Values: `not_found`, `not_open`, `not_pdf`, `unsupported`, `too_large`, `rate_limited`, `unavailable`, `tls`, `proxy`, `rejected`, `bad_reply`, `deadline_exceeded`, `offline`, `no_key`, `disabled`, `replay_miss`, `syncing`, `no_join_id`.

### SemanticScholarAuthorId

Type: string.

### SemanticScholarId

Type: string.

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

This page is generated from `biomcp schema biomcp.article_recommendations/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
