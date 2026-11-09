---
title: Reply envelope and shared types
description: The fields every BioMCP 1.0.0 JSON reply carries, and every type
  that more than one reply schema uses.
goal: Look up an envelope field or a type that several BioMCP replies share.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: Shared types
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. Every reply has the same outer fields, the **[envelope](/reference/schemas/types/)**: `schema`, `status`, `query`, `result`, `sources` and `next`. This page defines those fields and every type that more than one reply uses. Each [reply schema](/reference/schemas/) page shows a real reply and the types only that reply has.

## Envelope

One reply to one command. `schema` names the reply schema and its version, which decide the types of `result` and `query`. [Reply envelope values](/reference/envelope/) gives the meaning of each `status`, source state and reason.

| Field | Type | Meaning |
| --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | A problem with BioMCP's local cache: `busy`, `database` or `version`. The sources may still have answered. Optional. |
| `next` | list of [Step](#step) | Commands you can run next, such as the next page or another section. Optional. |
| `page` | [PageInfo](#pageinfo) or null | For a search page: how many results it skipped, returned and matched in all. Optional. |
| `problems` | list of [Problem](#problem) | Fields BioMCP could not accept from a source record, each with its reason. Optional. |
| `query` | the query type of each schema | The request as BioMCP read it from the command's arguments. |
| `result` | the result type of each schema, or null | The data the command found. Missing when there is none to return. Optional. |
| `schema` | string | The reply schema ID, such as `biomcp.gene_cards/4`. |
| `sources` | list of [SourceOutcome](#sourceoutcome) | One entry for each source the command asked, with what happened. Optional. |
| `status` | [Status](#status) | How the command went: `complete`, `partial`, `absent` or `failed`. |

## Words in the field descriptions

The field descriptions come from the program and use a few words in a fixed sense:

| Word | Meaning |
| --- | --- |
| native | As the source gives it: the source's own value, name or order, not one BioMCP made. |
| literal | Taken character for character. BioMCP does not interpret or correct it. |
| whole | Not cut. A whole text, row or record holds everything the source sent. |
| complete | Not cut, as whole. |
| exact | Matched or kept character for character, with no fuzzy matching. |
| checked | BioMCP has validated the value's form, such as an identifier's pattern. |
| bounded | Has a fixed maximum size; a longer list or file is cut or refused. |
| retained | Kept from an earlier reply, such as a stored search or report, so later pages read the same data. |
| opaque | An identifier to pass back to BioMCP as it is. It has no meaning to parse. |
| provider | The source: the service or [bundle](/local-data/) the data comes from. |
| typed | One value from a fixed list, such as a reason code. |

## Types

Each type below appears in more than one reply. A generic type holds another type, named T; each schema page says what T is, as in "Section of SupplementContent".

### Accession

Type: string.

### AcmgAmp2015Code

Checked AcmgAmp2015 codes.

Values: `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### AdverseEventGet

Exact native report identity and an optional independently paged section.

| Field | Type | Meaning |
| --- | --- | --- |
| `drug_index` | integer or null | Absolute source-array index; required only for identifier sections. Optional. |
| `id` | [ReportId](#reportid) | Published report ID with no source prefix. |
| `section` | [AdverseEventSection](#adverseeventsection) or null | FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only. Optional. |
| `section_limit` | integer (uint32) | Whole source rows per section page. Optional. |
| `section_offset` | integer (uint32) | Offset within the selected native section. Optional. |
| `snapshot` | [Text](#text) or null | Checked retained report snapshot for provider-free continuation. Optional. |
| `source` | `"faers"` or `"maude"` | FAERS by default; MAUDE is explicit. Optional. |

### AdverseEventHeader

Small source report header without a drug title, flattened lists or count buckets.

| Field | Type | Meaning |
| --- | --- | --- |
| `event_type` | string or null | Native event_type string; present empty values remain present. Optional. |
| `first_received` | string or null | Native first_received string; present empty values remain present. Optional. |
| `id` | [ReportId](#reportid) | Exact published source identity. |
| `serious` | string or null | Native serious string; present empty values remain present. Optional. |
| `source` | [AdverseEventSource](#adverseeventsource) | Explicit report family. |
| `version` | string or null | Native version string; present empty values remain present. Optional. |

### AdverseEventRecords

One complete-row page tied to the retained report snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Native section offset. |
| `records` | list of T | Source ordered whole rows. |
| `total` | integer (uint64) | Complete selected section length, not report or patient count. |

### AdverseEventSearch

Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Native report headers or VAERS rows per page; default five. Optional. |
| `offset` | integer (uint32) | Native row offset. VAERS continuation requires its retained snapshot. Optional. |
| `reaction` | [Text](#text) or null | FAERS MedDRA term phrase match, not exact equality. Optional. |
| `received_from` | [PartialDate](#partialdate) or null | Inclusive first FDA receipt lower bound, YYYY-MM-DD. Optional. |
| `received_to` | [PartialDate](#partialdate) or null | Inclusive first FDA receipt upper bound, YYYY-MM-DD. Optional. |
| `section` | [VaersSection](#vaerssection) or null | Independent VAERS table; omitted means reported symptoms. Optional. |
| `serious` | [AdverseEventSerious](#adverseeventserious) or null | FAERS report flag: death or hospitalization; never a reaction outcome. Optional. |
| `snapshot` | [Text](#text) or null | Retained VAERS table for provider-free continuation. Optional. |
| `source` | [AdverseEventSource](#adverseeventsource) | FAERS by default; VAERS accepts only exact supported vaccine names. Optional. |
| `text` | [Text](#text) | Literal drug text, MAUDE device phrase or exact supported VAERS vaccine. |

### AdverseEventSection

One source-owned section; no all or reporter section.

| Value | Meaning |
| --- | --- |
| `drugs` | Independent native drug rows. |
| `reactions` | Independent native reaction rows. |
| `unii` | Native UNII strings. |
| `rxcui` | Native RxCUI strings. |
| `product-ndc` | Native product NDC strings. |
| `package-ndc` | Native package NDC strings. |
| `spl-id` | Native SPL IDs. |
| `spl-set-id` | Native SPL set IDs. |
| `application-number` | Native application numbers. |
| `devices` | Native device rows. |
| `narrative` | Complete native narrative rows. |

### AdverseEventSerious

Report-level seriousness filter, independent of reaction outcome.

| Value | Meaning |
| --- | --- |
| `death` | Native report death flag. |
| `hospitalization` | Native report hospitalization flag. |

### AdverseEventSource

An explicit source; never probe the other family.

| Value | Meaning |
| --- | --- |
| `faers` | Drug event reports. |
| `maude` | Device event reports. |
| `vaers` | Explicit CDC WONDER vaccine aggregate tables. |

### Allele

Source-stated interbase replacement; equality never claims normalized equivalence.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate` | [Sequence](#sequence) |  |
| `location` | [SequenceLocation](#sequencelocation) |  |
| `reference` | [Sequence](#sequence) |  |

### AminoAcids

Type: string.

### AmpAscoCap2017LevelCode

Checked AmpAscoCap2017Level codes.

Values: `level_a`, `level_b`, `level_c`, `level_d`.

### AmpAscoCap2017TierCode

Checked AmpAscoCap2017Tier codes.

Values: `tier_i`, `tier_ii`, `tier_iii`, `tier_iv`.

### ArticleHeader

Citation metadata from PubMed or Europe PMC.

| Field | Type | Meaning |
| --- | --- | --- |
| `author_count` | integer (uint32) | Whole source author count. |
| `date` | [PartialDate](#partialdate) or null | Publication date at source precision. Optional. |
| `doi` | [Doi](#doi) or null | Checked DOI, when supplied. Optional. |
| `electronic_date` | [PartialDate](#partialdate) or null | Electronic publication date when different from the publication date. Optional. |
| `europepmc_id` | [EuropePmcId](#europepmcid) or null | Europe PMC source-local identity. Optional. |
| `first_author` | [Text](#text) or null | First author in source order. Optional. |
| `first_author_id` | [SemanticScholarAuthorId](#semanticscholarauthorid) or null | Exact Semantic Scholar ID for the displayed native first author, when supplied. Optional. |
| `journal` | [Text](#text) or null | Journal name. Optional. |
| `pmcid` | [Pmcid](#pmcid) or null | Checked PMC identity, when supplied. Optional. |
| `pmid` | [Pmid](#pmid) | Checked PubMed identity. Optional. |
| `title` | [Text](#text) | Whole source title. |
| `types` | list of [PublicationType](#publicationtype) | Clinical publication types. |
| `types_unknown` | boolean | The provider does not supply checked publication types. Optional. |

### ArticleInput

Type: string.

### ArticleItem

One original pool position, including a summary hole.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_by` | integer or null | iCite citing-paper count, only under citation ordering. Optional. |
| `contexts` | [Capped](#capped) of [Text](#text) or null | First citation context and the source's complete context count. Optional. |
| `found_by` | object | Native one-based source positions, disclosed only for multiple sources. Optional. |
| `header` | [ArticleHeader](#articleheader) or null | Citation metadata when accepted. Optional. |
| `influential` | boolean or null | Provider's influential flag; absent is distinct from false. Optional. |
| `intents` | list of [CitationIntent](#citationintent) or null | Citation intents in provider order; absent means not reported. Optional. |
| `key` | [ArticleKey](#articlekey) | Stable checked paper identity when the provider supplies one. Optional. |
| `pmid` | [Pmid](#pmid) | The selected PMID, retained when summary mapping fails. Optional. |
| `sentence` | [ArticleSentence](#articlesentence) or null | First matching sentence in LitSense2 provider order. Optional. |

### ArticleKey

Stable identity independent of whether citation metadata was accepted.

One of these forms:

- `namespace` is `"pubmed"`: PubMed identity. Fields: `id` ([Pmid](#pmid)).
- `namespace` is `"pmc"`: PMC identity when PubMed is absent. Fields: `id` ([Pmcid](#pmcid)).
- `namespace` is `"doi"`: DOI when both indexed numeric identities are absent. Fields: `id` ([Doi](#doi)).
- `namespace` is `"europepmc"`: Europe PMC local source identity. Fields: `id` ([EuropePmcId](#europepmcid)).
- `namespace` is `"semantic_scholar"`: Semantic Scholar local identity when no indexed identifier exists. Fields: `id` ([SemanticScholarId](#semanticscholarid)).

### ArticleSaved

Explicit saved article file, owned by the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Exact byte count. |
| `parts` | integer or null | Number of ordered text parts; absent for original supplement bytes. Optional. |
| `path` | [Text](#text) | Absolute caller destination. |
| `sha256` | [Text](#text) | SHA-256 of the saved bytes. |
| `url` | [Text](#text) | Source URL for the original material. |

### ArticleSentence

Source-provided sentence resemblance, without a claim of clinical relevance.

| Field | Type | Meaning |
| --- | --- | --- |
| `section` | [Text](#text) or null | Source section, when present. Optional. |
| `text` | [Text](#text) | Source sentence text. |

### AssertionAssessment

Source assessment labels and identifiers remain independent.

| Field | Type | Meaning |
| --- | --- | --- |
| `criterion` | [Text](#text) | Native criterion code. |
| `criterion_id` | [Text](#text) or null | Native criterion identifier. Optional. |
| `criterion_iri` | [Text](#text) or null | Native criterion IRI. Optional. |
| `default_strength` | [Text](#text) or null | Native default strength, distinct from an applied evidence strength. Optional. |
| `default_strength_id` | [Text](#text) or null | Native default-strength identifier; not an applied evidence strength. Optional. |
| `default_strength_iri` | [Text](#text) or null | Native default-strength IRI. Optional. |
| `outcome` | [Text](#text) | Source outcome label. |
| `outcome_conflict` | boolean | The supplied outcome identifier contradicts its supplied Met/Not Met label. Optional. |
| `outcome_id` | [Text](#text) or null | Source outcome identifier, including known conflicting ID/label pairs. Optional. |
| `outcome_iri` | [Text](#text) or null | Source outcome IRI. Optional. |

### AssertionCondition

A native condition with independently optional identifier and label.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Text](#text) or null | Native accession text, preserved even when no CURIE namespace is recognized. Optional. |
| `database` | [Text](#text) or null | Native database name without inventing a CURIE prefix. Optional. |
| `id` | [Curie](#curie) or null | Identifier exactly supplied by the source. Optional. |
| `label` | [Text](#text) or null | Label only when supplied. Optional. |

### AssertionConditionRelationship

A source trait relationship without assigning a clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `related_conditions` | list of [AssertionCondition](#assertioncondition) | Conditions named by the related source trait. |
| `relationship_type` | [Text](#text) or null | Relationship type exactly supplied by the source. Optional. |
| `source_conditions` | list of [AssertionCondition](#assertioncondition) | Conditions on the source trait that owns this relationship. |

### AssertionDetail

Source details that do not fit the compact shared statement.

| Field | Type | Meaning |
| --- | --- | --- |
| `approved` | [PartialDate](#partialdate) or null | Source approval date, distinct from evaluation and publication. Optional. |
| `citations` | list of [Pmid](#pmid) | Source citations, including assertion-method citations. Optional. |
| `comments` | list of [Text](#text) | Native classification comments, without extracting clinical claims. Optional. |
| `condition_relationships` | list of [AssertionConditionRelationship](#assertionconditionrelationship) | Native trait relationships, kept separate from assertion conditions. Optional. |
| `conditions` | list of [AssertionCondition](#assertioncondition) | All source conditions; ID-only conditions stay ID-only. Optional. |
| `document` | [AssertionDocument](#assertiondocument) or null | Source document facts for an explicitly selected historical version. Optional. |
| `impact_assertion_type` | [Text](#text) or null | Native somatic impact assertion type, without deriving a predicate. Optional. |
| `impact_clinical_significance` | [Text](#text) or null | Native somatic significance, independent of the tier label. Optional. |
| `impact_therapies` | [Text](#text) or null | Native therapy text, without splitting or assigning it to a predicate. Optional. |
| `methods` | list of [Text](#text) | Assertion method labels, not inferred from clinical testing. Optional. |
| `origins` | list of [Text](#text) | Source origin labels without reducing multiple observations. Optional. |
| `published` | [PartialDate](#partialdate) or null | Source publication date; never an inferred evaluation date. Optional. |
| `summary` | [Text](#text) or null | Source prose, kept separately from structured criteria. Optional. |
| `unmet_criteria` | list of [Text](#text) | Criteria explicitly marked unmet. Optional. |
| `versions` | list of [Text](#text) | Native document versions offered by the source. Optional. |

### AssertionDocument

Native SEPIO facts kept separate from common interpretation fields.

| Field | Type | Meaning |
| --- | --- | --- |
| `assessments` | list of [AssertionAssessment](#assertionassessment) | Assessments directly supporting the interpretation. Optional. |
| `document_type` | [Text](#text) | Native interpretation type. |
| `native_id` | [Text](#text) | Native interpretation identifier, separate from the summary PCER identifier. |
| `outcome_id` | [Text](#text) or null | Native outcome identifier without assigning a common coding system. Optional. |
| `outcome_iri` | [Text](#text) or null | Native outcome IRI. Optional. |
| `uuid` | [Text](#text) | Source UUID checked against the selected summary link. |

### AssertionDomain

Source classification domains remain separate.

Values: `germline`, `somatic_clinical_impact`, `oncogenicity`.

### AssertionKind

Native record scope.

Values: `vcv`, `rcv`, `scv`, `erepo`.

### AssertionSource

Native assertion provider.

Values: `clinvar`, `erepo`.

### Attribution

Provenance for one source record.

| Field | Type | Meaning |
| --- | --- | --- |
| `evaluated` | [PartialDate](#partialdate) or null | Source evaluation date. Optional. |
| `record` | [Text](#text) or null | Native assertion record. Optional. |
| `release` | [Text](#text) or null | Dataset release. Optional. |
| `source` | [Text](#text) | Source responsible for the claim. |
| `submitter` | [Text](#text) or null | Source submitter or panel. Optional. |
| `url` | [Text](#text) or null | Source record link. Optional. |
| `version` | [Text](#text) or null | Record version. Optional. |

### AuthorCounts

Counts describe only this provider record, never a global publication corpus.

| Field | Type | Meaning |
| --- | --- | --- |
| `citation_count` | integer or null | Native citations to this record’s indexed papers. Optional. |
| `h_index` | integer or null | Native h-index for this record’s indexed papers. Optional. |
| `paper_count` | integer or null | Native indexed paper count. Optional. |
| `scope` | [AuthorCountScope](#authorcountscope) | Counts apply only to this provider record. |

### AuthorCountScope

Scope of source-reported bibliographic counts.

Values: `provider_record`.

### AuthorHeader

One source candidate; absent affiliations differ from an empty source collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `affiliations` | list of [Text](#text) or null | Native affiliation labels; no employment claim. Optional. |
| `counts` | [AuthorCounts](#authorcounts) or null | Source record counts when supplied. Optional. |
| `id` | [AuthorId](#authorid) | Exact qualified provider record. |
| `name` | [Text](#text) | Source display name, never a join key. |

### AuthorId

Type: string.

### AuthorizationDate

A native date kept with its original role.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [PartialDate](#partialdate) | Source date at its stated precision. |
| `role` | [Text](#text) | Native date role, without assigning approval meaning. |

### AuthorizationEvent

A source-scoped regulator event; never an identity-card field.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | One source record and its native provenance. |
| `dates` | list of [AuthorizationDate](#authorizationdate) | All native dates with separate roles. Optional. |
| `indication` | [Text](#text) or null | Original indication scope when present. Optional. |
| `jurisdiction` | [Text](#text) or null | Jurisdiction explicitly stated by the source. Optional. |
| `kind` | [Text](#text) or null | Original native event type. Optional. |
| `product_number` | [Text](#text) or null | Explicit native product number, never a synthesized first product. Optional. |
| `record` | [DrugId](#drugid) | Native regulator record identifier. |
| `status` | [Text](#text) or null | Original source status; historical positives imply no current status. Optional. |

### AuthorPaperExternalId

An open native identifier namespace and value; no identity inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Curie](#curie) | Native namespace and identifier. |

### AuthorPaperHeader

One indexed paper in source order; identifiers come only from that source.

| Field | Type | Meaning |
| --- | --- | --- |
| `doi` | [Doi](#doi) or null | Exact native DOI. Optional. |
| `external_ids` | list of [AuthorPaperExternalId](#authorpaperexternalid) | Every other native paper identifier. |
| `paper_id` | [SemanticScholarId](#semanticscholarid) | The paper's ID at the source (Semantic Scholar). |
| `pmcid` | [Pmcid](#pmcid) or null | Exact native PMCID. Optional. |
| `pmid` | [Pmid](#pmid) or null | Exact native PMID. Optional. |
| `title` | [Text](#text) | Whole source title. |
| `venue` | [Text](#text) or null | Source venue when supplied and nonempty. Optional. |
| `year` | integer or null | Source publication year. Optional. |

### AuthorPaperSection

How much to show for each paper. `details` adds the paper's full metadata.

| Value | Meaning |
| --- | --- |
| `details` | Native detailed metadata for the same paper window. |

### AuthorPapersQuery

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

| Field | Type | Meaning |
| --- | --- | --- |
| `full` | boolean | Legacy alias for details; normalized on construction. Optional. |
| `id` | [AuthorId](#authorid) | Exact qualified provider record. |
| `limit` | integer (uint32) | Bounded row count. Optional. |
| `offset` | integer (uint32) | Native or retained collection offset. Optional. |
| `pool` | [Text](#text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |
| `section` | [AuthorPaperSection](#authorpapersection) | Detailed disclosure when requested. Optional. |

### Bound

A bounded source range; equal min and max denote an exact boundary.

| Field | Type | Meaning |
| --- | --- | --- |
| `max` | integer or null | Optional. |
| `min` | integer or null | Optional. |

### Build

The assembly stated by the source; no default is inferred.

| Value | Meaning |
| --- | --- |
| `GRCh37` | Genome Reference Consortium human build 37. |
| `GRCh38` | Genome Reference Consortium human build 38. |

### CacheProblem

A problem with BioMCP's local cache. The sources' answers still stand.

Values: `busy`, `database`, `version`.

### Capped

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of T | The items, in the source's order. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### CbioportalCancerTypeId

Type: string.

### CellLineChembl

Exact accession-linked metadata with its own native collection total.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | Requested exact accession. |
| `fetched` | integer or null | Original retrieval seconds; never a source release date. Optional. |
| `items` | T | Every selected whole metadata row and the native total. |
| `offset` | integer (uint32) | Native metadata page offset. |
| `release` | [CellLineRelease](#celllinerelease) or null | Independent ChEMBL release, distinct from retrieval time. Optional. |

### CellLineChemblRecord

One exact accession-linked ChEMBL cell identity; no assay request is implied.

| Field | Type | Meaning |
| --- | --- | --- |
| `cell_id` | integer (uint64) | Independent native integer cell record identity. |
| `cellosaurus_id` | [CellosaurusAccession](#cellosaurusaccession) | Native exact accession; adapter verifies the join. |
| `clo` | [Text](#text) or null | Native CLO identifier; omission is source absence. Optional. |
| `description` | [Text](#text) or null | Native description. Optional. |
| `efo` | [Text](#text) or null | Native EFO identifier; omission is source absence. Optional. |
| `id` | [ChemblCellId](#chemblcellid) | Native cell ChEMBL ID, never a compound or target. |
| `name` | [Text](#text) or null | Native display name without name-based joining. Optional. |
| `organism` | [Text](#text) or null | Native organism label. Optional. |
| `taxon` | integer or null | Native organism taxon. Optional. |
| `tissue` | [Text](#text) or null | Native tissue label. Optional. |

### CellLineGene

A native variation gene member with only a source-supplied HGNC identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `hgnc` | [HgncId](#hgncid) or null | Present only if this source member explicitly carries the HGNC identifier. Optional. |
| `reference` | [CellLineXref](#celllinexref) | Complete original source gene cross-reference. |

### CellLineGet

One exact identity and an optional bounded named section.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [CellLineInput](#celllineinput) | One exact primary accession or qualified source ID. |
| `section` | [CellLineSection](#celllinesection) | Omit for the default card; all returns bounded first pages without an assay count. Optional. |
| `section_limit` | integer (uint32) | Explicit row limit for each requested collection; native variation defaults to three. Optional. |
| `section_offset` | integer (uint32) | Offset within each independently requested collection. Optional. |

### CellLineHeader

Compact source identity with independently capped species and diseases.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | Exact primary accession. |
| `category` | [Text](#text) | Native category. |
| `diseases` | [Capped](#capped) of [CellLineXref](#celllinexref) | Up to three complete disease references and their independent total. |
| `name` | [Text](#text) | Recommended display name. |
| `species` | [Capped](#capped) of [CellLineXref](#celllinexref) | Up to three complete species references and their native total. |

### CellLineIdentityPage

Species, diseases and warnings page independently; each total belongs to its own collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | Selected exact primary accession. |
| `diseases` | [Capped](#capped) of [CellLineXref](#celllinexref) | Whole disease rows and their independent source total. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `offset` | integer (uint32) | Requested offset applied independently to each collection. |
| `release` | [CellLineRelease](#celllinerelease) or null | Independently fetched publisher release. Optional. |
| `species` | [Capped](#capped) of [CellLineXref](#celllinexref) | Whole species rows and their source total. |
| `warnings` | [Capped](#capped) of [CellLineWarning](#celllinewarning) | Whole scoped warning rows and their independent source total. |

### CellLineInput

An exact accession or a database-qualified source identity; never a name lookup.

Type: string.

### CellLineName

One native display name entry, never an identity key.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Text](#text) | Native name type, preserving identifier and synonym labels. |
| `value` | [Text](#text) | Native name spelling and punctuation. |

### CellLineNames

Native display names with collection-local totals and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | Selected exact primary accession. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `items` | T | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | Independently fetched publisher release. Optional. |

### CellLineProvenance

One complete native source row; reference, institution and database provenance stay independent.

| Field | Type | Meaning |
| --- | --- | --- |
| `institution` | [Text](#text) or null | Native institution name if supplied. Optional. |
| `reference` | [CellLineReference](#celllinereference) or null | Native publication reference if supplied. Optional. |
| `value` | [Text](#text) or null | Native institution value if supplied. Optional. |
| `xref` | [CellLineXref](#celllinexref) or null | Complete native database provenance if supplied. Optional. |

### CellLineReference

One native internal publication reference, without requiring PubMed.

| Field | Type | Meaning |
| --- | --- | --- |
| `resource_internal_ref` | [Text](#text) | Complete native resource reference, for example PubMed=9305600. |

### CellLineRelease

Publisher release identity, fetched independently from record data.

| Field | Type | Meaning |
| --- | --- | --- |
| `updated` | [PartialDate](#partialdate) | Native release update date. |
| `version` | [Text](#text) | Native release version. |

### CellLineSection

Named disclosure sections; all requests each bounded first page without an assay count.

| Value | Meaning |
| --- | --- |
| `names` | Native display names. |
| `identity` | Independent species, diseases and qualified warnings. |
| `xrefs` | Complete source cross-reference rows. |
| `variants` | Complete native variation rows. |
| `chembl` | Exact accession-linked ChEMBL cell metadata. |
| `all` | Bounded first page of every named section. |

### CellLineVariants

Whole native variation rows with collection-local totals and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | Selected exact primary accession. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `items` | T | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | Independently fetched publisher release. Optional. |

### CellLineVariation

One complete native variation row; descriptions do not become normalized alleles.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Text](#text) or null | Complete native mutation description including all coordinate spellings. Optional. |
| `genes` | list of [CellLineGene](#celllinegene) | Every native gene member, including both fusion members. |
| `mutation_type` | [Text](#text) or null | Native open mutation label. Optional. |
| `note` | [Text](#text) or null | Complete native variation note. Optional. |
| `provenance` | list of [CellLineProvenance](#celllineprovenance) | Every native provenance row in source order; no PMID is required. |
| `references` | list of [CellLineXref](#celllinexref) | Every other variation cross-reference in native order. |
| `variation_type` | [Text](#text) | Native open variation label. |
| `zygosity` | [Text](#text) or null | Native zygosity label; absence is not inferred. Optional. |

### CellLineWarning

Complete source warning with its native scope; no stock classifier is derived.

| Field | Type | Meaning |
| --- | --- | --- |
| `scope` | [Text](#text) | Native warning category, such as Problematic cell line. |
| `text` | [Text](#text) | Complete source text including historical and stock qualifications. |

### CellLineXref

One complete native cross-reference; database and accession form the source identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Text](#text) | Native accession, including any internal prefix. |
| `category` | [Text](#text) or null | Native source category when supplied. Optional. |
| `database` | [Text](#text) | Native database name; no fixed vocabulary. |
| `discontinued` | [Text](#text) or null | Native discontinuation qualifier, including values such as probable. Optional. |
| `iri` | [Text](#text) or null | Native IRI when supplied. Optional. |
| `label` | [Text](#text) or null | Native display label when supplied. Optional. |
| `url` | [Text](#text) or null | Native source URL when supplied. Optional. |

### CellLineXrefs

Native source cross-references with collection-local totals and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | Selected exact primary accession. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `items` | [Capped](#capped) of [CellLineXref](#celllinexref) | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | Independently fetched publisher release. Optional. |

### CellosaurusAccession

Type: string.

### ChemblCellId

Type: string.

### Chromosome

Type: string.

### CitationIntent

Source-reported reason for citing a paper.

| Value | Meaning |
| --- | --- |
| `background` | The source cites background information. |
| `result` | The source cites a reported result. |
| `methodology` | The source cites a method. |

### CivicClaimKind

CIViC native claim collection.

Values: `evidence`, `assertions`.

### CivicClaimPage

Native stream bounds; evidence and assertions have separate cursors.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | T or null | Continue the same source candidate, profile and claim stream. Optional. |
| `returned` | integer (uint32) | Number of native rows displayed in this stream. |
| `total` | integer (uint64) | Native total for this stream. |

### CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

### CivicMember

A native CIViC member, including members without a common-model subject.

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | [Text](#text) | Native feature label, including fusion features. |
| `hgvs_descriptions` | list of [Text](#text) | Every native notation, without selecting a transcript or repairing a spelling. Optional. |
| `label` | [Text](#text) | Whole source variant label. |
| `record` | [SourceRecord](#sourcerecord) | Native variant identity and link. |
| `subject` | [VariantRef](#variantref) or null | Source-qualified subject when supported by the common model. Optional. |

### CivicMembership

Identity strength of a CIViC profile lookup.

Values: `exact`, `candidate`.

### CivicNativeClaim

Source facts retained without inventing a common-model subject or proposition.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_origin` | [Term](#term) or null | Native origin, when stated. Optional. |
| `attribution` | [Attribution](#attribution) | Original source provenance. |
| `citations` | list of [Pmid](#pmid) | Native publication identities. Optional. |
| `classification` | [Term](#term) | Whole native classification. |
| `condition` | [Concept](#concept) or null | Source disease context. Optional. |
| `strength` | [Term](#term) or null | Native level, when present. Optional. |
| `therapies` | list of [Concept](#concept) | Native therapies in source order. Optional. |

### CivicProfile

One source molecular profile, preserving compound expressions.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertion_count` | integer (uint64) | Native assertion count, independent of the displayed page. |
| `assertion_page` | [CivicClaimPage](#civicclaimpage) of [VariantGet](#variantget) | Native assertion stream and its independent continuation. |
| `compound` | boolean | Native complex-profile flag; a member match does not satisfy the whole profile. |
| `evidence_count` | integer (uint64) | Native evidence count, independent of the displayed page. |
| `evidence_page` | [CivicClaimPage](#civicclaimpage) of [VariantGet](#variantget) | Native evidence stream and its independent continuation. |
| `label` | [Text](#text) | Whole source profile label, including its logical expression. |
| `members` | list of [CivicMember](#civicmember) | Every source member reference, without collapsing compound profiles. |
| `membership` | [CivicMembership](#civicmembership) | Whether a source member was joined to the requested exact identity. |
| `record` | [SourceRecord](#sourcerecord) | Native profile identity and link. |
| `statements` | list of [CivicStatement](#civicstatement) | Source claims; a compound claim retains a profile subject. |

### CivicStatement

One CIViC claim with native labels that exceed the common statement model.

| Field | Type | Meaning |
| --- | --- | --- |
| `direction` | [Text](#text) or null | Original provider direction; disputes never implies the opposite effect. Optional. |
| `kind` | [Text](#text) | Source claim category, without a fallback proposition. |
| `native` | [CivicNativeClaim](#civicnativeclaim) or null | Complete native claim facts when the common subject or proposition is unavailable. Optional. |
| `record` | [SourceRecord](#sourcerecord) | Native claim identity, retained even when no common proposition applies. |
| `significance` | [Text](#text) or null | Original significance, including when the classification holds an AMP tier. Optional. |
| `statement` | [Statement](#statement) or null | Common source-attributed claim. Optional. |
| `statement_unmapped` | boolean | Native claim cannot be represented by the common subject or proposition model. Optional. |
| `status` | [Text](#text) | Native curation state, without implying that every row is accepted. |

### ClingenCgcVicc2022Code

Checked ClingenCgcVicc2022 codes.

Values: `oncogenic`, `likely_oncogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### ClingenLowPenetrance2024Code

Checked ClingenLowPenetrance2024 codes.

Values: `pathogenic_low_penetrance`, `likely_pathogenic_low_penetrance`, `established_risk_allele`, `likely_risk_allele`, `uncertain_risk_allele`.

### ClinPgxAccession

Type: string.

### Coding

A code in one explicitly named source framework.

One of these forms:

- `system` is `"oncokb_oncogenicity"`: OncoKB oncogenicity vocabulary. Fields: `code` ([OncokbOncogenicityCode](#oncokboncogenicitycode)).
- `system` is `"oncokb_level"`: OncoKB evidence levels. Fields: `code` ([OncokbLevelCode](#oncokblevelcode)).
- `system` is `"civic_evidence_level"`: CIViC evidence level. Fields: `code` ([CivicEvidenceLevelCode](#civicevidencelevelcode)).
- `system` is `"acmg_amp_2015"`: acmg_amp_2015 vocabulary. Fields: `code` ([AcmgAmp2015Code](#acmgamp2015code)).
- `system` is `"clingen_low_penetrance_2024"`: clingen_low_penetrance_2024 vocabulary. Fields: `code` ([ClingenLowPenetrance2024Code](#clingenlowpenetrance2024code)).
- `system` is `"amp_asco_cap_2017_tier"`: amp_asco_cap_2017_tier vocabulary. Fields: `code` ([AmpAscoCap2017TierCode](#ampascocap2017tiercode)).
- `system` is `"amp_asco_cap_2017_level"`: amp_asco_cap_2017_level vocabulary. Fields: `code` ([AmpAscoCap2017LevelCode](#ampascocap2017levelcode)).
- `system` is `"clingen_cgc_vicc_2022"`: clingen_cgc_vicc_2022 vocabulary. Fields: `code` ([ClingenCgcVicc2022Code](#clingencgcvicc2022code)).

### ComplexPortalId

Type: string.

### ComponentLabel

Terms attached to a named carried component on a consumer response.

| Field | Type | Meaning |
| --- | --- | --- |
| `component` | [Text](#text) | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | Canonical publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Text](#text) | Publisher terms name. |
| `url` | [Text](#text) | Publisher terms page. |
| `usage` | [Use](#use) | Publisher use category. |

### ComponentUpload

One source-named component upload timestamp.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [Text](#text) | Unchanged source timestamp. |
| `name` | [Text](#text) | Source subcollection name. |

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Curie](#curie) or null | Source-stated concept identifier. Optional. |
| `label` | [Text](#text) | Original concept label. |

### CosmicCoordinates

Historical coordinates exactly as carried by the component.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | integer (uint64) | Source end value, without coordinate conversion. |
| `start` | integer (uint64) | Source start value, without coordinate conversion. |

### CosmicIdentity

Relationship between a carried historical fragment and its aggregator record.

Values: `unverified`.

### CosmicRecord

One source record from MyVariant's historical COSMIC copy.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate` | [Text](#text) or null | Unmodified source alternate spelling. Optional. |
| `chromosome` | [Text](#text) or null | Source chromosome label. Optional. |
| `cosmic_id` | [Text](#text) | Native COSMIC record identifier. |
| `hg19` | [CosmicCoordinates](#cosmiccoordinates) or null | Source hg19 coordinates, without allele construction. Optional. |
| `identity` | [CosmicIdentity](#cosmicidentity) | Fragment identity has not been independently verified. |
| `mut_freq` | [FiniteNumber](#finitenumber) or null | Source frequency; units and denominator are not inferred. Optional. |
| `mut_nt` | [Text](#text) or null | Source nucleotide text, without repair or normalization. Optional. |
| `myvariant_record` | [Text](#text) | MyVariant parent record association; this does not establish allele equivalence. |
| `reference` | [Text](#text) or null | Unmodified source reference spelling. Optional. |
| `tumor_site` | list of [Text](#text) | Every source tumor-site label in native order. Optional. |

### Curie

Type: string.

### Cytoband

Type: string.

### DiagnosticCountUnit

Meaning of the envelope's total for a source-owned diagnostic page.

Values: `product_identities`, `test_identities`, `native_relation_rows`, `native_method_entries`.

### DiagnosticDeviceId

Type: string.

### DiagnosticGet

Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Meaning |
| --- | --- | --- |
| `bundle_revision` | [Text](#text) or null | Immutable stored bundle revision retained by a continuation. Optional. |
| `id` | [Text](#text) | Exact native identity, including version for GTR. |
| `section` | [DiagnosticSection](#diagnosticsection) or null | gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card. Optional. |
| `section_limit` | integer (uint32) | Maximum native rows; omitted means five rows, or one regulatory fragment. Optional. |
| `section_offset` | integer (uint32) | Zero-based native row or fragment offset. Optional. |
| `source` | [DiagnosticGetSource](#diagnosticgetsource) | Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number. |
| `supplement` | [PmaSupplement](#pmasupplement) or null | Exact native PMA supplement; absent selects the base row. Optional. |

### DiagnosticGetSource

One explicit exact diagnostic record source.

Values: `gtr`, `who-ivd`, `fda-pma`, `fda-510k`.

### DiagnosticLaboratory

Compact native laboratory context without authorization interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `country` | [Text](#text) or null | Native country. Optional. |
| `id` | [Text](#text) | Native laboratory identifier. |
| `institution` | [Text](#text) or null | Native institution name. Optional. |
| `name` | [Text](#text) or null | Source laboratory name, absent when the native cell is empty. Optional. |

### DiagnosticPmaHeader

Compact exact PMA row identity; its native empty base cell is explicit.

| Field | Type | Meaning |
| --- | --- | --- |
| `applicant` | string or null | Native applicant when supplied, including explicit empty. Optional. |
| `decision_code` | string or null | Native decision_code when supplied, including explicit empty. Optional. |
| `decision_date` | string or null | Native decision_date when supplied, including explicit empty. Optional. |
| `pma_number` | [PmaNumber](#pmanumber) | Native application. |
| `supplement_number` | [PmaSupplementCell](#pmasupplementcell) or null | Absent differs from explicit empty base cell. Optional. |
| `trade_name` | string or null | Native trade_name when supplied, including explicit empty. Optional. |
| `url` | [Text](#text) | Exact source record URL. |

### DiagnosticSearch

Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Meaning |
| --- | --- | --- |
| `biomarker` | [Text](#text) or null | Exact table-0 biomarker cell; unsupported in the group table. Optional. |
| `bundle_revision` | [Text](#text) or null | Immutable stored bundle revision retained by a continuation. Optional. |
| `condition_id` | [Text](#text) or null | Exact MedGen:CUI identifier only. Optional. |
| `device_id` | [DiagnosticDeviceId](#diagnosticdeviceid) or null | Exact PMA application, application/supplement or native K identity. Optional. |
| `drug_application` | [FdaDrugApplication](#fdadrugapplication) or null | Cell-owned typed NDA/BLA application. Optional. |
| `gene` | [GeneSymbol](#genesymbol) or null | Exact source gene_symbol field, without alias expansion. Optional. |
| `gene_id` | [NcbiGeneId](#ncbigeneid) or null | Exact source GeneID, exclusive with gene symbol. Optional. |
| `limit` | integer (uint32) | Five results by default; at most fifty source rows or product headers. Optional. |
| `manufacturer` | [Text](#text) or null | Literal whole-word phrase in WHO's Manufacturer name cell. Optional. |
| `marker` | [Text](#text) or null | Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. Optional. |
| `offset` | integer (uint32) | Zero-based source result offset. Optional. |
| `source` | [DiagnosticSearchSource](#diagnosticsearchsource) | Explicit native GTR, WHO IVD or FDA companion discovery. |
| `type` | [GtrTestType](#gtrtesttype) or null | Exact native test_type value. Optional. |

### DiagnosticSearchSource

One explicit diagnostic discovery source.

Values: `gtr`, `fda-cdx`, `who-ivd`.

### DiagnosticSection

Source-supported diagnostic sections; no all section.

| Value | Meaning |
| --- | --- |
| `genes` | GTR native gene rows; five per page. |
| `conditions` | GTR native condition rows; five per page. |
| `methods` | GTR native method entries; five per page. |
| `regulatory` | Selected PMA submission statement; one fragment per page. |
| `supplements` | PMA base application supplement headers; five per page. |

### Direction

Assessment direction stated by the source.

Values: `supports`, `disputes`, `neutral`.

### DiseaseAssociation

A native source association without clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `agent_type` | [Text](#text) or null | Native agent type. Optional. |
| `aggregator_knowledge_source` | list of [Curie](#curie) or null | Native aggregator knowledge sources; missing, null and lists remain distinct. Optional. |
| `attribution` | [Attribution](#attribution) | Native row provenance and version. |
| `category` | [Curie](#curie) or null | Native association category. Optional. |
| `evidence` | list of [DiseaseAssociationReference](#diseaseassociationreference) or null | Source evidence terms; unavailable evidence stays absent. Optional. |
| `evidence_count` | integer or null | Native evidence count, distinct from returned reference count. Optional. |
| `has_count` | integer or null | Native annotation count, without a computed denominator. Optional. |
| `has_percentage` | [FiniteNumber](#finitenumber) or null | Native percentage, never computed from another field. Optional. |
| `has_quotient` | [FiniteNumber](#finitenumber) or null | Native quotient, never computed from another field. Optional. |
| `has_total` | integer or null | Native annotation total, without inferring an assayed denominator. Optional. |
| `knowledge_level` | [Text](#text) or null | Native source knowledge level. Optional. |
| `negated` | boolean or null | Source negation; absent is not false. Optional. |
| `object` | [Concept](#concept) | Native returned object. |
| `object_closure` | list of [Curie](#curie) | Native object closure. |
| `original_object` | [Curie](#curie) or null | Original source object before source normalization. Optional. |
| `original_predicate` | [Text](#text) or null | Native pre-normalization predicate, without interpretation. Optional. |
| `original_subject` | [Curie](#curie) or null | Original source subject before source normalization. Optional. |
| `predicate` | [Curie](#curie) | Native source predicate. |
| `primary_knowledge_source` | [Curie](#curie) or null | Native primary knowledge source. Optional. |
| `provided_by` | [Text](#text) or null | Native providing collection. Optional. |
| `publications` | list of [DiseaseAssociationReference](#diseaseassociationreference) | Native publication references, without inferred links or labels. Optional. |
| `qualifiers` | list of [DiseaseAssociationQualifier](#diseaseassociationqualifier) | Native qualifiers, including frequency or onset if supplied. |
| `requested_id` | [Curie](#curie) | The exact requested concept, including a queried parent. |
| `subject` | [Concept](#concept) | Native returned subject. |
| `subject_closure` | list of [Curie](#curie) | Native subject closure. |
| `subject_taxon` | [Curie](#curie) or null | Native subject taxon, only as supplied by the publisher. Optional. |

### DiseaseAssociationItem

One native association and the component terms governing that row.

| Field | Type | Meaning |
| --- | --- | --- |
| `association` | [DiseaseAssociation](#diseaseassociation) | Complete neutral association, including requested and actual endpoints. |
| `components` | list of [ComponentLabel](#componentlabel) | Independently attributed annotation components for this row. |

### DiseaseAssociationQualifier

A named native qualifier without an inferred role.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [Text](#text) | Original native qualifier field. |
| `id` | [Curie](#curie) | Native qualifier identity. |
| `label` | [Text](#text) or null | Native label when supplied. Optional. |

### DiseaseAssociationReference

One native association reference, with no invented display label or link.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Curie](#curie) | Native evidence or publication identity. |
| `label` | [Text](#text) or null | Native label when supplied. Optional. |
| `url` | [Text](#text) or null | Unchanged native link when supplied. Optional. |

### DiseaseEndpoint

One native endpoint with an identifier and an optional source label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Curie](#curie) | Exact source identifier. |
| `label` | [Text](#text) or null | Source label when supplied. Optional. |

### DiseaseGet

Exact concept or native SEER site with independently paged section.

| Field | Type | Meaning |
| --- | --- | --- |
| `age_range` | [Text](#text) or null | Native age-range code. Optional. |
| `dataset` | [Text](#text) or null | Requested native dataset selector, not proof of returned membership. Optional. |
| `id` | [Text](#text) | Exact ontology CURIE or an explicit SEER catalog site ID. |
| `interval` | [Text](#text) or null | Survival-only interval code. Optional. |
| `ontology` | [DiseaseOntology](#diseaseontology) | One OLS ontology. Optional. |
| `period` | [Text](#text) or null | Incidence-only native period code. Optional. |
| `race` | [Text](#text) or null | Native race code. Optional. |
| `section` | [DiseaseSection](#diseasesection) or null | One named section; absent means identity card only. Optional. |
| `section_limit` | integer (uint32) | Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. Optional. |
| `section_offset` | integer (uint32) | Native row offset within the selected section. Optional. |
| `sex` | [Text](#text) or null | Native sex code. Optional. |
| `source` | [DiseaseSource](#diseasesource) | Default MyDisease; no automatic fallback. Optional. |

### DiseaseOntology

One explicit OLS ontology.

Values: `mondo`, `doid`, `ncit`.

### DiseaseRelation

A publisher relation with distinct endpoints and predicate.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Publisher record and version. |
| `description` | string or null | Unchanged native cross-reference description when supplied. Optional. |
| `object` | [DiseaseEndpoint](#diseaseendpoint) | Native object. |
| `predicate` | [Curie](#curie) | Publisher predicate, without equivalence promotion. |
| `subject` | [DiseaseEndpoint](#diseaseendpoint) | Native subject. |

### DiseaseSearch

Literal discovery text or an explicit SEER catalog page.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum source rows on one page. Optional. |
| `offset` | integer (uint32) | Native row offset. Optional. |
| `ontology` | [DiseaseOntology](#diseaseontology) | Exactly one ontology, default MONDO. Optional. |
| `source` | [DiseaseSource](#diseasesource) | One explicit source, default OLS. Optional. |
| `text` | string or null | Literal source search text; absent for the SEER site catalog. Optional. |

### DiseaseSection

One named disease page.

| Value | Meaning |
| --- | --- |
| `synonyms` | Publisher scoped synonyms. |
| `xrefs` | Native cross references. |
| `hierarchy` | Native parent relations. |
| `genes` | Native gene association rows. |
| `phenotypes` | Native phenotype association rows. |
| `incidence` | Native stage incidence rows. |
| `survival` | Annual survival rows. |

### DiseaseSource

An explicit disease source without fallback.

Values: `ols`, `mydisease`, `monarch`, `seer`.

### DiseaseStatistic

One native statistics row with requested and returned dimensions.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Publisher record and dataset version when supplied. |
| `measure` | [Text](#text) | Native measure name. |
| `modeled` | [StatisticEstimate](#statisticestimate) or null | Modeled estimate kept separately. Optional. |
| `observed` | [StatisticEstimate](#statisticestimate) or null | Observed estimate; missing stays absent. Optional. |
| `requested_selectors` | list of [StatisticSelector](#statisticselector) | Settings sent to the publisher. |
| `requested_site` | [Text](#text) | Explicit selected native site ID. |
| `returned_selectors` | list of [StatisticSelector](#statisticselector) | Settings actually returned, never filled from request. |
| `year` | integer or null | Native annual row year. Optional. |

### Doi

Type: string.

### DrugFdaIngredient

An unchanged native ingredient and strength, without local normalization.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Text](#text) | Native ingredient name. |
| `strength` | [Text](#text) | Full source strength wording. Optional. |

### DrugFdaProduct

One explicit native product within the selected FDA application.

| Field | Type | Meaning |
| --- | --- | --- |
| `active_ingredients` | list of [DrugFdaIngredient](#drugfdaingredient) | Complete source-listed ingredient and strength pairs. |
| `brand_name` | [Text](#text) | Source brand wording, without inferred ingredient equivalence. |
| `dosage_form` | [Text](#text) | Native dosage form. |
| `marketing_status` | [Text](#text) | Product marketing status, independent of submission history. |
| `product_number` | [Text](#text) | Exact source product number, scoped by the response application. |
| `reference_drug` | [Text](#text) | Original source reference-drug designation. |
| `reference_standard` | [Text](#text) or null | Original source reference-standard designation when supplied. Optional. |
| `route` | [Text](#text) or null | Native route; absent and null remain unavailable. Optional. |
| `te_code` | [Text](#text) or null | Native therapeutic-equivalence code when supplied. Optional. |

### DrugGet

Exact native ID retrieval with an explicitly selected source.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic_cursor` | [Text](#text) or null | Native continuation cursor for that exact drug, profile and claim collection. Optional. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | Independent native evidence or assertion collection within the selected profile. Optional. |
| `civic_profile` | integer or null | Selected native CIViC molecular profile for a drug-scoped claim stream. Optional. |
| `input` | [DrugId](#drugid) | Published source-native identifier, never SOURCE:ID or a selected name. |
| `label_section` | [Text](#text) or null | Exact native label heading, required only for the label section. Optional. |
| `product_number` | [Text](#text) or null | Explicit native application product number, only for approvals. Optional. |
| `section` | list of [DrugSection](#drugsection) | Explicit named disclosures; empty means identity only. Optional. |
| `section_limit` | integer or null | Named section page size, refused on an identity-only card. Optional. |
| `section_offset` | integer or null | Named section continuation, refused on an identity-only card. Optional. |
| `source` | [DrugSource](#drugsource) | Required source; there is no default exact-ID resolver. |

### DrugHeader

One source candidate, without authorization, targets, label text or interactions.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [DrugId](#drugid) | Published native identifier. |
| `identity_level` | [DrugIdentityLevel](#drugidentitylevel) | Level established only by the endpoint or record type. |
| `name` | [Text](#text) | Original source display name. Optional. |
| `source` | [DrugSource](#drugsource) | The single source responsible for this header. |
| `type` | [Text](#text) or null | Native record type. Optional. |
| `url` | [Text](#text) or null | Source record URL. Optional. |
| `version` | [Text](#text) or null | Native source revision, omitted when the source supplies none. Optional. |

### DrugId

Type: string.

### DrugIdentity

One source record identity, without any equivalence or authorization claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [DrugId](#drugid) | Published native record identifier. |
| `identity_level` | [DrugIdentityLevel](#drugidentitylevel) | Level established by the endpoint or record type; unknown stays unknown. |
| `name` | [Text](#text) | Original source name. Optional. |
| `source` | [Text](#text) | Source responsible for this identity. |
| `type` | [Text](#text) or null | Native record type, without local classification. Optional. |
| `url` | [Text](#text) or null | Native record URL. Optional. |

### DrugIdentityLevel

Identity level established by the source endpoint or record type, never inferred from ID shape.

| Value | Meaning |
| --- | --- |
| `ingredient` | Source-stated ingredient identity. |
| `salt` | Source-stated salt identity. |
| `clinical_formulation` | Source-stated clinical formulation identity. |
| `branded_product` | Source-stated branded product identity. |
| `spl_set` | Source-stated spl set identity. |
| `spl_version` | Source-stated spl version identity. |
| `ndc_package` | Source-stated ndc package identity. |
| `fda_application` | Source-stated fda application identity. |
| `fda_product` | Source-stated fda product identity. |
| `ema_authorization` | Source-stated ema authorization identity. |
| `ema_application` | Source-stated ema application identity. |
| `orphan_designation` | Source-stated orphan designation identity. |
| `unknown` | Source-stated unknown identity. |

### DrugIdentityRelation

One explicitly source-stated identifier, parent, product or formulation relation.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Record supplying the relationship. |
| `identity` | [DrugIdentity](#drugidentity) | Identity at the other end of the source relation. |
| `relation` | [Text](#text) | Native relationship label; no inferred equivalence. |

### DrugNameSelector

One explicit ChEMBL name filter without fallback or synonym expansion.

| Value | Meaning |
| --- | --- |
| `preferred` | Match the native preferred name exactly, ignoring case. |
| `synonym` | Match an explicit native synonym exactly, ignoring case. |

### DrugNativeReference

Typed source reference shared by mechanism and indication records.

| Field | Type | Meaning |
| --- | --- | --- |
| `ref_id` | [Text](#text) or null | Source reference identifier, when supplied. Optional. |
| `ref_type` | [Text](#text) | Native reference type. |
| `ref_url` | [Text](#text) or null | Source reference link, when supplied. Optional. |

### DrugRecords

One bounded native row page, without fabricating an unknown total.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Source row offset. |
| `records` | list of T | Native source rows in order. Optional. |
| `retrieved` | [PartialDate](#partialdate) or null | Actual retrieval date, never substituted for source revision. Optional. |
| `total` | integer or null | Native total only when supplied. Optional. |
| `truncated` | boolean or null | Native source truncation indicator. Optional. |
| `version` | [Text](#text) or null | Dataset or bundle revision. Optional. |

### DrugSearch

Literal single-source candidate discovery, without an implicit identity selection.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Native candidate rows per page. Optional. |
| `name_selector` | [DrugNameSelector](#drugnameselector) | Explicit ChEMBL native name selector; omitted means preferred name. Optional. |
| `offset` | integer (uint32) | Native rows to skip. Optional. |
| `query` | [Text](#text) | Literal source name selector, unchanged by the consumer. |
| `source` | [DrugSource](#drugsource) | Exactly one source; default MyChem never fans out. Optional. |

### DrugSection

Explicit drug disclosures; identity cards never expand into these sections.

| Value | Meaning |
| --- | --- |
| `xrefs` | All native identity cross-references and product links, mechanically paged. |
| `targets` | Native target and mechanism relations. |
| `indications` | Native indication wording. |
| `civic` | Native CIViC therapy evidence and profile context. |
| `label` | One exact native label heading. |
| `approvals` | Native application and product history rows. |
| `regulatory` | Source-scoped regulator events. |
| `interactions` | Native DDInter partner rows and levels; five rows per default page, with explicit section limits preserved. |

### DrugSource

Values: `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter`.

### DrugWhoRow

Every published cell of one WHO finished-product record, including empty cells.

| Field | Type | Meaning |
| --- | --- | --- |
| `applicant` | string | Native Applicant wording. |
| `basis_of_listing` | string | Native Basis of Listing wording. |
| `date_of_prequalification` | string | Native Date of Prequalification text, including spacing and empty strings. |
| `dosage_form` | string | Native Dosage Form cell, independent of the combined cell. |
| `inn_dosage_form_strength` | string | Native combined INN, Dosage Form and Strength cell; never split or interpreted. |
| `product_type` | string | Native Product Type, retained even when empty. |
| `reference` | [Text](#text) | Exact native WHO Reference Number, including spaces and punctuation. |
| `therapeutic_area` | string | Native Therapeutic Area wording. |

### EnsemblGeneId

Type: string.

### EuropePmcId

Type: string.

### EvidenceNotice

Explicit successful empty outcomes, never inferred from a source failure.

Values: `not_in_references`, `no_contexts`, `no_recommendations`.

### Expression

Source expression checked for its declared published form, without normalization.

| Field | Type | Meaning |
| --- | --- | --- |
| `syntax` | [ExpressionSyntax](#expressionsyntax) |  |
| `syntax_version` | [Text](#text) or null | Optional. |
| `value` | [Text](#text) |  |

### ExpressionSyntax

Published syntax label for a source-preserved expression.

| Value | Meaning |
| --- | --- |
| `hgvs.g` | HGVS genomic description. |
| `hgvs.c` | HGVS coding description. |
| `hgvs.n` | HGVS noncoding transcript description. |
| `hgvs.r` | HGVS RNA description. |
| `hgvs.p` | HGVS protein description. |
| `hgvs.m` | HGVS mitochondrial description. |
| `hgvs.o` | HGVS circular genomic description. |
| `spdi` | NCBI SPDI interbase description. |
| `gnomad` | gnomAD chromosome-position-reference-alternate identifier. |
| `vcf` | Padded VCF chromosome-position-reference-alternate text. |
| `iscn` | Source-preserved cytogenetic notation. |

### FdaDrugApplication

Type: string.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### Gene

Human gene identity and whole source annotations, independent of response cards.

| Field | Type | Meaning |
| --- | --- | --- |
| `aliases` | list of [Text](#text) | All stated HGNC aliases. |
| `ensembl_id` | [EnsemblGeneId](#ensemblgeneid) or null | Ensembl gene identifier, if stated. Optional. |
| `entrez_id` | [NcbiGeneId](#ncbigeneid) or null | NCBI Gene identifier, if stated. Optional. |
| `hgnc_id` | [HgncId](#hgncid) | HGNC identifier. |
| `location` | [Cytoband](#cytoband) or null | Single stated band, if available. Optional. |
| `locus_group` | [LocusGroup](#locusgroup) | HGNC locus group. |
| `locus_type` | [LocusType](#locustype) or null | HGNC locus type, if stated and recognized. Optional. |
| `mane` | list of [Mane](#mane) | Versioned MANE Select transcript pair, if stated. Optional. |
| `name` | [Text](#text) | Gene name. |
| `omim` | list of [OmimId](#omimid) | All stated OMIM identifiers. |
| `previous_symbols` | list of [GeneSymbol](#genesymbol) | All stated previous symbols. |
| `spans` | list of [GenomicInterval](#genomicinterval) | Source intervals with explicit assembly and counting rule. |
| `status` | [GeneStatus](#genestatus) | HGNC record status. |
| `summary` | [Summary](#summary) or null | Whole attributed summary, if stated. Optional. |
| `symbol` | [GeneSymbol](#genesymbol) | Source-preserved symbol. |
| `uniprot` | list of [UniProtAcc](#uniprotacc) | All stated UniProt base accessions. |

### GeneHeader

Enough gene identity to choose a search result.

| Field | Type | Meaning |
| --- | --- | --- |
| `hgnc_id` | [HgncId](#hgncid) | HGNC identifier. |
| `location` | [Cytoband](#cytoband) or null | A single stated band, if available. Optional. |
| `locus_group` | [LocusGroup](#locusgroup) | HGNC locus group. |
| `match` | [Match](#match) or null | Match information when this header answers an input. Optional. |
| `name` | [Text](#text) | Gene name. |
| `status` | [GeneStatus](#genestatus) | HGNC record status. |
| `symbol` | [GeneSymbol](#genesymbol) | Source-preserved symbol. |

### GeneInput

Type: string.

### GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl_gene_id` | [EnsemblGeneId](#ensemblgeneid) or null | Optional. |
| `hgnc_id` | [HgncId](#hgncid) or null | Optional. |
| `label` | [Text](#text) |  |
| `ncbi_gene_id` | [NcbiGeneId](#ncbigeneid) or null | Optional. |

### GeneResolution

Resolution outcomes preserve ambiguity and withdrawn identity.

One of these forms:

- `resolution` is `"resolved"`: One resolved identity and its source match. Fields: `gene` ([Gene](#gene)), `match` ([Match](#match)).
- `resolution` is `"ambiguous"`: Every candidate, with no silent selection. Fields: `candidates` (list of [GeneHeader](#geneheader)).
- `resolution` is `"withdrawn"`: The original withdrawn identity and stated successors. Fields: `hgnc_id` ([HgncId](#hgncid)), `hgnc_status` ([HgncStatus](#hgncstatus)), `merged_into` (list of [HgncId](#hgncid)), `symbol` ([GeneSymbol](#genesymbol)).
- `resolution` is `"not_found"`: No source identity matched.

### GeneStatus

HGNC approval status.

| Value | Meaning |
| --- | --- |
| `approved` | An HGNC approved record. |
| `withdrawn` | An HGNC withdrawn record. |

### GeneSymbol

Type: string.

### GenomicInterval

An assembly-qualified zero-based half-open source interval.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](#build) or null | Source-stated assembly, omitted when absent. Optional. |
| `chromosome` | [Chromosome](#chromosome) | Canonical primary chromosome. |
| `span` | [Span0](#span0) | Checked zero-based half-open bounds; an empty span remains valid. |
| `strand` | [Strand](#strand) or null | Source-stated strand, if available. Optional. |

### GnomadCounts

Native gnomAD counts retained when a dataset release cannot be established.

| Field | Type | Meaning |
| --- | --- | --- |
| `ac` | integer (uint64) | Native allele count, including reported zero. |
| `an` | integer (uint64) | Native allele denominator. |
| `ancestry_group` | [Text](#text) or null | Source ancestry code decoded only by the documented source grammar. Optional. |
| `data_type` | [PopulationDataType](#populationdatatype) | Source sequencing sample set. |
| `filters` | list of [Text](#text) | Native source filters, never converted to a classification. Optional. |
| `frequency` | [PopulationFrequency](#populationfrequency) or null | Checked total from a release supplied alongside matching source counts. Optional. |
| `group_id` | [Text](#text) or null | Complete native group ID, including its source sex suffix when present. Optional. |
| `sex` | [PopulationSex](#populationsex) or null | Source sex stratum decoded only by the documented source grammar. Optional. |

### GtrConditionRelation

Native condition membership; SNOMED cells never become GeneID.

| Field | Type | Meaning |
| --- | --- | --- |
| `condition` | [Concept](#concept) | Shared MedGen concept with source label. |
| `mim_number` | string | Native MIM cell, including empty. |
| `snomed_ct_id` | string | Native SNOMED CT cell, including empty; never a gene ID. |
| `test_id` | [GtrTestId](#gtrtestid) | Complete owning test accession/version. |
| `test_type` | [GtrTestType](#gtrtesttype) | Native source test type. |

### GtrGeneRelation

Native gene relation; its numeric ID comes only from the gene row.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [GeneRef](#generef) | Shared source-given GeneID and gene_symbol; absent GeneID stays absent. |
| `mim_number` | string | Native MIM cell, including empty. |
| `native_gene_or_snomed_ct_id` | string or null | Uninterpreted nonempty `gene_or_SNOMED_CT_ID` when it is not a checked NCBI Gene ID. Optional. |
| `native_medgen` | [Curie](#curie) | Independent native MedGen identity from GTR_identifier. |
| `object_name` | string | Full source object_name without interpretation. |
| `test_id` | [GtrTestId](#gtrtestid) | Complete owning test accession/version. |
| `test_type` | [GtrTestType](#gtrtesttype) | Native source test type. |

### GtrHeader

Small resolved GTR candidate; broad gene and condition arrays stay off the header.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [GtrTestId](#gtrtestid) | Complete source accession and version. |
| `laboratory` | [DiagnosticLaboratory](#diagnosticlaboratory) | Native laboratory context. |
| `name` | [Text](#text) or null | Native test name, absent when the cell is empty. Optional. |
| `url` | [Text](#text) | Source record URL. |

### GtrMethod

One entry from one independent native method list.

| Field | Type | Meaning |
| --- | --- | --- |
| `index` | integer (uint32) | Zero-based native list position. |
| `list` | [GtrMethodList](#gtrmethodlist) | Independent source list; lists are never positionally paired. |
| `test_id` | [GtrTestId](#gtrtestid) | Complete owning test accession/version. |
| `value` | string | Complete unmodified source entry. |

### GtrMethodList

Independent native GTR method lists.

Values: `method_categories`, `methods`, `platforms`.

### GtrTestId

Type: string.

### GtrTestType

Native GTR relation test category, not an offering or authorization judgment.

Values: `Clinical`, `Research`, `Clinical microbial`.

### GwasAssociation

Source association with unchanged trait and effect labels.

| Field | Type | Meaning |
| --- | --- | --- |
| `alleles` | list of [GwasEffectAllele](#gwaseffectallele) | Every source effect allele paired with its rsID. |
| `author` | [Text](#text) or null | Source author attribution, without local publication inference. Optional. |
| `beta` | [Text](#text) or null | Source beta text, including units and direction. Optional. |
| `ci_lower` | [FiniteNumber](#finitenumber) or null | Source lower effect interval bound. Optional. |
| `ci_upper` | [FiniteNumber](#finitenumber) or null | Source upper effect interval bound. Optional. |
| `exact_p_value` | [ScientificPValue](#scientificpvalue) or null | Authoritative exact source probability, including double underflow. Optional. |
| `id` | integer (uint64) | Native association identity. |
| `interval` | [Text](#text) or null | Source effect interval. Optional. |
| `locations` | list of [Text](#text) | Native mapping location strings; no assumed assembly or coordinate conversion. Optional. |
| `mapped_genes` | list of [Text](#text) | Original native mapped gene context; reported genes remain in loci. Optional. |
| `odds_ratio` | [FiniteNumber](#finitenumber) or null | Source odds ratio. Optional. |
| `ontology_traits` | list of [GwasTrait](#gwastrait) | Source mapped trait identifiers and whole labels, without clinical inference. Optional. |
| `p_value` | [FiniteNumber](#finitenumber) | Reported p value. |
| `publication` | [Text](#text) or null | Source publication identifier. Optional. |
| `risk_frequency` | [Text](#text) or null | Source allele frequency text, without interpreting NR. Optional. |
| `study` | [Text](#text) | Study accession. |
| `traits` | list of [Text](#text) | Source reported traits. |

### GwasAssociationId

Type: string.

### GwasAssociationSection

Association-only section.

| Value | Meaning |
| --- | --- |
| `loci` | Native locus and reported gene context. |

### GwasEffectAllele

Catalog effect allele paired with its exact rsID.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele` | [Text](#text) | Source allele text. |
| `rs_id` | [Text](#text) | Source rsID. |

### GwasGet

Explicit resource sum; impossible section and identity pairings fail during decoding.

One of these forms:

- `kind` is `"association"`: Read an association. Fields: `id` ([GwasAssociationId](#gwasassociationid)), `section` ([GwasAssociationSection](#gwasassociationsection) or null), `section_limit` (integer (uint32)), `section_offset` (integer (uint32)), `snapshot` ([Text](#text)).
- `kind` is `"study"`: Read a study. Fields: `id` ([GwasStudyId](#gwasstudyid)), `section` ([GwasStudySection](#gwasstudysection) or null), `section_limit` (integer (uint32)), `section_offset` (integer (uint32)), `snapshot` ([Text](#text)).

### GwasStudyId

Type: string.

### GwasStudySection

Study-only sections.

| Value | Meaning |
| --- | --- |
| `associations` | Shared association rows. |
| `ancestries` | Independent native ancestry rows. |

### GwasTrait

Source ontology mapping as supplied by GWAS Catalog.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Text](#text) | Exact source ontology identifier spelling. |
| `label` | [Text](#text) | Whole source trait label. |

### HgncId

Type: string.

### HgncStatus

HGNC's stated withdrawn-file status.

| Value | Meaning |
| --- | --- |
| `entry_withdrawn` | Entry withdrawn. |
| `merged_split` | Entry merged or split. |

### HgvsText

Type: string.

### HotspotAlternate

One source amino-acid alternate count.

| Field | Type | Meaning |
| --- | --- | --- |
| `amino_acid` | [Text](#text) | Source alternate amino-acid spelling. |
| `count` | integer (uint64) | Source count, without deriving a frequency. |

### HotspotMembership

Source transcript and consequence join state.

Values: `exact`, `candidate`.

### HotspotRecord

Counts for one source transcript and residue label.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternates` | list of [HotspotAlternate](#hotspotalternate) | Source alternate amino acids and their counts. |
| `gene` | [GeneSymbol](#genesymbol) | Gene as stated by the source. |
| `membership` | [HotspotMembership](#hotspotmembership) | Whether transcript and consequence were linked by the identity source. |
| `residue` | [Text](#text) | Native residue label; no genomic allele is inferred. |
| `transcript` | [Text](#text) or null | Native transcript spelling, without an inferred accession version. Optional. |
| `tumor_count` | integer (uint64) | Tumors reported for this residue by the source. |

### HpId

Type: string.

### InterProId

Type: string.

### LeafRef

One of these forms:

- `level` is `"allele"`: no description. Fields: `hgvs` ([HgvsText](#hgvstext) or null), `id` ([VariantId](#variantid) or null).
- `level` is `"protein"`: no description. Fields: `gene` ([GeneRef](#generef)), `hgvs` ([HgvsText](#hgvstext) or null), `label` ([Text](#text) or null).

### License

License reported by the bucket metadata record.

| Value | Meaning |
| --- | --- |
| `cc_by` | Creative Commons Attribution. |
| `cc_by_nc` | Creative Commons Attribution NonCommercial. |
| `cc_by_sa` | Creative Commons Attribution ShareAlike. |
| `cc_by_nc_sa` | Creative Commons Attribution NonCommercial ShareAlike. |
| `cc_by_nd` | Creative Commons Attribution NoDerivatives. |
| `cc_by_nc_nd` | Creative Commons Attribution NonCommercial NoDerivatives. |
| `cc0` | Creative Commons public-domain dedication. |
| `public_domain` | Public domain. |
| `tdm` | Text and data mining manuscript terms. |
| `other` | The bucket reports another license. |

### LocusGroup

HGNC locus group, separate from a missing source value.

| Value | Meaning |
| --- | --- |
| `protein_coding` | Protein coding. |
| `non_coding_rna` | Non-coding RNA. |
| `pseudogene` | Pseudogene. |
| `phenotype` | Phenotype. |
| `other` | HGNC's stated other group. |

### LocusType

HGNC locus type; Unknown is a source-stated value, distinct from absence.

| Value | Meaning |
| --- | --- |
| `gene_with_protein_product` | HGNC gene with protein product. |
| `rna_cluster` | HGNC rna cluster. |
| `long_non_coding_rna` | HGNC long non coding rna. |
| `micro_rna` | HGNC micro rna. |
| `misc_rna` | HGNC misc rna. |
| `ribosomal_rna` | HGNC ribosomal rna. |
| `small_nuclear_rna` | HGNC small nuclear rna. |
| `small_nucleolar_rna` | HGNC small nucleolar rna. |
| `transfer_rna` | HGNC transfer rna. |
| `vault_rna` | HGNC vault rna. |
| `y_rna` | HGNC y rna. |
| `phenotype_only` | HGNC phenotype only. |
| `pseudogene` | HGNC pseudogene. |
| `complex_locus_constituent` | HGNC complex locus constituent. |
| `endogenous_retrovirus` | HGNC endogenous retrovirus. |
| `fragile_site` | HGNC fragile site. |
| `immunoglobulin_gene` | HGNC immunoglobulin gene. |
| `immunoglobulin_pseudogene` | HGNC immunoglobulin pseudogene. |
| `readthrough` | HGNC readthrough. |
| `region` | HGNC region. |
| `t_cell_receptor_gene` | HGNC t cell receptor gene. |
| `t_cell_receptor_pseudogene` | HGNC t cell receptor pseudogene. |
| `transposable_element` | HGNC transposable element. |
| `unknown` | HGNC unknown. |
| `virus_integration_site` | HGNC virus integration site. |

### Mane

A source-stated MANE pair with versioned sequence accessions.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl` | string | Versioned Ensembl accession. |
| `refseq` | string | Versioned RefSeq accession. |
| `status` | [ManeStatus](#manestatus) | Source-stated transcript status. |

### ManeStatus

The source-stated MANE transcript status.

| Value | Meaning |
| --- | --- |
| `select` | MANE Select transcript pair. |
| `plus_clinical` | MANE Plus Clinical transcript pair. |

### Markdown

Text that is never empty, with spaces removed from both ends.

Type: string (markdown).

### Match

A source-stated match, preserving the text that matched.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [MatchKind](#matchkind) | The match category. |
| `text` | [Text](#text) | The source text that matched the input. |

### MatchKind

How a source matched a gene input.

| Value | Meaning |
| --- | --- |
| `symbol` | Current symbol. |
| `previous_symbol` | Previous symbol. |
| `alias` | HGNC alias. |
| `ncbi_alias` | NCBI-only alias. |
| `hgnc_id` | HGNC identifier. |
| `entrez_id` | NCBI Gene identifier. |
| `ensembl_id` | Ensembl gene identifier. |
| `band` | Cytogenetic band. |

### NcbiGeneId

Type: string.

### NciDiseaseRef

Compact source disease reference; the diseases section retains every coding field.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Text](#text) | Native source name. |
| `nci_thesaurus_concept_id` | [Text](#text) | Source NCIt identity. |

### NciTextPage

Exact source text fragment with an honest original character count.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [TrialGet](#trialget) or null | Checked request for the next original characters, absent when complete. Optional. |
| `offset` | integer (uint32) | Original Unicode scalar position. |
| `text` | string | Original source fragment, including whitespace. |
| `total_characters` | integer (uint32) | Complete source length in Unicode scalar values. |

### NciTrial

NCI trial card without registry status or phase coercion.

| Field | Type | Meaning |
| --- | --- | --- |
| `brief_summary` | [NciTextPage](#ncitextpage) or null | Native short summary, only on exact retrieval. Optional. |
| `brief_title` | [Text](#text) | Native title. |
| `current_trial_status` | [Text](#text) | Native status, without a registry translation. |
| `diseases` | [TrialRecords](#trialrecords) of [NciDiseaseRef](#ncidiseaseref) | Publisher-provided disease concepts in source order. |
| `nci_id` | [Text](#text) | Publisher NCI identifier. |
| `nct_id` | [NctId](#nctid) | Registry cross-reference supplied by NCI. |
| `phase` | [Text](#text) or null | Native phase. Optional. |

### NctId

Type: string.

### OmimId

Type: string.

### OncokbCitation

One native non-PubMed source citation.

| Field | Type | Meaning |
| --- | --- | --- |
| `link` | [Text](#text) or null | Native citation link. Optional. |
| `text` | string or null | Complete native citation content. Optional. |

### OncokbLevelCode

Source-defined OncoKB vocabulary.

Values: `level_1`, `level_2`, `level_3a`, `level_3b`, `level_4`, `level_r1`, `level_r2`, `level_dx1`, `level_dx2`, `level_dx3`, `level_px1`, `level_px2`, `level_px3`.

### OncokbOncogenicityCode

Source-defined OncoKB vocabulary.

Values: `oncogenic`, `likely_oncogenic`, `likely_neutral`, `inconclusive`, `resistance`, `unknown`.

### OncokbVariantRecord

A native annotation row; no level or classification is inferred from other fields.

One of these forms:

- `kind` is `"statement"`: A complete source statement with its original label. Fields: `statement` ([Statement](#statement)).
- `kind` is `"citation"`: Native non-PubMed citation, preserving source text and link. Fields: `field` (string), `link` ([Text](#text) or null), `text` (string or null).
- `kind` is `"text"`: Native narrative, split only at checked character boundaries. Fields: `field` (string), `offset` (integer (uint32)), `text` (string), `total` (integer (uint32)).
- `kind` is `"unmapped_label"`: A source label without a representable source protein subject. Fields: `citations` (list of [Pmid](#pmid)), `classification` ([Term](#term)), `field` (string), `statement_unmapped` (boolean).
- `kind` is `"implication"`: Native therapeutic, diagnostic or prognostic implication. Fields: `abstracts` (list of [OncokbCitation](#oncokbcitation)), `alterations` (list of [Text](#text)), `approved_indications` (list of [Text](#text)), `citations` (list of [Pmid](#pmid)), `description` (string or null), `drugs` (list of [Concept](#concept)), `excluded_tumor_types` (list of [Concept](#concept)), `fda_level` ([Term](#term) or null), `field` (string), `level` ([Term](#term)), `statement` ([Statement](#statement) or null), `statement_unmapped` (boolean), `tumor_type` ([Concept](#concept) or null).

### OpenAccessStatus

Open-access status supplied by the paper-link provider.

| Value | Meaning |
| --- | --- |
| `gold` | Publisher open access. |
| `hybrid` | Open article in a subscription journal. |
| `bronze` | Free publisher copy without an open license claim. |
| `green` | Repository copy. |
| `closed` | No open copy. |

### PageInfo

Counts for one search page, after BioMCP removes duplicates and applies filters.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | How many results come before this page. |
| `pool` | [PoolInfo](#poolinfo) or null | The stored search this page comes from, when BioMCP kept one. Optional. |
| `returned` | integer (uint32) | How many results this page holds. |
| `total` | integer (uint64) | How many matching records the search holds in all. |
| `truncated` | boolean or null | True when the source had more matches than BioMCP fetched. Optional. |

### PartialDate

Type: string.

### PathwayEventRef

An event object or unresolved integer reference; never the parent's inferred ID.

One of these forms:

- `kind` is `"database"`: Native unresolved integer database reference. Fields: `db_id` (integer (uint64)).
- `kind` is `"object"`: Native event object; fields are absent only when omitted by the source. Fields: `db_id` (integer (uint64)), `id` ([ReactomeId](#reactomeid) or null), `name` ([Text](#text) or null), `schema_class` ([Text](#text) or null), `source_url` ([Text](#text) or null), `version` ([Text](#text) or null).

### PathwayEvents

Independent source-ordered page of whole native references.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | Exact source pathway owning this relation. |
| `items` | list of [PathwayEventRef](#pathwayeventref) | Whole native references; unresolved identities remain visible. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | Relation-specific count provenance and continuation. |

### PathwayGenes

Independent source-ordered distinct membership references and their observed counts.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | Exact source pathway owning this relation. |
| `items` | list of [PathwayMember](#pathwaymember) | Distinct namespaced source references in first occurrence order. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | Distinct-reference total and continuation. |
| `physical_entities` | integer (uint64) | Every Reactome `PhysicalEntity` of the pathway, as Reactome lists them. They are not gene counts. |
| `reference_occurrences` | integer (uint64) | All reference occurrences across physical entities before distinct-reference grouping. |

### PathwayHeader

Small selectable native pathway identity, without its potentially long description.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [PathwayId](#pathwayid) | Checked source-owned identity. |
| `name` | [Text](#text) | Whole native name. |
| `revision` | [Text](#text) or null | WikiPathways revision, separate from identity. Optional. |
| `species` | [PathwaySpecies](#pathwayspecies) | Explicit native human species. |
| `version` | [Text](#text) or null | Whole native Reactome stIdVersion, separate from stable identity. Optional. |

### PathwayId

Source-owned pathway identity; equal names never merge sources.

One of these forms:

- `source` is `"reactome"`: Human Reactome stable identity without a version suffix. Fields: `id` ([ReactomeId](#reactomeid)).
- `source` is `"wikipathways"`: WikiPathways identity without a revision suffix. Fields: `id` ([WikiPathwaysId](#wikipathwaysid)).

### PathwayMember

One source membership reference, without name-based gene inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [HgncId](#hgncid) or null | Exact resolved gene identifier, absent when no checked join exists. Optional. |
| `name` | [Text](#text) or null | Native label, never interpreted as a gene symbol. Optional. |
| `node_id` | [Text](#text) or null | Native data-node identity; repeated references remain separate nodes. Optional. |
| `participants` | list of [PathwayParticipant](#pathwayparticipant) or null | Complete physical contexts, disclosed only by the explicit details view. Optional. |
| `physical_context_count` | integer (uint64) | Number of source physical entity contexts carrying this exact reference. |
| `reference` | [PathwaySourceRef](#pathwaysourceref) | Namespaced native reference. |
| `schema_class` | [Text](#text) or null | Native reference schema class. Optional. |
| `source_url` | [Text](#text) or null | Native reference URL, including therapeutic references without a drug action. Optional. |

### PathwayPageInfo

Native page position and explicit count provenance; a short admitted page can continue.

| Field | Type | Meaning |
| --- | --- | --- |
| `next_offset` | integer or null | Next native position, independent of returned rows; absent only at the end. Optional. |
| `offset` | integer (uint32) | Native source or retained relation offset. |
| `pool` | [PoolInfo](#poolinfo) or null | Snapshot identity and lifetime when retained. Optional. |
| `returned` | integer (uint32) | Whole admitted rows on this page. |
| `total` | integer (uint64) | Source-reported or observed relation total, as qualified by total_kind. |
| `total_kind` | [PathwayTotalKind](#pathwaytotalkind) | Whether the total comes from the source or this retained collection. |

### PathwayParticipant

One native physical [entity](/concepts/progressive-disclosure/) carrying a membership reference.

| Field | Type | Meaning |
| --- | --- | --- |
| `db_id` | integer (uint64) | Native physical entity database identity, distinct from reference identity. |
| `name` | [Text](#text) | Whole physical entity label, including source-stated modification context. |
| `schema_class` | [Text](#text) | Native physical entity schema class, separate from reference schema class. |

### PathwaySourceRef

An open source namespace paired with its complete native identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `namespace` | string or null | Native namespace; never discarded during a gene join. Optional. |
| `value` | [Text](#text) | Complete namespace-local identifier, including native isoform suffixes. |

### PathwaySpecies

Source-stated human species; no species inference from a title.

Values: `Homo sapiens`.

### PathwayTotalKind

Meaning of the collection total, independent of admitted rows on a page.

Values: `source`, `observed`, `distinct_references`, `data_nodes`.

### PgxAnnotationKind

Independent ClinPGx collection kinds, never merged.

| Value | Meaning |
| --- | --- |
| `clinical` | Clinical annotation min tier. |
| `guideline` | Guideline references. |
| `label` | Label references. |

### PgxCrossReference

Source-native PgxCrossReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `_url` | string | The source's own `_url` value, unchanged. |
| `id` | integer (int64) | The source's own `id` value, unchanged. |
| `name` | string or null | Native `name` when supplied. Optional. |
| `resource` | string | The source's own `resource` value, unchanged. |
| `resourceId` | string | The source's own `resourceId` value, unchanged. |

### PgxDiplotypeExpression

Type: string.

### PgxDiplotypeReference

Native location diplotype reference; its numeric ID is not a ClinPGx accession.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | integer (int64) | Native `id`; no expression, phase or phenotype is inferred. |

### PgxEvidenceTerm

Source-native PgxEvidenceTerm fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | integer (int64) | The source's own `id` value, unchanged. |
| `resource` | string | The source's own `resource` value, unchanged. |
| `synonyms` | list of [PgxUnobserved](#pgxunobserved) or null | Native `synonyms` when supplied. Optional. |
| `term` | string | The source's own `term` value, unchanged. |
| `termId` | string | The source's own `termId` value, unchanged. |
| `valid` | boolean or null | Native `valid` when supplied. Optional. |

### PgxGet

PGx card or section. No phenotype, phase or dose is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `diplotype` | [PgxDiplotypeExpression](#pgxdiplotypeexpression) or null | Unchanged declared expression required for the diplotype section only. Optional. |
| `drug_id` | [Curie](#curie) or null | Exact published CPIC drug key; recommendations only. Optional. |
| `gene` | [GeneSymbol](#genesymbol) | Explicit source gene symbol. |
| `kind` | [PgxAnnotationKind](#pgxannotationkind) or null | One ClinPGx collection; clinical is the default only on annotations. Optional. |
| `section` | [PgxSection](#pgxsection) or null | One named section; omitted means the CPIC gene card. Optional. |
| `section_limit` | integer (uint32) | Rows per page: three clinical, five elsewhere. Optional. |
| `section_offset` | integer (uint32) | Offset within the selected section. Optional. |
| `snapshot` | [Text](#text) or null | Retained collection for local continuation. Optional. |
| `source` | [PgxSource](#pgxsource) | Required source, checked against the section. |

### PgxHeader

Source and declared gene context; no chosen pair or patient result.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [GeneSymbol](#genesymbol) | Declared source symbol. |
| `source` | [PgxSource](#pgxsource) | Explicit source. |

### PgxLocation

Source-native PgxLocation fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `buildVersion` | string or null | Native `buildVersion` when supplied. Optional. |
| `chromosomeId` | string or null | Native `chromosomeId` when supplied. Optional. |
| `chromosomeName` | string or null | Native `chromosomeName` when supplied. Optional. |
| `copyNumber` | string | The source's own `copyNumber` value, unchanged. |
| `diplotypes` | list of [PgxDiplotypeReference](#pgxdiplotypereference) | The source's own `diplotypes` value, unchanged. |
| `displayName` | string | The source's own `displayName` value, unchanged. |
| `genes` | list of [PgxObjectReference](#pgxobjectreference) | The source's own `genes` value, unchanged. |
| `gpPosition` | integer (int64) | The source's own `gpPosition` value, unchanged. |
| `haplotypes` | list of [PgxObjectReference](#pgxobjectreference) | The source's own `haplotypes` value, unchanged. |
| `id` | integer (int64) | The source's own `id` value, unchanged. |
| `linkedObjects` | list of [PgxObjectReference](#pgxobjectreference) | The source's own `linkedObjects` value, unchanged. |
| `refSeqCrossReference` | [PgxCrossReference](#pgxcrossreference) or null | Native `refSeqCrossReference` when supplied. Optional. |
| `refSeqPosition` | integer (int64) | The source's own `refSeqPosition` value, unchanged. |
| `rsid` | string or null | Native `rsid` when supplied. Optional. |
| `tagGene` | boolean | The source's own `tagGene` value, unchanged. |
| `type` | string | The source's own `type` value, unchanged. |
| `variant` | [PgxObjectReference](#pgxobjectreference) or null | Native `variant` when supplied. Optional. |

### PgxObjectReference

Source-native PgxObjectReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [ClinPgxAccession](#clinpgxaccession) | The source's own `id` value, unchanged. |
| `name` | string or null | Native `name` when supplied. Optional. |
| `objCls` | string or null | Native `objCls` when supplied. Optional. |
| `symbol` | string or null | Native `symbol` when supplied. Optional. |

### PgxPageInfo

Independent collection scope; observed counts never imply native source totals.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Offset in this collection. |
| `returned` | integer (uint32) | Number of whole returned rows. |
| `total` | integer (uint64) | Exact source total or received bounded collection length. |
| `total_kind` | [PgxTotalKind](#pgxtotalkind) | Provenance of total. |

### PgxSection

One independent named section.

| Value | Meaning |
| --- | --- |
| `recommendations` | Complete native recommendation contexts. |
| `guidelines` | Native guideline references. |
| `diplotype` | Declared source diplotype lookup. |
| `annotations` | One ClinPGx annotation kind. |

### PgxSource

Explicit independent source.

| Value | Meaning |
| --- | --- |
| `cpic` | CPIC source-native records. |
| `clinpgx` | ClinPGx annotation records. |

### PgxTotalKind

Meaning of the stated collection total.

| Value | Meaning |
| --- | --- |
| `source` | The source returned an exact total. |
| `observed` | Length of a bounded received collection. |

### PgxUnobserved

No element shape was captured for these empty arrays. Populated values require new source proof.

Type: any.

### PhaseFilter

Explicit source phase selection, including the registry's combined phases.

Values: `early-1`, `1`, `2`, `3`, `4`, `not-applicable`, `1/2`, `2/3`.

### PhenotypeGet

Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [HpId](#hpid) | Exact HP identifier. |
| `section` | [PhenotypeSection](#phenotypesection) | Absent means the source identity card. Optional. |
| `section_limit` | integer (uint32) | Default five text/link rows or one complete association row. Optional. |
| `section_offset` | integer (uint32) | Native section row offset. Optional. |
| `snapshot` | [Text](#text) | Retained exact source reply for local text and link pages; unsupported for diseases and genes. Optional. |
| `source` | [PhenotypeSource](#phenotypesource) | HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. Optional. |

### PhenotypeSection

A named source term section.

| Value | Meaning |
| --- | --- |
| `synonyms` | Native synonyms. |
| `xrefs` | Native cross references. |
| `hierarchy` | Native direct parent and child links. |
| `diseases` | Native disease association rows. |
| `genes` | Native human gene association rows. |

### PhenotypeSource

One independent term publisher.

| Value | Meaning |
| --- | --- |
| `hpo` | Direct HPO term metadata. |
| `monarch` | Monarch term search and associations. |

### PmaNumber

Type: string.

### PmaSupplement

Type: string.

### PmaSupplementCell

Type: string.

### Pmcid

Type: string.

### Pmid

Type: string.

### PoolInfo

A search BioMCP stored so later pages read the same results: its ID, size, and when it was made and expires.

| Field | Type | Meaning |
| --- | --- | --- |
| `created` | integer (uint64) | When the stored search was made, in Unix seconds. |
| `expires` | integer (uint64) | When the stored search expires, in Unix seconds. |
| `id` | [Text](#text) | The stored search's ID. Pass it back to BioMCP as it is. |
| `size` | integer (uint32) | How many records the stored search holds. |

### PopulationDataType

Dataset sequencing sample set.

Values: `exomes`, `genomes`, `joint`.

### PopulationFrequency

One checked stratum of one source dataset release.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_count` | integer or null | Optional. |
| `allele_frequency` | [FiniteNumber](#finitenumber) or null | Optional. |
| `allele_number` | integer or null | Optional. |
| `ancestry_group` | [Text](#text) or null | Optional. |
| `attribution` | [Attribution](#attribution) |  |
| `build` | [Build](#build) |  |
| `data_type` | [PopulationDataType](#populationdatatype) or null | Optional. |
| `dataset` | [Text](#text) |  |
| `dataset_version` | [Text](#text) |  |
| `faf95` | [FiniteNumber](#finitenumber) or null | Optional. |
| `filters` | list of [Text](#text) | Optional. |
| `hemizygote_count` | integer or null | Optional. |
| `homozygote_count` | integer or null | Optional. |
| `sex` | [PopulationSex](#populationsex) or null | Optional. |
| `subject` | [VariantRef](#variantref) |  |
| `subset` | [Text](#text) or null | Optional. |

### PopulationSex

Sex stratum stated by a population source.

Values: `xx`, `xy`.

### Predicate

VA-Spec 1.1 proposition relationships in native snake_case.

Values: `is_causal_for`, `is_oncogenic_for`, `has_clinical_significance_for`, `predicts_sensitivity_to`, `predicts_resistance_to`, `is_diagnostic_inclusion_criterion_for`, `is_diagnostic_exclusion_criterion_for`, `associated_with_better_outcome_for`, `associated_with_worse_outcome_for`, `impacts_function_of`.

### PredictorContext

Unchanged native context values associated with a source field path.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [Text](#text) | Native source field path. |
| `values` | list of [Text](#text) | Every native value in source array order. |

### PredictorScore

One unmodified source score, never a locally blended assessment.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Source-owned record and dataset release. |
| `label` | [Text](#text) or null | Source label without code expansion or interpretation. Optional. |
| `method` | [Text](#text) | Published component field path. |
| `score` | [FiniteNumber](#finitenumber) | Source number, including zero. |
| `subject` | [VariantRef](#variantref) | Exact source-qualified subject. |

### Problem

A field that could not be accepted from a source record.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [ProblemCode](#problemcode) | Why the field value was not accepted. |
| `field` | string | The name of the rejected field. |
| `record` | [Text](#text) or null | The record's identifier, when known. Optional. |
| `source` | [Source](#source) | The source that supplied the record. |

### ProblemCode

Why a field value was rejected. [Reply envelope values](/reference/envelope/#problemscode) lists each value and its meaning.

### Proposition

Domain of a source claim.

Values: `pathogenicity`, `oncogenicity`, `clinical_significance`, `therapeutic_response`, `diagnostic`, `prognostic`, `functional_impact`.

### Protein

A reviewed human protein listed by the authoritative gene record.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | T | Checked primary accession. |
| `function` | list of [Text](#text) | Whole source function paragraphs, omitted when absent. Optional. |
| `length` | integer (uint32) | Sequence length in amino acids. |
| `name` | [Text](#text) | Recommended protein name. |
| `subcellular_locations` | list of [Text](#text) | Source subcellular location names. Optional. |

### ProteinAudit

Native source audit metadata, independent from local retrieval metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `entry_version` | integer or null | Native entry version, never an isoform version. Optional. |
| `first_public` | [PartialDate](#partialdate) or null | Native first-public date. Optional. |
| `last_annotation_update` | [PartialDate](#partialdate) or null | Native annotation update date. Optional. |
| `last_sequence_update` | [PartialDate](#partialdate) or null | Native sequence update date. Optional. |
| `release` | [Text](#text) or null | Native release when supplied. Optional. |
| `sequence_version` | integer or null | Native sequence version when supplied. Optional. |

### ProteinComplexes

Whole native rows with the independent unfiltered source count.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinComplexHeader](#proteincomplexheader) | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | Native source total before type selection or exact participant admission. |

### ProteinComplexHeader

Exact source complex header admitted by a native participant identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [ComplexPortalId](#complexportalid) | Native complex accession. |
| `matched_participant` | [Text](#text) | Exact native participant identifier supporting admission. |
| `name` | [Text](#text) | Native source name. |
| `participant_total` | integer (uint64) | Actual participants in this exact complex, never a complex search total. |
| `predicted` | boolean | Literal native predictedComplex flag. |
| `taxon` | integer (uint32) | Source taxonomy identifier. |
| `url` | [Text](#text) | Native complex record URL. |

### ProteinCoordinateConvention

Coordinate convention on the native feature before conversion.

Values: `one_based_inclusive`.

### ProteinDomainFragment

One native match fragment after coordinate conversion.

| Field | Type | Meaning |
| --- | --- | --- |
| `dc_status` | [Text](#text) or null | Literal discontinuity status supplied by InterPro. Optional. |
| `span` | [UncertainSpan0](#uncertainspan0) | Converted interbase span with uncertainty retained. |

### ProteinDomainLocation

One native InterPro location group retaining all discontinuous fragments.

| Field | Type | Meaning |
| --- | --- | --- |
| `fragments` | list of [ProteinDomainFragment](#proteindomainfragment) | All native fragments; never reduced to a min/max hull. Optional. |

### ProteinDomainMatch

One native integrated InterPro match; not every entry type is a domain.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Source record and release facts. |
| `entry_type` | [Text](#text) | Literal source entry type, including family, site and repeat. |
| `id` | [InterProId](#interproid) | Integrated entry accession, distinct from member signatures. |
| `locations` | list of [ProteinDomainLocation](#proteindomainlocation) | Every native location group in source order. Optional. |
| `member_signatures` | list of [ProteinMemberSignature](#proteinmembersignature) | Source member signatures remain distinct from integrated entries. Optional. |
| `name` | [Text](#text) | Native integrated name. |
| `protein` | [UniProtIsoform](#uniprotisoform) | Exact native protein identity. |
| `protein_length` | integer or null | Source protein length when supplied. Optional. |
| `taxon` | integer or null | Source taxon when supplied. Optional. |

### ProteinDomains

Whole native rows with the independent unfiltered source count.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinDomainMatch](#proteindomainmatch) | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | Native source total before type selection or exact participant admission. |

### ProteinFeature

UniProt feature with source attribution and coordinate provenance.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Source record and release facts. |
| `feature` | [UniProtFeature](#uniprotfeature) | Shared feature, including native label and residues. |
| `source_coordinates` | [ProteinCoordinateConvention](#proteincoordinateconvention) | Convention before mathematical conversion. |

### ProteinFeatures

Native UniProt feature rows, preserving exact labels and uncertain bounds.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinFeature](#proteinfeature) | Whole native rows in source order; totals apply only to this collection. |

### ProteinFunction

One whole source function paragraph and its own evidence.

| Field | Type | Meaning |
| --- | --- | --- |
| `evidence` | list of [Text](#text) | All evidence references supplied for this text. Optional. |
| `molecule` | [Text](#text) or null | Native molecule qualifier; never attributed to another isoform. Optional. |
| `text` | [Text](#text) | Whole native source text, without clinical interpretation. |

### ProteinFunctionResult

Whole source function paragraphs and their own evidence.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinFunction](#proteinfunction) | Native paragraphs in source order, without inferred interpretation. Optional. |

### ProteinGet

Read source protein cards; a symbol returns candidates without choosing a protein. Sections disclose native facts with independent row bounds; function keeps all paragraphs. No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [ProteinInput](#proteininput) | Source accessions or gene symbols, preserving caller order. |
| `section` | list of [ProteinSection](#proteinsection) | Select native sections or all bounded first pages; function keeps one whole text record. Optional. |
| `section_limit` | integer (uint32) | First-page row limit; function keeps one whole native text record and does not page its paragraphs. Optional. |

### ProteinHeader

Native entry metadata wrapping the same Protein identity used by the gene slice.

| Field | Type | Meaning |
| --- | --- | --- |
| `entry_id` | [Text](#text) | Native source entry name. |
| `gene_ids` | list of [Curie](#curie) | Only identifiers explicitly supplied by the source. Optional. |
| `gene_names` | list of [Text](#text) | Display names only; never identifier joins. Optional. |
| `protein` | [Protein](#protein) of [UniProtIsoform](#uniprotisoform) | Shared gene/protein identity fields; never independently restated. |
| `reviewed` | boolean | Native reviewed status. |
| `taxon` | integer (uint32) | Native source taxonomy identifier. |
| `url` | [Text](#text) | Source entry URL. |

### ProteinInput

Exact source accession or a checked display gene symbol needing later resolution. The schema describes the string surface; deserialization checks either identity grammar.

Type: string.

### ProteinIsoform

One source alternative-product row; all stated identifiers stay together.

| Field | Type | Meaning |
| --- | --- | --- |
| `direct_sequence` | [ProteinSequenceAvailability](#proteinsequenceavailability) | Explicit direct-sequence retrieval state, independent of descriptive status. |
| `ids` | list of [UniProtIsoform](#uniprotisoform) | All source-stated isoform identifiers, including External base identities. Optional. |
| `name` | [Text](#text) | Source isoform name. |
| `sequence_ids` | list of [Text](#text) | Source VSP sequence identifiers. Optional. |
| `sequence_status` | [ProteinIsoformStatus](#proteinisoformstatus) | Native sequence status; no sequence is inferred from it. |
| `synonyms` | list of [Text](#text) | Every source synonym. Optional. |

### ProteinIsoforms

Alternative products with every stated identifier and explicit direct sequence state.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinIsoform](#proteinisoform) | Whole native rows in source order; totals apply only to this collection. |

### ProteinIsoformStatus

Native UniProt alternative-product sequence status.

Values: `Displayed`, `Described`, `External`.

### ProteinMemberSignature

A member database signature, not an integrated IPR entry.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Text](#text) | Native signature accession. |
| `database` | [Text](#text) | Native member database name. |
| `name` | [Text](#text) or null | Native member signature name. Optional. |

### ProteinRelationQuery

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | Exact source protein sequence identity. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Text](#text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |

### ProteinSection

Named protein sections; all expands to bounded first pages.

| Value | Meaning |
| --- | --- |
| `features` | Native UniProt features. |
| `domains` | Native InterPro entry matches. |
| `isoforms` | Native alternative products. |
| `complexes` | Exact participant-admitted complex headers. |
| `structures` | Native structure cross-reference links. |
| `function` | One whole native function text record with all source paragraphs. |
| `sequence` | Direct exact-identity sequence and audit metadata. |
| `all` | Bounded first page of every named section. |

### ProteinSequence

A directly returned sequence with exact returned identity and audit facts.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | Exact returned accession; never replaced by the requested base entry. |
| `audit` | [ProteinAudit](#proteinaudit) | Native audit facts for this record. |
| `crc64` | [Text](#text) or null | Native checksum when supplied. Optional. |
| `length` | integer (uint32) | Native sequence length. |
| `md5` | [Text](#text) or null | Native checksum when supplied. Optional. |
| `sequence` | [AminoAcids](#aminoacids) | Source residues, never constructed from another entry. |

### ProteinSequenceAvailability

Direct sequence availability is a retrieval fact, never constructed from another entry.

Values: `available`, `not_available`, `not_checked`.

### ProteinSequenceResult

One directly returned source sequence, with exact identity and audit metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `sequence` | [ProteinSequence](#proteinsequence) | Native sequence record; never constructed from another entry. |

### ProteinStructureProperty

A native cross-reference property pair.

| Field | Type | Meaning |
| --- | --- | --- |
| `key` | [Text](#text) | Native property key. |
| `value` | [Text](#text) | Native property value, without interpreting residue coverage. |

### ProteinStructureRef

A source structure cross-reference link; no residue coverage claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `database` | [Text](#text) | Native cross-reference database. |
| `id` | [Text](#text) | Native cross-reference identifier. |
| `properties` | list of [ProteinStructureProperty](#proteinstructureproperty) | All native property pairs, including method, resolution and chains. Optional. |
| `url` | [Text](#text) | Publisher link; no structure file is retrieved. |

### ProteinStructures

Source structure links and native properties; no residue coverage claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinStructureRef](#proteinstructureref) | Whole native rows in source order; totals apply only to this collection. |

### PublicationType

Clinically meaningful PubMed publication types.

| Value | Meaning |
| --- | --- |
| `randomized_controlled_trial` | Randomized controlled trial. |
| `clinical_trial` | Clinical trial. |
| `clinical_trial_phase_1` | Phase one trial. |
| `clinical_trial_phase_2` | Phase two trial. |
| `clinical_trial_phase_3` | Phase three trial. |
| `clinical_trial_phase_4` | Phase four trial. |
| `meta_analysis` | Meta-analysis. |
| `systematic_review` | Systematic review. |
| `review` | Review. |
| `practice_guideline` | Practice guideline. |
| `guideline` | Guideline. |
| `case_report` | Case report. |
| `observational_study` | Observational study. |
| `multicenter_study` | Multicenter study. |
| `letter` | Letter. |
| `comment` | Comment. |
| `editorial` | Editorial. |
| `preprint` | Preprint. |
| `retracted_publication` | Retracted publication. |
| `retraction_notice` | Retraction notice. |
| `erratum` | Erratum. |

### ReactomeId

Type: string.

### Reason

Why a source could not supply data. [Reply envelope values](/reference/envelope/#sourcesreason) lists each value and its meaning.

### ReportId

Type: string.

### Representation

One source's own expression, assembly and optional source-stated interbase allele.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele` | [Allele](#allele) or null | Source-supplied allele, without local normalization or equivalence inference. Optional. |
| `build` | [Build](#build) or null | Assembly stated by the source, omitted when absent. Optional. |
| `expression` | [Expression](#expression) | Published notation kept unchanged. |
| `source` | [Text](#text) | Source asserting this form. |

### ScientificPValue

Exact normalized scientific p value; no floating-point conversion is used.

| Field | Type | Meaning |
| --- | --- | --- |
| `exponent` | integer (int64) | Source base-ten exponent; values below the double range remain exact. |
| `mantissa` | string | Source mantissa spelling, including significant trailing zeros. |

### Section

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` (T), `source` ([Source](#source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([SourceRecord](#sourcerecord) or null), `source` ([Source](#source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reason](#reason)), `source` ([Source](#source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reason](#reason)), `source` ([Source](#source)).
- `state` is `"not_applicable"`: The section does not apply to this record.

### SemanticScholarAuthorId

Type: string.

### SemanticScholarId

Type: string.

### Sequence

Type: string.

### SequenceLocation

An interbase interval on one versioned sequence accession.

| Field | Type | Meaning |
| --- | --- | --- |
| `sequence` | [Accession](#accession) | Versioned source sequence, without an inferred genome build. |
| `span` | [Span0](#span0) | Zero-based half-open bounds. |

### Source

A source of BioMCP data, identified by its canonical short name.

| Value | Meaning |
| --- | --- |
| `hgnc` | HGNC. |
| `mygene` | MyGene.info. |
| `reactome` | Reactome. |
| `wikipathways` | WikiPathways published pathway catalogs. |
| `hpa` | Human Protein Atlas. |
| `uniprot` | UniProt. |
| `interpro` | Integrated InterPro entries. |
| `complex-portal` | Complex Portal. |
| `gnomad` | gnomAD. |
| `clinvar` | ClinVar component source policy. |
| `erepo` | ClinGen Evidence Repository. |
| `gwas` | GWAS Catalog v2. |
| `clingen` | ClinGen. |
| `pubmed` | PubMed. |
| `europepmc` | Europe PMC. |
| `litvar` | NCBI LitVar2. |
| `litsense` | NCBI LitSense2. |
| `semantic-scholar` | Semantic Scholar. |
| `orcid` | ORCID public registry. |
| `pmc-oa` | PMC Open Access bucket. |
| `icite` | NIH iCite. |
| `pubtator` | PubTator. |
| `ctgov` | ClinicalTrials.gov. |
| `nci` | NCI Clinical Trials Search. |
| `civic` | CIViC. |
| `hotspots` | Cancer Hotspots residue recurrence. |
| `alphagenome` | AlphaGenome. |
| `oncokb` | OncoKB. |
| `myvariant` | MyVariant.info aggregation. |
| `car` | ClinGen Allele Registry. |
| `mutalyzer` | Mutalyzer. |
| `variantvalidator` | VariantValidator. |
| `mychem` | MyChem.info aggregation. |
| `chembl` | ChEMBL native records and carried component. |
| `opentargets` | Open Targets. |
| `openfda-label` | openFDA SPL labels. |
| `drugsfda` | Drugs@FDA application records. |
| `ema` | European Medicines Agency. |
| `who-pq` | WHO finished pharmaceutical products. |
| `ddinter` | DDInter interaction records. |
| `drugbank-open` | MyChem DrugBank Open identity subset only. |
| `drugcentral` | MyChem DrugCentral component. |
| `gtopdb` | MyChem Guide to Pharmacology component. |
| `unii` | MyChem UNII identities. |
| `ndc` | MyChem NDC package identities. |
| `openfda-xref` | MyChem openFDA cross-references only. |
| `ols` | Ontology Lookup Service. |
| `mydisease` | MyDisease.info aggregation. |
| `monarch` | Monarch associations. |
| `seer` | SEER Explorer. |
| `mondo` | MONDO published ontology content. |
| `doid` | Disease Ontology published content. |
| `ncit` | NCI Thesaurus ontology content. |
| `hpo` | HPO-published annotations. |
| `orphanet` | Reviewed Orphadata Science content. |
| `omim` | Separately licensed OMIM payload only. |
| `faers` | openFDA FAERS drug adverse event reports. |
| `vaers` | CDC WONDER vaccine report summaries. |
| `maude` | openFDA MAUDE device event reports. |
| `meddra` | Source-reported MedDRA reaction labels and versions. |
| `gmdn` | Source-carried GMDN device terminology. |
| `gtr` | NCBI Genetic Testing Registry public reports. |
| `who-ivd` | WHO prequalified in vitro diagnostics. |
| `fda-cdx` | FDA companion diagnostic device tables. |
| `fda-pma` | openFDA premarket approval submissions. |
| `fda-510k` | FDA native 510(k) clearance records. |
| `cpic` | Clinical Pharmacogenetics Implementation Consortium. |
| `clinpgx` | ClinPGx pharmacogenomics annotations. |
| `cbioportal` | Public cBioPortal with study-specific reuse terms. |
| `cellosaurus` | Cellosaurus cell line identity. |

### SourceNotice

A note from the source about how it read the query.

One of these forms:

- `kind` is `"cache_problem"`: A checked provider answer survived a local cache failure. Fields: `reason` ([CacheProblem](#cacheproblem)).
- `kind` is `"graph_truncated"`: The source still has graph pages after the bounded fetch window. Fields: `fallback` ([Text](#text)).
- `kind` is `"litvar_rsid_hint"`: An empty LitVar lookup can be repeated with a caller-supplied rsID. Fields: `message` (string).
- `kind` is `"pubmed_warning"`: PubMed supplied a named ESearch warning without changing its phrases. Fields: `name` (string), `phrases` (list of string).
- `kind` is `"not_found_words"`: PubMed did not index these words. Fields: `words` (list of [Text](#text)).

### SourceOutcome

What happened when BioMCP asked one source.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer or null | How many records matched at the source. Optional. |
| `fetched` | integer or null | How many matching records BioMCP retrieved from the source. Optional. |
| `notices` | list of [SourceNotice](#sourcenotice) | Notes from the source about the query, such as words it did not index. They are not failures. Optional. |
| `reason` | [Reason](#reason) or null | Why the source failed, or why BioMCP answered with older stored data. Optional. |
| `source` | [Source](#source) | The source BioMCP asked. |
| `state` | [SourceState](#sourcestate) | What happened: `ok`, `stale`, `syncing`, `failed` or `skipped`. |
| `term` | [Text](#text) or null | The search term BioMCP sent to the source. Optional. |
| `terms` | string | The name of the publisher's terms. |
| `terms_url` | string | The publisher's page with those terms. |
| `use` | [Use](#use) | The source's use group: `open`, `attribution`, `noncommercial` or `restricted`. The publisher's terms decide what you may do with the data. |

### SourceRecord

The source's own entry for an entity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Text](#text) | Native source identifier. |
| `url` | [Text](#text) or null | Link to that entry. Optional. |

### SourceState

State of a source actually asked by the command. [Reply envelope values](/reference/envelope/#sourcesstate) lists each value and its meaning.

### Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | integer (uint32) |  |
| `start` | integer (uint32) |  |

### SponsorType

Registry funding category facet.

Values: `nih`, `fed`, `industry`, `other`.

### Statement

One source claim, kept in its own framework and context.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_origin` | [Term](#term) or null | Source-stated origin. Optional. |
| `attribution` | [Attribution](#attribution) | One source record and its provenance. |
| `citations` | list of [Pmid](#pmid) | Source publications. Optional. |
| `classification` | [Term](#term) | Whole source classification label. |
| `condition` | [Concept](#concept) or null | Source disease or tumor context. Optional. |
| `criteria` | list of [Text](#text) | Source criterion labels in source order. Optional. |
| `direction` | [Direction](#direction) or null | Source-stated assessment direction. Optional. |
| `gene_context` | [GeneRef](#generef) or null | Source gene context. Optional. |
| `predicate` | [Predicate](#predicate) or null | Source-stated relationship. Optional. |
| `proposition` | [Proposition](#proposition) | Claim domain. |
| `strength` | [Term](#term) or null | Source evidence level or review status. Optional. |
| `subject` | [VariantRef](#variantref) | Source-owned subject reference. |
| `therapies` | list of [Concept](#concept) | Source therapies in their native order. Optional. |

### StatisticEstimate

Source estimate and uncertainty without computed quantities.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer or null | Native count; no inferred denominator. Optional. |
| `lower_ci` | [FiniteNumber](#finitenumber) or null | Native lower confidence bound. Optional. |
| `standard_error` | [FiniteNumber](#finitenumber) or null | Native standard error. Optional. |
| `upper_ci` | [FiniteNumber](#finitenumber) or null | Native upper confidence bound. Optional. |
| `value` | [FiniteNumber](#finitenumber) or null | Native estimate. Optional. |

### StatisticSelector

One source-native selector without a local interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [Text](#text) | Source selector code. |
| `label` | [Text](#text) or null | Publisher label when supplied. Optional. |
| `name` | [Text](#text) | Source field name. |

### Status

Overall command outcome. [Reply envelope values](/reference/envelope/#status) lists each value and its meaning.

### Step

A command you can run next. `args` is the command without the leading `biomcp`; `kind` says what it does.

| Field | Type | Meaning |
| --- | --- | --- |
| `args` | list of string | The command's words without the leading `biomcp`. The same list works as the MCP tool's arguments. |
| `kind` | [StepKind](#stepkind) | What the command does, such as `page` or `section`. |

### StepKind

What a next command does. [Reply envelope values](/reference/envelope/#nextkind) lists each value and its meaning.

### Strand

Source-stated forward or reverse strand.

| Value | Meaning |
| --- | --- |
| `+` | Forward strand. |
| `-` | Reverse strand. |

### StudyBlobSha

Type: string.

### StudyFile

Publisher file reference; repository blob bytes never mean data-content bytes.

| Field | Type | Meaning |
| --- | --- | --- |
| `downloadUrl` | [Text](#text) or null | Native download link, absent for directory entries. Optional. |
| `htmlUrl` | [Text](#text) | Publisher browser link. |
| `kind` | [StudyFileKind](#studyfilekind) | Native file or directory kind. |
| `name` | [Text](#text) | Native repository entry name. |
| `path` | [Text](#text) | Exact native repository path. |
| `repositoryBlobSize` | integer (uint64) | GitHub size in repository blob bytes; LFS content size is unknown. |
| `sha` | [StudyBlobSha](#studyblobsha) | Repository blob or tree revision, not a data-content checksum. |

### StudyFileKind

Native GitHub contents entry kind.

Values: `file`, `dir`.

### StudyFiles

One native repository directory page; blob sizes do not describe data content.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer (uint64) | Directory acquisition wall time in Unix seconds. |
| `items` | list of [StudyFile](#studyfile) | Source-ordered repository file references. |
| `license_notice` | [Text](#text) or null | Native LICENSE publisher link, independent of the current file page. Optional. |
| `page` | [PageInfo](#pageinfo) | Bounds within the complete received directory. |
| `study_id` | [StudyId](#studyid) | Exact study directory scope. |

### StudyGet

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [StudyId](#studyid) | Exact source study ID; no default or repair. |
| `section` | [StudySection](#studysection) or null | Omit for the card; all returns bounded first pages only. Optional. |
| `section_limit` | integer (uint32) | Maximum records in each selected section. Optional. |
| `section_offset` | integer (uint32) | Offset within one named section; all requires zero. Optional. |

### StudyHeader

Compact source study identity; source codes and counts retain their own scope.

| Field | Type | Meaning |
| --- | --- | --- |
| `allSampleCount` | integer (uint64) | Native allSampleCount, never a patient or profiled-gene denominator. |
| `cancerTypeId` | [CbioportalCancerTypeId](#cbioportalcancertypeid) | cBioPortal namespace only; no ontology inference. |
| `citation` | string or null | Native citation, absent for unpublished records. Optional. |
| `name` | [Text](#text) | Native study name. |
| `pmids` | list of [Pmid](#pmid) or null | Every validated source PMID in native order, including repetitions; absent remains absent. Optional. |
| `referenceGenome` | [Text](#text) | Native study metadata label, never a coordinate assembly. |
| `studyId` | [StudyId](#studyid) | Exact case-sensitive source identity. |

### StudyId

Type: string.

### StudyProfile

Molecular profile reference with exact study membership.

| Field | Type | Meaning |
| --- | --- | --- |
| `datatype` | [Text](#text) | Extensible native data-type label. |
| `description` | string | Source description. |
| `genericAssayType` | [Text](#text) or null | Native generic assay type, absent outside those profiles. Optional. |
| `molecularAlterationType` | [Text](#text) | Extensible source alteration-type label. |
| `molecularProfileId` | [StudyProfileId](#studyprofileid) | Exact native profile ID; never constructed from a study ID. |
| `name` | string | Source profile name. |
| `patientLevel` | boolean | Native patient-level flag, not a unit inferred for other records. |
| `showProfileInAnalysisTab` | boolean | Native profile display setting. |
| `studyId` | [StudyId](#studyid) | Native profile study scope. |

### StudyProfileId

Type: string.

### StudyProfiles

One independently acquired, bounded native metadata collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer (uint64) | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. |
| `items` | list of [StudyProfile](#studyprofile) | Complete ordered metadata records, never member sample IDs. |
| `page` | [PageInfo](#pageinfo) | Bounds and source-stated total for this collection. |
| `study_id` | [StudyId](#studyid) | Exact study scope of this collection. |

### StudySampleList

Sample-list metadata without a representation for member sample IDs.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Text](#text) | Extensible native list category. |
| `description` | string | Native list description. |
| `name` | string | Native list name. |
| `sampleCount` | integer or null | Stated source sampleCount when supplied, never computed from member IDs. Optional. |
| `sampleListId` | [StudySampleListId](#studysamplelistid) | Exact native list ID. |
| `studyId` | [StudyId](#studyid) | Native study scope. |

### StudySampleListId

Type: string.

### StudySampleLists

One independently acquired, bounded native metadata collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer (uint64) | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. |
| `items` | list of [StudySampleList](#studysamplelist) | Complete ordered metadata records, never member sample IDs. |
| `page` | [PageInfo](#pageinfo) | Bounds and source-stated total for this collection. |
| `study_id` | [StudyId](#studyid) | Exact study scope of this collection. |

### StudySection

Supported bounded study metadata sections; gene counts are deferred.

| Value | Meaning |
| --- | --- |
| `profiles` | Five exact native molecular profile references. |
| `sample-lists` | Five sample-list metadata records, without member sample IDs. |
| `files` | Five repository file references; no automatic download. |
| `all` | Each named section's bounded first page; never a gene aggregate request. |

### StudySnapshot

Metadata catalog revision and acquisition time; never a biological release date.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer (uint64) | Catalog acquisition wall time in Unix seconds. |
| `revision` | [Text](#text) | Immutable bundle revision. |

### StudyType

Source-native registry value.

Values: `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS`.

### SubjectQualification

Provenance of the subject's accession qualification, without allele normalization.

Values: `adapter_derived_reference`.

### Summary

A whole source-provided summary with attribution and stated date precision.

| Field | Type | Meaning |
| --- | --- | --- |
| `dated` | [PartialDate](#partialdate) or null | Date stated by the source, if any. Optional. |
| `source` | [SummarySource](#summarysource) | The summary's attribution, independent of the fetching provider. |
| `text` | [Text](#text) | Entire summary text. |

### SummarySource

The attribution on a source-provided summary, independent of fetch providers.

| Value | Meaning |
| --- | --- |
| `refseq` | RefSeq attribution. |

### SyncQuery

Explicit source synchronization selector.

| Field | Type | Meaning |
| --- | --- | --- |
| `source` | [Source](#source) | Source whose complete identity data is synchronized. |

### Term

A source label with optional explicitly supported coding.

| Field | Type | Meaning |
| --- | --- | --- |
| `coding` | [Coding](#coding) or null | Documented framework coding. Optional. |
| `label` | [Text](#text) | Original label. |

### Text

Text that is never empty, with spaces removed from both ends.

Type: string.

### TrialConditionResolution

Exact condition identity used for one trial request, without eligibility inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `alias` | [DiseaseRelation](#diseaserelation) or null | Native relation proving a requested alias, when required. Optional. |
| `attribution` | [Attribution](#attribution) or null | Resolver record provenance, independent of the trial provider. Optional. |
| `components` | list of [ComponentLabel](#componentlabel) | Admitted resolver components and their own terms. Optional. |
| `concept` | [Concept](#concept) or null | Returned concept with its unchanged native primary label. Optional. |
| `input` | [Curie](#curie) | Caller-selected CURIE, never a source-name search. |
| `ncit_candidates` | list of [DiseaseRelation](#diseaserelation) | Native NCIt mapping rows; duplicate targets keep their independent provenance. Optional. |
| `resolver` | [Source](#source) or null | Named exact resolver; absent for explicit NCIt input. Optional. |
| `selected_ncit` | [Curie](#curie) or null | Explicit or unique source-provided NCIt identifier actually sent to NCI. Optional. |

### TrialDocument

A posted source document; a URL never comes from caller-supplied filename text.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [PartialDate](#partialdate) or null | Source document date. Optional. |
| `filename` | [Text](#text) | Exact advertised portable source filename. |
| `has_icf` | boolean or null | Source says this contains an informed consent form. Optional. |
| `has_protocol` | boolean or null | Source says this contains a protocol. Optional. |
| `has_sap` | boolean or null | Source says this contains a statistical analysis plan. Optional. |
| `kind` | string | Native document type abbreviation. |
| `label` | string or null | Source label. Optional. |
| `license` | [License](#license) | No blanket reuse license is inferred from registry availability. |
| `size` | integer or null | Source byte count, when supplied. Optional. |
| `uploaded` | string or null | Original upload timestamp, without guessing a timezone. Optional. |
| `url` | [Text](#text) | Fixed registry CDN URL for this manifest entry. |

### TrialGet

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [NctId](#nctid) | One to twenty exact NCT identifiers, in caller order. |
| `limit` | integer (uint32) | Whole-record count per source collection page; text fragments follow the text budget. Optional. |
| `offset` | integer (uint32) | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. Optional. |
| `pool` | [Text](#text) | Checked stored source record, supplied by a continuation. Optional. |
| `section` | list of [TrialSection](#trialsection) | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. Optional. |
| `source` | [TrialSource](#trialsource) | Explicit provider; defaults to ClinicalTrials.gov and never falls back. Optional. |

### TrialParameter

One exact source request parameter, in canonical name order.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | string | Native parameter name. |
| `value` | string | Exact unencoded parameter value. |

### TrialPhase

Source-native registry value.

Values: `EARLY_PHASE1`, `PHASE1`, `PHASE2`, `PHASE3`, `PHASE4`, `NA`.

### TrialRecords

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [TrialGet](#trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of T | Whole records, in the source's order. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### TrialSearch

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

| Field | Type | Meaning |
| --- | --- | --- |
| `age` | integer (uint32) | Age in years; unstated source bounds count as no limit. Optional. |
| `condition` | [Text](#text) | Registry condition text, or an NCIt code for NCI; use --keyword for NCI free text. Optional. |
| `condition_id` | [Curie](#curie) | Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. Optional. |
| `count_only` | boolean | Source total only; no returned study rows. Optional. |
| `criteria` | [Text](#text) | Search literal registry eligibility text for discovery only; no eligibility judgment. Optional. |
| `distance` | [Text](#text) | Source distance with explicit mi or km units. Optional. |
| `facility` | [Text](#text) | Exact registry facility field; cannot combine with radius. Optional. |
| `gene` | [GeneInput](#geneinput) | Resolve one checked gene for registry text discovery only; no eligibility judgment. Optional. |
| `has_results` | boolean | Require posted source results. Optional. |
| `id` | [NctId](#nctid) | Exact NCT identity in a conjunction with other filters. Optional. |
| `intervention` | [Text](#text) | Literal source intervention text. Optional. |
| `keyword` | [Text](#text) | NCI free text; explicit keyword mode never substitutes for a disease identity. Optional. |
| `lat` | [Text](#text) | Radius center latitude in degrees. Optional. |
| `limit` | integer (uint32) | Headers per page; NCI applies its own lower publisher limit. Optional. |
| `lon` | [Text](#text) | Radius center longitude in degrees. Optional. |
| `next_page` | [Text](#text) | Opaque registry continuation; keep filters unchanged. Optional. |
| `phase` | [PhaseFilter](#phasefilter) | Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3. Optional. |
| `query` | [Text](#text) | General registry discovery text. Optional. |
| `sex` | [TrialSex](#trialsex) | Registry sex facet. Optional. |
| `source` | [TrialSource](#trialsource) | Explicit provider; defaults to ClinicalTrials.gov and never falls back. Optional. |
| `sponsor` | [Text](#text) | Exact lead sponsor field. Optional. |
| `sponsor_type` | [SponsorType](#sponsortype) | Registry sponsor funding facet. Optional. |
| `status` | [TrialStatus](#trialstatus) | Exact registry overall status. Optional. |
| `study_type` | [StudyType](#studytype) | Exact registry study type. Optional. |
| `updated_since` | [PartialDate](#partialdate) | First included registry update date. Optional. |
| `updated_until` | [PartialDate](#partialdate) | Last included registry update date. Optional. |
| `variant` | [Text](#text) | Literal variant text for registry discovery only; no eligibility judgment. Optional. |

### TrialSection

Available source protocol sections.

| Value | Meaning |
| --- | --- |
| `eligibility` | Intact criteria and structured age/sex fields. |
| `outcomes` | Primary, secondary and other outcome records. |
| `arms` | Arms and interventions with original relationships. |
| `locations` | Whole source sites with their attached contacts. |
| `contacts` | Central contacts and overall officials in their original roles. |
| `references` | Registry publication identities and native reference types. |
| `documents` | Posted document metadata and source URLs. |
| `all` | Every implemented trial section, each with independent continuations. |
| `diseases` | Native NCI disease concepts, including source enrichment metadata. |
| `summary` | Complete NCI summary, paged by original character offsets. |

### TrialSex

Source-native registry value.

Values: `ALL`, `FEMALE`, `MALE`.

### TrialSource

Independent trial providers; results are never folded or used as fallbacks.

Values: `ctgov`, `nci`.

### TrialStatus

Source-native registry value.

Values: `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN`.

### UncertainSpan0

Zero-based interbase span retaining uncertain boundary ranges.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | [Bound](#bound) |  |
| `start` | [Bound](#bound) |  |

### UniProtAcc

Type: string.

### UniProtFeature

Shared native feature contract without source label remapping.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternative` | list of [AminoAcids](#aminoacids) | Every stated alternative, without translation. Optional. |
| `description` | [Text](#text) or null | Complete source description. Optional. |
| `evidence` | list of [Text](#text) | All original source evidence references. Optional. |
| `id` | [Text](#text) or null | Native feature identifier. Optional. |
| `kind` | [Term](#term) | Source feature label; coding only for documented exact feature types. |
| `original` | [AminoAcids](#aminoacids) or null | Source original residues, when supplied. Optional. |
| `protein` | [UniProtIsoform](#uniprotisoform) | Exact source sequence identity. |
| `span` | [UncertainSpan0](#uncertainspan0) | Zero-based interbase coordinates retaining uncertain ranges. |

### UniProtIsoform

Type: string.

### UnmappedAssertion

Source classification retained when its category has no approved shared proposition.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Attribution](#attribution) | Original record provenance. |
| `classification` | [Text](#text) | Unchanged source label. |
| `conditions` | list of [AssertionCondition](#assertioncondition) | Native conditions, including identifiers without supplied labels. Optional. |
| `review_status` | [Text](#text) or null | Native review status. Optional. |

### Use

The source's use group. It describes the source, not the rights in any one record. [Reply envelope values](/reference/envelope/#sourcesuse) lists each value and its meaning.

### VaersSection

One independent CDC WONDER grouping.

| Value | Meaning |
| --- | --- |
| `symptoms` | Native reported symptom rows. |
| `seriousness` | Reports classified by native seriousness. |
| `age` | Native age groups. |

### Variant

Variant entity with source-stated identities, forms and genes; no disclosure policy.

| Field | Type | Meaning |
| --- | --- | --- |
| `genes` | list of [GeneRef](#generef) | Genes stated by the sources. Optional. |
| `ids` | list of [VariantId](#variantid) | All source identifiers, without inferred aliases. Optional. |
| `representations` | list of [Representation](#representation) | Every source-qualified representation. Optional. |

### VariantAssertion

A native record/domain pair, never a vote or merged classification.

| Field | Type | Meaning |
| --- | --- | --- |
| `detail` | [AssertionDetail](#assertiondetail) or null | Original source detail, returned only for an explicitly selected record. Optional. |
| `domain` | [AssertionDomain](#assertiondomain) | Source classification domain. |
| `kind` | [AssertionKind](#assertionkind) | Aggregate/submission/document context. |
| `statement` | [Statement](#statement) or null | Supported mapping into the common model. Optional. |
| `statement_unmapped` | boolean | Native category has no approved common-model proposition. Optional. |
| `unmapped` | [UnmappedAssertion](#unmappedassertion) or null | Native assertion whose category the common model cannot represent. Optional. |

### VariantAssertions

One source-ordered page, with native record and classification boundaries intact.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertions` | list of [VariantAssertion](#variantassertion) | Source assertion rows in native order. |
| `next` | [VariantAssertionsGet](#variantassertionsget) or null | Typed continuation preserving source and selectors. Optional. |
| `page` | [PageInfo](#pageinfo) | Page bounds over native rows. |

### VariantAssertionsGet

Page assertions from one explicitly selected evidence source.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertion` | [Text](#text) or null | Native assertion accession or document ID. Optional. |
| `input` | [VariantInput](#variantinput) | Checked Variation ID for ClinVar, or CAid for ERepo. |
| `limit` | integer (uint32) | Assertions per page; omitted uses the native source default. Optional. |
| `offset` | integer (uint32) | Assertions to skip in source order. Optional. |
| `source` | [AssertionSource](#assertionsource) | One native source; no fallback or text search. |
| `version` | [Text](#text) or null | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. Optional. |

### VariantCivic

CIViC molecular profiles with their original membership and separate source claims.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | Continue this section without changing the input or source selectors. Optional. |
| `page` | [PageInfo](#pageinfo) | Disclosure bounds for this section. |
| `profiles` | list of [CivicProfile](#civicprofile) | Source profiles in native order; name candidates carry no attached statements. |

### VariantComponent

Metadata for a component carried by an aggregator.

| Field | Type | Meaning |
| --- | --- | --- |
| `aggregator_build` | [Text](#text) | Aggregator build version. |
| `aggregator_built` | [Text](#text) | Aggregator build timestamp, distinct from a component release. |
| `license` | [Text](#text) | Component license URL, without any inferred grant. |
| `name` | [Text](#text) | Component name from aggregator metadata. |
| `note` | [Text](#text) or null | Whole source snapshot warning. Optional. |
| `terms` | [ComponentTerms](#componentterms) | Independently reviewed publisher terms; no blanket aggregator grant. |
| `uploaded` | [Text](#text) or null | Component upload timestamp when the source states one. Optional. |
| `uploads` | list of [ComponentUpload](#componentupload) | Source-named component upload timestamps when the publisher supplies a map. Optional. |
| `version` | [Text](#text) or null | Component version; omitted when the source reports none. Optional. |

### VariantConservation

Source conservation measurements; no local pathogenicity inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [VariantComponent](#variantcomponent) | Independent component versions and licenses. |
| `context` | list of [PredictorContext](#predictorcontext) | Original context arrays, without inferred cross-field alignment. Optional. |
| `scores` | list of [PredictorScore](#predictorscore) | Every retained source measurement in declared field order. |
| `subject_qualification` | [SubjectQualification](#subjectqualification) | How the source chromosome label was reference-qualified. |

### VariantCosmic

Historical COSMIC fields carried by MyVariant, without direct COSMIC access.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [VariantComponent](#variantcomponent) | Snapshot version, dates, warning and terms. |
| `next` | [VariantGet](#variantget) or null | Typed next page for the same identity and section selection. Optional. |
| `records` | list of [CosmicRecord](#cosmicrecord) | Native historical records in source order. |
| `total` | integer (uint64) | Number of native historical fragments before paging. |

### VariantGet

Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. Use search variant --gene BRAF --protein V600E for shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](#build) or null | Required for chromosome notation; no assembly is guessed. Optional. |
| `civic_cursor` | [Text](#text) or null | Native CIViC cursor, retained unchanged. Optional. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | Native CIViC claim collection within the selected profile. Optional. |
| `civic_profile` | integer or null | Selected CIViC molecular profile for native claim paging. Optional. |
| `input` | [VariantInput](#variantinput) | Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS. |
| `limit` | integer (uint32) | Candidate rows per page; more remain reachable through typed next commands. Optional. |
| `offset` | integer (uint32) | Candidate rows to skip in source order. Optional. |
| `resolver` | [VariantResolver](#variantresolver) | Resolver for reference-qualified notation; never repairs the input locally. Optional. |
| `section` | list of [VariantSection](#variantsection) | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. Optional. |
| `section_limit` | integer (uint32) | Rows per named source section page; omitted uses the native source default. Optional. |
| `section_offset` | integer (uint32) | Source rows to skip independently of identity candidate paging. Optional. |
| `tumor_type` | [Text](#text) or null | Explicit tumor context for keyed OncoKB annotation. Optional. |

### VariantGwas

One page of exact rsID catalog associations.

| Field | Type | Meaning |
| --- | --- | --- |
| `associations` | list of [GwasAssociation](#gwasassociation) | Native source associations, without inferred clinical meaning. |
| `continuation` | [VariantGet](#variantget) or null | Continue this section using the same rsID and source selectors. Optional. |
| `page` | [PageInfo](#pageinfo) | Exact source page bounds and total. |
| `rs_id` | [VariantId](#variantid) | Exact source rsID. |

### VariantHeader

Consumer identity header with an exact source selection and compact annotation entity.

| Field | Type | Meaning |
| --- | --- | --- |
| `component_terms` | list of [ComponentLabel](#componentlabel) | Terms for carried component identifiers on this consumer header. Optional. |
| `identity` | [Variant](#variant) | Source-qualified variant entity. |
| `myvariant` | [Variant](#variant) or null | Compact MyVariant annotations, attributed by their field name. Optional. |
| `record` | [SourceRecord](#sourcerecord) | Exact source record, including source keys that are expressions rather than identifiers. |
| `requested` | [VariantId](#variantid) or null | Checked caller ID used by a source-only section, without asserting resolved identity. Optional. |
| `sections` | [VariantSections](#variantsections) or null | Explicitly requested source sections for this exact candidate. Optional. |
| `source` | [Source](#source) | Source responsible for this identity. |

### VariantHotspots

Source residue recurrence counts without allele-frequency or clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | Continue this section without changing identity selectors. Optional. |
| `page` | [PageInfo](#pageinfo) | Disclosure bounds for this section. |
| `reason` | [Reason](#reason) or null | Source rows remain candidates when no transcript/consequence join exists. Optional. |
| `records` | list of [HotspotRecord](#hotspotrecord) | Native residue records in source order. |

### VariantId

Type: string.

### VariantInput

Type: string.

### VariantOncokb

Keyed OncoKB annotation, preserving the caller's assembly and tumor context.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_exist` | boolean or null | Native alternate allele curation flag. Optional. |
| `build` | [Build](#build) | Explicit caller assembly, checked against the response query. |
| `data_version` | [Text](#text) or null | Native dataset version. Optional. |
| `gene_exist` | boolean or null | Source gene curation flag. Optional. |
| `hgvsg` | [Text](#text) | Exact genomic query sent to OncoKB. |
| `hotspot` | boolean or null | Source hotspot flag, without an actionability conclusion. Optional. |
| `last_update` | [Text](#text) or null | Native release date, kept in its published spelling. Optional. |
| `next` | [VariantGet](#variantget) or null | Continue without changing identity, build, tumor context or source. Optional. |
| `page` | [PageInfo](#pageinfo) | Disclosure bounds over the records, independent of identity paging. |
| `records` | list of [OncokbVariantRecord](#oncokbvariantrecord) | Source claims and text in native field order. |
| `tumor_type` | [Text](#text) | Explicit caller tumor type, checked against the response query. |
| `variant_exist` | boolean | Native exact variant curation flag; never a clinical conclusion. |
| `vus` | boolean or null | Source investigated-but-unknown flag, never used to create a classification. Optional. |

### VariantPopulation

Bounded population summary with independent optional sequencing datasets.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Build](#build) | Source dataset assembly. |
| `dataset` | [Text](#text) | Source dataset selector. |
| `exome` | [GnomadCounts](#gnomadcounts) or null | Exome observation; omitted when the source dataset is null. Optional. |
| `genome` | [GnomadCounts](#gnomadcounts) or null | Genome observation; omitted when the source dataset is null. Optional. |
| `variant` | [Text](#text) | Source allele identifier. |

### VariantPredictors

Source prediction scores and the terms of their individual components.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [VariantComponent](#variantcomponent) | Independent component versions and licenses. |
| `context` | list of [PredictorContext](#predictorcontext) | Original context arrays, without inferred cross-field alignment. Optional. |
| `scores` | list of [PredictorScore](#predictorscore) | Every retained source score in declared field order. |
| `subject_qualification` | [SubjectQualification](#subjectqualification) | How the source chromosome label was reference-qualified. |

### VariantRef

Checked source subject reference; at least one source identity or label is required.

One of these forms:

- `level` is `"allele"`: Exact allele identifier or reference-qualified notation. Fields: `hgvs` ([HgvsText](#hgvstext) or null), `id` ([VariantId](#variantid) or null).
- `level` is `"protein"`: Protein-level assertion, without genomic conversion. Fields: `gene` ([GeneRef](#generef)), `hgvs` ([HgvsText](#hgvstext) or null), `label` ([Text](#text) or null).
- `level` is `"profile"`: Compound source profile, without an inferred allele. Fields: `label` ([Text](#text)), `members` (list of [LeafRef](#leafref)), `record` ([Text](#text)).

### VariantResolution

Identity resolution preserves candidates and source validation outcomes.

Values: `unresolved`, `resolved`, `candidates`, `absent`, `rejected`, `unavailable`.

### VariantResolver

Explicit source service for reference-qualified notation.

Values: `car`, `mutalyzer`, `variantvalidator`.

### VariantSection

Named variant source sections, without automatic expansion of identity cards.

| Value | Meaning |
| --- | --- |
| `clinvar` | ClinVar native assertions. |
| `erepo` | ClinGen native assertions. |
| `population` | gnomAD population observations. |
| `gwas` | GWAS Catalog associations. |
| `civic` | CIViC molecular-profile evidence. |
| `oncokb` | Keyed OncoKB annotation with explicit tumor context and build. |
| `hotspots` | Cancer Hotspots candidate residue counts; no established exact allele join or clinical actionability. |
| `predict` | MyVariant source predictor scores. |
| `conservation` | MyVariant conservation scores. |
| `cosmic` | Historical COSMIC fragments carried by MyVariant; parent-to-fragment allele identity remains unverified. |
| `all` | Every implemented variant section. |

### VariantSections

Source-specific sections, absent unless explicitly selected.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic` | [Section](#section) of [VariantCivic](#variantcivic) or null | Explicit civic source result. Optional. |
| `clinvar` | [Section](#section) of [VariantAssertions](#variantassertions) or null | ClinVar native assertion rows. Optional. |
| `conservation` | [Section](#section) of [VariantConservation](#variantconservation) or null | Native conservation scores. Optional. |
| `cosmic` | [Section](#section) of [VariantCosmic](#variantcosmic) or null | Historical MyVariant-associated COSMIC fragments. Optional. |
| `erepo` | [Section](#section) of [VariantAssertions](#variantassertions) or null | ClinGen native assertion rows. Optional. |
| `gwas` | [Section](#section) of [VariantGwas](#variantgwas) or null | Explicit GWAS Catalog source result. Optional. |
| `hotspots` | [Section](#section) of [VariantHotspots](#varianthotspots) or null | Explicit hotspots source result. Optional. |
| `oncokb` | [Section](#section) of [VariantOncokb](#variantoncokb) or null | Explicit keyed OncoKB source result. Optional. |
| `population` | [Section](#section) of [VariantPopulation](#variantpopulation) or null | Explicit population source result. Optional. |
| `predict` | [Section](#section) of [VariantPredictors](#variantpredictors) or null | Native prediction scores. Optional. |

### WhoProductId

Type: string.

### WikiPathwaysId

Type: string.

## Next

[Reply schemas](/reference/schemas/) lists every reply and the commands that return it. [Results](/results/) explains how to read a reply from the command line.
