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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | A problem with BioMCP's local cache: `busy`, `database` or `version`. The sources may still have answered. |
| `next` | list of [Step](#step) | no | Commands you can run next, such as the next page or another section. |
| `page` | [PageInfo](#pageinfo) or null | no | For a search page: how many results it skipped, returned and matched in all. |
| `problems` | list of [Problem](#problem) | no | Fields BioMCP could not accept from a source record, each with its reason. |
| `query` | the query type of each schema | yes | The request as BioMCP read it from the command's arguments. |
| `result` | the result type of each schema, or null | no | The data the command found. Missing when there is none to return. |
| `schema` | string | yes | The reply schema ID, such as `biomcp.gene_cards/4`. |
| `sources` | list of [SourceOutcome](#sourceoutcome) | no | One entry for each source the command asked, with what happened. |
| `status` | [Status](#status) | yes | How the command went: `complete`, `partial`, `absent` or `failed`. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `drug_index` | integer or null | no | Absolute source-array index; required only for identifier sections. |
| `id` | [ReportId](#reportid) | yes | Published report ID with no source prefix. |
| `section` | [AdverseEventSection](#adverseeventsection) or null | no | FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only. |
| `section_limit` | integer (uint32) | no | Whole source rows per section page. |
| `section_offset` | integer (uint32) | no | Offset within the selected native section. |
| `snapshot` | [Text](#text) or null | no | Checked retained report snapshot for provider-free continuation. |
| `source` | `"faers"` or `"maude"` | no | FAERS by default; MAUDE is explicit. |

### AdverseEventHeader

Small source report header without a drug title, flattened lists or count buckets.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `event_type` | string or null | no | Native event_type string; present empty values remain present. |
| `first_received` | string or null | no | Native first_received string; present empty values remain present. |
| `id` | [ReportId](#reportid) | yes | Exact published source identity. |
| `serious` | string or null | no | Native serious string; present empty values remain present. |
| `source` | [AdverseEventSource](#adverseeventsource) | yes | Explicit report family. |
| `version` | string or null | no | Native version string; present empty values remain present. |

### AdverseEventRecords

One complete-row page tied to the retained report snapshot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Native section offset. |
| `records` | list of T | yes | Source ordered whole rows. |
| `total` | integer (uint64) | yes | Complete selected section length, not report or patient count. |

### AdverseEventSearch

Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

| Field | Type | Always present | Meaning |
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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alternate` | [Sequence](#sequence) | yes |  |
| `location` | [SequenceLocation](#sequencelocation) | yes |  |
| `reference` | [Sequence](#sequence) | yes |  |

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

| Field | Type | Always present | Meaning |
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
| `types` | list of [PublicationType](#publicationtype) | yes | Clinical publication types. |
| `types_unknown` | boolean | no | The provider does not supply checked publication types. |

### ArticleInput

Type: string.

### ArticleItem

One original pool position, including a summary hole.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cited_by` | integer or null | no | iCite citing-paper count, only under citation ordering. |
| `contexts` | [Capped](#capped) of [Text](#text) or null | no | First citation context and the source's complete context count. |
| `found_by` | object | no | Native one-based source positions, disclosed only for multiple sources. |
| `header` | [ArticleHeader](#articleheader) or null | no | Citation metadata when accepted. |
| `influential` | boolean or null | no | Provider's influential flag; absent is distinct from false. |
| `intents` | list of [CitationIntent](#citationintent) or null | no | Citation intents in provider order; absent means not reported. |
| `key` | [ArticleKey](#articlekey) | no | Stable checked paper identity when the provider supplies one. |
| `pmid` | [Pmid](#pmid) | no | The selected PMID, retained when summary mapping fails. |
| `sentence` | [ArticleSentence](#articlesentence) or null | no | First matching sentence in LitSense2 provider order. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `bytes` | integer (uint64) | yes | Exact byte count. |
| `parts` | integer or null | no | Number of ordered text parts; absent for original supplement bytes. |
| `path` | [Text](#text) | yes | Absolute caller destination. |
| `sha256` | [Text](#text) | yes | SHA-256 of the saved bytes. |
| `url` | [Text](#text) | yes | Source URL for the original material. |

### ArticleSentence

Source-provided sentence resemblance, without a claim of clinical relevance.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `section` | [Text](#text) or null | no | Source section, when present. |
| `text` | [Text](#text) | yes | Source sentence text. |

### AssertionAssessment

Source assessment labels and identifiers remain independent.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `criterion` | [Text](#text) | yes | Native criterion code. |
| `criterion_id` | [Text](#text) or null | no | Native criterion identifier. |
| `criterion_iri` | [Text](#text) or null | no | Native criterion IRI. |
| `default_strength` | [Text](#text) or null | no | Native default strength, distinct from an applied evidence strength. |
| `default_strength_id` | [Text](#text) or null | no | Native default-strength identifier; not an applied evidence strength. |
| `default_strength_iri` | [Text](#text) or null | no | Native default-strength IRI. |
| `outcome` | [Text](#text) | yes | Source outcome label. |
| `outcome_conflict` | boolean | no | The supplied outcome identifier contradicts its supplied Met/Not Met label. |
| `outcome_id` | [Text](#text) or null | no | Source outcome identifier, including known conflicting ID/label pairs. |
| `outcome_iri` | [Text](#text) or null | no | Source outcome IRI. |

### AssertionCondition

A native condition with independently optional identifier and label.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [Text](#text) or null | no | Native accession text, preserved even when no CURIE namespace is recognized. |
| `database` | [Text](#text) or null | no | Native database name without inventing a CURIE prefix. |
| `id` | [Curie](#curie) or null | no | Identifier exactly supplied by the source. |
| `label` | [Text](#text) or null | no | Label only when supplied. |

### AssertionConditionRelationship

A source trait relationship without assigning a clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `related_conditions` | list of [AssertionCondition](#assertioncondition) | yes | Conditions named by the related source trait. |
| `relationship_type` | [Text](#text) or null | no | Relationship type exactly supplied by the source. |
| `source_conditions` | list of [AssertionCondition](#assertioncondition) | yes | Conditions on the source trait that owns this relationship. |

### AssertionDetail

Source details that do not fit the compact shared statement.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `approved` | [PartialDate](#partialdate) or null | no | Source approval date, distinct from evaluation and publication. |
| `citations` | list of [Pmid](#pmid) | no | Source citations, including assertion-method citations. |
| `comments` | list of [Text](#text) | no | Native classification comments, without extracting clinical claims. |
| `condition_relationships` | list of [AssertionConditionRelationship](#assertionconditionrelationship) | no | Native trait relationships, kept separate from assertion conditions. |
| `conditions` | list of [AssertionCondition](#assertioncondition) | no | All source conditions; ID-only conditions stay ID-only. |
| `document` | [AssertionDocument](#assertiondocument) or null | no | Source document facts for an explicitly selected historical version. |
| `impact_assertion_type` | [Text](#text) or null | no | Native somatic impact assertion type, without deriving a predicate. |
| `impact_clinical_significance` | [Text](#text) or null | no | Native somatic significance, independent of the tier label. |
| `impact_therapies` | [Text](#text) or null | no | Native therapy text, without splitting or assigning it to a predicate. |
| `methods` | list of [Text](#text) | no | Assertion method labels, not inferred from clinical testing. |
| `origins` | list of [Text](#text) | no | Source origin labels without reducing multiple observations. |
| `published` | [PartialDate](#partialdate) or null | no | Source publication date; never an inferred evaluation date. |
| `summary` | [Text](#text) or null | no | Source prose, kept separately from structured criteria. |
| `unmet_criteria` | list of [Text](#text) | no | Criteria explicitly marked unmet. |
| `versions` | list of [Text](#text) | no | Native document versions offered by the source. |

### AssertionDocument

Native SEPIO facts kept separate from common interpretation fields.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assessments` | list of [AssertionAssessment](#assertionassessment) | no | Assessments directly supporting the interpretation. |
| `document_type` | [Text](#text) | yes | Native interpretation type. |
| `native_id` | [Text](#text) | yes | Native interpretation identifier, separate from the summary PCER identifier. |
| `outcome_id` | [Text](#text) or null | no | Native outcome identifier without assigning a common coding system. |
| `outcome_iri` | [Text](#text) or null | no | Native outcome IRI. |
| `uuid` | [Text](#text) | yes | Source UUID checked against the selected summary link. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `evaluated` | [PartialDate](#partialdate) or null | no | Source evaluation date. |
| `record` | [Text](#text) or null | no | Native assertion record. |
| `release` | [Text](#text) or null | no | Dataset release. |
| `source` | [Text](#text) | yes | Source responsible for the claim. |
| `submitter` | [Text](#text) or null | no | Source submitter or panel. |
| `url` | [Text](#text) or null | no | Source record link. |
| `version` | [Text](#text) or null | no | Record version. |

### AuthorCounts

Counts describe only this provider record, never a global publication corpus.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `citation_count` | integer or null | no | Native citations to this record’s indexed papers. |
| `h_index` | integer or null | no | Native h-index for this record’s indexed papers. |
| `paper_count` | integer or null | no | Native indexed paper count. |
| `scope` | [AuthorCountScope](#authorcountscope) | yes | Counts apply only to this provider record. |

### AuthorCountScope

Scope of source-reported bibliographic counts.

Values: `provider_record`.

### AuthorHeader

One source candidate; absent affiliations differ from an empty source collection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `affiliations` | list of [Text](#text) or null | no | Native affiliation labels; no employment claim. |
| `counts` | [AuthorCounts](#authorcounts) or null | no | Source record counts when supplied. |
| `id` | [AuthorId](#authorid) | yes | Exact qualified provider record. |
| `name` | [Text](#text) | yes | Source display name, never a join key. |

### AuthorId

Type: string.

### AuthorizationDate

A native date kept with its original role.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `date` | [PartialDate](#partialdate) | yes | Source date at its stated precision. |
| `role` | [Text](#text) | yes | Native date role, without assigning approval meaning. |

### AuthorizationEvent

A source-scoped regulator event; never an identity-card field.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | One source record and its native provenance. |
| `dates` | list of [AuthorizationDate](#authorizationdate) | no | All native dates with separate roles. |
| `indication` | [Text](#text) or null | no | Original indication scope when present. |
| `jurisdiction` | [Text](#text) or null | no | Jurisdiction explicitly stated by the source. |
| `kind` | [Text](#text) or null | no | Original native event type. |
| `product_number` | [Text](#text) or null | no | Explicit native product number, never a synthesized first product. |
| `record` | [DrugId](#drugid) | yes | Native regulator record identifier. |
| `status` | [Text](#text) or null | no | Original source status; historical positives imply no current status. |

### AuthorPaperExternalId

An open native identifier namespace and value; no identity inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) | yes | Native namespace and identifier. |

### AuthorPaperHeader

One indexed paper in source order; identifiers come only from that source.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `doi` | [Doi](#doi) or null | no | Exact native DOI. |
| `external_ids` | list of [AuthorPaperExternalId](#authorpaperexternalid) | yes | Every other native paper identifier. |
| `paper_id` | [SemanticScholarId](#semanticscholarid) | yes | The paper's ID at the source (Semantic Scholar). |
| `pmcid` | [Pmcid](#pmcid) or null | no | Exact native PMCID. |
| `pmid` | [Pmid](#pmid) or null | no | Exact native PMID. |
| `title` | [Text](#text) | yes | Whole source title. |
| `venue` | [Text](#text) or null | no | Source venue when supplied and nonempty. |
| `year` | integer or null | no | Source publication year. |

### AuthorPaperSection

How much to show for each paper. `details` adds the paper's full metadata.

| Value | Meaning |
| --- | --- |
| `details` | Native detailed metadata for the same paper window. |

### AuthorPapersQuery

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `full` | boolean | no | Legacy alias for details; normalized on construction. |
| `id` | [AuthorId](#authorid) | yes | Exact qualified provider record. |
| `limit` | integer (uint32) | no | Bounded row count. |
| `offset` | integer (uint32) | no | Native or retained collection offset. |
| `pool` | [Text](#text) | no | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `section` | [AuthorPaperSection](#authorpapersection) | no | Detailed disclosure when requested. |

### Bound

A bounded source range; equal min and max denote an exact boundary.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `max` | integer or null | no |  |
| `min` | integer or null | no |  |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of T | yes | The items, in the source's order. |
| `total` | integer (uint64) | yes | How many distinct items there were before the cut. |

### CbioportalCancerTypeId

Type: string.

### CellLineChembl

Exact accession-linked metadata with its own native collection total.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Requested exact accession. |
| `fetched` | integer or null | no | Original retrieval seconds; never a source release date. |
| `items` | T | yes | Every selected whole metadata row and the native total. |
| `offset` | integer (uint32) | yes | Native metadata page offset. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independent ChEMBL release, distinct from retrieval time. |

### CellLineChemblRecord

One exact accession-linked ChEMBL cell identity; no assay request is implied.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cell_id` | integer (uint64) | yes | Independent native integer cell record identity. |
| `cellosaurus_id` | [CellosaurusAccession](#cellosaurusaccession) | yes | Native exact accession; adapter verifies the join. |
| `clo` | [Text](#text) or null | no | Native CLO identifier; omission is source absence. |
| `description` | [Text](#text) or null | no | Native description. |
| `efo` | [Text](#text) or null | no | Native EFO identifier; omission is source absence. |
| `id` | [ChemblCellId](#chemblcellid) | yes | Native cell ChEMBL ID, never a compound or target. |
| `name` | [Text](#text) or null | no | Native display name without name-based joining. |
| `organism` | [Text](#text) or null | no | Native organism label. |
| `taxon` | integer or null | no | Native organism taxon. |
| `tissue` | [Text](#text) or null | no | Native tissue label. |

### CellLineGene

A native variation gene member with only a source-supplied HGNC identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `hgnc` | [HgncId](#hgncid) or null | no | Present only if this source member explicitly carries the HGNC identifier. |
| `reference` | [CellLineXref](#celllinexref) | yes | Complete original source gene cross-reference. |

### CellLineGet

One exact identity and an optional bounded named section.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [CellLineInput](#celllineinput) | yes | One exact primary accession or qualified source ID. |
| `section` | [CellLineSection](#celllinesection) | no | Omit for the default card; all returns bounded first pages without an assay count. |
| `section_limit` | integer (uint32) | no | Explicit row limit for each requested collection; native variation defaults to three. |
| `section_offset` | integer (uint32) | no | Offset within each independently requested collection. |

### CellLineHeader

Compact source identity with independently capped species and diseases.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Exact primary accession. |
| `category` | [Text](#text) | yes | Native category. |
| `diseases` | [Capped](#capped) of [CellLineXref](#celllinexref) | yes | Up to three complete disease references and their independent total. |
| `name` | [Text](#text) | yes | Recommended display name. |
| `species` | [Capped](#capped) of [CellLineXref](#celllinexref) | yes | Up to three complete species references and their native total. |

### CellLineIdentityPage

Species, diseases and warnings page independently; each total belongs to its own collection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `diseases` | [Capped](#capped) of [CellLineXref](#celllinexref) | yes | Whole disease rows and their independent source total. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `offset` | integer (uint32) | yes | Requested offset applied independently to each collection. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |
| `species` | [Capped](#capped) of [CellLineXref](#celllinexref) | yes | Whole species rows and their source total. |
| `warnings` | [Capped](#capped) of [CellLineWarning](#celllinewarning) | yes | Whole scoped warning rows and their independent source total. |

### CellLineInput

An exact accession or a database-qualified source identity; never a name lookup.

Type: string.

### CellLineName

One native display name entry, never an identity key.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | [Text](#text) | yes | Native name type, preserving identifier and synonym labels. |
| `value` | [Text](#text) | yes | Native name spelling and punctuation. |

### CellLineNames

Native display names with collection-local totals and offset.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `items` | T | yes | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | yes | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |

### CellLineProvenance

One complete native source row; reference, institution and database provenance stay independent.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `institution` | [Text](#text) or null | no | Native institution name if supplied. |
| `reference` | [CellLineReference](#celllinereference) or null | no | Native publication reference if supplied. |
| `value` | [Text](#text) or null | no | Native institution value if supplied. |
| `xref` | [CellLineXref](#celllinexref) or null | no | Complete native database provenance if supplied. |

### CellLineReference

One native internal publication reference, without requiring PubMed.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `resource_internal_ref` | [Text](#text) | yes | Complete native resource reference, for example PubMed=9305600. |

### CellLineRelease

Publisher release identity, fetched independently from record data.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `updated` | [PartialDate](#partialdate) | yes | Native release update date. |
| `version` | [Text](#text) | yes | Native release version. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `items` | T | yes | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | yes | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |

### CellLineVariation

One complete native variation row; descriptions do not become normalized alleles.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | [Text](#text) or null | no | Complete native mutation description including all coordinate spellings. |
| `genes` | list of [CellLineGene](#celllinegene) | yes | Every native gene member, including both fusion members. |
| `mutation_type` | [Text](#text) or null | no | Native open mutation label. |
| `note` | [Text](#text) or null | no | Complete native variation note. |
| `provenance` | list of [CellLineProvenance](#celllineprovenance) | yes | Every native provenance row in source order; no PMID is required. |
| `references` | list of [CellLineXref](#celllinexref) | yes | Every other variation cross-reference in native order. |
| `variation_type` | [Text](#text) | yes | Native open variation label. |
| `zygosity` | [Text](#text) or null | no | Native zygosity label; absence is not inferred. |

### CellLineWarning

Complete source warning with its native scope; no stock classifier is derived.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `scope` | [Text](#text) | yes | Native warning category, such as Problematic cell line. |
| `text` | [Text](#text) | yes | Complete source text including historical and stock qualifications. |

### CellLineXref

One complete native cross-reference; database and accession form the source identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [Text](#text) | yes | Native accession, including any internal prefix. |
| `category` | [Text](#text) or null | no | Native source category when supplied. |
| `database` | [Text](#text) | yes | Native database name; no fixed vocabulary. |
| `discontinued` | [Text](#text) or null | no | Native discontinuation qualifier, including values such as probable. |
| `iri` | [Text](#text) or null | no | Native IRI when supplied. |
| `label` | [Text](#text) or null | no | Native display label when supplied. |
| `url` | [Text](#text) or null | no | Native source URL when supplied. |

### CellLineXrefs

Native source cross-references with collection-local totals and offset.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [CellosaurusAccession](#cellosaurusaccession) | yes | Selected exact primary accession. |
| `fetched` | integer or null | no | Original complete-card retrieval seconds, distinct from source release. |
| `items` | [Capped](#capped) of [CellLineXref](#celllinexref) | yes | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | yes | Offset within this collection, never within another section. |
| `release` | [CellLineRelease](#celllinerelease) or null | no | Independently fetched publisher release. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `next` | T or null | no | Continue the same source candidate, profile and claim stream. |
| `returned` | integer (uint32) | yes | Number of native rows displayed in this stream. |
| `total` | integer (uint64) | yes | Native total for this stream. |

### CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

### CivicMember

A native CIViC member, including members without a common-model subject.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `feature` | [Text](#text) | yes | Native feature label, including fusion features. |
| `hgvs_descriptions` | list of [Text](#text) | no | Every native notation, without selecting a transcript or repairing a spelling. |
| `label` | [Text](#text) | yes | Whole source variant label. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native variant identity and link. |
| `subject` | [VariantRef](#variantref) or null | no | Source-qualified subject when supported by the common model. |

### CivicMembership

Identity strength of a CIViC profile lookup.

Values: `exact`, `candidate`.

### CivicNativeClaim

Source facts retained without inventing a common-model subject or proposition.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele_origin` | [Term](#term) or null | no | Native origin, when stated. |
| `attribution` | [Attribution](#attribution) | yes | Original source provenance. |
| `citations` | list of [Pmid](#pmid) | no | Native publication identities. |
| `classification` | [Term](#term) | yes | Whole native classification. |
| `condition` | [Concept](#concept) or null | no | Source disease context. |
| `strength` | [Term](#term) or null | no | Native level, when present. |
| `therapies` | list of [Concept](#concept) | no | Native therapies in source order. |

### CivicProfile

One source molecular profile, preserving compound expressions.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assertion_count` | integer (uint64) | yes | Native assertion count, independent of the displayed page. |
| `assertion_page` | [CivicClaimPage](#civicclaimpage) of [VariantGet](#variantget) | yes | Native assertion stream and its independent continuation. |
| `compound` | boolean | yes | Native complex-profile flag; a member match does not satisfy the whole profile. |
| `evidence_count` | integer (uint64) | yes | Native evidence count, independent of the displayed page. |
| `evidence_page` | [CivicClaimPage](#civicclaimpage) of [VariantGet](#variantget) | yes | Native evidence stream and its independent continuation. |
| `label` | [Text](#text) | yes | Whole source profile label, including its logical expression. |
| `members` | list of [CivicMember](#civicmember) | yes | Every source member reference, without collapsing compound profiles. |
| `membership` | [CivicMembership](#civicmembership) | yes | Whether a source member was joined to the requested exact identity. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native profile identity and link. |
| `statements` | list of [CivicStatement](#civicstatement) | yes | Source claims; a compound claim retains a profile subject. |

### CivicStatement

One CIViC claim with native labels that exceed the common statement model.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `direction` | [Text](#text) or null | no | Original provider direction; disputes never implies the opposite effect. |
| `kind` | [Text](#text) | yes | Source claim category, without a fallback proposition. |
| `native` | [CivicNativeClaim](#civicnativeclaim) or null | no | Complete native claim facts when the common subject or proposition is unavailable. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native claim identity, retained even when no common proposition applies. |
| `significance` | [Text](#text) or null | no | Original significance, including when the classification holds an AMP tier. |
| `statement` | [Statement](#statement) or null | no | Common source-attributed claim. |
| `statement_unmapped` | boolean | no | Native claim cannot be represented by the common subject or proposition model. |
| `status` | [Text](#text) | yes | Native curation state, without implying that every row is accepted. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `component` | [Text](#text) | yes | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | yes | Canonical publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Publisher terms name. |
| `url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Publisher use category. |

### ComponentUpload

One source-named component upload timestamp.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `date` | [Text](#text) | yes | Unchanged source timestamp. |
| `name` | [Text](#text) | yes | Source subcollection name. |

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) or null | no | Source-stated concept identifier. |
| `label` | [Text](#text) | yes | Original concept label. |

### CosmicCoordinates

Historical coordinates exactly as carried by the component.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `end` | integer (uint64) | yes | Source end value, without coordinate conversion. |
| `start` | integer (uint64) | yes | Source start value, without coordinate conversion. |

### CosmicIdentity

Relationship between a carried historical fragment and its aggregator record.

Values: `unverified`.

### CosmicRecord

One source record from MyVariant's historical COSMIC copy.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alternate` | [Text](#text) or null | no | Unmodified source alternate spelling. |
| `chromosome` | [Text](#text) or null | no | Source chromosome label. |
| `cosmic_id` | [Text](#text) | yes | Native COSMIC record identifier. |
| `hg19` | [CosmicCoordinates](#cosmiccoordinates) or null | no | Source hg19 coordinates, without allele construction. |
| `identity` | [CosmicIdentity](#cosmicidentity) | yes | Fragment identity has not been independently verified. |
| `mut_freq` | [FiniteNumber](#finitenumber) or null | no | Source frequency; units and denominator are not inferred. |
| `mut_nt` | [Text](#text) or null | no | Source nucleotide text, without repair or normalization. |
| `myvariant_record` | [Text](#text) | yes | MyVariant parent record association; this does not establish allele equivalence. |
| `reference` | [Text](#text) or null | no | Unmodified source reference spelling. |
| `tumor_site` | list of [Text](#text) | no | Every source tumor-site label in native order. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `bundle_revision` | [Text](#text) or null | no | Immutable stored bundle revision retained by a continuation. |
| `id` | [Text](#text) | yes | Exact native identity, including version for GTR. |
| `section` | [DiagnosticSection](#diagnosticsection) or null | no | gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card. |
| `section_limit` | integer (uint32) | no | Maximum native rows; omitted means five rows, or one regulatory fragment. |
| `section_offset` | integer (uint32) | no | Zero-based native row or fragment offset. |
| `source` | [DiagnosticGetSource](#diagnosticgetsource) | yes | Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number. |
| `supplement` | [PmaSupplement](#pmasupplement) or null | no | Exact native PMA supplement; absent selects the base row. |

### DiagnosticGetSource

One explicit exact diagnostic record source.

Values: `gtr`, `who-ivd`, `fda-pma`, `fda-510k`.

### DiagnosticLaboratory

Compact native laboratory context without authorization interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `country` | [Text](#text) or null | no | Native country. |
| `id` | [Text](#text) | yes | Native laboratory identifier. |
| `institution` | [Text](#text) or null | no | Native institution name. |
| `name` | [Text](#text) or null | no | Source laboratory name, absent when the native cell is empty. |

### DiagnosticPmaHeader

Compact exact PMA row identity; its native empty base cell is explicit.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `applicant` | string or null | no | Native applicant when supplied, including explicit empty. |
| `decision_code` | string or null | no | Native decision_code when supplied, including explicit empty. |
| `decision_date` | string or null | no | Native decision_date when supplied, including explicit empty. |
| `pma_number` | [PmaNumber](#pmanumber) | yes | Native application. |
| `supplement_number` | [PmaSupplementCell](#pmasupplementcell) or null | no | Absent differs from explicit empty base cell. |
| `trade_name` | string or null | no | Native trade_name when supplied, including explicit empty. |
| `url` | [Text](#text) | yes | Exact source record URL. |

### DiagnosticSearch

Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `biomarker` | [Text](#text) or null | no | Exact table-0 biomarker cell; unsupported in the group table. |
| `bundle_revision` | [Text](#text) or null | no | Immutable stored bundle revision retained by a continuation. |
| `condition_id` | [Text](#text) or null | no | Exact MedGen:CUI identifier only. |
| `device_id` | [DiagnosticDeviceId](#diagnosticdeviceid) or null | no | Exact PMA application, application/supplement or native K identity. |
| `drug_application` | [FdaDrugApplication](#fdadrugapplication) or null | no | Cell-owned typed NDA/BLA application. |
| `gene` | [GeneSymbol](#genesymbol) or null | no | Exact source gene_symbol field, without alias expansion. |
| `gene_id` | [NcbiGeneId](#ncbigeneid) or null | no | Exact source GeneID, exclusive with gene symbol. |
| `limit` | integer (uint32) | no | Five results by default; at most fifty source rows or product headers. |
| `manufacturer` | [Text](#text) or null | no | Literal whole-word phrase in WHO's Manufacturer name cell. |
| `marker` | [Text](#text) or null | no | Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. |
| `offset` | integer (uint32) | no | Zero-based source result offset. |
| `source` | [DiagnosticSearchSource](#diagnosticsearchsource) | yes | Explicit native GTR, WHO IVD or FDA companion discovery. |
| `type` | [GtrTestType](#gtrtesttype) or null | no | Exact native test_type value. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `agent_type` | [Text](#text) or null | no | Native agent type. |
| `aggregator_knowledge_source` | list of [Curie](#curie) or null | no | Native aggregator knowledge sources; missing, null and lists remain distinct. |
| `attribution` | [Attribution](#attribution) | yes | Native row provenance and version. |
| `category` | [Curie](#curie) or null | no | Native association category. |
| `evidence` | list of [DiseaseAssociationReference](#diseaseassociationreference) or null | no | Source evidence terms; unavailable evidence stays absent. |
| `evidence_count` | integer or null | no | Native evidence count, distinct from returned reference count. |
| `has_count` | integer or null | no | Native annotation count, without a computed denominator. |
| `has_percentage` | [FiniteNumber](#finitenumber) or null | no | Native percentage, never computed from another field. |
| `has_quotient` | [FiniteNumber](#finitenumber) or null | no | Native quotient, never computed from another field. |
| `has_total` | integer or null | no | Native annotation total, without inferring an assayed denominator. |
| `knowledge_level` | [Text](#text) or null | no | Native source knowledge level. |
| `negated` | boolean or null | no | Source negation; absent is not false. |
| `object` | [Concept](#concept) | yes | Native returned object. |
| `object_closure` | list of [Curie](#curie) | yes | Native object closure. |
| `original_object` | [Curie](#curie) or null | no | Original source object before source normalization. |
| `original_predicate` | [Text](#text) or null | no | Native pre-normalization predicate, without interpretation. |
| `original_subject` | [Curie](#curie) or null | no | Original source subject before source normalization. |
| `predicate` | [Curie](#curie) | yes | Native source predicate. |
| `primary_knowledge_source` | [Curie](#curie) or null | no | Native primary knowledge source. |
| `provided_by` | [Text](#text) or null | no | Native providing collection. |
| `publications` | list of [DiseaseAssociationReference](#diseaseassociationreference) | no | Native publication references, without inferred links or labels. |
| `qualifiers` | list of [DiseaseAssociationQualifier](#diseaseassociationqualifier) | yes | Native qualifiers, including frequency or onset if supplied. |
| `requested_id` | [Curie](#curie) | yes | The exact requested concept, including a queried parent. |
| `subject` | [Concept](#concept) | yes | Native returned subject. |
| `subject_closure` | list of [Curie](#curie) | yes | Native subject closure. |
| `subject_taxon` | [Curie](#curie) or null | no | Native subject taxon, only as supplied by the publisher. |

### DiseaseAssociationItem

One native association and the component terms governing that row.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `association` | [DiseaseAssociation](#diseaseassociation) | yes | Complete neutral association, including requested and actual endpoints. |
| `components` | list of [ComponentLabel](#componentlabel) | yes | Independently attributed annotation components for this row. |

### DiseaseAssociationQualifier

A named native qualifier without an inferred role.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `field` | [Text](#text) | yes | Original native qualifier field. |
| `id` | [Curie](#curie) | yes | Native qualifier identity. |
| `label` | [Text](#text) or null | no | Native label when supplied. |

### DiseaseAssociationReference

One native association reference, with no invented display label or link.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) | yes | Native evidence or publication identity. |
| `label` | [Text](#text) or null | no | Native label when supplied. |
| `url` | [Text](#text) or null | no | Unchanged native link when supplied. |

### DiseaseEndpoint

One native endpoint with an identifier and an optional source label.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) | yes | Exact source identifier. |
| `label` | [Text](#text) or null | no | Source label when supplied. |

### DiseaseGet

Exact concept or native SEER site with independently paged section.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `age_range` | [Text](#text) or null | no | Native age-range code. |
| `dataset` | [Text](#text) or null | no | Requested native dataset selector, not proof of returned membership. |
| `id` | [Text](#text) | yes | Exact ontology CURIE or an explicit SEER catalog site ID. |
| `interval` | [Text](#text) or null | no | Survival-only interval code. |
| `ontology` | [DiseaseOntology](#diseaseontology) | no | One OLS ontology. |
| `period` | [Text](#text) or null | no | Incidence-only native period code. |
| `race` | [Text](#text) or null | no | Native race code. |
| `section` | [DiseaseSection](#diseasesection) or null | no | One named section; absent means identity card only. |
| `section_limit` | integer (uint32) | no | Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. |
| `section_offset` | integer (uint32) | no | Native row offset within the selected section. |
| `sex` | [Text](#text) or null | no | Native sex code. |
| `source` | [DiseaseSource](#diseasesource) | no | Default MyDisease; no automatic fallback. |

### DiseaseOntology

One explicit OLS ontology.

Values: `mondo`, `doid`, `ncit`.

### DiseaseRelation

A publisher relation with distinct endpoints and predicate.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Publisher record and version. |
| `description` | string or null | no | Unchanged native cross-reference description when supplied. |
| `object` | [DiseaseEndpoint](#diseaseendpoint) | yes | Native object. |
| `predicate` | [Curie](#curie) | yes | Publisher predicate, without equivalence promotion. |
| `subject` | [DiseaseEndpoint](#diseaseendpoint) | yes | Native subject. |

### DiseaseSearch

Literal discovery text or an explicit SEER catalog page.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum source rows on one page. |
| `offset` | integer (uint32) | no | Native row offset. |
| `ontology` | [DiseaseOntology](#diseaseontology) | no | Exactly one ontology, default MONDO. |
| `source` | [DiseaseSource](#diseasesource) | no | One explicit source, default OLS. |
| `text` | string or null | no | Literal source search text; absent for the SEER site catalog. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Publisher record and dataset version when supplied. |
| `measure` | [Text](#text) | yes | Native measure name. |
| `modeled` | [StatisticEstimate](#statisticestimate) or null | no | Modeled estimate kept separately. |
| `observed` | [StatisticEstimate](#statisticestimate) or null | no | Observed estimate; missing stays absent. |
| `requested_selectors` | list of [StatisticSelector](#statisticselector) | yes | Settings sent to the publisher. |
| `requested_site` | [Text](#text) | yes | Explicit selected native site ID. |
| `returned_selectors` | list of [StatisticSelector](#statisticselector) | yes | Settings actually returned, never filled from request. |
| `year` | integer or null | no | Native annual row year. |

### Doi

Type: string.

### DrugFdaIngredient

An unchanged native ingredient and strength, without local normalization.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Native ingredient name. |
| `strength` | [Text](#text) | no | Full source strength wording. |

### DrugFdaProduct

One explicit native product within the selected FDA application.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `active_ingredients` | list of [DrugFdaIngredient](#drugfdaingredient) | yes | Complete source-listed ingredient and strength pairs. |
| `brand_name` | [Text](#text) | yes | Source brand wording, without inferred ingredient equivalence. |
| `dosage_form` | [Text](#text) | yes | Native dosage form. |
| `marketing_status` | [Text](#text) | yes | Product marketing status, independent of submission history. |
| `product_number` | [Text](#text) | yes | Exact source product number, scoped by the response application. |
| `reference_drug` | [Text](#text) | yes | Original source reference-drug designation. |
| `reference_standard` | [Text](#text) or null | no | Original source reference-standard designation when supplied. |
| `route` | [Text](#text) or null | no | Native route; absent and null remain unavailable. |
| `te_code` | [Text](#text) or null | no | Native therapeutic-equivalence code when supplied. |

### DrugGet

Exact native ID retrieval with an explicitly selected source.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `civic_cursor` | [Text](#text) or null | no | Native continuation cursor for that exact drug, profile and claim collection. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | no | Independent native evidence or assertion collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected native CIViC molecular profile for a drug-scoped claim stream. |
| `input` | [DrugId](#drugid) | yes | Published source-native identifier, never SOURCE:ID or a selected name. |
| `label_section` | [Text](#text) or null | no | Exact native label heading, required only for the label section. |
| `product_number` | [Text](#text) or null | no | Explicit native application product number, only for approvals. |
| `section` | list of [DrugSection](#drugsection) | no | Explicit named disclosures; empty means identity only. |
| `section_limit` | integer or null | no | Named section page size, refused on an identity-only card. |
| `section_offset` | integer or null | no | Named section continuation, refused on an identity-only card. |
| `source` | [DrugSource](#drugsource) | yes | Required source; there is no default exact-ID resolver. |

### DrugHeader

One source candidate, without authorization, targets, label text or interactions.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [DrugId](#drugid) | yes | Published native identifier. |
| `identity_level` | [DrugIdentityLevel](#drugidentitylevel) | yes | Level established only by the endpoint or record type. |
| `name` | [Text](#text) | no | Original source display name. |
| `source` | [DrugSource](#drugsource) | yes | The single source responsible for this header. |
| `type` | [Text](#text) or null | no | Native record type. |
| `url` | [Text](#text) or null | no | Source record URL. |
| `version` | [Text](#text) or null | no | Native source revision, omitted when the source supplies none. |

### DrugId

Type: string.

### DrugIdentity

One source record identity, without any equivalence or authorization claim.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [DrugId](#drugid) | yes | Published native record identifier. |
| `identity_level` | [DrugIdentityLevel](#drugidentitylevel) | yes | Level established by the endpoint or record type; unknown stays unknown. |
| `name` | [Text](#text) | no | Original source name. |
| `source` | [Text](#text) | yes | Source responsible for this identity. |
| `type` | [Text](#text) or null | no | Native record type, without local classification. |
| `url` | [Text](#text) or null | no | Native record URL. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Record supplying the relationship. |
| `identity` | [DrugIdentity](#drugidentity) | yes | Identity at the other end of the source relation. |
| `relation` | [Text](#text) | yes | Native relationship label; no inferred equivalence. |

### DrugNameSelector

One explicit ChEMBL name filter without fallback or synonym expansion.

| Value | Meaning |
| --- | --- |
| `preferred` | Match the native preferred name exactly, ignoring case. |
| `synonym` | Match an explicit native synonym exactly, ignoring case. |

### DrugNativeReference

Typed source reference shared by mechanism and indication records.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ref_id` | [Text](#text) or null | no | Source reference identifier, when supplied. |
| `ref_type` | [Text](#text) | yes | Native reference type. |
| `ref_url` | [Text](#text) or null | no | Source reference link, when supplied. |

### DrugRecords

One bounded native row page, without fabricating an unknown total.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Source row offset. |
| `records` | list of T | no | Native source rows in order. |
| `retrieved` | [PartialDate](#partialdate) or null | no | Actual retrieval date, never substituted for source revision. |
| `total` | integer or null | no | Native total only when supplied. |
| `truncated` | boolean or null | no | Native source truncation indicator. |
| `version` | [Text](#text) or null | no | Dataset or bundle revision. |

### DrugSearch

Literal single-source candidate discovery, without an implicit identity selection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Native candidate rows per page. |
| `name_selector` | [DrugNameSelector](#drugnameselector) | no | Explicit ChEMBL native name selector; omitted means preferred name. |
| `offset` | integer (uint32) | no | Native rows to skip. |
| `query` | [Text](#text) | yes | Literal source name selector, unchanged by the consumer. |
| `source` | [DrugSource](#drugsource) | no | Exactly one source; default MyChem never fans out. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `applicant` | string | yes | Native Applicant wording. |
| `basis_of_listing` | string | yes | Native Basis of Listing wording. |
| `date_of_prequalification` | string | yes | Native Date of Prequalification text, including spacing and empty strings. |
| `dosage_form` | string | yes | Native Dosage Form cell, independent of the combined cell. |
| `inn_dosage_form_strength` | string | yes | Native combined INN, Dosage Form and Strength cell; never split or interpreted. |
| `product_type` | string | yes | Native Product Type, retained even when empty. |
| `reference` | [Text](#text) | yes | Exact native WHO Reference Number, including spaces and punctuation. |
| `therapeutic_area` | string | yes | Native Therapeutic Area wording. |

### EnsemblGeneId

Type: string.

### EuropePmcId

Type: string.

### EvidenceNotice

Explicit successful empty outcomes, never inferred from a source failure.

Values: `not_in_references`, `no_contexts`, `no_recommendations`.

### Expression

Source expression checked for its declared published form, without normalization.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `syntax` | [ExpressionSyntax](#expressionsyntax) | yes |  |
| `syntax_version` | [Text](#text) or null | no |  |
| `value` | [Text](#text) | yes |  |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `aliases` | list of [Text](#text) | yes | All stated HGNC aliases. |
| `ensembl_id` | [EnsemblGeneId](#ensemblgeneid) or null | no | Ensembl gene identifier, if stated. |
| `entrez_id` | [NcbiGeneId](#ncbigeneid) or null | no | NCBI Gene identifier, if stated. |
| `hgnc_id` | [HgncId](#hgncid) | yes | HGNC identifier. |
| `location` | [Cytoband](#cytoband) or null | no | Single stated band, if available. |
| `locus_group` | [LocusGroup](#locusgroup) | yes | HGNC locus group. |
| `locus_type` | [LocusType](#locustype) or null | no | HGNC locus type, if stated and recognized. |
| `mane` | list of [Mane](#mane) | no | Versioned MANE Select transcript pair, if stated. |
| `name` | [Text](#text) | yes | Gene name. |
| `omim` | list of [OmimId](#omimid) | yes | All stated OMIM identifiers. |
| `previous_symbols` | list of [GeneSymbol](#genesymbol) | yes | All stated previous symbols. |
| `spans` | list of [GenomicInterval](#genomicinterval) | yes | Source intervals with explicit assembly and counting rule. |
| `status` | [GeneStatus](#genestatus) | yes | HGNC record status. |
| `summary` | [Summary](#summary) or null | no | Whole attributed summary, if stated. |
| `symbol` | [GeneSymbol](#genesymbol) | yes | Source-preserved symbol. |
| `uniprot` | list of [UniProtAcc](#uniprotacc) | yes | All stated UniProt base accessions. |

### GeneHeader

Enough gene identity to choose a search result.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `hgnc_id` | [HgncId](#hgncid) | yes | HGNC identifier. |
| `location` | [Cytoband](#cytoband) or null | no | A single stated band, if available. |
| `locus_group` | [LocusGroup](#locusgroup) | yes | HGNC locus group. |
| `match` | [Match](#match) or null | no | Match information when this header answers an input. |
| `name` | [Text](#text) | yes | Gene name. |
| `status` | [GeneStatus](#genestatus) | yes | HGNC record status. |
| `symbol` | [GeneSymbol](#genesymbol) | yes | Source-preserved symbol. |

### GeneInput

Type: string.

### GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ensembl_gene_id` | [EnsemblGeneId](#ensemblgeneid) or null | no |  |
| `hgnc_id` | [HgncId](#hgncid) or null | no |  |
| `label` | [Text](#text) | yes |  |
| `ncbi_gene_id` | [NcbiGeneId](#ncbigeneid) or null | no |  |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Source-stated assembly, omitted when absent. |
| `chromosome` | [Chromosome](#chromosome) | yes | Canonical primary chromosome. |
| `span` | [Span0](#span0) | yes | Checked zero-based half-open bounds; an empty span remains valid. |
| `strand` | [Strand](#strand) or null | no | Source-stated strand, if available. |

### GnomadCounts

Native gnomAD counts retained when a dataset release cannot be established.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ac` | integer (uint64) | yes | Native allele count, including reported zero. |
| `an` | integer (uint64) | yes | Native allele denominator. |
| `ancestry_group` | [Text](#text) or null | no | Source ancestry code decoded only by the documented source grammar. |
| `data_type` | [PopulationDataType](#populationdatatype) | yes | Source sequencing sample set. |
| `filters` | list of [Text](#text) | no | Native source filters, never converted to a classification. |
| `frequency` | [PopulationFrequency](#populationfrequency) or null | no | Checked total from a release supplied alongside matching source counts. |
| `group_id` | [Text](#text) or null | no | Complete native group ID, including its source sex suffix when present. |
| `sex` | [PopulationSex](#populationsex) or null | no | Source sex stratum decoded only by the documented source grammar. |

### GtrConditionRelation

Native condition membership; SNOMED cells never become GeneID.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `condition` | [Concept](#concept) | yes | Shared MedGen concept with source label. |
| `mim_number` | string | yes | Native MIM cell, including empty. |
| `snomed_ct_id` | string | yes | Native SNOMED CT cell, including empty; never a gene ID. |
| `test_id` | [GtrTestId](#gtrtestid) | yes | Complete owning test accession/version. |
| `test_type` | [GtrTestType](#gtrtesttype) | yes | Native source test type. |

### GtrGeneRelation

Native gene relation; its numeric ID comes only from the gene row.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `gene` | [GeneRef](#generef) | yes | Shared source-given GeneID and gene_symbol; absent GeneID stays absent. |
| `mim_number` | string | yes | Native MIM cell, including empty. |
| `native_gene_or_snomed_ct_id` | string or null | no | Uninterpreted nonempty `gene_or_SNOMED_CT_ID` when it is not a checked NCBI Gene ID. |
| `native_medgen` | [Curie](#curie) | yes | Independent native MedGen identity from GTR_identifier. |
| `object_name` | string | yes | Full source object_name without interpretation. |
| `test_id` | [GtrTestId](#gtrtestid) | yes | Complete owning test accession/version. |
| `test_type` | [GtrTestType](#gtrtesttype) | yes | Native source test type. |

### GtrHeader

Small resolved GTR candidate; broad gene and condition arrays stay off the header.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [GtrTestId](#gtrtestid) | yes | Complete source accession and version. |
| `laboratory` | [DiagnosticLaboratory](#diagnosticlaboratory) | yes | Native laboratory context. |
| `name` | [Text](#text) or null | no | Native test name, absent when the cell is empty. |
| `url` | [Text](#text) | yes | Source record URL. |

### GtrMethod

One entry from one independent native method list.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `index` | integer (uint32) | yes | Zero-based native list position. |
| `list` | [GtrMethodList](#gtrmethodlist) | yes | Independent source list; lists are never positionally paired. |
| `test_id` | [GtrTestId](#gtrtestid) | yes | Complete owning test accession/version. |
| `value` | string | yes | Complete unmodified source entry. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alleles` | list of [GwasEffectAllele](#gwaseffectallele) | yes | Every source effect allele paired with its rsID. |
| `author` | [Text](#text) or null | no | Source author attribution, without local publication inference. |
| `beta` | [Text](#text) or null | no | Source beta text, including units and direction. |
| `ci_lower` | [FiniteNumber](#finitenumber) or null | no | Source lower effect interval bound. |
| `ci_upper` | [FiniteNumber](#finitenumber) or null | no | Source upper effect interval bound. |
| `exact_p_value` | [ScientificPValue](#scientificpvalue) or null | no | Authoritative exact source probability, including double underflow. |
| `id` | integer (uint64) | yes | Native association identity. |
| `interval` | [Text](#text) or null | no | Source effect interval. |
| `locations` | list of [Text](#text) | no | Native mapping location strings; no assumed assembly or coordinate conversion. |
| `mapped_genes` | list of [Text](#text) | no | Original native mapped gene context; reported genes remain in loci. |
| `odds_ratio` | [FiniteNumber](#finitenumber) or null | no | Source odds ratio. |
| `ontology_traits` | list of [GwasTrait](#gwastrait) | no | Source mapped trait identifiers and whole labels, without clinical inference. |
| `p_value` | [FiniteNumber](#finitenumber) | yes | Reported p value. |
| `publication` | [Text](#text) or null | no | Source publication identifier. |
| `risk_frequency` | [Text](#text) or null | no | Source allele frequency text, without interpreting NR. |
| `study` | [Text](#text) | yes | Study accession. |
| `traits` | list of [Text](#text) | yes | Source reported traits. |

### GwasAssociationId

Type: string.

### GwasAssociationSection

Association-only section.

| Value | Meaning |
| --- | --- |
| `loci` | Native locus and reported gene context. |

### GwasEffectAllele

Catalog effect allele paired with its exact rsID.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele` | [Text](#text) | yes | Source allele text. |
| `rs_id` | [Text](#text) | yes | Source rsID. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Exact source ontology identifier spelling. |
| `label` | [Text](#text) | yes | Whole source trait label. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `amino_acid` | [Text](#text) | yes | Source alternate amino-acid spelling. |
| `count` | integer (uint64) | yes | Source count, without deriving a frequency. |

### HotspotMembership

Source transcript and consequence join state.

Values: `exact`, `candidate`.

### HotspotRecord

Counts for one source transcript and residue label.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alternates` | list of [HotspotAlternate](#hotspotalternate) | yes | Source alternate amino acids and their counts. |
| `gene` | [GeneSymbol](#genesymbol) | yes | Gene as stated by the source. |
| `membership` | [HotspotMembership](#hotspotmembership) | yes | Whether transcript and consequence were linked by the identity source. |
| `residue` | [Text](#text) | yes | Native residue label; no genomic allele is inferred. |
| `transcript` | [Text](#text) or null | no | Native transcript spelling, without an inferred accession version. |
| `tumor_count` | integer (uint64) | yes | Tumors reported for this residue by the source. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ensembl` | string | yes | Versioned Ensembl accession. |
| `refseq` | string | yes | Versioned RefSeq accession. |
| `status` | [ManeStatus](#manestatus) | yes | Source-stated transcript status. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | [MatchKind](#matchkind) | yes | The match category. |
| `text` | [Text](#text) | yes | The source text that matched the input. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Native source name. |
| `nci_thesaurus_concept_id` | [Text](#text) | yes | Source NCIt identity. |

### NciTextPage

Exact source text fragment with an honest original character count.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#trialget) or null | no | Checked request for the next original characters, absent when complete. |
| `offset` | integer (uint32) | yes | Original Unicode scalar position. |
| `text` | string | yes | Original source fragment, including whitespace. |
| `total_characters` | integer (uint32) | yes | Complete source length in Unicode scalar values. |

### NciTrial

NCI trial card without registry status or phase coercion.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `brief_summary` | [NciTextPage](#ncitextpage) or null | no | Native short summary, only on exact retrieval. |
| `brief_title` | [Text](#text) | yes | Native title. |
| `current_trial_status` | [Text](#text) | yes | Native status, without a registry translation. |
| `diseases` | [TrialRecords](#trialrecords) of [NciDiseaseRef](#ncidiseaseref) | yes | Publisher-provided disease concepts in source order. |
| `nci_id` | [Text](#text) | yes | Publisher NCI identifier. |
| `nct_id` | [NctId](#nctid) | yes | Registry cross-reference supplied by NCI. |
| `phase` | [Text](#text) or null | no | Native phase. |

### NctId

Type: string.

### OmimId

Type: string.

### OncokbCitation

One native non-PubMed source citation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `link` | [Text](#text) or null | no | Native citation link. |
| `text` | string or null | no | Complete native citation content. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | How many results come before this page. |
| `pool` | [PoolInfo](#poolinfo) or null | no | The stored search this page comes from, when BioMCP kept one. |
| `returned` | integer (uint32) | yes | How many results this page holds. |
| `total` | integer (uint64) | yes | How many matching records the search holds in all. |
| `truncated` | boolean or null | no | True when the source had more matches than BioMCP fetched. |

### PartialDate

Type: string.

### PathwayEventRef

An event object or unresolved integer reference; never the parent's inferred ID.

One of these forms:

- `kind` is `"database"`: Native unresolved integer database reference. Fields: `db_id` (integer (uint64)).
- `kind` is `"object"`: Native event object; fields are absent only when omitted by the source. Fields: `db_id` (integer (uint64)), `id` ([ReactomeId](#reactomeid) or null), `name` ([Text](#text) or null), `schema_class` ([Text](#text) or null), `source_url` ([Text](#text) or null), `version` ([Text](#text) or null).

### PathwayEvents

Independent source-ordered page of whole native references.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | yes | Exact source pathway owning this relation. |
| `items` | list of [PathwayEventRef](#pathwayeventref) | yes | Whole native references; unresolved identities remain visible. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | yes | Relation-specific count provenance and continuation. |

### PathwayGenes

Independent source-ordered distinct membership references and their observed counts.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `header` | [PathwayHeader](#pathwayheader) | yes | Exact source pathway owning this relation. |
| `items` | list of [PathwayMember](#pathwaymember) | yes | Distinct namespaced source references in first occurrence order. |
| `page` | [PathwayPageInfo](#pathwaypageinfo) | yes | Distinct-reference total and continuation. |
| `physical_entities` | integer (uint64) | yes | All native physical entities, never described as gene counts. |
| `reference_occurrences` | integer (uint64) | yes | All reference occurrences across physical entities before distinct-reference grouping. |

### PathwayHeader

Small selectable native pathway identity, without its potentially long description.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [PathwayId](#pathwayid) | yes | Checked source-owned identity. |
| `name` | [Text](#text) | yes | Whole native name. |
| `revision` | [Text](#text) or null | no | WikiPathways revision, separate from identity. |
| `species` | [PathwaySpecies](#pathwayspecies) | yes | Explicit native human species. |
| `version` | [Text](#text) or null | no | Whole native Reactome stIdVersion, separate from stable identity. |

### PathwayId

Source-owned pathway identity; equal names never merge sources.

One of these forms:

- `source` is `"reactome"`: Human Reactome stable identity without a version suffix. Fields: `id` ([ReactomeId](#reactomeid)).
- `source` is `"wikipathways"`: WikiPathways identity without a revision suffix. Fields: `id` ([WikiPathwaysId](#wikipathwaysid)).

### PathwayMember

One source membership reference, without name-based gene inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `gene` | [HgncId](#hgncid) or null | no | Exact resolved gene identifier, absent when no checked join exists. |
| `name` | [Text](#text) or null | no | Native label, never interpreted as a gene symbol. |
| `node_id` | [Text](#text) or null | no | Native data-node identity; repeated references remain separate nodes. |
| `participants` | list of [PathwayParticipant](#pathwayparticipant) or null | no | Complete physical contexts, disclosed only by the explicit details view. |
| `physical_context_count` | integer (uint64) | yes | Number of source physical entity contexts carrying this exact reference. |
| `reference` | [PathwaySourceRef](#pathwaysourceref) | yes | Namespaced native reference. |
| `schema_class` | [Text](#text) or null | no | Native reference schema class. |
| `source_url` | [Text](#text) or null | no | Native reference URL, including therapeutic references without a drug action. |

### PathwayPageInfo

Native page position and explicit count provenance; a short admitted page can continue.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `next_offset` | integer or null | no | Next native position, independent of returned rows; absent only at the end. |
| `offset` | integer (uint32) | yes | Native source or retained relation offset. |
| `pool` | [PoolInfo](#poolinfo) or null | no | Snapshot identity and lifetime when retained. |
| `returned` | integer (uint32) | yes | Whole admitted rows on this page. |
| `total` | integer (uint64) | yes | Source-reported or observed relation total, as qualified by total_kind. |
| `total_kind` | [PathwayTotalKind](#pathwaytotalkind) | yes | Whether the total comes from the source or this retained collection. |

### PathwayParticipant

One native physical [entity](/concepts/progressive-disclosure/) carrying a membership reference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `db_id` | integer (uint64) | yes | Native physical entity database identity, distinct from reference identity. |
| `name` | [Text](#text) | yes | Whole physical entity label, including source-stated modification context. |
| `schema_class` | [Text](#text) | yes | Native physical entity schema class, separate from reference schema class. |

### PathwaySourceRef

An open source namespace paired with its complete native identifier.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `namespace` | string or null | no | Native namespace; never discarded during a gene join. |
| `value` | [Text](#text) | yes | Complete namespace-local identifier, including native isoform suffixes. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `_url` | string | yes | The source's own `_url` value, unchanged. |
| `id` | integer (int64) | yes | The source's own `id` value, unchanged. |
| `name` | string or null | no | Native `name` when supplied. |
| `resource` | string | yes | The source's own `resource` value, unchanged. |
| `resourceId` | string | yes | The source's own `resourceId` value, unchanged. |

### PgxDiplotypeExpression

Type: string.

### PgxDiplotypeReference

Native location diplotype reference; its numeric ID is not a ClinPGx accession.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | integer (int64) | yes | Native `id`; no expression, phase or phenotype is inferred. |

### PgxEvidenceTerm

Source-native PgxEvidenceTerm fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | integer (int64) | yes | The source's own `id` value, unchanged. |
| `resource` | string | yes | The source's own `resource` value, unchanged. |
| `synonyms` | list of [PgxUnobserved](#pgxunobserved) or null | no | Native `synonyms` when supplied. |
| `term` | string | yes | The source's own `term` value, unchanged. |
| `termId` | string | yes | The source's own `termId` value, unchanged. |
| `valid` | boolean or null | no | Native `valid` when supplied. |

### PgxGet

PGx card or section. No phenotype, phase or dose is calculated.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `diplotype` | [PgxDiplotypeExpression](#pgxdiplotypeexpression) or null | no | Unchanged declared expression required for the diplotype section only. |
| `drug_id` | [Curie](#curie) or null | no | Exact published CPIC drug key; recommendations only. |
| `gene` | [GeneSymbol](#genesymbol) | yes | Explicit source gene symbol. |
| `kind` | [PgxAnnotationKind](#pgxannotationkind) or null | no | One ClinPGx collection; clinical is the default only on annotations. |
| `section` | [PgxSection](#pgxsection) or null | no | One named section; omitted means the CPIC gene card. |
| `section_limit` | integer (uint32) | no | Rows per page: three clinical, five elsewhere. |
| `section_offset` | integer (uint32) | no | Offset within the selected section. |
| `snapshot` | [Text](#text) or null | no | Retained collection for local continuation. |
| `source` | [PgxSource](#pgxsource) | yes | Required source, checked against the section. |

### PgxHeader

Source and declared gene context; no chosen pair or patient result.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `gene` | [GeneSymbol](#genesymbol) | yes | Declared source symbol. |
| `source` | [PgxSource](#pgxsource) | yes | Explicit source. |

### PgxLocation

Source-native PgxLocation fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `buildVersion` | string or null | no | Native `buildVersion` when supplied. |
| `chromosomeId` | string or null | no | Native `chromosomeId` when supplied. |
| `chromosomeName` | string or null | no | Native `chromosomeName` when supplied. |
| `copyNumber` | string | yes | The source's own `copyNumber` value, unchanged. |
| `diplotypes` | list of [PgxDiplotypeReference](#pgxdiplotypereference) | yes | The source's own `diplotypes` value, unchanged. |
| `displayName` | string | yes | The source's own `displayName` value, unchanged. |
| `genes` | list of [PgxObjectReference](#pgxobjectreference) | yes | The source's own `genes` value, unchanged. |
| `gpPosition` | integer (int64) | yes | The source's own `gpPosition` value, unchanged. |
| `haplotypes` | list of [PgxObjectReference](#pgxobjectreference) | yes | The source's own `haplotypes` value, unchanged. |
| `id` | integer (int64) | yes | The source's own `id` value, unchanged. |
| `linkedObjects` | list of [PgxObjectReference](#pgxobjectreference) | yes | The source's own `linkedObjects` value, unchanged. |
| `refSeqCrossReference` | [PgxCrossReference](#pgxcrossreference) or null | no | Native `refSeqCrossReference` when supplied. |
| `refSeqPosition` | integer (int64) | yes | The source's own `refSeqPosition` value, unchanged. |
| `rsid` | string or null | no | Native `rsid` when supplied. |
| `tagGene` | boolean | yes | The source's own `tagGene` value, unchanged. |
| `type` | string | yes | The source's own `type` value, unchanged. |
| `variant` | [PgxObjectReference](#pgxobjectreference) or null | no | Native `variant` when supplied. |

### PgxObjectReference

Source-native PgxObjectReference fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [ClinPgxAccession](#clinpgxaccession) | yes | The source's own `id` value, unchanged. |
| `name` | string or null | no | Native `name` when supplied. |
| `objCls` | string or null | no | Native `objCls` when supplied. |
| `symbol` | string or null | no | Native `symbol` when supplied. |

### PgxPageInfo

Independent collection scope; observed counts never imply native source totals.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `offset` | integer (uint32) | yes | Offset in this collection. |
| `returned` | integer (uint32) | yes | Number of whole returned rows. |
| `total` | integer (uint64) | yes | Exact source total or received bounded collection length. |
| `total_kind` | [PgxTotalKind](#pgxtotalkind) | yes | Provenance of total. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [HpId](#hpid) | yes | Exact HP identifier. |
| `section` | [PhenotypeSection](#phenotypesection) | no | Absent means the source identity card. |
| `section_limit` | integer (uint32) | no | Default five text/link rows or one complete association row. |
| `section_offset` | integer (uint32) | no | Native section row offset. |
| `snapshot` | [Text](#text) | no | Retained exact source reply for local text and link pages; unsupported for diseases and genes. |
| `source` | [PhenotypeSource](#phenotypesource) | no | HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `created` | integer (uint64) | yes | When the stored search was made, in Unix seconds. |
| `expires` | integer (uint64) | yes | When the stored search expires, in Unix seconds. |
| `id` | [Text](#text) | yes | The stored search's ID. Pass it back to BioMCP as it is. |
| `size` | integer (uint32) | yes | How many records the stored search holds. |

### PopulationDataType

Dataset sequencing sample set.

Values: `exomes`, `genomes`, `joint`.

### PopulationFrequency

One checked stratum of one source dataset release.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele_count` | integer or null | no |  |
| `allele_frequency` | [FiniteNumber](#finitenumber) or null | no |  |
| `allele_number` | integer or null | no |  |
| `ancestry_group` | [Text](#text) or null | no |  |
| `attribution` | [Attribution](#attribution) | yes |  |
| `build` | [Build](#build) | yes |  |
| `data_type` | [PopulationDataType](#populationdatatype) or null | no |  |
| `dataset` | [Text](#text) | yes |  |
| `dataset_version` | [Text](#text) | yes |  |
| `faf95` | [FiniteNumber](#finitenumber) or null | no |  |
| `filters` | list of [Text](#text) | no |  |
| `hemizygote_count` | integer or null | no |  |
| `homozygote_count` | integer or null | no |  |
| `sex` | [PopulationSex](#populationsex) or null | no |  |
| `subject` | [VariantRef](#variantref) | yes |  |
| `subset` | [Text](#text) or null | no |  |

### PopulationSex

Sex stratum stated by a population source.

Values: `xx`, `xy`.

### Predicate

VA-Spec 1.1 proposition relationships in native snake_case.

Values: `is_causal_for`, `is_oncogenic_for`, `has_clinical_significance_for`, `predicts_sensitivity_to`, `predicts_resistance_to`, `is_diagnostic_inclusion_criterion_for`, `is_diagnostic_exclusion_criterion_for`, `associated_with_better_outcome_for`, `associated_with_worse_outcome_for`, `impacts_function_of`.

### PredictorContext

Unchanged native context values associated with a source field path.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `field` | [Text](#text) | yes | Native source field path. |
| `values` | list of [Text](#text) | yes | Every native value in source array order. |

### PredictorScore

One unmodified source score, never a locally blended assessment.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source-owned record and dataset release. |
| `label` | [Text](#text) or null | no | Source label without code expansion or interpretation. |
| `method` | [Text](#text) | yes | Published component field path. |
| `score` | [FiniteNumber](#finitenumber) | yes | Source number, including zero. |
| `subject` | [VariantRef](#variantref) | yes | Exact source-qualified subject. |

### Problem

A field that could not be accepted from a source record.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `code` | [ProblemCode](#problemcode) | yes | Why the field value was not accepted. |
| `field` | string | yes | The name of the rejected field. |
| `record` | [Text](#text) or null | no | The record's identifier, when known. |
| `source` | [Source](#source) | yes | The source that supplied the record. |

### ProblemCode

Why a field value was rejected. [Reply envelope values](/reference/envelope/#problemscode) lists each value and its meaning.

### Proposition

Domain of a source claim.

Values: `pathogenicity`, `oncogenicity`, `clinical_significance`, `therapeutic_response`, `diagnostic`, `prognostic`, `functional_impact`.

### Protein

A reviewed human protein listed by the authoritative gene record.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | T | yes | Checked primary accession. |
| `function` | list of [Text](#text) | no | Whole source function paragraphs, omitted when absent. |
| `length` | integer (uint32) | yes | Sequence length in amino acids. |
| `name` | [Text](#text) | yes | Recommended protein name. |
| `subcellular_locations` | list of [Text](#text) | no | Source subcellular location names. |

### ProteinAudit

Native source audit metadata, independent from local retrieval metadata.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `entry_version` | integer or null | no | Native entry version, never an isoform version. |
| `first_public` | [PartialDate](#partialdate) or null | no | Native first-public date. |
| `last_annotation_update` | [PartialDate](#partialdate) or null | no | Native annotation update date. |
| `last_sequence_update` | [PartialDate](#partialdate) or null | no | Native sequence update date. |
| `release` | [Text](#text) or null | no | Native release when supplied. |
| `sequence_version` | integer or null | no | Native sequence version when supplied. |

### ProteinComplexes

Whole native rows with the independent unfiltered source count.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinComplexHeader](#proteincomplexheader) | yes | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | yes | Native source total before type selection or exact participant admission. |

### ProteinComplexHeader

Exact source complex header admitted by a native participant identity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [ComplexPortalId](#complexportalid) | yes | Native complex accession. |
| `matched_participant` | [Text](#text) | yes | Exact native participant identifier supporting admission. |
| `name` | [Text](#text) | yes | Native source name. |
| `participant_total` | integer (uint64) | yes | Actual participants in this exact complex, never a complex search total. |
| `predicted` | boolean | yes | Literal native predictedComplex flag. |
| `taxon` | integer (uint32) | yes | Source taxonomy identifier. |
| `url` | [Text](#text) | yes | Native complex record URL. |

### ProteinCoordinateConvention

Coordinate convention on the native feature before conversion.

Values: `one_based_inclusive`.

### ProteinDomainFragment

One native match fragment after coordinate conversion.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `dc_status` | [Text](#text) or null | no | Literal discontinuity status supplied by InterPro. |
| `span` | [UncertainSpan0](#uncertainspan0) | yes | Converted interbase span with uncertainty retained. |

### ProteinDomainLocation

One native InterPro location group retaining all discontinuous fragments.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `fragments` | list of [ProteinDomainFragment](#proteindomainfragment) | no | All native fragments; never reduced to a min/max hull. |

### ProteinDomainMatch

One native integrated InterPro match; not every entry type is a domain.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source record and release facts. |
| `entry_type` | [Text](#text) | yes | Literal source entry type, including family, site and repeat. |
| `id` | [InterProId](#interproid) | yes | Integrated entry accession, distinct from member signatures. |
| `locations` | list of [ProteinDomainLocation](#proteindomainlocation) | no | Every native location group in source order. |
| `member_signatures` | list of [ProteinMemberSignature](#proteinmembersignature) | no | Source member signatures remain distinct from integrated entries. |
| `name` | [Text](#text) | yes | Native integrated name. |
| `protein` | [UniProtIsoform](#uniprotisoform) | yes | Exact native protein identity. |
| `protein_length` | integer or null | no | Source protein length when supplied. |
| `taxon` | integer or null | no | Source taxon when supplied. |

### ProteinDomains

Whole native rows with the independent unfiltered source count.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinDomainMatch](#proteindomainmatch) | yes | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | yes | Native source total before type selection or exact participant admission. |

### ProteinFeature

UniProt feature with source attribution and coordinate provenance.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source record and release facts. |
| `feature` | [UniProtFeature](#uniprotfeature) | yes | Shared feature, including native label and residues. |
| `source_coordinates` | [ProteinCoordinateConvention](#proteincoordinateconvention) | yes | Convention before mathematical conversion. |

### ProteinFeatures

Native UniProt feature rows, preserving exact labels and uncertain bounds.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinFeature](#proteinfeature) | yes | Whole native rows in source order; totals apply only to this collection. |

### ProteinFunction

One whole source function paragraph and its own evidence.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `evidence` | list of [Text](#text) | no | All evidence references supplied for this text. |
| `molecule` | [Text](#text) or null | no | Native molecule qualifier; never attributed to another isoform. |
| `text` | [Text](#text) | yes | Whole native source text, without clinical interpretation. |

### ProteinFunctionResult

Whole source function paragraphs and their own evidence.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinFunction](#proteinfunction) | no | Native paragraphs in source order, without inferred interpretation. |

### ProteinGet

Read source protein cards; a symbol returns candidates without choosing a protein. Sections disclose native facts with independent row bounds; function keeps all paragraphs. No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `input` | list of [ProteinInput](#proteininput) | yes | Source accessions or gene symbols, preserving caller order. |
| `section` | list of [ProteinSection](#proteinsection) | no | Select native sections or all bounded first pages; function keeps one whole text record. |
| `section_limit` | integer (uint32) | no | First-page row limit; function keeps one whole native text record and does not page its paragraphs. |

### ProteinHeader

Native entry metadata wrapping the same Protein identity used by the gene slice.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `entry_id` | [Text](#text) | yes | Native source entry name. |
| `gene_ids` | list of [Curie](#curie) | no | Only identifiers explicitly supplied by the source. |
| `gene_names` | list of [Text](#text) | no | Display names only; never identifier joins. |
| `protein` | [Protein](#protein) of [UniProtIsoform](#uniprotisoform) | yes | Shared gene/protein identity fields; never independently restated. |
| `reviewed` | boolean | yes | Native reviewed status. |
| `taxon` | integer (uint32) | yes | Native source taxonomy identifier. |
| `url` | [Text](#text) | yes | Source entry URL. |

### ProteinInput

Exact source accession or a checked display gene symbol needing later resolution. The schema describes the string surface; deserialization checks either identity grammar.

Type: string.

### ProteinIsoform

One source alternative-product row; all stated identifiers stay together.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `direct_sequence` | [ProteinSequenceAvailability](#proteinsequenceavailability) | yes | Explicit direct-sequence retrieval state, independent of descriptive status. |
| `ids` | list of [UniProtIsoform](#uniprotisoform) | no | All source-stated isoform identifiers, including External base identities. |
| `name` | [Text](#text) | yes | Source isoform name. |
| `sequence_ids` | list of [Text](#text) | no | Source VSP sequence identifiers. |
| `sequence_status` | [ProteinIsoformStatus](#proteinisoformstatus) | yes | Native sequence status; no sequence is inferred from it. |
| `synonyms` | list of [Text](#text) | no | Every source synonym. |

### ProteinIsoforms

Alternative products with every stated identifier and explicit direct sequence state.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinIsoform](#proteinisoform) | yes | Whole native rows in source order; totals apply only to this collection. |

### ProteinIsoformStatus

Native UniProt alternative-product sequence status.

Values: `Displayed`, `Described`, `External`.

### ProteinMemberSignature

A member database signature, not an integrated IPR entry.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [Text](#text) | yes | Native signature accession. |
| `database` | [Text](#text) | yes | Native member database name. |
| `name` | [Text](#text) or null | no | Native member signature name. |

### ProteinRelationQuery

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | [Text](#text) | no | The ID of a stored list, from an earlier reply, so later pages read the same data. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](#uniprotisoform) | yes | Exact returned accession; never replaced by the requested base entry. |
| `audit` | [ProteinAudit](#proteinaudit) | yes | Native audit facts for this record. |
| `crc64` | [Text](#text) or null | no | Native checksum when supplied. |
| `length` | integer (uint32) | yes | Native sequence length. |
| `md5` | [Text](#text) or null | no | Native checksum when supplied. |
| `sequence` | [AminoAcids](#aminoacids) | yes | Source residues, never constructed from another entry. |

### ProteinSequenceAvailability

Direct sequence availability is a retrieval fact, never constructed from another entry.

Values: `available`, `not_available`, `not_checked`.

### ProteinSequenceResult

One directly returned source sequence, with exact identity and audit metadata.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `sequence` | [ProteinSequence](#proteinsequence) | yes | Native sequence record; never constructed from another entry. |

### ProteinStructureProperty

A native cross-reference property pair.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `key` | [Text](#text) | yes | Native property key. |
| `value` | [Text](#text) | yes | Native property value, without interpreting residue coverage. |

### ProteinStructureRef

A source structure cross-reference link; no residue coverage claim.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `database` | [Text](#text) | yes | Native cross-reference database. |
| `id` | [Text](#text) | yes | Native cross-reference identifier. |
| `properties` | list of [ProteinStructureProperty](#proteinstructureproperty) | no | All native property pairs, including method, resolution and chains. |
| `url` | [Text](#text) | yes | Publisher link; no structure file is retrieved. |

### ProteinStructures

Source structure links and native properties; no residue coverage claim.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinStructureRef](#proteinstructureref) | yes | Whole native rows in source order; totals apply only to this collection. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele` | [Allele](#allele) or null | no | Source-supplied allele, without local normalization or equivalence inference. |
| `build` | [Build](#build) or null | no | Assembly stated by the source, omitted when absent. |
| `expression` | [Expression](#expression) | yes | Published notation kept unchanged. |
| `source` | [Text](#text) | yes | Source asserting this form. |

### ScientificPValue

Exact normalized scientific p value; no floating-point conversion is used.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `exponent` | integer (int64) | yes | Source base-ten exponent; values below the double range remain exact. |
| `mantissa` | string | yes | Source mantissa spelling, including significant trailing zeros. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `sequence` | [Accession](#accession) | yes | Versioned source sequence, without an inferred genome build. |
| `span` | [Span0](#span0) | yes | Zero-based half-open bounds. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count` | integer or null | no | How many records matched at the source. |
| `fetched` | integer or null | no | How many matching records BioMCP retrieved from the source. |
| `notices` | list of [SourceNotice](#sourcenotice) | no | Notes from the source about the query, such as words it did not index. They are not failures. |
| `reason` | [Reason](#reason) or null | no | Why the source failed, or why BioMCP answered with older stored data. |
| `source` | [Source](#source) | yes | The source BioMCP asked. |
| `state` | [SourceState](#sourcestate) | yes | What happened: `ok`, `stale`, `syncing`, `failed` or `skipped`. |
| `term` | [Text](#text) or null | no | The search term BioMCP sent to the source. |
| `terms` | string | yes | The name of the publisher's terms. |
| `terms_url` | string | yes | The publisher's page with those terms. |
| `use` | [Use](#use) | yes | The source's use group: `open`, `attribution`, `noncommercial` or `restricted`. The publisher's terms decide what you may do with the data. |

### SourceRecord

The source's own entry for an entity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Native source identifier. |
| `url` | [Text](#text) or null | no | Link to that entry. |

### SourceState

State of a source actually asked by the command. [Reply envelope values](/reference/envelope/#sourcesstate) lists each value and its meaning.

### Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `end` | integer (uint32) | yes |  |
| `start` | integer (uint32) | yes |  |

### SponsorType

Registry funding category facet.

Values: `nih`, `fed`, `industry`, `other`.

### Statement

One source claim, kept in its own framework and context.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele_origin` | [Term](#term) or null | no | Source-stated origin. |
| `attribution` | [Attribution](#attribution) | yes | One source record and its provenance. |
| `citations` | list of [Pmid](#pmid) | no | Source publications. |
| `classification` | [Term](#term) | yes | Whole source classification label. |
| `condition` | [Concept](#concept) or null | no | Source disease or tumor context. |
| `criteria` | list of [Text](#text) | no | Source criterion labels in source order. |
| `direction` | [Direction](#direction) or null | no | Source-stated assessment direction. |
| `gene_context` | [GeneRef](#generef) or null | no | Source gene context. |
| `predicate` | [Predicate](#predicate) or null | no | Source-stated relationship. |
| `proposition` | [Proposition](#proposition) | yes | Claim domain. |
| `strength` | [Term](#term) or null | no | Source evidence level or review status. |
| `subject` | [VariantRef](#variantref) | yes | Source-owned subject reference. |
| `therapies` | list of [Concept](#concept) | no | Source therapies in their native order. |

### StatisticEstimate

Source estimate and uncertainty without computed quantities.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count` | integer or null | no | Native count; no inferred denominator. |
| `lower_ci` | [FiniteNumber](#finitenumber) or null | no | Native lower confidence bound. |
| `standard_error` | [FiniteNumber](#finitenumber) or null | no | Native standard error. |
| `upper_ci` | [FiniteNumber](#finitenumber) or null | no | Native upper confidence bound. |
| `value` | [FiniteNumber](#finitenumber) or null | no | Native estimate. |

### StatisticSelector

One source-native selector without a local interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `code` | [Text](#text) | yes | Source selector code. |
| `label` | [Text](#text) or null | no | Publisher label when supplied. |
| `name` | [Text](#text) | yes | Source field name. |

### Status

Overall command outcome. [Reply envelope values](/reference/envelope/#status) lists each value and its meaning.

### Step

A command you can run next. `args` is the command without the leading `biomcp`; `kind` says what it does.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `args` | list of string | yes | The command's words without the leading `biomcp`. The same list works as the MCP tool's arguments. |
| `kind` | [StepKind](#stepkind) | yes | What the command does, such as `page` or `section`. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `downloadUrl` | [Text](#text) or null | no | Native download link, absent for directory entries. |
| `htmlUrl` | [Text](#text) | yes | Publisher browser link. |
| `kind` | [StudyFileKind](#studyfilekind) | yes | Native file or directory kind. |
| `name` | [Text](#text) | yes | Native repository entry name. |
| `path` | [Text](#text) | yes | Exact native repository path. |
| `repositoryBlobSize` | integer (uint64) | yes | GitHub size in repository blob bytes; LFS content size is unknown. |
| `sha` | [StudyBlobSha](#studyblobsha) | yes | Repository blob or tree revision, not a data-content checksum. |

### StudyFileKind

Native GitHub contents entry kind.

Values: `file`, `dir`.

### StudyFiles

One native repository directory page; blob sizes do not describe data content.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Directory acquisition wall time in Unix seconds. |
| `items` | list of [StudyFile](#studyfile) | yes | Source-ordered repository file references. |
| `license_notice` | [Text](#text) or null | no | Native LICENSE publisher link, independent of the current file page. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds within the complete received directory. |
| `study_id` | [StudyId](#studyid) | yes | Exact study directory scope. |

### StudyGet

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [StudyId](#studyid) | yes | Exact source study ID; no default or repair. |
| `section` | [StudySection](#studysection) or null | no | Omit for the card; all returns bounded first pages only. |
| `section_limit` | integer (uint32) | no | Maximum records in each selected section. |
| `section_offset` | integer (uint32) | no | Offset within one named section; all requires zero. |

### StudyHeader

Compact source study identity; source codes and counts retain their own scope.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allSampleCount` | integer (uint64) | yes | Native allSampleCount, never a patient or profiled-gene denominator. |
| `cancerTypeId` | [CbioportalCancerTypeId](#cbioportalcancertypeid) | yes | cBioPortal namespace only; no ontology inference. |
| `citation` | string or null | no | Native citation, absent for unpublished records. |
| `name` | [Text](#text) | yes | Native study name. |
| `pmids` | list of [Pmid](#pmid) or null | no | Every validated source PMID in native order, including repetitions; absent remains absent. |
| `referenceGenome` | [Text](#text) | yes | Native study metadata label, never a coordinate assembly. |
| `studyId` | [StudyId](#studyid) | yes | Exact case-sensitive source identity. |

### StudyId

Type: string.

### StudyProfile

Molecular profile reference with exact study membership.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `datatype` | [Text](#text) | yes | Extensible native data-type label. |
| `description` | string | yes | Source description. |
| `genericAssayType` | [Text](#text) or null | no | Native generic assay type, absent outside those profiles. |
| `molecularAlterationType` | [Text](#text) | yes | Extensible source alteration-type label. |
| `molecularProfileId` | [StudyProfileId](#studyprofileid) | yes | Exact native profile ID; never constructed from a study ID. |
| `name` | string | yes | Source profile name. |
| `patientLevel` | boolean | yes | Native patient-level flag, not a unit inferred for other records. |
| `showProfileInAnalysisTab` | boolean | yes | Native profile display setting. |
| `studyId` | [StudyId](#studyid) | yes | Native profile study scope. |

### StudyProfileId

Type: string.

### StudyProfiles

One independently acquired, bounded native metadata collection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. |
| `items` | list of [StudyProfile](#studyprofile) | yes | Complete ordered metadata records, never member sample IDs. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds and source-stated total for this collection. |
| `study_id` | [StudyId](#studyid) | yes | Exact study scope of this collection. |

### StudySampleList

Sample-list metadata without a representation for member sample IDs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `category` | [Text](#text) | yes | Extensible native list category. |
| `description` | string | yes | Native list description. |
| `name` | string | yes | Native list name. |
| `sampleCount` | integer or null | no | Stated source sampleCount when supplied, never computed from member IDs. |
| `sampleListId` | [StudySampleListId](#studysamplelistid) | yes | Exact native list ID. |
| `studyId` | [StudyId](#studyid) | yes | Native study scope. |

### StudySampleListId

Type: string.

### StudySampleLists

One independently acquired, bounded native metadata collection.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. |
| `items` | list of [StudySampleList](#studysamplelist) | yes | Complete ordered metadata records, never member sample IDs. |
| `page` | [PageInfo](#pageinfo) | yes | Bounds and source-stated total for this collection. |
| `study_id` | [StudyId](#studyid) | yes | Exact study scope of this collection. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `acquired` | integer (uint64) | yes | Catalog acquisition wall time in Unix seconds. |
| `revision` | [Text](#text) | yes | Immutable bundle revision. |

### StudyType

Source-native registry value.

Values: `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS`.

### SubjectQualification

Provenance of the subject's accession qualification, without allele normalization.

Values: `adapter_derived_reference`.

### Summary

A whole source-provided summary with attribution and stated date precision.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `dated` | [PartialDate](#partialdate) or null | no | Date stated by the source, if any. |
| `source` | [SummarySource](#summarysource) | yes | The summary's attribution, independent of the fetching provider. |
| `text` | [Text](#text) | yes | Entire summary text. |

### SummarySource

The attribution on a source-provided summary, independent of fetch providers.

| Value | Meaning |
| --- | --- |
| `refseq` | RefSeq attribution. |

### SyncQuery

Explicit source synchronization selector.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `source` | [Source](#source) | yes | Source whose complete identity data is synchronized. |

### Term

A source label with optional explicitly supported coding.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `coding` | [Coding](#coding) or null | no | Documented framework coding. |
| `label` | [Text](#text) | yes | Original label. |

### Text

Text that is never empty, with spaces removed from both ends.

Type: string.

### TrialConditionResolution

Exact condition identity used for one trial request, without eligibility inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alias` | [DiseaseRelation](#diseaserelation) or null | no | Native relation proving a requested alias, when required. |
| `attribution` | [Attribution](#attribution) or null | no | Resolver record provenance, independent of the trial provider. |
| `components` | list of [ComponentLabel](#componentlabel) | no | Admitted resolver components and their own terms. |
| `concept` | [Concept](#concept) or null | no | Returned concept with its unchanged native primary label. |
| `input` | [Curie](#curie) | yes | Caller-selected CURIE, never a source-name search. |
| `ncit_candidates` | list of [DiseaseRelation](#diseaserelation) | no | Native NCIt mapping rows; duplicate targets keep their independent provenance. |
| `resolver` | [Source](#source) or null | no | Named exact resolver; absent for explicit NCIt input. |
| `selected_ncit` | [Curie](#curie) or null | no | Explicit or unique source-provided NCIt identifier actually sent to NCI. |

### TrialDocument

A posted source document; a URL never comes from caller-supplied filename text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `date` | [PartialDate](#partialdate) or null | no | Source document date. |
| `filename` | [Text](#text) | yes | Exact advertised portable source filename. |
| `has_icf` | boolean or null | no | Source says this contains an informed consent form. |
| `has_protocol` | boolean or null | no | Source says this contains a protocol. |
| `has_sap` | boolean or null | no | Source says this contains a statistical analysis plan. |
| `kind` | string | yes | Native document type abbreviation. |
| `label` | string or null | no | Source label. |
| `license` | [License](#license) | yes | No blanket reuse license is inferred from registry availability. |
| `size` | integer or null | no | Source byte count, when supplied. |
| `uploaded` | string or null | no | Original upload timestamp, without guessing a timezone. |
| `url` | [Text](#text) | yes | Fixed registry CDN URL for this manifest entry. |

### TrialGet

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `input` | list of [NctId](#nctid) | yes | One to twenty exact NCT identifiers, in caller order. |
| `limit` | integer (uint32) | no | Whole-record count per source collection page; text fragments follow the text budget. |
| `offset` | integer (uint32) | no | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. |
| `pool` | [Text](#text) | no | Checked stored source record, supplied by a continuation. |
| `section` | list of [TrialSection](#trialsection) | no | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. |
| `source` | [TrialSource](#trialsource) | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |

### TrialParameter

One exact source request parameter, in canonical name order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `name` | string | yes | Native parameter name. |
| `value` | string | yes | Exact unencoded parameter value. |

### TrialPhase

Source-native registry value.

Values: `EARLY_PHASE1`, `PHASE1`, `PHASE2`, `PHASE3`, `PHASE4`, `NA`.

### TrialRecords

One page of whole records, in the source's order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `continuation` | [TrialGet](#trialget) or null | no | The request that reads the next records. Missing when there are no more. |
| `items` | list of T | yes | Whole records, in the source's order. |
| `offset` | integer (uint32) | yes | How many of the source's records come before this page. |
| `total` | integer (uint32) | yes | How many records the source's list holds. |

### TrialSearch

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `age` | integer (uint32) | no | Age in years; unstated source bounds count as no limit. |
| `condition` | [Text](#text) | no | Registry condition text, or an NCIt code for NCI; use --keyword for NCI free text. |
| `condition_id` | [Curie](#curie) | no | Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. |
| `count_only` | boolean | no | Source total only; no returned study rows. |
| `criteria` | [Text](#text) | no | Search literal registry eligibility text for discovery only; no eligibility judgment. |
| `distance` | [Text](#text) | no | Source distance with explicit mi or km units. |
| `facility` | [Text](#text) | no | Exact registry facility field; cannot combine with radius. |
| `gene` | [GeneInput](#geneinput) | no | Resolve one checked gene for registry text discovery only; no eligibility judgment. |
| `has_results` | boolean | no | Require posted source results. |
| `id` | [NctId](#nctid) | no | Exact NCT identity in a conjunction with other filters. |
| `intervention` | [Text](#text) | no | Literal source intervention text. |
| `keyword` | [Text](#text) | no | NCI free text; explicit keyword mode never substitutes for a disease identity. |
| `lat` | [Text](#text) | no | Radius center latitude in degrees. |
| `limit` | integer (uint32) | no | Headers per page; NCI applies its own lower publisher limit. |
| `lon` | [Text](#text) | no | Radius center longitude in degrees. |
| `next_page` | [Text](#text) | no | Opaque registry continuation; keep filters unchanged. |
| `phase` | [PhaseFilter](#phasefilter) | no | Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3. |
| `query` | [Text](#text) | no | General registry discovery text. |
| `sex` | [TrialSex](#trialsex) | no | Registry sex facet. |
| `source` | [TrialSource](#trialsource) | no | Explicit provider; defaults to ClinicalTrials.gov and never falls back. |
| `sponsor` | [Text](#text) | no | Exact lead sponsor field. |
| `sponsor_type` | [SponsorType](#sponsortype) | no | Registry sponsor funding facet. |
| `status` | [TrialStatus](#trialstatus) | no | Exact registry overall status. |
| `study_type` | [StudyType](#studytype) | no | Exact registry study type. |
| `updated_since` | [PartialDate](#partialdate) | no | First included registry update date. |
| `updated_until` | [PartialDate](#partialdate) | no | Last included registry update date. |
| `variant` | [Text](#text) | no | Literal variant text for registry discovery only; no eligibility judgment. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `end` | [Bound](#bound) | yes |  |
| `start` | [Bound](#bound) | yes |  |

### UniProtAcc

Type: string.

### UniProtFeature

Shared native feature contract without source label remapping.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `alternative` | list of [AminoAcids](#aminoacids) | no | Every stated alternative, without translation. |
| `description` | [Text](#text) or null | no | Complete source description. |
| `evidence` | list of [Text](#text) | no | All original source evidence references. |
| `id` | [Text](#text) or null | no | Native feature identifier. |
| `kind` | [Term](#term) | yes | Source feature label; coding only for documented exact feature types. |
| `original` | [AminoAcids](#aminoacids) or null | no | Source original residues, when supplied. |
| `protein` | [UniProtIsoform](#uniprotisoform) | yes | Exact source sequence identity. |
| `span` | [UncertainSpan0](#uncertainspan0) | yes | Zero-based interbase coordinates retaining uncertain ranges. |

### UniProtIsoform

Type: string.

### UnmappedAssertion

Source classification retained when its category has no approved shared proposition.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Original record provenance. |
| `classification` | [Text](#text) | yes | Unchanged source label. |
| `conditions` | list of [AssertionCondition](#assertioncondition) | no | Native conditions, including identifiers without supplied labels. |
| `review_status` | [Text](#text) or null | no | Native review status. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `genes` | list of [GeneRef](#generef) | no | Genes stated by the sources. |
| `ids` | list of [VariantId](#variantid) | no | All source identifiers, without inferred aliases. |
| `representations` | list of [Representation](#representation) | no | Every source-qualified representation. |

### VariantAssertion

A native record/domain pair, never a vote or merged classification.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `detail` | [AssertionDetail](#assertiondetail) or null | no | Original source detail, returned only for an explicitly selected record. |
| `domain` | [AssertionDomain](#assertiondomain) | yes | Source classification domain. |
| `kind` | [AssertionKind](#assertionkind) | yes | Aggregate/submission/document context. |
| `statement` | [Statement](#statement) or null | no | Supported mapping into the common model. |
| `statement_unmapped` | boolean | no | Native category has no approved common-model proposition. |
| `unmapped` | [UnmappedAssertion](#unmappedassertion) or null | no | Native assertion whose category the common model cannot represent. |

### VariantAssertions

One source-ordered page, with native record and classification boundaries intact.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assertions` | list of [VariantAssertion](#variantassertion) | yes | Source assertion rows in native order. |
| `next` | [VariantAssertionsGet](#variantassertionsget) or null | no | Typed continuation preserving source and selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Page bounds over native rows. |

### VariantAssertionsGet

Page assertions from one explicitly selected evidence source.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assertion` | [Text](#text) or null | no | Native assertion accession or document ID. |
| `input` | [VariantInput](#variantinput) | yes | Checked Variation ID for ClinVar, or CAid for ERepo. |
| `limit` | integer (uint32) | no | Assertions per page; omitted uses the native source default. |
| `offset` | integer (uint32) | no | Assertions to skip in source order. |
| `source` | [AssertionSource](#assertionsource) | yes | One native source; no fallback or text search. |
| `version` | [Text](#text) or null | no | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. |

### VariantCivic

CIViC molecular profiles with their original membership and separate source claims.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | no | Continue this section without changing the input or source selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Disclosure bounds for this section. |
| `profiles` | list of [CivicProfile](#civicprofile) | yes | Source profiles in native order; name candidates carry no attached statements. |

### VariantComponent

Metadata for a component carried by an aggregator.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `aggregator_build` | [Text](#text) | yes | Aggregator build version. |
| `aggregator_built` | [Text](#text) | yes | Aggregator build timestamp, distinct from a component release. |
| `license` | [Text](#text) | yes | Component license URL, without any inferred grant. |
| `name` | [Text](#text) | yes | Component name from aggregator metadata. |
| `note` | [Text](#text) or null | no | Whole source snapshot warning. |
| `terms` | [ComponentTerms](#componentterms) | yes | Independently reviewed publisher terms; no blanket aggregator grant. |
| `uploaded` | [Text](#text) or null | no | Component upload timestamp when the source states one. |
| `uploads` | list of [ComponentUpload](#componentupload) | no | Source-named component upload timestamps when the publisher supplies a map. |
| `version` | [Text](#text) or null | no | Component version; omitted when the source reports none. |

### VariantConservation

Source conservation measurements; no local pathogenicity inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [VariantComponent](#variantcomponent) | yes | Independent component versions and licenses. |
| `context` | list of [PredictorContext](#predictorcontext) | no | Original context arrays, without inferred cross-field alignment. |
| `scores` | list of [PredictorScore](#predictorscore) | yes | Every retained source measurement in declared field order. |
| `subject_qualification` | [SubjectQualification](#subjectqualification) | yes | How the source chromosome label was reference-qualified. |

### VariantCosmic

Historical COSMIC fields carried by MyVariant, without direct COSMIC access.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [VariantComponent](#variantcomponent) | yes | Snapshot version, dates, warning and terms. |
| `next` | [VariantGet](#variantget) or null | no | Typed next page for the same identity and section selection. |
| `records` | list of [CosmicRecord](#cosmicrecord) | yes | Native historical records in source order. |
| `total` | integer (uint64) | yes | Number of native historical fragments before paging. |

### VariantGet

Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. Use search variant --gene BRAF --protein V600E for shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Required for chromosome notation; no assembly is guessed. |
| `civic_cursor` | [Text](#text) or null | no | Native CIViC cursor, retained unchanged. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | no | Native CIViC claim collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected CIViC molecular profile for native claim paging. |
| `input` | [VariantInput](#variantinput) | yes | Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS. |
| `limit` | integer (uint32) | no | Candidate rows per page; more remain reachable through typed next commands. |
| `offset` | integer (uint32) | no | Candidate rows to skip in source order. |
| `resolver` | [VariantResolver](#variantresolver) | no | Resolver for reference-qualified notation; never repairs the input locally. |
| `section` | list of [VariantSection](#variantsection) | no | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. |
| `section_limit` | integer (uint32) | no | Rows per named source section page; omitted uses the native source default. |
| `section_offset` | integer (uint32) | no | Source rows to skip independently of identity candidate paging. |
| `tumor_type` | [Text](#text) or null | no | Explicit tumor context for keyed OncoKB annotation. |

### VariantGwas

One page of exact rsID catalog associations.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `associations` | list of [GwasAssociation](#gwasassociation) | yes | Native source associations, without inferred clinical meaning. |
| `continuation` | [VariantGet](#variantget) or null | no | Continue this section using the same rsID and source selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Exact source page bounds and total. |
| `rs_id` | [VariantId](#variantid) | yes | Exact source rsID. |

### VariantHeader

Consumer identity header with an exact source selection and compact annotation entity.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `component_terms` | list of [ComponentLabel](#componentlabel) | no | Terms for carried component identifiers on this consumer header. |
| `identity` | [Variant](#variant) | yes | Source-qualified variant entity. |
| `myvariant` | [Variant](#variant) or null | no | Compact MyVariant annotations, attributed by their field name. |
| `record` | [SourceRecord](#sourcerecord) | yes | Exact source record, including source keys that are expressions rather than identifiers. |
| `requested` | [VariantId](#variantid) or null | no | Checked caller ID used by a source-only section, without asserting resolved identity. |
| `sections` | [VariantSections](#variantsections) or null | no | Explicitly requested source sections for this exact candidate. |
| `source` | [Source](#source) | yes | Source responsible for this identity. |

### VariantHotspots

Source residue recurrence counts without allele-frequency or clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | no | Continue this section without changing identity selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Disclosure bounds for this section. |
| `reason` | [Reason](#reason) or null | no | Source rows remain candidates when no transcript/consequence join exists. |
| `records` | list of [HotspotRecord](#hotspotrecord) | yes | Native residue records in source order. |

### VariantId

Type: string.

### VariantInput

Type: string.

### VariantOncokb

Keyed OncoKB annotation, preserving the caller's assembly and tumor context.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `allele_exist` | boolean or null | no | Native alternate allele curation flag. |
| `build` | [Build](#build) | yes | Explicit caller assembly, checked against the response query. |
| `data_version` | [Text](#text) or null | no | Native dataset version. |
| `gene_exist` | boolean or null | no | Source gene curation flag. |
| `hgvsg` | [Text](#text) | yes | Exact genomic query sent to OncoKB. |
| `hotspot` | boolean or null | no | Source hotspot flag, without an actionability conclusion. |
| `last_update` | [Text](#text) or null | no | Native release date, kept in its published spelling. |
| `next` | [VariantGet](#variantget) or null | no | Continue without changing identity, build, tumor context or source. |
| `page` | [PageInfo](#pageinfo) | yes | Disclosure bounds over the records, independent of identity paging. |
| `records` | list of [OncokbVariantRecord](#oncokbvariantrecord) | yes | Source claims and text in native field order. |
| `tumor_type` | [Text](#text) | yes | Explicit caller tumor type, checked against the response query. |
| `variant_exist` | boolean | yes | Native exact variant curation flag; never a clinical conclusion. |
| `vus` | boolean or null | no | Source investigated-but-unknown flag, never used to create a classification. |

### VariantPopulation

Bounded population summary with independent optional sequencing datasets.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `build` | [Build](#build) | yes | Source dataset assembly. |
| `dataset` | [Text](#text) | yes | Source dataset selector. |
| `exome` | [GnomadCounts](#gnomadcounts) or null | no | Exome observation; omitted when the source dataset is null. |
| `genome` | [GnomadCounts](#gnomadcounts) or null | no | Genome observation; omitted when the source dataset is null. |
| `variant` | [Text](#text) | yes | Source allele identifier. |

### VariantPredictors

Source prediction scores and the terms of their individual components.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [VariantComponent](#variantcomponent) | yes | Independent component versions and licenses. |
| `context` | list of [PredictorContext](#predictorcontext) | no | Original context arrays, without inferred cross-field alignment. |
| `scores` | list of [PredictorScore](#predictorscore) | yes | Every retained source score in declared field order. |
| `subject_qualification` | [SubjectQualification](#subjectqualification) | yes | How the source chromosome label was reference-qualified. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `civic` | [Section](#section) of [VariantCivic](#variantcivic) or null | no | Explicit civic source result. |
| `clinvar` | [Section](#section) of [VariantAssertions](#variantassertions) or null | no | ClinVar native assertion rows. |
| `conservation` | [Section](#section) of [VariantConservation](#variantconservation) or null | no | Native conservation scores. |
| `cosmic` | [Section](#section) of [VariantCosmic](#variantcosmic) or null | no | Historical MyVariant-associated COSMIC fragments. |
| `erepo` | [Section](#section) of [VariantAssertions](#variantassertions) or null | no | ClinGen native assertion rows. |
| `gwas` | [Section](#section) of [VariantGwas](#variantgwas) or null | no | Explicit GWAS Catalog source result. |
| `hotspots` | [Section](#section) of [VariantHotspots](#varianthotspots) or null | no | Explicit hotspots source result. |
| `oncokb` | [Section](#section) of [VariantOncokb](#variantoncokb) or null | no | Explicit keyed OncoKB source result. |
| `population` | [Section](#section) of [VariantPopulation](#variantpopulation) or null | no | Explicit population source result. |
| `predict` | [Section](#section) of [VariantPredictors](#variantpredictors) or null | no | Native prediction scores. |

### WhoProductId

Type: string.

### WikiPathwaysId

Type: string.

## Next

[Reply schemas](/reference/schemas/) lists every reply and the commands that return it. [Results](/results/) explains how to read a reply from the command line.
