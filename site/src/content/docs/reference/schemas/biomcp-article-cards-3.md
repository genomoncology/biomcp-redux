---
title: biomcp.article_cards/3
description: "The BioMCP 1.0.0 reply schema biomcp.article_cards/3: its envelope
  fields, its result type ArticleCards and the commands that return it."
goal: Look up the fields of a biomcp.article_cards/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_cards/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.article_cards/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.article_cards/3`.

These commands return it: [`biomcp get article`](/reference/commands/get-article/).

Calls that returned it:

- `biomcp get article 33123537` ([`biomcp get article`](/reference/commands/get-article/))
- `biomcp get article 33123537 indexing` ([`biomcp get article`](/reference/commands/get-article/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ArticleGet](#query-articleget) | yes | Parsed query. |
| `result` | [ArticleCards](#result-articlecards) or null | no | Answer when available. |
| `schema` | "biomcp.article_cards/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ArticleCards

Cards in caller order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ArticleCardItem](#articlecarditem) | yes | Per-input answers. |

## Query: ArticleGet

Get one to twenty article inputs in caller order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of [ArticleInput](#articleinput) | yes | PubMed, PMC or DOI identifiers in input order. |
| `section` | array of [ArticleSectionWord](#articlesectionword) | no | Requested section words; trailing words and --section share this field. |

## Types

Every other type this schema uses.

### AbstractPart

One source-labelled abstract paragraph.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `label` | [Text](#text) or null | no | Source label, when supplied. |
| `text` | [Markdown](#markdown) | yes | Whole paragraph text. |

### AnnotatedEntity

One normalized entity with its mention count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `count` | integer (uint64) | yes | Number of annotation mentions. |
| `entrez_id` | [NcbiGeneId](#ncbigeneid) or null | no | Gene identity, including a variant's associated gene. |
| `hgvs` | [Text](#text) or null | no | Source HGVS expression for a variant. |
| `kind` | [AnnotationKind](#annotationkind) | yes | Source concept type. |
| `mesh` | [Text](#text) or null | no | Source MeSH identity for a disease or chemical. |
| `name` | [Text](#text) | yes | Source normalized name. |
| `rsid` | [Text](#text) or null | no | Source reference SNP identity for a variant. |

### AnnotationKind

Normalized annotation category in deterministic tie order.

Values: `gene`, `variant`, `disease`, `chemical`.

### Article

Article facts with at least one checked identity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `abstract_parts` | array of [AbstractPart](#abstractpart) | yes | Whole abstract, preserving labels and source order. |
| `authors` | [Capped](#capped) | yes | Authors in source order. |
| `corrections` | array of [Correction](#correction) | yes | Related correction notices or originals. |
| `date` | [PartialDate](#partialdate) or null | no | Publication date at source precision. |
| `doi` | [Doi](#doi) | no | DOI assigned to this article. |
| `electronic_date` | [PartialDate](#partialdate) or null | no | Electronic publication date when different from the publication date. |
| `europepmc_id` | [EuropePmcId](#europepmcid) | no | Europe PMC identity when this card comes from its core record. |
| `journal` | [Text](#text) or null | no | Journal name. |
| `pmcid` | [Pmcid](#pmcid) | no | PMC identity assigned to this article. |
| `pmid` | [Pmid](#pmid) | no | PubMed identity. |
| `title` | [Text](#text) | yes | Whole title with inline text preserved. |
| `types` | array of [PublicationType](#publicationtype) | yes | Every clinically meaningful source publication type. |

### ArticleCardItem

Answer to one input; shared DOIs may expand to several consecutive items.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `answer` | [ArticleResolution](#articleresolution) or null | no | Absent only when a provider failed. |
| `input` | [ArticleInput](#articleinput) | yes | Original selector. |
| `more` | array of [ArticleSection](#articlesection) | no | Applicable sections omitted by this request. |
| `sections` | [ArticleSections](#articlesections) or null | no | Requested section states for a resolved card. |
| `status` | [Status](#status) | yes | This card's outcome. |

### ArticleInput

Type: string.

### ArticleMatch

Identity evidence for an article card.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | [ArticleMatchKind](#articlematchkind) | yes | Namespace used to resolve the card. |

### ArticleMatchKind

Identifier used to resolve this article.

Values: `pmid`, `pmcid`, `doi`, `europepmc_id`.

### ArticleResolution

Result of resolving an article selector.

One of:

- object: A record whose own identity agrees with the input.
- object: The source confirmed no matching record.

### ArticleSection

A landed article section.

Values: `fulltext`, `supplements`, `annotations`, `tldr`, `citations`, `indexing`, `pdf`.

### ArticleSectionWord

Article section request including all landed sections.

Values: `annotations`, `fulltext`, `supplements`, `tldr`, `citations`, `indexing`, `pdf`, `all`.

### ArticleSections

Landed article sections with independent source states.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `annotations` | [Section7](#section7) or null | no | Normalized PubTator3 entities. |
| `citations` | [Section6](#section6) or null | no | Citation counts from NIH iCite. |
| `fulltext` | [Section4](#section4) or null | no | Licensed PMC body outline. |
| `indexing` | [Section2](#section2) or null | no | Index terms from the card's own EFetch citation. |
| `pdf` | [Section](#section) or null | no | Publisher PDF link; no file is downloaded. |
| `supplements` | [Section3](#section3) or null | no | JATS supplementary files confirmed by the bucket record. |
| `tldr` | [Section5](#section5) or null | no | Machine-written Semantic Scholar summary. |

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [Text](#text) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Capped2

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [AnnotatedEntity](#annotatedentity) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Citations

Independently reported iCite counts; absent lists are never confirmed zeros.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cited_by` | integer or null | no | Number of citing PubMed papers. |
| `cited_by_clinical` | integer or null | no | Number of distinct clinical citing papers, when the list was present. |
| `rcr` | [FiniteNumber](#finitenumber) or null | no | Relative citation ratio, only when confirmed non-provisional. |
| `references` | integer or null | no | Number of distinct references, when the list was present. |

### Correction

One correction relationship supplied by PubMed.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | [CorrectionKind](#correctionkind) | yes | Correction category. |
| `pmid` | [Pmid](#pmid) | no | Related PMID, when supplied. |
| `relation` | [CorrectionRelation](#correctionrelation) | yes | Whether the related record is a notice or original. |

### CorrectionKind

Source correction relationship kind.

Values: `retraction`, `erratum`, `expression_of_concern`.

### CorrectionRelation

Direction of a source correction relationship.

Values: `notice`, `original`.

### Doi

Type: string.

### EuropePmcId

Type: string.

### FigureCaption

Figure text in document order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `caption` | [Markdown](#markdown) | yes | Source caption. |
| `label` | [Text](#text) | yes | Source label. |

### FileKind

Supplement format from an explicit JATS MIME declaration, with suffix fallback.

Values: `xlsx`, `csv`, `tsv`, `txt`, `pdf`, `other`.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### FullText

Bucket license and body outline; processing version is not publication chronology.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `figures` | array of [FigureCaption](#figurecaption) | yes | Source figure captions in document order. |
| `license` | [License](#license) | yes | Selected bucket license. |
| `manuscript` | boolean | no | Whether this selected version is an author manuscript. |
| `parts` | array of [TextPart](#textpart) | yes | Bounded body parts in reading order. |
| `processing_version` | integer (uint32) | no | Highest numeric processing version. |

### Indexing

Index terms and author identities supplied by this article's EFetch citation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `affiliations` | [Capped](#capped) | yes | First ten author affiliations and the complete count. |
| `keywords` | array of [Text](#text) | yes | Author keywords in source order. |
| `mesh` | array of [MeshHeading](#meshheading) | yes | MeSH headings in source order. |
| `orcids` | array of [Text](#text) | yes | Author ORCID identifiers, as supplied by the source. |

### License

License reported by the bucket metadata record.

Values: `cc_by`, `cc_by_nc`, `cc_by_sa`, `cc_by_nc_sa`, `cc_by_nd`, `cc_by_nc_nd`, `cc0`, `public_domain`, `tdm`, `other`.

### Markdown

Nonempty text with surrounding Unicode whitespace removed.

Type: string (markdown).

### MeshHeading

A MeSH descriptor and its qualifiers retain their independent major-topic flags.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `descriptor` | [Text](#text) | yes | Source descriptor. |
| `major` | boolean | yes | Descriptor major-topic flag. |
| `qualifiers` | array of [MeshQualifier](#meshqualifier) | yes | Source qualifiers. |

### MeshQualifier

A qualifier attached to a MeSH descriptor.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `major` | boolean | yes | Qualifier major-topic flag. |
| `name` | [Text](#text) | yes | Source qualifier. |

### NcbiGeneId

Type: string.

### OpenAccessStatus

Open-access status supplied by the paper-link provider.

Values: `gold`, `hybrid`, `bronze`, `green`, `closed`.

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

### PdfLink

Source-reported PDF link. Reading this section never downloads the file.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `license` | [License](#license) | yes | Normalized license; other grants no reuse rights. |
| `status` | [OpenAccessStatus](#openaccessstatus) | yes | Source access status. |
| `url` | [Text](#text) | yes | Source publisher URL. |

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

### Section3

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section4

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section5

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section6

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section7

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

### SupplementFile

A JATS supplement confirmed by the same processing record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `caption` | [Markdown](#markdown) or null | no | JATS caption when present. |
| `kind` | [FileKind](#filekind) | yes | Checked format. |
| `label` | [Text](#text) or null | no | JATS label when present. |
| `name` | [Text](#text) | yes | Exact source filename. |

### Supplements

Available supplements with their owning article license.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `files` | array of [SupplementFile](#supplementfile) | yes | Exact JATS/record intersection. |
| `license` | [License](#license) | yes | Article license from the record. |

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### TextPart

One independently readable bounded part.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | [TextPartKind](#textpartkind) | yes | Source structure kind. |
| `number` | integer (uint32) | yes | One-based reader selector. |
| `title` | [Text](#text) | yes | Source section title or table label. |
| `tokens` | integer (uint32) | yes | Compact serialized text bytes divided by four, rounded up. |

### TextPartKind

Body part kind.

Values: `section`, `table`.

### Tldr

Machine-written source summary with independently reported counts.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `cited_by` | integer or null | no | Total source citation count, when supplied. |
| `influential_cited_by` | integer or null | no | Source influential citation count, when supplied. |
| `machine_written` | boolean | yes | Explicitly identifies generated prose. |
| `model` | [Text](#text) | yes | Source model version. |
| `text` | [Text](#text) | yes | Complete one-sentence source summary. |

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.article_cards/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
