---
title: Reply envelope and shared types
description: The fields every BioMCP 1.0.0 JSON reply carries, and every type
  that more than one reply schema uses.
goal: Look up an envelope field or a type that several BioMCP replies share.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: Shared types
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. Every reply has the same outer fields, the **[envelope](/reference/schemas/types/)**: `schema`, `status`, `query`, `result`, `sources` and `next`. This page defines those fields and every type that more than one reply uses. Each [reply schema](/reference/schemas/) page shows a real reply and the types only that reply has.

## Envelope

One versioned command response; schema identity comes only from its body type. JSON readers ignore unknown fields and unknown growable enum values within a major. [Reply envelope values](/reference/envelope/) gives the meaning of each `status`, source state and reason.

| Field | Type | Meaning |
| --- | --- | --- |
| `local_error` | [Reply_CacheProblem](#reply-cacheproblem) or null | A problem with BioMCP's local cache: `busy`, `database` or `version`. The sources may still have answered. Optional. |
| `next` | list of [Reply_Step](#reply-step) | Commands you can run next, such as the next page or another section. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) or null | For a search page: how many results it skipped, returned and matched in all. Optional. |
| `problems` | list of [Reply_Problem](#reply-problem) | Fields BioMCP could not accept from a source record, each with its reason. Optional. |
| `query` | the query type of each schema | The request as BioMCP read it from the command's arguments. |
| `result` | the result type of each schema, or null | The data the command found. Missing when there is none to return. Optional. |
| `schema` | string | The versioned reply schema ID. |
| `sources` | list of [Reply_SourceOutcome](#reply-sourceoutcome) | One entry for each source the command asked, with what happened. Optional. |
| `status` | [Reply_Status](#reply-status) | How the command went: `complete`, `partial`, `absent` or `failed`. |

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

### Reply_Accession

A reference sequence accession: RefSeq or Ensembl with its version, or LRG.

Type: string.

### Reply_AcmgAmp2015Code

Checked AcmgAmp2015 codes.

Values: `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### Reply_AdverseEventAggregate

Source-native aggregation of matching FAERS reports.

| Value | Meaning |
| --- | --- |
| `reactions` | Exact reaction-term buckets from openFDA; reports may contribute to several terms. |

### Reply_AdverseEventGet

Exact native report identity and an optional independently paged section.

| Field | Type | Meaning |
| --- | --- | --- |
| `drug_index` | integer or null | Absolute source-array index; required only for identifier sections. Optional. |
| `id` | [Reply_ReportId](#reply-reportid) | Published report ID with no source prefix. |
| `section` | [Reply_AdverseEventSection](#reply-adverseeventsection) or null | FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only. Optional. |
| `section_limit` | integer (uint32) | Whole source rows per section page. Optional. |
| `section_offset` | integer (uint32) | Offset within the selected native section. Optional. |
| `snapshot` | [Reply_Text](#reply-text) or null | Checked retained report snapshot for provider-free continuation. Optional. |
| `source` | `"faers"` or `"maude"` | FAERS by default; MAUDE is explicit. Optional. |

### Reply_AdverseEventHeader

Small source report header without a drug title, flattened lists or count buckets.

| Field | Type | Meaning |
| --- | --- | --- |
| `event_type` | string or null | Native event_type string; present empty values remain present. Optional. |
| `first_received` | string or null | Native first_received string; present empty values remain present. Optional. |
| `id` | [Reply_ReportId](#reply-reportid) | Exact published source identity. |
| `serious` | string or null | Native serious string; present empty values remain present. Optional. |
| `source` | [Reply_AdverseEventSource](#reply-adverseeventsource) | Explicit report family. |
| `version` | string or null | Native version string; present empty values remain present. Optional. |

### Reply_AdverseEventSearch

Search five report headers by default. MAUDE accepts device text or a manufacturer, product-code or problem filter. One report may list several devices; filters may match different devices. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

| Field | Type | Meaning |
| --- | --- | --- |
| `aggregate` | [Reply_AdverseEventAggregate](#reply-adverseeventaggregate) or null | FAERS reaction counts. Use reactions for source-native count buckets across matching reports. Counts are not rates or causal associations; a report may contribute to several terms. Requires MedDRA and offset zero. Optional. |
| `drug` | list of [Reply_Text](#reply-text) | FAERS drug names; OR. FAERS literal drug name; repeat --drug for OR. A standalone OR joining names inside one --drug value is refused. Other filters AND with the names. BioMCP caps the escaped native query, including filters, at FAERS_QUERY_MAX_BYTES (4096 bytes). Optional. |
| `limit` | integer (uint32) | Native report headers, counted FAERS reaction terms or VAERS rows; default five. Optional. |
| `manufacturer` | [Reply_Text](#reply-text) or null | MAUDE manufacturer. MAUDE manufacturer phrase on device.manufacturer_d_name. A report may list several devices; filters may match different devices. Optional. |
| `offset` | integer (uint32) | Native report row offset; reaction counts require zero. VAERS needs its snapshot. Optional. |
| `problem` | [Reply_Text](#reply-text) or null | MAUDE product problem. MAUDE phrase on native product_problems. Optional. |
| `product_code` | list of [Reply_Text](#reply-text) | MAUDE product codes; OR. MAUDE exact native device.device_report_product_code; repeated codes join with OR. Optional. |
| `reaction` | [Reply_Text](#reply-text) or null | FAERS MedDRA phrase. FAERS MedDRA term phrase match, not exact equality. Optional. |
| `received_from` | [Reply_PartialDate](#reply-partialdate) or null | First date, YYYY-MM[-DD]. Inclusive FAERS or MAUDE FDA receipt lower bound, YYYY-MM-DD or YYYY-MM; a month starts on its first day. Optional. |
| `received_to` | [Reply_PartialDate](#reply-partialdate) or null | Last date, YYYY-MM[-DD]. Inclusive FAERS or MAUDE FDA receipt upper bound, YYYY-MM-DD or YYYY-MM; a month ends on its last day. Optional. |
| `section` | [Reply_VaersSection](#reply-vaerssection) or null | Independent VAERS table; omitted means reported symptoms. Optional. |
| `serious` | [Reply_AdverseEventSerious](#reply-adverseeventserious) or null | FAERS seriousness flag. FAERS report flag: any uses the source seriousness flag; the specific flags can overlap. Never a reaction outcome. Optional. |
| `snapshot` | [Reply_Text](#reply-text) or null | VAERS snapshot. Retained VAERS table for provider-free continuation. Optional. |
| `source` | [Reply_AdverseEventSource](#reply-adverseeventsource) | Source; default FAERS. FAERS by default; VAERS accepts only exact supported vaccine names. Optional. |
| `text` | [Reply_Text](#reply-text) | Drug, device or vaccine. Literal drug text, MAUDE device phrase or exact supported VAERS vaccine. Optional. |

### Reply_AdverseEventSection

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

### Reply_AdverseEventSerious

Report-level seriousness filter, independent of reaction outcome.

| Value | Meaning |
| --- | --- |
| `any` | Native report seriousness flag. |
| `death` | Native report death flag. |
| `hospitalization` | Native report hospitalization flag. |
| `lifethreatening` | Native report life-threatening flag. |
| `disabling` | Native report disabling flag. |
| `congenital-anomaly` | Native report congenital anomaly flag. |
| `other` | Native report other serious flag. |

### Reply_AdverseEventSource

An explicit source; never probe the other family.

| Value | Meaning |
| --- | --- |
| `faers` | Drug event reports. |
| `maude` | Device event reports. |
| `vaers` | Explicit CDC WONDER vaccine aggregate tables. |

### Reply_Allele

Source-stated interbase replacement; equality never claims normalized equivalence.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate` | [Reply_Sequence](#reply-sequence) |  |
| `location` | [Reply_SequenceLocation](#reply-sequencelocation) |  |
| `reference` | [Reply_Sequence](#reply-sequence) |  |

### Reply_AlphaGenomeBiosample

Native biosample metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | string | Publisher sample name. |
| `stage` | string or null | Publisher stage text, when supplied. Optional. |
| `type` | [Reply_AlphaGenomeBiosampleType](#reply-alphagenomebiosampletype) or null | Publisher sample category. Optional. |

### Reply_AlphaGenomeBiosampleType

Publisher BiosampleType vocabulary.

Values: `BIOSAMPLE_TYPE_UNSPECIFIED`, `BIOSAMPLE_TYPE_PRIMARY_CELL`, `BIOSAMPLE_TYPE_IN_VITRO_DIFFERENTIATED_CELLS`, `BIOSAMPLE_TYPE_CELL_LINE`, `BIOSAMPLE_TYPE_TISSUE`, `BIOSAMPLE_TYPE_TECHNICAL_SAMPLE`, `BIOSAMPLE_TYPE_ORGANOID`.

### Reply_AlphaGenomeDataType

Original tensor scalar representation.

Values: `DATA_TYPE_FLOAT32`.

### Reply_AlphaGenomeEndedness

Publisher Endedness vocabulary.

Values: `ENDEDNESS_UNSPECIFIED`, `ENDEDNESS_SINGLE`, `ENDEDNESS_PAIRED`.

### Reply_AlphaGenomeGene

Native gene metadata for a tensor row.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene_id` | [Reply_Text](#reply-text) | Publisher gene identifier. |
| `junction_end` | integer or null | Publisher junction end, when supplied. Optional. |
| `junction_start` | integer or null | Publisher junction start, when supplied. Optional. |
| `name` | string or null | Publisher gene name. Optional. |
| `strand` | [Reply_AlphaGenomeStrand](#reply-alphagenomestrand) or null | Publisher gene strand. Optional. |
| `type` | string or null | Publisher gene type. Optional. |

### Reply_AlphaGenomeInterval

A requested interval; the scoring response does not echo an interval field.

| Field | Type | Meaning |
| --- | --- | --- |
| `chromosome` | [Reply_Contig](#reply-contig) | Exact chromosome sent. |
| `end` | integer (int64) | Zero-based exclusive end. |
| `start` | integer (int64) | Zero-based inclusive start. |
| `strand` | [Reply_AlphaGenomeStrand](#reply-alphagenomestrand) | Requested strand. |

### Reply_AlphaGenomeNonfiniteKind

Native nonfinite scalar category, without assigning a missing-value meaning.

Values: `nan`, `positive-infinity`, `negative-infinity`.

### Reply_AlphaGenomeOntology

Native ontology identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | integer (int64) | Publisher numeric identifier. |
| `namespace` | [Reply_AlphaGenomeOntologyType](#reply-alphagenomeontologytype) | Publisher ontology vocabulary. |

### Reply_AlphaGenomeOntologyType

Publisher OntologyType vocabulary.

Values: `ONTOLOGY_TYPE_UNSPECIFIED`, `ONTOLOGY_TYPE_CLO`, `ONTOLOGY_TYPE_UBERON`, `ONTOLOGY_TYPE_CL`, `ONTOLOGY_TYPE_EFO`, `ONTOLOGY_TYPE_NTR`.

### Reply_AlphaGenomeScalar

One native scalar; finite values remain JSON numbers and nonfinite values stay explicit.

One of these forms:

- [Reply_FiniteNumber](#reply-finitenumber): A finite JSON number.
- An object: A native nonfinite value without invented units, payload meaning or missingness. Fields: `kind` ([Reply_AlphaGenomeNonfiniteKind](#reply-alphagenomenonfinitekind)).

### Reply_AlphaGenomeScorer

The scorer explicitly requested at this ordinal; not a native response label.

Values: `gene-mask-lfc-rna-seq`, `gene-mask-splicing-splice-sites`, `center-mask-diff-mean-dnase-501`.

### Reply_AlphaGenomeScorerPage

Independent page of one requested scorer's native values.

| Field | Type | Meaning |
| --- | --- | --- |
| `data_type` | [Reply_AlphaGenomeDataType](#reply-alphagenomedatatype) | Original scalar representation. |
| `next` | [Reply_VariantGet](#reply-variantget) or null | Next explicit section request, when more native values remain. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Value paging within this scorer only. |
| `request_ordinal` | integer (uint32) | Request position used to associate this unlabelled response group. |
| `requested_scorer` | [Reply_AlphaGenomeScorer](#reply-alphagenomescorer) | Requested scorer identity, distinct from native metadata. |
| `rows` | list of [Reply_AlphaGenomeValue](#reply-alphagenomevalue) | Paged values in native row-major order. Optional. |
| `shape` | list of integer (uint32) | Complete native tensor dimensions, including every plane. |

### Reply_AlphaGenomeStrand

Publisher Strand vocabulary.

Values: `STRAND_UNSPECIFIED`, `STRAND_POSITIVE`, `STRAND_NEGATIVE`, `STRAND_UNSTRANDED`.

### Reply_AlphaGenomeTrack

Native metadata for one tensor track.

| Field | Type | Meaning |
| --- | --- | --- |
| `assay` | string or null | Publisher assay text. Optional. |
| `biosample` | [Reply_AlphaGenomeBiosample](#reply-alphagenomebiosample) or null | Native biosample, when supplied. Optional. |
| `data_source` | string or null | Publisher data source. Optional. |
| `endedness` | [Reply_AlphaGenomeEndedness](#reply-alphagenomeendedness) or null | Publisher sequence endedness. Optional. |
| `genetically_modified` | boolean or null | Publisher modification flag. Optional. |
| `gtex_tissue` | string or null | Publisher GTEx tissue. Optional. |
| `histone_mark_code` | string or null | Publisher histone mark. Optional. |
| `name` | string | Publisher track name. |
| `nonzero_mean` | number or null | Publisher nonzero mean, without assigning score units. Optional. |
| `ontology_term` | [Reply_AlphaGenomeOntology](#reply-alphagenomeontology) or null | Native ontology identity, when supplied. Optional. |
| `strand` | [Reply_AlphaGenomeStrand](#reply-alphagenomestrand) | Publisher strand value. |
| `transcription_factor_code` | string or null | Publisher transcription factor code. Optional. |

### Reply_AlphaGenomeValue

One native tensor value with its unchanged coordinate and metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [Reply_AlphaGenomeGene](#reply-alphagenomegene) or null | Native gene metadata, omitted for a nongene aggregate row. Optional. |
| `tensor_index` | list of integer (uint32) | Native row-major tensor coordinate; plane, row and track indices. |
| `track` | [Reply_AlphaGenomeTrack](#reply-alphagenometrack) | Native metadata for the corresponding track. |
| `value` | [Reply_AlphaGenomeScalar](#reply-alphagenomescalar) | Exact numeric value represented by the source scalar. |

### Reply_AlphaGenomeVariant

Exact genomic coordinates supplied in the request and echoed by the source.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate_bases` | [Reply_Sequence](#reply-sequence) | Exact alternate sequence sent. |
| `chromosome` | [Reply_Contig](#reply-contig) | Exact chromosome sent. |
| `position` | integer (int64) | One-based source position. |
| `reference_bases` | [Reply_Sequence](#reply-sequence) | Exact reference sequence sent. |

### Reply_AminoAcids

Nonempty uppercase standard residues plus U, O and X.

Type: string.

### Reply_AmpAscoCap2017LevelCode

Checked AmpAscoCap2017Level codes.

Values: `level_a`, `level_b`, `level_c`, `level_d`.

### Reply_AmpAscoCap2017TierCode

Checked AmpAscoCap2017Tier codes.

Values: `tier_i`, `tier_ii`, `tier_iii`, `tier_iv`.

### Reply_ArticleHeader

Citation metadata from PubMed or Europe PMC.

| Field | Type | Meaning |
| --- | --- | --- |
| `author_count` | integer (uint32) | Whole source author count. |
| `date` | [Reply_PartialDate](#reply-partialdate) or null | Publication date at source precision. Optional. |
| `doi` | [Reply_Doi](#reply-doi) or null | Checked DOI, when supplied. Optional. |
| `electronic_date` | [Reply_PartialDate](#reply-partialdate) or null | Electronic publication date when different from the publication date. Optional. |
| `europepmc_id` | [Reply_EuropePmcId](#reply-europepmcid) or null | Europe PMC source-local identity. Optional. |
| `first_author` | [Reply_Text](#reply-text) or null | First author in source order. Optional. |
| `first_author_id` | [Reply_SemanticScholarAuthorId](#reply-semanticscholarauthorid) or null | Exact Semantic Scholar ID for the displayed native first author, when supplied. Optional. |
| `journal` | [Reply_Text](#reply-text) or null | Journal name. Optional. |
| `pmcid` | [Reply_Pmcid](#reply-pmcid) or null | Checked PMC identity, when supplied. Optional. |
| `pmid` | [Reply_Pmid](#reply-pmid) | Checked PubMed identity. Optional. |
| `title` | [Reply_Text](#reply-text) | Whole source title. Optional. |
| `types` | list of [Reply_PublicationType](#reply-publicationtype) | Clinical publication types. Optional. |
| `types_unknown` | boolean | The provider does not supply checked publication types. Optional. |

### Reply_ArticleInput

A validated canonical ArticleInput string.

Type: string.

### Reply_ArticleItem

One original pool position, including a summary hole.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_by` | integer or null | iCite citing-paper count, only under citation ordering. Optional. |
| `context_license` | [Reply_License](#reply-license) or null | Record-owned license of the citing paper that supplied context text. Optional. |
| `contexts` | [Reply_Capped_Text](#reply-capped-text) or null | First citation context and the source's complete context count. Optional. |
| `found_by` | object | Native one-based source positions, disclosed only for multiple sources. Optional. |
| `header` | [Reply_ArticleHeader](#reply-articleheader) or null | Citation metadata when accepted. Optional. |
| `influential` | boolean or null | Provider's influential flag; absent is distinct from false. Optional. |
| `intents` | list of [Reply_CitationIntent](#reply-citationintent) or null | Citation intents in provider order; absent means not reported. Optional. |
| `key` | [Reply_ArticleKey](#reply-articlekey) | Stable checked paper identity when the provider supplies one. Optional. |
| `pmid` | [Reply_Pmid](#reply-pmid) | The selected PMID, retained when summary mapping fails. Optional. |
| `sentence` | [Reply_ArticleSentence](#reply-articlesentence) or null | First matching sentence in LitSense2 provider order. Optional. |
| `text_withheld` | [Reply_WithheldArticleText](#reply-withheldarticletext) or null | Record-owned text was not admitted under the current use policy. Optional. |

### Reply_ArticleKey

Stable identity independent of whether citation metadata was accepted.

One of these forms:

- `namespace` is `"pubmed"`: PubMed identity. Fields: `id` ([Reply_Pmid](#reply-pmid)).
- `namespace` is `"pmc"`: PMC identity when PubMed is absent. Fields: `id` ([Reply_Pmcid](#reply-pmcid)).
- `namespace` is `"doi"`: DOI when both indexed numeric identities are absent. Fields: `id` ([Reply_Doi](#reply-doi)).
- `namespace` is `"europepmc"`: Europe PMC local source identity. Fields: `id` ([Reply_EuropePmcId](#reply-europepmcid)).
- `namespace` is `"semantic-scholar"`: Semantic Scholar local identity when no indexed identifier exists. Fields: `id` ([Reply_SemanticScholarId](#reply-semanticscholarid)).

### Reply_ArticleSaved

Explicit saved article file, owned by the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Exact byte count. |
| `parts` | integer or null | Number of ordered text parts; absent for original supplement bytes. Optional. |
| `path` | [Reply_Text](#reply-text) | Absolute caller destination. |
| `sha256` | [Reply_Text](#reply-text) | SHA-256 of the saved bytes. |
| `url` | [Reply_Text](#reply-text) | Source URL for the original material. |

### Reply_ArticleSentence

Source-provided sentence resemblance, without a claim of clinical relevance.

| Field | Type | Meaning |
| --- | --- | --- |
| `license` | [Reply_License](#reply-license) or null | Explicit record-owned license, when independently resolved. Optional. |
| `section` | [Reply_Text](#reply-text) or null | Source section, when present. Optional. |
| `text` | [Reply_Text](#reply-text) or null | Source sentence text. Optional. |

### Reply_AssertionAssessment

Source assessment labels and identifiers remain independent.

| Field | Type | Meaning |
| --- | --- | --- |
| `criterion` | [Reply_Text](#reply-text) or null | Native criterion code. Optional. |
| `criterion_id` | [Reply_Text](#reply-text) or null | Native criterion identifier. Optional. |
| `criterion_iri` | [Reply_Text](#reply-text) or null | Native criterion IRI. Optional. |
| `default_strength` | [Reply_Text](#reply-text) or null | Native default strength, distinct from an applied evidence strength. Optional. |
| `default_strength_id` | [Reply_Text](#reply-text) or null | Native default-strength identifier; not an applied evidence strength. Optional. |
| `default_strength_iri` | [Reply_Text](#reply-text) or null | Native default-strength IRI. Optional. |
| `outcome` | [Reply_Text](#reply-text) or null | Source outcome label. Optional. |
| `outcome_conflict` | boolean | The supplied outcome identifier contradicts its supplied Met/Not Met label. Optional. |
| `outcome_id` | [Reply_Text](#reply-text) or null | Source outcome identifier, including known conflicting ID/label pairs. Optional. |
| `outcome_iri` | [Reply_Text](#reply-text) or null | Source outcome IRI. Optional. |

### Reply_AssertionCondition

A native condition with independently optional identifier and label.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_Text](#reply-text) or null | Native accession text, preserved even when no CURIE namespace is recognized. Optional. |
| `database` | [Reply_Text](#reply-text) or null | Native database name without inventing a CURIE prefix. Optional. |
| `id` | [Reply_Curie](#reply-curie) or null | Identifier exactly supplied by the source. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Label only when supplied. Optional. |

### Reply_AssertionConditionRelationship

A source trait relationship without assigning a clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `related_conditions` | list of [Reply_AssertionCondition](#reply-assertioncondition) | Conditions named by the related source trait. Optional. |
| `relationship_type` | [Reply_Text](#reply-text) or null | Relationship type exactly supplied by the source. Optional. |
| `source_conditions` | list of [Reply_AssertionCondition](#reply-assertioncondition) | Conditions on the source trait that owns this relationship. Optional. |

### Reply_AssertionDetail

Source details that do not fit the compact shared statement.

| Field | Type | Meaning |
| --- | --- | --- |
| `approved` | [Reply_PartialDate](#reply-partialdate) or null | Source approval date, distinct from evaluation and publication. Optional. |
| `citations` | list of [Reply_Pmid](#reply-pmid) | Source citations, including assertion-method citations. Optional. |
| `comments` | list of [Reply_Text](#reply-text) | Native classification comments, without extracting clinical claims. Optional. |
| `condition_relationships` | list of [Reply_AssertionConditionRelationship](#reply-assertionconditionrelationship) | Native trait relationships, kept separate from assertion conditions. Optional. |
| `conditions` | list of [Reply_AssertionCondition](#reply-assertioncondition) | All source conditions; ID-only conditions stay ID-only. Optional. |
| `document` | [Reply_AssertionDocument](#reply-assertiondocument) or null | Source document facts for an explicitly selected historical version. Optional. |
| `gene_context` | [Reply_GeneRef](#reply-generef) or null | Native gene context retained when the conclusion cannot map to a statement. Optional. |
| `impact_assertion_type` | [Reply_Text](#reply-text) or null | Native somatic impact assertion type, without deriving a predicate. Optional. |
| `impact_clinical_significance` | [Reply_Text](#reply-text) or null | Native somatic significance, independent of the tier label. Optional. |
| `impact_therapies` | [Reply_Text](#reply-text) or null | Native therapy text, without splitting or assigning it to a predicate. Optional. |
| `met_criteria` | list of [Reply_Text](#reply-text) | Native met criteria retained when no common statement can carry them. Optional. |
| `methods` | list of [Reply_Text](#reply-text) | Assertion method labels, not inferred from clinical testing. Optional. |
| `origins` | list of [Reply_Text](#reply-text) | Source origin labels without reducing multiple observations. Optional. |
| `published` | [Reply_PartialDate](#reply-partialdate) or null | Source publication date; never an inferred evaluation date. Optional. |
| `summary` | [Reply_Text](#reply-text) or null | Source prose, kept separately from structured criteria. Optional. |
| `unmet_criteria` | list of [Reply_Text](#reply-text) | Criteria explicitly marked unmet. Optional. |
| `versions` | list of [Reply_Text](#reply-text) | Native document versions offered by the source. Optional. |

### Reply_AssertionDocument

Native SEPIO facts kept separate from common interpretation fields.

| Field | Type | Meaning |
| --- | --- | --- |
| `assessments` | list of [Reply_AssertionAssessment](#reply-assertionassessment) | Assessments directly supporting the interpretation. Optional. |
| `document_type` | [Reply_Text](#reply-text) | Native interpretation type. |
| `native_id` | [Reply_Text](#reply-text) | Native interpretation identifier, separate from the summary PCER identifier. |
| `outcome_id` | [Reply_Text](#reply-text) or null | Native outcome identifier without assigning a common coding system. Optional. |
| `outcome_iri` | [Reply_Text](#reply-text) or null | Native outcome IRI. Optional. |
| `uuid` | [Reply_Text](#reply-text) | Source UUID checked against the selected summary link. |

### Reply_AssertionDomain

Source classification domains remain separate.

Values: `germline`, `somatic_clinical_impact`, `oncogenicity`.

### Reply_AssertionKind

Native record scope.

Values: `vcv`, `rcv`, `scv`, `erepo`.

### Reply_AssertionSource

Native assertion provider.

Values: `clinvar`, `erepo`.

### Reply_Attribution

Provenance for one source record.

| Field | Type | Meaning |
| --- | --- | --- |
| `approved` | [Reply_PartialDate](#reply-partialdate) or null | Source approval date. Optional. |
| `evaluated` | [Reply_PartialDate](#reply-partialdate) or null | Source evaluation date. Optional. |
| `published` | [Reply_PartialDate](#reply-partialdate) or null | Source publication date. Optional. |
| `record` | [Reply_Text](#reply-text) or null | Native assertion record. Optional. |
| `release` | [Reply_Text](#reply-text) or null | Dataset release. Optional. |
| `source` | [Reply_Text](#reply-text) | Source responsible for the claim. |
| `submitter` | [Reply_Text](#reply-text) or null | Source submitter or panel. Optional. |
| `url` | [Reply_Text](#reply-text) or null | Source record link. Optional. |
| `version` | [Reply_Text](#reply-text) or null | Record version. Optional. |

### Reply_AuthorCounts

Counts describe only this provider record, never a global publication corpus.

| Field | Type | Meaning |
| --- | --- | --- |
| `citation_count` | integer or null | Native citations to this record’s indexed papers. Optional. |
| `h_index` | integer or null | Native h-index for this record’s indexed papers. Optional. |
| `paper_count` | integer or null | Native indexed paper count. Optional. |
| `scope` | [Reply_AuthorCountScope](#reply-authorcountscope) | Counts apply only to this provider record. |

### Reply_AuthorCountScope

Scope of source-reported bibliographic counts.

Values: `provider_record`.

### Reply_AuthorHeader

One source candidate; absent affiliations differ from an empty source collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `affiliations` | list of [Reply_Text](#reply-text) or null | Native affiliation labels; no employment claim. Optional. |
| `counts` | [Reply_AuthorCounts](#reply-authorcounts) or null | Source record counts when supplied. Optional. |
| `id` | [Reply_AuthorId](#reply-authorid) | Exact qualified provider record. |
| `name` | [Reply_Text](#reply-text) | Source display name, never a join key. Optional. |

### Reply_AuthorId

One provider record. Semantic Scholar emits semantic-scholar:ID. ORCID checksums are checked. Variants imply no relationship.

Type: string.

### Reply_AuthorizationDate

A native date kept with its original role.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [Reply_PartialDate](#reply-partialdate) | Source date at its stated precision. |
| `role` | [Reply_Text](#reply-text) | Native date role, without assigning approval meaning. |

### Reply_AuthorizationEvent

A source-scoped regulator event; never an identity-card field.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | One source record and its native provenance. |
| `dates` | list of [Reply_AuthorizationDate](#reply-authorizationdate) | All native dates with separate roles. Optional. |
| `indication` | [Reply_Text](#reply-text) or null | Original indication scope when present. Optional. |
| `jurisdiction` | [Reply_Text](#reply-text) or null | Jurisdiction explicitly stated by the source. Optional. |
| `kind` | [Reply_Text](#reply-text) or null | Original native event type. Optional. |
| `product_number` | [Reply_Text](#reply-text) or null | Explicit native product number, never a synthesized first product. Optional. |
| `record` | [Reply_DrugId](#reply-drugid) | Native regulator record identifier. |
| `status` | [Reply_Text](#reply-text) or null | Original source status; historical positives imply no current status. Optional. |

### Reply_AuthorPaperExternalId

An open native identifier namespace and value; no identity inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) | Native namespace and identifier. |
| `namespace` | [Reply_Text](#reply-text) | Explicit native namespace supplied with the external identifier. |

### Reply_AuthorPaperHeader

One indexed paper in source order; identifiers come only from that source.

| Field | Type | Meaning |
| --- | --- | --- |
| `doi` | [Reply_Doi](#reply-doi) or null | Exact native DOI. Optional. |
| `external_ids` | list of [Reply_AuthorPaperExternalId](#reply-authorpaperexternalid) | Every other native paper identifier. Optional. |
| `paper_id` | [Reply_SemanticScholarId](#reply-semanticscholarid) | The paper's ID at the source (Semantic Scholar). |
| `pmcid` | [Reply_Pmcid](#reply-pmcid) or null | Exact native PMCID. Optional. |
| `pmid` | [Reply_Pmid](#reply-pmid) or null | Exact native PMID. Optional. |
| `title` | [Reply_Text](#reply-text) or null | Whole source title. Optional. |
| `venue` | [Reply_Text](#reply-text) or null | Source venue when supplied and nonempty. Optional. |
| `year` | integer or null | Source publication year. Optional. |

### Reply_AuthorPaperSection

How much to show for each paper. `details` adds the paper's full metadata.

| Value | Meaning |
| --- | --- |
| `details` | Native detailed metadata for the same paper window. |

### Reply_AuthorPapersQuery

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

| Field | Type | Meaning |
| --- | --- | --- |
| `full` | boolean | Legacy alias for details; normalized on construction. Optional. |
| `id` | [Reply_AuthorId](#reply-authorid) | Exact qualified provider record. |
| `limit` | integer (uint32) | Bounded row count. Optional. |
| `offset` | integer (uint32) | Native or retained collection offset. Optional. |
| `pool` | [Reply_Text](#reply-text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |
| `section` | [Reply_AuthorPaperSection](#reply-authorpapersection) | Detailed disclosure when requested. Optional. |

### Reply_BiomarkerHeader

One native biomarker header. Codes identify concepts, not unique result rows.

| Field | Type | Meaning |
| --- | --- | --- |
| `codes` | list of [Reply_NcitId](#reply-ncitid) | Published NCIt codes, in source order. |
| `count` | integer or null | Native NCI trial count across trial statuses. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Native biomarker concept name. Optional. |
| `type` | list of [Reply_Text](#reply-text) | Native type values; an empty source list is omitted. Optional. |

### Reply_BiomarkerSource

Biomarker concept publisher.

Values: `nci`.

### Reply_Bound

A bounded source range; equal min and max denote an exact boundary.

| Field | Type | Meaning |
| --- | --- | --- |
| `max` | integer or null | Optional. |
| `min` | integer or null | Optional. |

### Reply_Build

The assembly stated by the source; no default is inferred.

Type: string.

### Reply_CacheProblem

A problem with BioMCP's local cache. The sources' answers still stand.

Values: `busy`, `database`, `encoding`, `filesystem`, `internal`, `limit`, `snapshot`, `version`.

### Reply_Capped_CellLineChemblRecord

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_CellLineChemblRecord](#reply-celllinechemblrecord) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Capped_CellLineName

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_CellLineName](#reply-celllinename) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Capped_CellLineVariation

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_CellLineVariation](#reply-celllinevariation) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Capped_CellLineWarning

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_CellLineWarning](#reply-celllinewarning) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Capped_CellLineXref

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_CellLineXref](#reply-celllinexref) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Capped_Text

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_Text](#reply-text) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Category

A source gene-level category.

| Value | Meaning |
| --- | --- |
| `amplification` | A stated gene amplification. |
| `deletion` | A stated gene deletion. |
| `fusion` | A stated gene fusion. |
| `exon_mutation` | An exon-scoped molecular change. |
| `oncogenic_mutation` | A source category of oncogenic mutations, without classifying a member. |
| `residue` | Every variant at a stated protein site. |

### Reply_CbioportalCancerTypeId

Native cBioPortal cancer-type code, with no OncoTree equivalence.

Type: string.

### Reply_CellLineChembl

Exact accession-linked metadata with its own native collection total.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Requested exact accession. |
| `fetched` | integer or null | Original retrieval seconds; never a source release date. Optional. |
| `items` | [Reply_Capped_CellLineChemblRecord](#reply-capped-celllinechemblrecord) | Every selected whole metadata row and the native total. |
| `offset` | integer (uint32) | Native metadata page offset. |
| `release` | [Reply_CellLineRelease](#reply-celllinerelease) or null | Independent ChEMBL release, distinct from retrieval time. Optional. |

### Reply_CellLineChemblRecord

One exact accession-linked ChEMBL cell identity; no assay request is implied.

| Field | Type | Meaning |
| --- | --- | --- |
| `cell_id` | [Reply_Identifier_uint64](#reply-identifier-uint64) | Independent native integer cell record identity. |
| `cellosaurus_id` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Native exact accession; adapter verifies the join. |
| `clo` | [Reply_Text](#reply-text) or null | Native CLO identifier; omission is source absence. Optional. |
| `description` | [Reply_Text](#reply-text) or null | Native description. Optional. |
| `efo` | [Reply_Text](#reply-text) or null | Native EFO identifier; omission is source absence. Optional. |
| `id` | [Reply_ChemblCellId](#reply-chemblcellid) | Native cell ChEMBL ID, never a compound or target. |
| `name` | [Reply_Text](#reply-text) or null | Native display name without name-based joining. Optional. |
| `organism` | [Reply_Text](#reply-text) or null | Native organism label. Optional. |
| `taxon` | integer or null | Native organism taxon. Optional. |
| `tissue` | [Reply_Text](#reply-text) or null | Native tissue label. Optional. |

### Reply_CellLineGene

A native variation gene member with only a source-supplied HGNC identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `hgnc` | [Reply_HgncId](#reply-hgncid) or null | Present only if this source member explicitly carries the HGNC identifier. Optional. |
| `reference` | [Reply_CellLineXref](#reply-celllinexref) | Complete original source gene cross-reference. |

### Reply_CellLineGet

One exact identity and an optional bounded named section.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_CellLineInput](#reply-celllineinput) | One exact primary accession or qualified source ID. |
| `section` | [Reply_CellLineSection](#reply-celllinesection) | Omit for the default card; all returns bounded first pages without an assay count. Optional. |
| `section_limit` | integer (uint32) | Explicit row limit for each requested collection; native variation defaults to three. Optional. |
| `section_offset` | integer (uint32) | Offset within each independently requested collection. Optional. |

### Reply_CellLineHeader

Compact source identity with independently capped species and diseases.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Exact primary accession. |
| `category` | [Reply_Text](#reply-text) or null | Native category. Optional. |
| `diseases` | [Reply_Capped_CellLineXref](#reply-capped-celllinexref) | Up to three complete disease references and their independent total. |
| `name` | [Reply_Text](#reply-text) or null | Recommended display name. Optional. |
| `species` | [Reply_Capped_CellLineXref](#reply-capped-celllinexref) | Up to three complete species references and their native total. |
| `variations` | [Reply_Capped_CellLineVariation](#reply-capped-celllinevariation) or null | Up to three of this line's variations that show every filter term, and their total. Only variation-filtered searches carry it. Optional. |

### Reply_CellLineIdentityPage

Species, diseases and warnings page independently; each total belongs to its own collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Selected exact primary accession. |
| `diseases` | [Reply_Capped_CellLineXref](#reply-capped-celllinexref) | Whole disease rows and their independent source total. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `offset` | integer (uint32) | Requested offset applied independently to each collection. |
| `release` | [Reply_CellLineRelease](#reply-celllinerelease) or null | Independently fetched publisher release. Optional. |
| `species` | [Reply_Capped_CellLineXref](#reply-capped-celllinexref) | Whole species rows and their source total. |
| `warnings` | [Reply_Capped_CellLineWarning](#reply-capped-celllinewarning) | Whole scoped warning rows and their independent source total. |

### Reply_CellLineInput

An exact accession or a database-qualified source identity; never a name lookup.

Type: string.

### Reply_CellLineName

One native display name entry, never an identity key.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Reply_Text](#reply-text) | Native name type, preserving identifier and synonym labels. |
| `value` | [Reply_Text](#reply-text) | Native name spelling and punctuation. |

### Reply_CellLineNames

Native display names with collection-local totals and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Selected exact primary accession. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `items` | [Reply_Capped_CellLineName](#reply-capped-celllinename) | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | Offset within this collection, never within another section. |
| `release` | [Reply_CellLineRelease](#reply-celllinerelease) or null | Independently fetched publisher release. Optional. |

### Reply_CellLinePage

Source-ordered cell line search headers; totals remain unknown.

| Field | Type | Meaning |
| --- | --- | --- |
| `fetched` | integer or null | Original retrieval seconds; not a source release date. Optional. |
| `items` | list of [Reply_CellLineHeader](#reply-celllineheader) | Native result order, without reranking or selected identity. Optional. |
| `release` | [Reply_CellLineRelease](#reply-celllinerelease) or null | Independently fetched publisher release, never a result total. Optional. |
| `variation_match` | [Reply_CellLineVariationMatch](#reply-celllinevariationmatch) or null | What a variation filter matched; only variation-filtered searches carry it. Optional. |

### Reply_CellLineProvenance

One complete native source row; reference, institution and database provenance stay independent.

| Field | Type | Meaning |
| --- | --- | --- |
| `institution` | [Reply_Text](#reply-text) or null | Native institution name if supplied. Optional. |
| `reference` | [Reply_CellLineReference](#reply-celllinereference) or null | Native publication reference if supplied. Optional. |
| `value` | [Reply_Text](#reply-text) or null | Native institution value if supplied. Optional. |
| `xref` | [Reply_CellLineXref](#reply-celllinexref) or null | Complete native database provenance if supplied. Optional. |

### Reply_CellLineReference

One native internal publication reference, without requiring PubMed.

| Field | Type | Meaning |
| --- | --- | --- |
| `resource_internal_ref` | [Reply_Text](#reply-text) | Complete native resource reference, for example PubMed=9305600. |

### Reply_CellLineRelease

Publisher release identity, fetched independently from record data.

| Field | Type | Meaning |
| --- | --- | --- |
| `updated` | [Reply_PartialDate](#reply-partialdate) or null | Native release update date. Optional. |
| `version` | [Reply_Text](#reply-text) or null | Native release version. Optional. |

### Reply_CellLineSection

Named disclosure sections; all requests each bounded first page without an assay count.

| Value | Meaning |
| --- | --- |
| `names` | Native display names. |
| `identity` | Independent species, diseases and qualified warnings. |
| `xrefs` | Complete source cross-reference rows. |
| `variants` | Complete native variation rows. |
| `chembl` | Exact accession-linked ChEMBL cell metadata. |
| `all` | Bounded first page of every named section. |

### Reply_CellLineVariants

Whole native variation rows with collection-local totals and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Selected exact primary accession. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `items` | [Reply_Capped_CellLineVariation](#reply-capped-celllinevariation) | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | Offset within this collection, never within another section. |
| `release` | [Reply_CellLineRelease](#reply-celllinerelease) or null | Independently fetched publisher release. Optional. |

### Reply_CellLineVariation

One complete native variation row; descriptions do not become normalized alleles.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Reply_Text](#reply-text) or null | Complete native mutation description including all coordinate spellings. Optional. |
| `genes` | list of [Reply_CellLineGene](#reply-celllinegene) | Every native gene member, including both fusion members. Optional. |
| `mutation_type` | [Reply_Text](#reply-text) or null | Native open mutation label. Optional. |
| `note` | [Reply_Text](#reply-text) or null | Complete native variation note. Optional. |
| `provenance` | list of [Reply_CellLineProvenance](#reply-celllineprovenance) | Every native provenance row in source order; no PMID is required. Optional. |
| `references` | list of [Reply_CellLineXref](#reply-celllinexref) | Every other variation cross-reference in native order. Optional. |
| `variation_type` | [Reply_Text](#reply-text) or null | Native open variation label. Optional. |
| `zygosity` | [Reply_Text](#reply-text) or null | Native zygosity label; absence is not inferred. Optional. |

### Reply_CellLineVariationMatch

The source terms a variation filter sent, and what a match on them means.

| Field | Type | Meaning |
| --- | --- | --- |
| `note` | string | How far a match on those terms can be trusted. |
| `terms` | list of [Reply_Text](#reply-text) | Terms sent to the source, in its spelling. |

### Reply_CellLineWarning

Complete source warning with its native scope; no stock classifier is derived.

| Field | Type | Meaning |
| --- | --- | --- |
| `scope` | [Reply_Text](#reply-text) | Native warning category, such as Problematic cell line. |
| `text` | [Reply_Text](#reply-text) | Complete source text including historical and stock qualifications. |

### Reply_CellLineXref

One complete native cross-reference; database and accession form the source identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Reply_Text](#reply-text) or null | Native source category when supplied. Optional. |
| `discontinued` | [Reply_Text](#reply-text) or null | Native discontinuation qualifier, including values such as probable. Optional. |
| `id` | [Reply_Text](#reply-text) | Native accession, including any internal prefix. |
| `iri` | [Reply_Text](#reply-text) or null | Native IRI when supplied. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Native display label when supplied. Optional. |
| `namespace` | [Reply_Text](#reply-text) | Native database name; no fixed vocabulary. |
| `url` | [Reply_Text](#reply-text) or null | Native source URL when supplied. Optional. |

### Reply_CellLineXrefs

Native source cross-references with collection-local totals and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_CellosaurusAccession](#reply-cellosaurusaccession) | Selected exact primary accession. |
| `fetched` | integer or null | Original complete-card retrieval seconds, distinct from source release. Optional. |
| `items` | [Reply_Capped_CellLineXref](#reply-capped-celllinexref) | Whole source rows and the total in this collection only. |
| `offset` | integer (uint32) | Offset within this collection, never within another section. |
| `release` | [Reply_CellLineRelease](#reply-celllinerelease) or null | Independently fetched publisher release. Optional. |

### Reply_CellosaurusAccession

A checked Cellosaurus accession; primary identity is verified by the adapter.

Type: string.

### Reply_ChemblCellId

A ChEMBL source-local cell ID, never a compound or target identity.

Type: string.

### Reply_Chromosome

A primary chromosome in the caller's reference build.

Type: string.

### Reply_CitationIntent

Source-reported reason for citing a paper.

Type: string.

### Reply_CivicClaimKind

CIViC native claim collection.

Values: `evidence`, `assertions`.

### Reply_CivicClaimPage_VariantGet

Native stream bounds; evidence and assertions have separate cursors.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | [Reply_VariantGet](#reply-variantget) or null | Continue the same source candidate, profile and claim stream. Optional. |
| `returned` | integer (uint32) | Number of native rows displayed in this stream. |
| `total` | integer (uint64) | Native total for this stream. |

### Reply_CivicCondition

Native disease fields remain independent when one value is malformed.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) or null | Source DOID, without a disease-name conversion. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Complete source disease name. Optional. |

### Reply_CivicEvidenceLevel

Native CIViC evidence level.

Values: `A`, `B`, `C`, `D`, `E`.

### Reply_CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

### Reply_CivicEvidenceStatus

Native CIViC evidence review status.

Values: `accepted`, `submitted`, `rejected`.

### Reply_CivicKind

Native publisher vocabulary; new values are retained unchanged. Known values: DIAGNOSTIC; PROGNOSTIC; PREDICTIVE; PREDISPOSING; FUNCTIONAL; ONCOGENIC;

Type: string.

### Reply_CivicMember

A native CIViC member, including members without a common-model subject.

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | [Reply_Text](#reply-text) or null | Native feature label, including fusion features. Optional. |
| `feature_record` | [Reply_Curie](#reply-curie) or null | Native feature identifier, distinct from the variant identifier. Optional. |
| `hgvs_descriptions` | list of [Reply_Text](#reply-text) | Every native notation, without selecting a transcript or repairing a spelling. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Whole source variant label. Optional. |
| `record` | [Reply_SourceRecord](#reply-sourcerecord) | Native variant identity and link. |
| `subject` | [Reply_VariantRef](#reply-variantref) or null | Source-qualified subject when supported by the common model. Optional. |

### Reply_CivicMembership

Identity strength of a CIViC profile lookup.

Values: `exact`, `candidate`.

### Reply_CivicNativeClaim

Source facts retained without inventing a common-model subject or proposition.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_origin` | [Reply_Term](#reply-term) or null | Native origin, when stated. Optional. |
| `attribution` | [Reply_Attribution](#reply-attribution) | Original source provenance. |
| `citations` | list of [Reply_Pmid](#reply-pmid) | Native publication identities. Optional. |
| `classification` | [Reply_Term](#reply-term) or null | Native classification when its source text is nonempty; significance remains on the claim. Optional. |
| `condition` | [Reply_CivicCondition](#reply-civiccondition) or null | Source disease context. Optional. |
| `strength` | [Reply_Term](#reply-term) or null | Native level, when present. Optional. |
| `therapies` | list of [Reply_CivicTherapy](#reply-civictherapy) | Native therapies in source order. Optional. |

### Reply_CivicProfile_VariantGet

One source molecular profile, preserving compound expressions.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertion_count` | integer (uint64) | Native assertion count, independent of the displayed page. |
| `assertion_page` | [Reply_CivicClaimPage_VariantGet](#reply-civicclaimpage-variantget) | Native assertion stream and its independent continuation. |
| `compound` | boolean or null | Native complex-profile flag; a member match does not satisfy the whole profile. Optional. |
| `evidence_count` | integer (uint64) | Native evidence count, independent of the displayed page. |
| `evidence_page` | [Reply_CivicClaimPage_VariantGet](#reply-civicclaimpage-variantget) | Native evidence stream and its independent continuation. |
| `label` | [Reply_Text](#reply-text) or null | Whole source profile label, including its logical expression. Optional. |
| `members` | list of [Reply_CivicMember](#reply-civicmember) | Every source member reference, without collapsing compound profiles. Optional. |
| `membership` | [Reply_CivicMembership](#reply-civicmembership) | Whether a source member was joined to the requested exact identity. |
| `raw_name` | [Reply_Text](#reply-text) or null | Exact source expression with native variant references. Optional. |
| `record` | [Reply_SourceRecord](#reply-sourcerecord) | Native profile identity and link. |
| `statements` | list of [Reply_CivicStatement](#reply-civicstatement) | Source claims; a compound claim retains a profile subject. Optional. |
| `subject` | [Reply_VariantRef](#reply-variantref) or null | Checked source profile subject, independent of an allele join or a clinical claim. Optional. |

### Reply_CivicSignificance

Native publisher vocabulary; new values are retained unchanged. Known values: SENSITIVITYRESPONSE; RESISTANCE; BETTER_OUTCOME; POOR_OUTCOME; POSITIVE; NEGATIVE; NA; ADVERSE_RESPONSE; PATHOGENIC; LIKELY_PATHOGENIC; BENIGN; LIKELY_BENIGN; UNCERTAIN_SIGNIFICANCE; REDUCED_SENSITIVITY; GAIN_OF_FUNCTION; LOSS_OF_FUNCTION; UNALTERED_FUNCTION; NEOMORPHIC; UNKNOWN; DOMINANT_NEGATIVE; PREDISPOSITION; PROTECTIVENESS; ONCOGENICITY; LIKELY_ONCOGENIC; ONCOGENIC;

Type: string.

### Reply_CivicStatement

One CIViC claim with native labels that exceed the common statement model.

| Field | Type | Meaning |
| --- | --- | --- |
| `amp_level` | [Reply_Text](#reply-text) or null | Exact nullable native AMP value, including Not Applicable. Optional. |
| `direction` | [Reply_Text](#reply-text) or null | Original provider direction; disputes never implies the opposite effect. Optional. |
| `kind` | [Reply_CivicKind](#reply-civickind) or null | Source claim category, without a fallback proposition. Optional. |
| `native` | [Reply_CivicNativeClaim](#reply-civicnativeclaim) or null | Complete native claim facts when the common subject or proposition is unavailable. Optional. |
| `record` | [Reply_SourceRecord](#reply-sourcerecord) | Native claim identity, retained even when no common proposition applies. |
| `significance` | [Reply_CivicSignificance](#reply-civicsignificance) or null | Original significance, including when the classification holds an AMP tier. Optional. |
| `statement` | [Reply_Statement](#reply-statement) or null | Common source-attributed claim. Optional. |
| `statement_unmapped` | boolean | Native claim cannot be represented by the common subject or proposition model. Optional. |
| `status` | [Reply_CivicStatus](#reply-civicstatus) or null | Native curation state, without implying that every row is accepted. Optional. |
| `therapy_interaction_type` | [Reply_Text](#reply-text) or null | Native relationship between therapies; the common statement cannot express it. Optional. |

### Reply_CivicStatus

Native publisher vocabulary; new values are retained unchanged. Known values: ACCEPTED; SUBMITTED; REJECTED;

Type: string.

### Reply_CivicTherapy

One native therapy member; a label does not imply a source identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Identifier_uint64](#reply-identifier-uint64) or null | Native therapy identity when requested by this source route. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Complete source therapy name. Optional. |

### Reply_ClingenCgcVicc2022Code

Checked ClingenCgcVicc2022 codes.

Values: `oncogenic`, `likely_oncogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### Reply_ClingenDosageScoreCode

Values: `0`, `1`, `2`, `3`, `30`, `40`.

### Reply_ClingenGeneValidity2017Code

Values: `definitive`, `strong`, `moderate`, `limited`, `disputed`, `refuted`, `no_known_disease_relationship`.

### Reply_ClingenLowPenetrance2024Code

Checked ClingenLowPenetrance2024 codes.

Values: `pathogenic_low_penetrance`, `likely_pathogenic_low_penetrance`, `established_risk_allele`, `likely_risk_allele`, `uncertain_risk_allele`.

### Reply_ClinPgxAccession

A ClinPGx object accession, distinct from the protein allele namespace PaId.

Type: string.

### Reply_ClinVarAlleleId

An allele identifier inside a ClinVar record, not a variant identifier.

Type: string.

### Reply_ClinvarAlleleOriginCode

Checked ClinVar allele origin codes.

Values: `germline`, `somatic`, `de_novo`, `inherited`, `maternal`, `paternal`, `biparental`, `uniparental`, `not_provided`, `unknown`.

### Reply_ClinvarClassificationCode

Values: `germline`, `somatic_clinical_impact`, `oncogenicity`.

### Reply_ClinvarHaplotypeKind

Values: `Haplotype`.

### Reply_ClinvarHaplotypeMember

One source-stated member of a native ClinVar haplotype.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_id` | [Reply_ClinVarAlleleId](#reply-clinvaralleleid) | Native ClinVar allele identity. |
| `id` | [Reply_ClinVarVariationId](#reply-clinvarvariationid) | Native ClinVar variation identity. |

### Reply_ClinvarReviewStatusCode

Checked ClinVar review status codes.

Values: `practice_guideline`, `expert_panel`, `criteria_provided_multiple_submitters_no_conflicts`, `criteria_provided_conflicting_classifications`, `criteria_provided_multiple_submitters`, `criteria_provided_single_submitter`, `no_assertion_criteria_provided`, `no_classification_provided`, `no_classification_for_the_individual_variant`.

### Reply_ClinVarVariationId

A ClinVar variation record, which may represent a compound.

Type: string.

### Reply_Coding

A code in one explicitly named source framework.

One of these forms:

- `system` is `"clinvar_review_status"`: ClinVar publisher review status. Fields: `code` ([Reply_ClinvarReviewStatusCode](#reply-clinvarreviewstatuscode)).
- `system` is `"clinvar_allele_origin"`: ClinVar publisher allele origin. Fields: `code` ([Reply_ClinvarAlleleOriginCode](#reply-clinvaralleleorigincode)).
- `system` is `"oncokb_oncogenicity"`: OncoKB oncogenicity vocabulary. Fields: `code` ([Reply_OncokbOncogenicityCode](#reply-oncokboncogenicitycode)).
- `system` is `"oncokb_level"`: OncoKB evidence levels. Fields: `code` ([Reply_OncokbLevelCode](#reply-oncokblevelcode)).
- `system` is `"civic_evidence_level"`: CIViC evidence level. Fields: `code` ([Reply_CivicEvidenceLevelCode](#reply-civicevidencelevelcode)).
- `system` is `"acmg_amp_2015"`: acmg_amp_2015 vocabulary. Fields: `code` ([Reply_AcmgAmp2015Code](#reply-acmgamp2015code)).
- `system` is `"clingen_low_penetrance_2024"`: clingen_low_penetrance_2024 vocabulary. Fields: `code` ([Reply_ClingenLowPenetrance2024Code](#reply-clingenlowpenetrance2024code)).
- `system` is `"amp_asco_cap_2017_tier"`: amp_asco_cap_2017_tier vocabulary. Fields: `code` ([Reply_AmpAscoCap2017TierCode](#reply-ampascocap2017tiercode)).
- `system` is `"amp_asco_cap_2017_level"`: amp_asco_cap_2017_level vocabulary. Fields: `code` ([Reply_AmpAscoCap2017LevelCode](#reply-ampascocap2017levelcode)).
- `system` is `"clingen_cgc_vicc_2022"`: clingen_cgc_vicc_2022 vocabulary. Fields: `code` ([Reply_ClingenCgcVicc2022Code](#reply-clingencgcvicc2022code)).
- `system` is `"clinvar_classification_type"`: ClinVar classification type. Fields: `code` ([Reply_ClinvarClassificationCode](#reply-clinvarclassificationcode)).
- `system` is `"so"`: Sequence Ontology consequence or edit. Fields: `code` ([Reply_SoCode](#reply-socode)).
- `system` is `"hpo"`: HPO mode of inheritance. Fields: `code` ([Reply_HpoInheritanceCode](#reply-hpoinheritancecode)).
- `system` is `"clingen_gene_validity_2017"`: ClinGen gene validity. Fields: `code` ([Reply_ClingenGeneValidity2017Code](#reply-clingengenevalidity2017code)).
- `system` is `"clingen_dosage_score"`: ClinGen dosage score. Fields: `code` ([Reply_ClingenDosageScoreCode](#reply-clingendosagescorecode)).

### Reply_ComplexPortalId

An exact Complex Portal accession.

Type: string.

### Reply_ComponentLabel

Terms attached to a named carried component on a consumer response.

| Field | Type | Meaning |
| --- | --- | --- |
| `component` | [Reply_Text](#reply-text) | Native component name. |
| `terms` | [Reply_ComponentTerms](#reply-componentterms) | Canonical publisher terms. |

### Reply_ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Reply_Text](#reply-text) | Publisher terms name. |
| `url` | [Reply_Text](#reply-text) | Publisher terms page. |
| `usage` | [Reply_Use](#reply-use) | Publisher use category. |

### Reply_ComponentUpload

One source-named component upload timestamp.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [Reply_SourceDateTime](#reply-sourcedatetime) | Unchanged source timestamp. |
| `name` | [Reply_Text](#reply-text) | Source subcollection name. |

### Reply_Concept

One source concept with its optional ontology identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) or null | Source-stated concept identifier. Optional. |
| `label` | [Reply_Text](#reply-text) | Original concept label. |

### Reply_Contig

A source chromosome or scaffold name without spelling normalization.

Type: string.

### Reply_CosmicCoordinates

Historical coordinates exactly as carried by the component.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | integer (uint64) | Source end value, without coordinate conversion. |
| `start` | integer (uint64) | Source start value, without coordinate conversion. |

### Reply_CosmicId

A COSMIC genomic, legacy transcript or noncoding variant identifier; its kind is retained.

Type: string.

### Reply_CosmicIdentity

Relationship between a carried historical fragment and its aggregator record.

Values: `unverified`.

### Reply_CosmicRecord

One source record from MyVariant's historical COSMIC copy.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate` | [Reply_Text](#reply-text) or null | Unmodified source alternate spelling. Optional. |
| `chromosome` | [Reply_Text](#reply-text) or null | Source chromosome label. Optional. |
| `cosmic_id` | [Reply_CosmicId](#reply-cosmicid) | Native COSMIC record identifier. |
| `hg19` | [Reply_CosmicCoordinates](#reply-cosmiccoordinates) or null | Source hg19 coordinates, without allele construction. Optional. |
| `identity` | [Reply_CosmicIdentity](#reply-cosmicidentity) | Fragment identity has not been independently verified. |
| `mut_freq` | [Reply_FiniteNumber](#reply-finitenumber) or null | Source frequency; units and denominator are not inferred. Optional. |
| `mut_nt` | [Reply_Text](#reply-text) or null | Source nucleotide text, without repair or normalization. Optional. |
| `myvariant_record` | [Reply_Text](#reply-text) | MyVariant parent record association; this does not establish allele equivalence. |
| `reference` | [Reply_Text](#reply-text) or null | Unmodified source reference spelling. Optional. |
| `tumor_site` | list of [Reply_Text](#reply-text) | Every source tumor-site label in native order. Optional. |

### Reply_Curie

Checked prefix:local ontology identifier.

Type: string.

### Reply_DeviceRecallHeader

Native medical device product recall header, independent of event grouping.

| Field | Type | Meaning |
| --- | --- | --- |
| `event_number` | [Reply_Text](#reply-text) or null | Native res_event_number. Optional. |
| `firm` | [Reply_Text](#reply-text) or null | Native recalling_firm. Optional. |
| `initiated` | [Reply_PartialDate](#reply-partialdate) or null | Native event_date_initiated, preserving the source day. Optional. |
| `product_code` | [Reply_Text](#reply-text) or null | Native device category code. Optional. |
| `recall_number` | [Reply_DeviceRecallNumber](#reply-devicerecallnumber) | Exact native product_res_number. |
| `status` | [Reply_Text](#reply-text) or null | Native recall_status, without interpreting completion. Optional. |

### Reply_DeviceRecallNumber

Native FDA product recall number, independent of its event number.

Type: string.

### Reply_DiagnosticCountUnit

Meaning of the envelope's total for a source-owned diagnostic page.

Values: `recall-records`, `device-submissions`, `product-identities`, `test-identities`, `native-relation-rows`, `native-method-entries`.

### Reply_DiagnosticDeviceId

Source device identifier; DEN and H identifiers imply no executable get.

Type: string.

### Reply_DiagnosticGet

Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

| Field | Type | Meaning |
| --- | --- | --- |
| `bundle_revision` | [Reply_Text](#reply-text) or null | Immutable stored bundle revision retained by a continuation. Optional. |
| `id` | [Reply_Text](#reply-text) | Exact native identity, including version for GTR. |
| `section` | [Reply_DiagnosticSection](#reply-diagnosticsection) or null | GTR: genes, conditions, methods. PMA: regulatory, supplements, summary. WHO, 510(k) and device recalls have no sections; omit for the card. Optional. |
| `section_limit` | integer (uint32) | Maximum native rows; omitted means five rows, or one regulatory fragment. Optional. |
| `section_offset` | integer (uint32) | Zero-based native row or fragment offset. Optional. |
| `source` | [Reply_DiagnosticGetSource](#reply-diagnosticgetsource) | Explicit source: GTR accession/version, WHO Product ID, PMA P number, 510(k) K number or device recall Z-NNNN-YYYY. |
| `supplement` | [Reply_PmaSupplement](#reply-pmasupplement) or null | Exact native PMA supplement; absent selects the base row. Optional. |

### Reply_DiagnosticGetSource

One explicit exact diagnostic record source.

Values: `gtr`, `who-ivd`, `fda-pma`, `fda-510k`, `fda-device-recall`.

### Reply_DiagnosticLaboratory

Compact native laboratory context without authorization interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `country` | [Reply_Text](#reply-text) or null | Native country. Optional. |
| `id` | [Reply_Text](#reply-text) | Native laboratory identifier. |
| `institution` | [Reply_Text](#reply-text) or null | Native institution name. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Source laboratory name, absent when the native cell is empty. Optional. |

### Reply_DiagnosticPmaHeader

Compact exact PMA row identity; its native empty base cell is explicit.

| Field | Type | Meaning |
| --- | --- | --- |
| `applicant` | string or null | Native applicant when supplied, including explicit empty. Optional. |
| `decision_code` | string or null | Native decision_code when supplied, including explicit empty. Optional. |
| `decision_date` | string or null | Native decision_date when supplied, including explicit empty. Optional. |
| `pma_number` | [Reply_PmaNumber](#reply-pmanumber) | Native application. |
| `supplement_number` | [Reply_PmaSupplementCell](#reply-pmasupplementcell) or null | Absent differs from explicit empty base cell. Optional. |
| `trade_name` | string or null | Native trade_name when supplied, including explicit empty. Optional. |
| `url` | [Reply_Text](#reply-text) | Exact source record URL. |

### Reply_DiagnosticSearch

Search five source records by default. FDA device recalls cover every medical device; totals count recall records, without harm inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `biomarker` | [Reply_Text](#reply-text) or null | Exact table-0 cell. Exact table-0 biomarker cell; unsupported in the group table. Optional. |
| `bundle_revision` | [Reply_Text](#reply-text) or null | Stored bundle revision. Immutable stored bundle revision retained by a continuation. Optional. |
| `condition_id` | [Reply_Text](#reply-text) or null | Exact MedGen:CUI. Exact MedGen:CUI identifier only. Optional. |
| `decision_from` | [Reply_PartialDate](#reply-partialdate) or null | First date, YYYY-MM-DD. Inclusive native decision date, YYYY-MM-DD; PMA and 510(k) only. Optional. |
| `decision_to` | [Reply_PartialDate](#reply-partialdate) or null | Last date, YYYY-MM-DD. Inclusive native decision date, YYYY-MM-DD; PMA and 510(k) only. Optional. |
| `device_id` | [Reply_DiagnosticDeviceId](#reply-diagnosticdeviceid) or null | PMA or K identifier. Exact PMA application, application/supplement or native K identity. Optional. |
| `drug_application` | [Reply_FdaDrugApplication](#reply-fdadrugapplication) or null | NDA/BLA application. Cell-owned typed NDA/BLA application. Optional. |
| `gene` | [Reply_GeneSymbol](#reply-genesymbol) or null | Exact gene symbol. Exact source gene_symbol field, without alias expansion. Optional. |
| `gene_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) or null | Exact NCBI GeneID. Exact source GeneID, exclusive with gene symbol. Optional. |
| `initiated_from` | [Reply_PartialDate](#reply-partialdate) or null | First date, YYYY-MM-DD. Inclusive initiation date, YYYY-MM-DD; FDA device recalls only. Optional. |
| `initiated_to` | [Reply_PartialDate](#reply-partialdate) or null | Last date, YYYY-MM-DD. Inclusive initiation date, YYYY-MM-DD; FDA device recalls only. Optional. |
| `limit` | integer (uint32) | Five results by default; at most fifty source rows or product headers. Optional. |
| `manufacturer` | [Reply_Text](#reply-text) or null | Manufacturer phrase. WHO manufacturer words, native FDA applicant (PMA/510(k)) or recalling_firm phrase. Optional. |
| `marker` | [Reply_Text](#reply-text) or null | WHO marker phrase. Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Native device name. Native trade_name (PMA) or device_name (510(k)) phrase. Optional. |
| `offset` | integer (uint32) | Zero-based source result offset. Optional. |
| `product_code` | [Reply_Text](#reply-text) or null | FDA product code. Exact native FDA product code; PMA, 510(k) and device recalls only. Optional. |
| `source` | [Reply_DiagnosticSearchSource](#reply-diagnosticsearchsource) | Native diagnostic source. Explicit native GTR, WHO IVD, FDA PMA, 510(k), companion or medical device recall discovery. |
| `type` | [Reply_GtrTestType](#reply-gtrtesttype) or null | Native test_type. Exact native test_type value. Optional. |

### Reply_DiagnosticSearchSource

One explicit diagnostic discovery source.

Values: `gtr`, `fda-cdx`, `fda-pma`, `fda-510k`, `who-ivd`, `fda-device-recall`.

### Reply_DiagnosticSection

Source-supported diagnostic sections; no all section.

| Value | Meaning |
| --- | --- |
| `genes` | GTR native gene rows; five per page. |
| `conditions` | GTR native condition rows; five per page. |
| `methods` | GTR native method entries; five per page. |
| `summary` | FDA SSED PDF text: 25 MiB, 100 pages and 6144 escaped bytes per fragment; scans without text are explicit. |
| `regulatory` | Selected PMA submission statement; one fragment per page. |
| `supplements` | PMA base application supplement headers; five per page. |

### Reply_Direction

Assessment direction stated by the source.

Type: string.

### Reply_DiseaseAssociation

A native source association without clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `agent_type` | [Reply_Text](#reply-text) or null | Native agent type. Optional. |
| `aggregator_knowledge_source` | list of [Reply_Curie](#reply-curie) or null | Native aggregator knowledge sources; missing, null and lists remain distinct. Optional. |
| `attribution` | [Reply_Attribution](#reply-attribution) | Native row provenance and version. |
| `category` | [Reply_Curie](#reply-curie) or null | Native association category. Optional. |
| `evidence` | list of [Reply_DiseaseAssociationReference](#reply-diseaseassociationreference) or null | Source evidence terms; unavailable evidence stays absent. Optional. |
| `evidence_count` | integer or null | Native evidence count, distinct from returned reference count. Optional. |
| `has_count` | integer or null | Native annotation count, without a computed denominator. Optional. |
| `has_percentage` | [Reply_FiniteNumber](#reply-finitenumber) or null | Native percentage, never computed from another field. Optional. |
| `has_quotient` | [Reply_FiniteNumber](#reply-finitenumber) or null | Native quotient, never computed from another field. Optional. |
| `has_total` | integer or null | Native annotation total, without inferring an assayed denominator. Optional. |
| `knowledge_level` | [Reply_Text](#reply-text) or null | Native source knowledge level. Optional. |
| `negated` | boolean or null | Source negation; absent is not false. Optional. |
| `object` | [Reply_DiseaseEndpoint](#reply-diseaseendpoint) | Native returned object. |
| `object_closure` | list of [Reply_Curie](#reply-curie) | Native object closure. Optional. |
| `original_object` | [Reply_Curie](#reply-curie) or null | Original source object before source normalization. Optional. |
| `original_predicate` | [Reply_Text](#reply-text) or null | Native pre-normalization predicate, without interpretation. Optional. |
| `original_subject` | [Reply_Curie](#reply-curie) or null | Original source subject before source normalization. Optional. |
| `predicate` | [Reply_Curie](#reply-curie) | Native source predicate. |
| `primary_knowledge_source` | [Reply_Curie](#reply-curie) or null | Native primary knowledge source. Optional. |
| `provided_by` | [Reply_Text](#reply-text) or null | Native providing collection. Optional. |
| `publications` | list of [Reply_DiseaseAssociationReference](#reply-diseaseassociationreference) | Native publication references, without inferred links or labels. Optional. |
| `qualifiers` | list of [Reply_DiseaseAssociationQualifier](#reply-diseaseassociationqualifier) | Native qualifiers, including frequency or onset if supplied. Optional. |
| `requested_id` | [Reply_Curie](#reply-curie) | The exact requested concept, including a queried parent. |
| `subject` | [Reply_DiseaseEndpoint](#reply-diseaseendpoint) | Native returned subject. |
| `subject_closure` | list of [Reply_Curie](#reply-curie) | Native subject closure. Optional. |
| `subject_taxon` | [Reply_Curie](#reply-curie) or null | Native subject taxon, only as supplied by the publisher. Optional. |

### Reply_DiseaseAssociationItem

One native association and the component terms governing that row.

| Field | Type | Meaning |
| --- | --- | --- |
| `association` | [Reply_DiseaseAssociation](#reply-diseaseassociation) | Complete neutral association, including requested and actual endpoints. |
| `components` | list of [Reply_ComponentLabel](#reply-componentlabel) | Independently attributed annotation components for this row. Optional. |

### Reply_DiseaseAssociationQualifier

A named native qualifier without an inferred role.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [Reply_Text](#reply-text) | Original native qualifier field. |
| `id` | [Reply_Curie](#reply-curie) | Native qualifier identity. |
| `label` | [Reply_Text](#reply-text) or null | Native label when supplied. Optional. |

### Reply_DiseaseAssociationReference

One native association reference, with no invented display label or link.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) | Native evidence or publication identity. |
| `label` | [Reply_Text](#reply-text) or null | Native label when supplied. Optional. |
| `url` | [Reply_Text](#reply-text) or null | Unchanged native link when supplied. Optional. |

### Reply_DiseaseConcept

A disease identity whose optional display label never determines record admission.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) or null | Source-stated exact identifier. Optional. |
| `label` | [Reply_Text](#reply-text) | Native display label when supplied and valid. Optional. |

### Reply_DiseaseEndpoint

One native endpoint with an identifier and an optional source label. Native associations use this reference because a source may omit a display label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) | Exact source identifier. |
| `label` | [Reply_Text](#reply-text) or null | Source label when supplied. Optional. |

### Reply_DiseaseGet

Exact concept or native SEER site with independently paged section.

| Field | Type | Meaning |
| --- | --- | --- |
| `age_range` | [Reply_Text](#reply-text) or null | Native age-range code. Optional. |
| `dataset` | [Reply_Text](#reply-text) or null | Requested native dataset selector, not proof of returned membership. Optional. |
| `id` | [Reply_Text](#reply-text) | Exact ontology CURIE or a SEER site code or name. |
| `interval` | [Reply_Text](#reply-text) or null | Survival-only interval code. Optional. |
| `ontology` | [Reply_DiseaseOntology](#reply-diseaseontology) | One OLS ontology. Optional. |
| `period` | [Reply_Text](#reply-text) or null | Incidence-only native period code. Optional. |
| `race` | [Reply_Text](#reply-text) or null | Native race code. Optional. |
| `section` | [Reply_DiseaseSection](#reply-diseasesection) or null | One named section; absent means identity card only. Optional. |
| `section_limit` | integer (uint32) | Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. Optional. |
| `section_offset` | integer (uint32) | Native row offset within the selected section. Optional. |
| `sex` | [Reply_Text](#reply-text) or null | Native sex code. Optional. |
| `source` | [Reply_DiseaseSource](#reply-diseasesource) | Default MyDisease; no automatic fallback. Optional. |

### Reply_DiseaseOntology

One explicit OLS ontology.

Values: `mondo`, `doid`, `ncit`.

### Reply_DiseaseRelation

A publisher relation with distinct endpoints and predicate.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Publisher record and version. |
| `description` | string or null | Unchanged native cross-reference description when supplied. Optional. |
| `object` | [Reply_DiseaseEndpoint](#reply-diseaseendpoint) | Native object. |
| `predicate` | [Reply_Curie](#reply-curie) | Publisher predicate, without equivalence promotion. |
| `subject` | [Reply_DiseaseEndpoint](#reply-diseaseendpoint) | Native subject. |

### Reply_DiseaseSearch

Literal discovery text or an explicit SEER catalog page.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum source rows on one page. Optional. |
| `offset` | integer (uint32) | Native row offset. Optional. |
| `ontology` | [Reply_DiseaseOntology](#reply-diseaseontology) | Exactly one ontology, default MONDO. Optional. |
| `source` | [Reply_DiseaseSource](#reply-diseasesource) | One explicit source, default OLS. Optional. |
| `text` | string or null | Literal source search text; optional site-name filter for SEER. Optional. |

### Reply_DiseaseSection

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

### Reply_DiseaseSource

An explicit disease source without fallback.

Values: `ols`, `mydisease`, `monarch`, `seer`.

### Reply_DiseaseStatistic

One native statistics row with requested and returned dimensions.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Publisher record and dataset version when supplied. |
| `measure` | [Reply_Text](#reply-text) | Native measure name. |
| `modeled` | [Reply_StatisticEstimate](#reply-statisticestimate) or null | Modeled estimate kept separately. Optional. |
| `observed` | [Reply_StatisticEstimate](#reply-statisticestimate) or null | Observed estimate; missing stays absent. Optional. |
| `requested_selectors` | list of [Reply_StatisticSelector](#reply-statisticselector) | Settings sent to the publisher. Optional. |
| `requested_site` | [Reply_Text](#reply-text) | Explicit selected native site ID. |
| `returned_selectors` | list of [Reply_StatisticSelector](#reply-statisticselector) | Settings actually returned, never filled from request. Optional. |
| `year` | integer or null | Native annual row year. Optional. |

### Reply_Doi

DOI preserving source spelling with ASCII-insensitive equality.

Type: string.

### Reply_DrugApprovals

Explicit native application/product history without a global approval flag.

| Field | Type | Meaning |
| --- | --- | --- |
| `approvals` | [Reply_Section_DrugRecords_DrugFdaSubmission](#reply-section-drugrecords-drugfdasubmission) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [Reply_DrugHeader](#reply-drugheader) | Selected native identity, checked independently of section facts. |
| `order` | string | Presentation order of application submissions, without interpreting their statuses. |
| `products` | list of [Reply_DrugFdaProduct](#reply-drugfdaproduct) | Native products, filtered only by an explicit product number; independent of submissions. Optional. |
| `source_updated` | [Reply_SourceUpdated](#reply-sourceupdated) or null | Native FDA dataset last_updated date, independent of submission dates and retrieval. Optional. |

### Reply_DrugFdaDocument

A native application document link, with embedded text only when its document ID is selected.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [Reply_PartialDate](#reply-partialdate) or null | Native document date, independent of event dates. Optional. |
| `id` | [Reply_Text](#reply-text) | Source document identifier. |
| `text` | [Reply_PdfText](#reply-pdftext) or null | Bounded embedded letter text, present only when selected by document ID. Optional. |
| `type` | [Reply_Text](#reply-text) or null | Original document type. Optional. |
| `url` | [Reply_Text](#reply-text) or null | Original source URL. Optional. |

### Reply_DrugFdaIngredient

An unchanged native ingredient and strength, without local normalization.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Reply_Text](#reply-text) or null | Native ingredient name. Optional. |
| `strength` | [Reply_Text](#reply-text) | Full source strength wording. Optional. |

### Reply_DrugFdaProduct

One explicit native product within the selected FDA application.

| Field | Type | Meaning |
| --- | --- | --- |
| `active_ingredients` | list of [Reply_DrugFdaIngredient](#reply-drugfdaingredient) | Complete source-listed ingredient and strength pairs. Optional. |
| `brand_name` | [Reply_Text](#reply-text) or null | Source brand wording, without inferred ingredient equivalence. Optional. |
| `dosage_form` | [Reply_Text](#reply-text) or null | Native dosage form. Optional. |
| `marketing_status` | [Reply_Text](#reply-text) or null | Product marketing status, independent of submission history. Optional. |
| `product_number` | [Reply_Text](#reply-text) | Exact source product number, scoped by the response application. |
| `reference_drug` | [Reply_Text](#reply-text) or null | Original source reference-drug designation. Optional. |
| `reference_standard` | [Reply_Text](#reply-text) or null | Original source reference-standard designation when supplied. Optional. |
| `route` | [Reply_Text](#reply-text) or null | Native route; absent and null remain unavailable. Optional. |
| `te_code` | [Reply_Text](#reply-text) or null | Native therapeutic-equivalence code when supplied. Optional. |

### Reply_DrugFdaProperty

A source-stated submission property, without interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [Reply_Text](#reply-text) or null | Original property code. Optional. |

### Reply_DrugFdaSubmission

One application-owned submission with common event facts and native FDA context.

| Field | Type | Meaning |
| --- | --- | --- |
| `application_docs` | list of [Reply_DrugFdaDocument](#reply-drugfdadocument) or null | Native documents; document dates remain separate from the submission status date. Optional. |
| `event` | [Reply_AuthorizationEvent](#reply-authorizationevent) | Historical native status, type and status date; no product applicability is implied. |
| `review_priority` | [Reply_Text](#reply-text) or null | Original review priority. Optional. |
| `submission_class_code` | [Reply_Text](#reply-text) or null | Native submission class code. Optional. |
| `submission_class_code_description` | [Reply_Text](#reply-text) or null | Native submission class description. Optional. |
| `submission_number` | [Reply_Text](#reply-text) | Original submission number, scoped by application and event kind. |
| `submission_property_type` | list of [Reply_DrugFdaProperty](#reply-drugfdaproperty) or null | Native property collection; missing stays missing and an empty list stays empty. Optional. |
| `supplement_purpose` | [Reply_DrugFdaSupplementPurpose](#reply-drugfdasupplementpurpose) or null | Literal supplement purpose from its linked FDA letter; never a current-indication inference. Optional. |

### Reply_DrugFdaSupplementPurpose

FDA-letter evidence for an efficacy supplement, independent of the current product label.

One of these forms:

- `state` is `"stated"`: A complete source-stated purpose; may include several indications or other efficacy changes. Fields: `document_id` ([Reply_Text](#reply-text)), `text` ([Reply_Text](#reply-text)), `url` ([Reply_Text](#reply-text)).
- `state` is `"unavailable"`: Missing evidence does not mean the supplement has no indication. Fields: `document_id` ([Reply_Text](#reply-text) or null), `message` (string), `reason` ([Reply_Reason](#reply-reason) or null), `url` ([Reply_Text](#reply-text) or null).

### Reply_DrugGet

A source-selected drug request echoed with a native ID after name resolution.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic_cursor` | [Reply_Text](#reply-text) or null | CIViC continuation. Native continuation cursor for that exact drug, profile and claim collection. Optional. |
| `civic_kind` | [Reply_CivicClaimKind](#reply-civicclaimkind) or null | CIViC collection. Independent native evidence or assertion collection within the selected profile. Optional. |
| `civic_profile` | integer or null | Selected native CIViC molecular profile for a drug-scoped claim stream. Optional. |
| `document_id` | [Reply_Text](#reply-text) or null | FDA letter document ID. Exact native Drugs@FDA approval letter document ID. Text is bounded to 25 MiB, 100 pages and 6144 escaped bytes per fragment. No-text scans are explicit. Optional. |
| `evidence_level` | list of [Reply_CivicEvidenceLevel](#reply-civicevidencelevel) | A–E Native CIViC evidence levels; repeat to include several levels. Optional. |
| `evidence_status` | [Reply_CivicEvidenceStatus](#reply-civicevidencestatus) or null | Status Native CIViC evidence status; omitted preserves the source's existing default. Optional. |
| `input` | [Reply_DrugId](#reply-drugid) |  |
| `label_section` | [Reply_Text](#reply-text) or null | Exact openFDA SPL field. Exact published openFDA SPL section field, including table fields. Omit it to list names. Optional. |
| `product_number` | [Reply_Text](#reply-text) or null | FDA product number. Explicit native application product number, only for approvals. Optional. |
| `section` | list of [Reply_DrugSection](#reply-drugsection) | One named disclosure per call for every source; empty means identity only. Optional. |
| `section_limit` | integer or null | Named section page size, refused on an identity-only card. Optional. |
| `section_offset` | integer or null | Named section continuation, refused on an identity-only card. Optional. |
| `source` | [Reply_DrugSource](#reply-drugsource) |  |

### Reply_DrugHeader

One source candidate, without authorization, targets, label text or interactions.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_DrugId](#reply-drugid) | Published native identifier. |
| `identity_level` | [Reply_DrugIdentityLevel](#reply-drugidentitylevel) | Level established only by the endpoint or record type. |
| `matched_name` | [Reply_DrugNameMatch](#reply-drugnamematch) or null | Complete native name fields matching the search input; omitted outside label searches. Optional. |
| `name` | [Reply_Text](#reply-text) | Original source display name. Optional. |
| `name_resolution` | [Reply_DrugNameResolution](#reply-drugnameresolution) or null | Exact name choice, ranking rule and bounded alternative commands; omitted for native ID calls. Optional. |
| `source` | [Reply_DrugSource](#reply-drugsource) | The single source responsible for this header. |
| `type` | [Reply_Text](#reply-text) or null | Native record type. Optional. |
| `url` | [Reply_Text](#reply-text) or null | Source record URL. Optional. |
| `version` | [Reply_Text](#reply-text) or null | Native source revision, omitted when the source supplies none. Optional. |

### Reply_DrugId

A published native drug record identifier, without a consumer source prefix.

Type: string.

### Reply_DrugIdentity

One source record identity, without any equivalence or authorization claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_DrugId](#reply-drugid) | Published native record identifier. |
| `identity_level` | [Reply_DrugIdentityLevel](#reply-drugidentitylevel) | Level established by the endpoint or record type; unknown stays unknown. |
| `name` | [Reply_Text](#reply-text) | Original source name. Optional. |
| `namespace` | [Reply_Text](#reply-text) | Source responsible for this identity. |
| `type` | [Reply_Text](#reply-text) or null | Native record type, without local classification. Optional. |
| `url` | [Reply_Text](#reply-text) or null | Native record URL. Optional. |

### Reply_DrugIdentityLevel

Identity level established by the source endpoint or record type, never inferred from ID shape.

Type: string.

### Reply_DrugIdentityRelation

One explicitly source-stated identifier, parent, product or formulation relation.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Record supplying the relationship. |
| `identity` | [Reply_DrugIdentity](#reply-drugidentity) | Identity at the other end of the source relation. |
| `relation` | [Reply_Text](#reply-text) | Native relationship label; no inferred equivalence. |

### Reply_DrugNameMatch

Complete native name fields matching the requested label name.

Type: string.

### Reply_DrugNameResolution

Exact source name choice; brand before generic, original FDA application then earliest approval, otherwise source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternatives` | list of [Reply_Step](#reply-step) | Up to five other exact records as ready get commands. Optional. |
| `basis` | [Reply_DrugNameMatch](#reply-drugnamematch) | Name basis selected after demoting a brand identical to its own ingredient. |
| `input` | [Reply_Text](#reply-text) | Original complete name matched case-insensitively. |
| `ranking` | [Reply_Text](#reply-text) | Applied ambiguity ordering, emitted only from fixed nonempty rule text. |
| `total` | integer (uint64) | Total exact source records, including the selected record. |

### Reply_DrugNameSelector

One explicit ChEMBL name filter without fallback or synonym expansion.

Type: string.

### Reply_DrugNativeReference

Typed source reference shared by mechanism and indication records.

| Field | Type | Meaning |
| --- | --- | --- |
| `ref_id` | [Reply_Text](#reply-text) or null | Source reference identifier, when supplied. Optional. |
| `ref_type` | [Reply_Text](#reply-text) | Native reference type. |
| `ref_url` | [Reply_Text](#reply-text) or null | Source reference link, when supplied. Optional. |

### Reply_DrugRecallClassification

FDA's native recall classification, with no local ranking.

Type: string.

### Reply_DrugRecallHeader

One native drug recall record, independent of its event.

| Field | Type | Meaning |
| --- | --- | --- |
| `classification` | [Reply_Text](#reply-text) or null | Native FDA classification, without local interpretation. Optional. |
| `firm` | [Reply_Text](#reply-text) or null | Published recalling firm. Optional. |
| `initiated` | [Reply_PartialDate](#reply-partialdate) or null | Native recall initiation day. Optional. |
| `product_description` | [Reply_Text](#reply-text) or null | Published product wording; harmonized names never enter this field. Optional. |
| `recall_number` | [Reply_DrugRecallNumber](#reply-drugrecallnumber) | Native recall number. |
| `reported` | [Reply_PartialDate](#reply-partialdate) or null | Native enforcement report day. Optional. |
| `status` | [Reply_Text](#reply-text) or null | Published recall status. Optional. |

### Reply_DrugRecallNumber

Native FDA drug recall number, independent of its event number.

Type: string.

### Reply_DrugRecords_DrugFdaSubmission

One bounded native row page, without fabricating an unknown total.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Source row offset. |
| `records` | list of [Reply_DrugFdaSubmission](#reply-drugfdasubmission) | Native source rows in order. Optional. |
| `retrieved` | [Reply_PartialDate](#reply-partialdate) or null | Actual retrieval date, never substituted for source revision. Optional. |
| `total` | integer or null | Native total only when supplied. Optional. |
| `truncated` | boolean or null | Native source truncation indicator. Optional. |
| `version` | [Reply_Text](#reply-text) or null | Dataset or bundle revision. Optional. |

### Reply_DrugSearch

Search one drug source. FDA drug recalls accept a name or native firm, classification, status, reason and initiation dates; counts are recall records. WHO prequalification accepts a name or date range. openFDA labels also accept literal indication wording. Other sources require a name.

| Field | Type | Meaning |
| --- | --- | --- |
| `classification` | [Reply_DrugRecallClassification](#reply-drugrecallclassification) or null | Native FDA classification; drug recalls only. Optional. |
| `firm` | [Reply_Text](#reply-text) or null | Literal published recalling firm phrase; drug recalls only. Optional. |
| `indication` | [Reply_Text](#reply-text) | Literal label indication; openFDA labels only. Search the native Indications and Usage field without disease synonym expansion. An optional name query further narrows the same source search. Optional. |
| `initiated_from` | [Reply_PartialDate](#reply-partialdate) or null | Inclusive initiation day, YYYY-MM-DD; drug recalls only. Optional. |
| `initiated_to` | [Reply_PartialDate](#reply-partialdate) or null | Inclusive initiation day, YYYY-MM-DD; drug recalls only. Optional. |
| `limit` | integer (uint32) | Native candidate rows per page. Optional. |
| `name_selector` | [Reply_DrugNameSelector](#reply-drugnameselector) | Explicit ChEMBL native name selector; omitted means preferred name. Optional. |
| `offset` | integer (uint32) | Native rows to skip. Optional. |
| `prequalified_from` | [Reply_PartialDate](#reply-partialdate) or null | Inclusive WHO prequalification lower bound, YYYY, YYYY-MM or YYYY-MM-DD; WHO PQ only. Optional. |
| `prequalified_to` | [Reply_PartialDate](#reply-partialdate) or null | Inclusive WHO prequalification upper bound, YYYY, YYYY-MM or YYYY-MM-DD; WHO PQ only. Optional. |
| `query` | [Reply_Text](#reply-text) | Name phrase; optional with supported source filters. FDA drug recalls, WHO date ranges and label indication searches allow no name. Drugs@FDA searches brands and ingredients, exact names before word matches. openFDA labels search exact then word brand/generic names, then label text (including indications) when no names match; source matches need not be exact names. Harmonized recall names match but never become display names. Optional. |
| `reason` | [Reply_Text](#reply-text) or null | Phrase in the published recall reason; drug recalls only. Optional. |
| `source` | [Reply_DrugSource](#reply-drugsource) | Selected source. After an empty name lookup, existing sources may supply aliases. Optional. |
| `status` | [Reply_Text](#reply-text) or null | Literal native recall status; drug recalls only. Optional. |

### Reply_DrugSection

Explicit drug disclosures; identity cards never expand into these sections.

Type: string.

### Reply_DrugSource

A supported drug data source.

Type: string.

### Reply_DrugWhoRow

Every published cell of one WHO finished-product record, including empty cells.

| Field | Type | Meaning |
| --- | --- | --- |
| `applicant` | string | Native Applicant wording. |
| `basis_of_listing` | string | Native Basis of Listing wording. |
| `date_of_prequalification` | string | Native Date of Prequalification text, including spacing and empty strings. |
| `dosage_form` | string | Native Dosage Form cell, independent of the combined cell. |
| `inn_dosage_form_strength` | string | Native combined INN, Dosage Form and Strength cell; never split or interpreted. |
| `product_type` | string | Native Product Type, retained even when empty. |
| `reference` | [Reply_Text](#reply-text) | Exact native WHO Reference Number, including spaces and punctuation. |
| `therapeutic_area` | string | Native Therapeutic Area wording. |

### Reply_EmptyQuery

Explicit empty query for commands without selectors.

Type: object.

### Reply_EnsemblGeneId

A human Ensembl gene stable identifier with a separately retained version.

Type: string.

### Reply_EuropePmcId

Europe PMC source prefix followed by its positive numeric local identifier.

Type: string.

### Reply_EvidenceNotice

Explicit successful empty outcomes, never inferred from a source failure.

Values: `not-in-references`, `no-contexts`, `no-recommendations`.

### Reply_Expression

Source expression checked for its reference and syntax only, without a full parser or normalization.

| Field | Type | Meaning |
| --- | --- | --- |
| `syntax` | [Reply_ExpressionSyntax](#reply-expressionsyntax) |  |
| `syntax_version` | [Reply_Text](#reply-text) or null | Optional. |
| `value` | [Reply_Text](#reply-text) |  |

### Reply_ExpressionSyntax

Published syntax label for a source-preserved expression.

Type: string.

### Reply_FdaDrugApplication

Cell-owned drug application preserving native NDA/BLA prefix and leading zeros.

Type: string.

### Reply_FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### Reply_Fraction

Type: number.

### Reply_Gene

Human gene identity and whole source annotations, independent of response cards.

| Field | Type | Meaning |
| --- | --- | --- |
| `aliases` | list of [Reply_Text](#reply-text) | All stated HGNC aliases. Optional. |
| `ensembl_id` | [Reply_EnsemblGeneId](#reply-ensemblgeneid) or null | Ensembl gene identifier, if stated. Optional. |
| `entrez_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) or null | NCBI Gene identifier, if stated. Optional. |
| `hgnc_id` | [Reply_HgncId](#reply-hgncid) | HGNC identifier. |
| `location` | [Reply_GeneLocation](#reply-genelocation) or null | Source-stated location, including unparsed publisher text. Optional. |
| `locus_group` | [Reply_LocusGroup](#reply-locusgroup) or null | HGNC locus group. Optional. |
| `locus_type` | [Reply_LocusType](#reply-locustype) or null | HGNC locus type, if stated and recognized. Optional. |
| `mane` | list of [Reply_Mane](#reply-mane) | Versioned MANE Select transcript pair, if stated. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Gene name. Optional. |
| `omim` | list of [Reply_OmimId](#reply-omimid) | All stated OMIM identifiers. Optional. |
| `previous_symbols` | list of [Reply_GeneSymbol](#reply-genesymbol) | All stated previous symbols. Optional. |
| `spans` | list of [Reply_GenomicInterval](#reply-genomicinterval) | Source intervals with explicit assembly and counting rule. Optional. |
| `status` | [Reply_GeneStatus](#reply-genestatus) | HGNC record status. |
| `summary` | [Reply_Summary](#reply-summary) or null | Whole attributed summary, if stated. Optional. |
| `symbol` | [Reply_GeneSymbol](#reply-genesymbol) | Source-preserved symbol. |
| `uniprot` | list of [Reply_UniProtAcc](#reply-uniprotacc) | All stated UniProt base accessions. Optional. |

### Reply_GeneHeader

Enough gene identity to choose a search result.

| Field | Type | Meaning |
| --- | --- | --- |
| `hgnc_id` | [Reply_HgncId](#reply-hgncid) | HGNC identifier. |
| `location` | [Reply_GeneLocation](#reply-genelocation) or null | Source-stated location, including unparsed publisher text. Optional. |
| `locus_group` | [Reply_LocusGroup](#reply-locusgroup) or null | HGNC locus group. Optional. |
| `match` | [Reply_Match](#reply-match) or null | Match information when this header answers an input. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Gene name. Optional. |
| `status` | [Reply_GeneStatus](#reply-genestatus) | HGNC record status. |
| `symbol` | [Reply_GeneSymbol](#reply-genesymbol) | Source-preserved symbol. |

### Reply_GeneInput

An input classified before source resolution; aliases remain free text.

Type: string.

### Reply_GeneLevel

Values: `gene`.

### Reply_GeneLocation

HGNC location text with a parsed interpretation when its syntax is known.

Type: string.

### Reply_GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl_gene_id` | [Reply_EnsemblGeneId](#reply-ensemblgeneid) or null | Optional. |
| `hgnc_id` | [Reply_HgncId](#reply-hgncid) or null | Optional. |
| `label` | [Reply_Text](#reply-text) |  |
| `ncbi_gene_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) or null | Optional. |

### Reply_GeneResolution

Resolution outcomes preserve ambiguity and withdrawn identity.

One of these forms:

- `resolution` is `"resolved"`: One resolved identity and its source match. Fields: `gene` ([Reply_Gene](#reply-gene)), `match` ([Reply_Match](#reply-match)).
- `resolution` is `"ambiguous"`: Every candidate, with no silent selection. Fields: `candidates` (list of [Reply_GeneHeader](#reply-geneheader)).
- `resolution` is `"withdrawn"`: The original withdrawn identity and stated successors. Fields: `hgnc_id` ([Reply_HgncId](#reply-hgncid)), `hgnc_status` ([Reply_HgncStatus](#reply-hgncstatus)), `merged_into` (list of [Reply_HgncId](#reply-hgncid)), `symbol` ([Reply_GeneSymbol](#reply-genesymbol)).
- `resolution` is `"not-found"`: No source identity matched.

### Reply_GeneStatus

HGNC approval status.

Type: string.

### Reply_GeneSubjectWire

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl_gene_id` | string or null | Optional. |
| `hgnc_id` | [Reply_HgncId](#reply-hgncid) or null | Optional. |
| `label` | [Reply_GeneSymbol](#reply-genesymbol) |  |
| `level` | [Reply_GeneLevel](#reply-genelevel) |  |
| `ncbi_gene_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) or null | Optional. |

### Reply_GeneSymbol

Source spelling of a gene symbol, with ASCII case-insensitive identity.

Type: string.

### Reply_GenomicInterval

An assembly-qualified zero-based half-open source interval.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](#reply-build) or null | Source-stated assembly, omitted when absent. Optional. |
| `chromosome` | [Reply_Chromosome](#reply-chromosome) | Canonical primary chromosome. |
| `span` | [Reply_Span0](#reply-span0) | Checked zero-based half-open bounds; an empty span remains valid. |
| `strand` | [Reply_Strand](#reply-strand) or null | Source-stated strand, if available. Optional. |

### Reply_GenotypeFilter

Source count filtering state.

Values: `raw`.

### Reply_GnomadCounts

Native gnomAD counts retained when a dataset release cannot be established.

| Field | Type | Meaning |
| --- | --- | --- |
| `ac` | integer or null | Native allele count, including reported zero. Optional. |
| `an` | integer or null | Native allele denominator. Optional. |
| `ancestry_group` | [Reply_Text](#reply-text) or null | Source ancestry code decoded only by the documented source grammar. Optional. |
| `data_type` | [Reply_PopulationDataType](#reply-populationdatatype) | Source sequencing sample set. |
| `filters` | list of [Reply_Text](#reply-text) or null | Native source filters, never converted to a classification. Optional. |
| `frequency` | [Reply_PopulationFrequency](#reply-populationfrequency) or null | Checked total from a release supplied alongside matching source counts. Optional. |
| `group_id` | [Reply_Text](#reply-text) or null | Complete native group ID, including its source sex suffix when present. Optional. |
| `homozygote_count` | integer or null | Source-published total homozygote count, never calculated from other counts. Optional. |
| `sex` | [Reply_PopulationSex](#reply-populationsex) or null | Source sex stratum decoded only by the documented source grammar. Optional. |

### Reply_GtrConditionReference

A checked native MedGen target with independently optional source label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) | Native MedGen identity from the condition relation. |
| `label` | [Reply_Text](#reply-text) or null | Native source label. Optional. |

### Reply_GtrConditionRelation

Native condition membership; SNOMED cells never become GeneID.

| Field | Type | Meaning |
| --- | --- | --- |
| `condition` | [Reply_GtrConditionReference](#reply-gtrconditionreference) | Shared MedGen concept with source label. |
| `mim_number` | string | Native MIM cell, including empty. |
| `snomed_ct_id` | string | Native SNOMED CT cell, including empty; never a gene ID. |
| `test_id` | [Reply_GtrTestId](#reply-gtrtestid) | Complete owning test accession/version. |
| `test_type` | [Reply_GtrTestType](#reply-gtrtesttype) or null | Native source test type. Optional. |

### Reply_GtrGeneReference

Independently retained native gene target fields; mapping requires at least one usable key.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Reply_Text](#reply-text) or null | Source gene symbol, absent when only the source GeneID survives. Optional. |
| `ncbi_gene_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) or null | Native GeneID from a gene relation, never a condition's SNOMED field. Optional. |

### Reply_GtrGeneRelation

Native gene relation; its numeric ID comes only from the gene row.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [Reply_GtrGeneReference](#reply-gtrgenereference) | Shared source-given GeneID and gene_symbol; absent GeneID stays absent. |
| `mim_number` | string | Native MIM cell, including empty. |
| `native_gene_or_snomed_ct_id` | string or null | Uninterpreted nonempty `gene_or_SNOMED_CT_ID` when it is not a checked NCBI Gene ID. Optional. |
| `native_medgen` | [Reply_Curie](#reply-curie) or null | Independent native MedGen identity from GTR_identifier. Optional. |
| `object_name` | string | Full source object_name without interpretation. |
| `test_id` | [Reply_GtrTestId](#reply-gtrtestid) | Complete owning test accession/version. |
| `test_type` | [Reply_GtrTestType](#reply-gtrtesttype) or null | Native source test type. Optional. |

### Reply_GtrHeader

Small resolved GTR candidate; broad gene and condition arrays stay off the header.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_GtrTestId](#reply-gtrtestid) | Complete source accession and version. |
| `laboratory` | [Reply_DiagnosticLaboratory](#reply-diagnosticlaboratory) or null | Native laboratory context. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Native test name, absent when the cell is empty. Optional. |
| `url` | [Reply_Text](#reply-text) | Source record URL. |

### Reply_GtrMethod

One entry from one independent native method list.

| Field | Type | Meaning |
| --- | --- | --- |
| `index` | integer (uint32) | Zero-based native list position. |
| `list` | [Reply_GtrMethodList](#reply-gtrmethodlist) | Independent source list; lists are never positionally paired. |
| `test_id` | [Reply_GtrTestId](#reply-gtrtestid) | Complete owning test accession/version. |
| `value` | string | Complete unmodified source entry. |

### Reply_GtrMethodList

Independent native GTR method lists.

Values: `method_categories`, `methods`, `platforms`.

### Reply_GtrTestId

Complete native GTR accession and positive version.

Type: string.

### Reply_GtrTestType

Native GTR relation test category, not an offering or authorization judgment.

Values: `Clinical`, `Research`, `Clinical microbial`.

### Reply_GwasAssociation

Source association with unchanged trait and effect labels.

| Field | Type | Meaning |
| --- | --- | --- |
| `alleles` | list of [Reply_GwasEffectAllele](#reply-gwaseffectallele) | Every source effect allele paired with its rsID. Optional. |
| `author` | [Reply_Text](#reply-text) or null | Source author attribution, without local publication inference. Optional. |
| `beta` | [Reply_Text](#reply-text) or null | Source beta text, including units and direction. Optional. |
| `ci_lower` | [Reply_FiniteNumber](#reply-finitenumber) or null | Source lower effect interval bound. Optional. |
| `ci_upper` | [Reply_FiniteNumber](#reply-finitenumber) or null | Source upper effect interval bound. Optional. |
| `exact_p_value` | [Reply_ScientificPValue](#reply-scientificpvalue) or null | Authoritative exact source probability, including double underflow. Optional. |
| `id` | [Reply_GwasAssociationId](#reply-gwasassociationid) | Native association identity. |
| `interval` | [Reply_Text](#reply-text) or null | Source effect interval. Optional. |
| `locations` | list of [Reply_Text](#reply-text) | Native mapping location strings; no assumed assembly or coordinate conversion. Optional. |
| `mapped_genes` | list of [Reply_Text](#reply-text) | Original native mapped gene context; reported genes remain in loci. Optional. |
| `odds_ratio` | [Reply_FiniteNumber](#reply-finitenumber) or null | Source odds ratio. Optional. |
| `ontology_traits` | list of [Reply_GwasTrait](#reply-gwastrait) | Source mapped trait identifiers and whole labels, without clinical inference. Optional. |
| `p_value` | [Reply_FiniteNumber](#reply-finitenumber) or null | Reported p value. Optional. |
| `publication` | [Reply_Text](#reply-text) or null | Source publication identifier. Optional. |
| `risk_frequency` | [Reply_Text](#reply-text) or null | Source allele frequency text, without interpreting NR. Optional. |
| `study` | [Reply_Text](#reply-text) or null | Study accession. Optional. |
| `traits` | list of [Reply_Text](#reply-text) | Source reported traits. Optional. |

### Reply_GwasAssociationId

A positive Catalog association ID qualified by resource kind.

Type: string.

### Reply_GwasAssociationSection

Association-only section.

| Value | Meaning |
| --- | --- |
| `loci` | Native locus and reported gene context. |

### Reply_GwasEffectAllele

Catalog effect allele paired with its exact rsID.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele` | [Reply_Text](#reply-text) or null | Source allele text. Optional. |
| `rs_id` | [Reply_Text](#reply-text) | Source rsID. |

### Reply_GwasGet

Explicit resource sum; impossible section and identity pairings fail during decoding.

One of these forms:

- `kind` is `"association"`: Read an association. Fields: `id` ([Reply_GwasAssociationId](#reply-gwasassociationid)), `section` ([Reply_GwasAssociationSection](#reply-gwasassociationsection) or null), `section_limit` (integer (uint32)), `section_offset` (integer (uint32)), `snapshot` ([Reply_Text](#reply-text)).
- `kind` is `"study"`: Read a study. Fields: `id` ([Reply_GwasStudyId](#reply-gwasstudyid)), `section` ([Reply_GwasStudySection](#reply-gwasstudysection) or null), `section_limit` (integer (uint32)), `section_offset` (integer (uint32)), `snapshot` ([Reply_Text](#reply-text)).

### Reply_GwasOrder

Native source order, echoed without a configurable sort command.

| Value | Meaning |
| --- | --- |
| `p_value,asc` | Ascending source p value. |

### Reply_GwasSearch

Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

| Field | Type | Meaning |
| --- | --- | --- |
| `extended_geneset` | boolean | Source extended gene set; requires gene. Optional. |
| `gene` | [Reply_GeneSymbol](#reply-genesymbol) | Exact native mapped gene; legacy positional query aliases this field. Optional. |
| `include_child_traits` | boolean | Source child-trait expansion; requires trait_id. Optional. |
| `limit` | integer (uint32) | Maximum association rows. Optional. |
| `offset` | integer (uint32) | Native association offset. Optional. |
| `order` | [Reply_GwasOrder](#reply-gwasorder) | Source order; threshold requests use this same native order. Optional. |
| `p_value` | string | Inclusive positive probability in scientific notation, such as 1e-5. The qualifying total stays unknown until the ordered walk reaches its end. Optional. |
| `pool` | [Reply_Text](#reply-text) | Retained threshold walk identity; requires a threshold. Optional. |
| `pubmed` | [Reply_Pmid](#reply-pmid) | Exact source publication identity. Optional. |
| `studies` | boolean | Return study records for the selected publication instead of associations. Optional. |
| `study` | [Reply_GwasStudyId](#reply-gwasstudyid) | Exact source study accession. Optional. |
| `trait` | [Reply_Text](#reply-text) | Literal native efo_trait label; never searches reported_trait prose. Optional. |
| `trait_id` | [Reply_GwasTraitId](#reply-gwastraitid) | Exact native ontology identity. Optional. |
| `variant` | [Reply_RsId](#reply-rsid) | Exact source rsID. Optional. |

### Reply_GwasStudyId

Exact Catalog study accession; case and leading zeros survive.

Type: string.

### Reply_GwasStudySection

Study-only sections.

| Value | Meaning |
| --- | --- |
| `associations` | Shared association rows. |
| `ancestries` | Independent native ancestry rows. |

### Reply_GwasTrait

Source ontology mapping as supplied by GWAS Catalog.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](#reply-text) | Exact source ontology identifier spelling. |
| `label` | [Reply_Text](#reply-text) or null | Whole source trait label. Optional. |
| `namespace` | [Reply_IdentifierNamespace](#reply-identifiernamespace) | Native authority; unknown means the source did not state one. |

### Reply_GwasTraitId

Native ontology identifier; namespaces are preserved without an EFO restriction.

Type: string.

### Reply_HgncId

An HGNC identifier with canonical namespace and positive numeric value.

Type: string.

### Reply_HgncStatus

HGNC's stated withdrawn-file status.

Type: string.

### Reply_HgvsText

Reference-qualified notation with its original text, without a molecular parse.

Type: string.

### Reply_HotspotAlternate

One source amino-acid alternate count.

| Field | Type | Meaning |
| --- | --- | --- |
| `amino_acid` | [Reply_Text](#reply-text) | Source alternate amino-acid spelling. |
| `count` | integer or null | Source count, without deriving a frequency. Optional. |

### Reply_HotspotMembership

Source transcript and consequence join state.

Values: `exact`, `candidate`.

### Reply_HotspotRecord

Counts for one source transcript and residue label.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternates` | list of [Reply_HotspotAlternate](#reply-hotspotalternate) | Source alternate amino acids and their counts. Optional. |
| `gene` | [Reply_GeneSymbol](#reply-genesymbol) | Gene as stated by the source. |
| `membership` | [Reply_HotspotMembership](#reply-hotspotmembership) | Whether transcript and consequence were linked by the identity source. |
| `residue` | [Reply_Text](#reply-text) | Native residue label; no genomic allele is inferred. |
| `transcript` | [Reply_Text](#reply-text) or null | Native transcript spelling, without an inferred accession version. Optional. |
| `tumor_count` | integer or null | Tumors reported for this residue by the source. Optional. |

### Reply_HpId

Exact Human Phenotype Ontology identifier with seven decimal digits.

Type: string.

### Reply_HpoInheritanceCode

Values: `HP:0000006`, `HP:0000007`, `HP:0001417`, `HP:0001427`, `HP:0032113`.

### Reply_Identifier_int64

An authority and its unchanged native identifier; unknown means the authority was not stated.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | integer (int64) | Native identifier value, with its original scalar type and precision. |
| `namespace` | [Reply_IdentifierNamespace](#reply-identifiernamespace) | Native authority spelling; the literal unknown carries an unstated authority. |

### Reply_Identifier_NonZeroU64

An authority and its unchanged native identifier; unknown means the authority was not stated.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | integer (uint64) | Native identifier value, with its original scalar type and precision. |
| `namespace` | [Reply_IdentifierNamespace](#reply-identifiernamespace) | Native authority spelling; the literal unknown carries an unstated authority. |

### Reply_Identifier_uint64

An authority and its unchanged native identifier; unknown means the authority was not stated.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | integer (uint64) | Native identifier value, with its original scalar type and precision. |
| `namespace` | [Reply_IdentifierNamespace](#reply-identifiernamespace) | Native authority spelling; the literal unknown carries an unstated authority. |

### Reply_IdentifierNamespace

A source-stated authority or an explicit unknown authority, represented by the string unknown.

Type: string.

### Reply_InterProId

An integrated InterPro entry accession, distinct from a member signature.

Type: string.

### Reply_License

License reported by the bucket metadata record.

Type: string.

### Reply_LocusGroup

HGNC locus group, separate from a missing source value.

Type: string.

### Reply_LocusType

HGNC locus type; Unknown is a source-stated value, distinct from absence.

Type: string.

### Reply_Mane

A source-stated MANE pair with versioned sequence accessions.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl` | string | Versioned Ensembl accession. |
| `refseq` | string | Versioned RefSeq accession. |
| `status` | [Reply_ManeStatus](#reply-manestatus) | Source-stated transcript status. |

### Reply_ManeStatus

The source-stated MANE transcript status.

Type: string.

### Reply_Markdown

Nonempty source Markdown retains LF, CR and tab formatting; unsafe controls use visible escapes. The parser additionally rejects whitespace-only values.

Type: string (markdown).

### Reply_Match

A source-stated match, preserving the text that matched.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Reply_MatchKind](#reply-matchkind) | The match category. |
| `text` | [Reply_Text](#reply-text) | The source text that matched the input. |

### Reply_MatchKind

How a source matched a gene input.

Type: string.

### Reply_Method

Source-stated method or guideline.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Reply_Text](#reply-text) | Source method label. |
| `version` | [Reply_Text](#reply-text) or null | Source method version. Optional. |

### Reply_NcbiGeneId

A positive NCBI Gene identifier in canonical decimal text.

Type: string.

### Reply_NciBiomarkerUse

NCI's native biomarker eligibility criterion, without clinical interpretation.

Values: `inclusion`, `exclusion`.

### Reply_NciDiseaseRef

Compact source disease reference; the diseases section retains every coding field.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Reply_Text](#reply-text) or null | Native source name. Optional. |
| `nci_thesaurus_concept_id` | [Reply_NcitId](#reply-ncitid) | Source NCIt identity. |

### Reply_NciMatchedSite

Compact source site observation; its presence does not establish enrollment.

| Field | Type | Meaning |
| --- | --- | --- |
| `org_city` | [Reply_Text](#reply-text) or null | Native city, when provided. Optional. |
| `org_name` | [Reply_Text](#reply-text) | Published organization name. |
| `org_state_or_province` | [Reply_Text](#reply-text) or null | Native state or province, when provided. Optional. |
| `recruitment_status` | [Reply_Text](#reply-text) or null | Native recruitment status, without translation or enrollment inference. Optional. |

### Reply_NciTextPage

Exact source text fragment with an honest original character count.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](#reply-trialget) or null | Checked request for the next original characters, absent when complete. Optional. |
| `offset` | integer (uint32) | Original Unicode scalar position. |
| `text` | string | Original source fragment, including whitespace. |
| `total_characters` | integer (uint32) | Complete source length in Unicode scalar values. |

### Reply_NcitId

A native NCI Thesaurus concept code.

Type: string.

### Reply_NciTrial

NCI trial card without registry status or phase coercion.

| Field | Type | Meaning |
| --- | --- | --- |
| `brief_summary` | [Reply_NciTextPage](#reply-ncitextpage) or null | Native short summary, only on exact retrieval. Optional. |
| `brief_title` | [Reply_Text](#reply-text) or null | Native title. Optional. |
| `current_trial_status` | [Reply_Text](#reply-text) or null | Native status, without a registry translation. Optional. |
| `diseases` | [Reply_TrialRecords_NciDiseaseRef](#reply-trialrecords-ncidiseaseref) | Publisher-provided disease concepts in source order. |
| `matched_biomarkers` | list of [Reply_NciTrialBiomarker](#reply-ncitrialbiomarker) | Entries with the exact searched biomarker code, preserving source order and native fields. Optional. |
| `matched_site` | [Reply_NciMatchedSite](#reply-ncimatchedsite) or null | First site with an exact case-insensitive organization name, in source order. Optional. |
| `matching_sites` | integer or null | Number of sites with that name, without deduplication; present only for facility searches. Optional. |
| `nci_id` | [Reply_Text](#reply-text) | Publisher NCI identifier. |
| `nct_id` | [Reply_NctId](#reply-nctid) | Registry cross-reference supplied by NCI. |
| `phase` | [Reply_NciTrialPhase](#reply-ncitrialphase) or null | Native phase. Optional. |

### Reply_NciTrialBiomarker

One listed NCI biomarker entry; native values do not establish patient eligibility.

| Field | Type | Meaning |
| --- | --- | --- |
| `eligibility_criterion` | string or null | Native criterion, without rewriting or a positivity inference. Optional. |
| `inclusion_indicator` | string or null | Native inclusion indicator, without interpretation. Optional. |
| `nci_thesaurus_concept_id` | [Reply_NcitId](#reply-ncitid) | Exact source NCIt concept identity. |

### Reply_NciTrialPhase

Native publisher vocabulary; new values are retained unchanged. Known values: NA; 0; I; I_II; II; II_III; III; IV;

Type: string.

### Reply_NctId

Canonical ClinicalTrials.gov identifier.

Type: string.

### Reply_OmimId

A six-digit OMIM identifier, preserving all six digits.

Type: string.

### Reply_OncokbCitation

One native non-PubMed source citation.

| Field | Type | Meaning |
| --- | --- | --- |
| `link` | [Reply_Text](#reply-text) or null | Native citation link. Optional. |
| `text` | string or null | Complete native citation content. Optional. |

### Reply_OncokbConcept

Independent native concept values, retained when a display label is malformed.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Curie](#reply-curie) or null | Checked native external identifier, without a label conversion. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Complete native display label. Optional. |

### Reply_OncokbLevelCode

Source-defined OncoKB vocabulary.

Values: `level_1`, `level_2`, `level_3a`, `level_3b`, `level_4`, `level_r1`, `level_r2`, `level_dx1`, `level_dx2`, `level_dx3`, `level_px1`, `level_px2`, `level_px3`.

### Reply_OncokbOncogenicityCode

Source-defined OncoKB vocabulary.

Values: `oncogenic`, `likely_oncogenic`, `likely_neutral`, `inconclusive`, `resistance`, `unknown`.

### Reply_OncokbVariantRecord

A native annotation row; no level or classification is inferred from other fields.

One of these forms:

- `kind` is `"statement"`: A complete source statement with its original label. Fields: `statement` ([Reply_Statement](#reply-statement)).
- `kind` is `"citation"`: Native non-PubMed citation, preserving source text and link. Fields: `field` (string), `link` ([Reply_Text](#reply-text) or null), `text` (string or null).
- `kind` is `"text"`: Native narrative, split only at checked character boundaries. Fields: `field` (string), `offset` (integer (uint32)), `text` (string), `total` (integer (uint32)).
- `kind` is `"unmapped-label"`: A source label without a representable source protein subject. Fields: `citations` (list of [Reply_Pmid](#reply-pmid)), `classification` ([Reply_Term](#reply-term)), `field` (string), `statement_unmapped` (boolean).
- `kind` is `"implication"`: Native therapeutic, diagnostic or prognostic implication. Fields: `abstracts` (list of [Reply_OncokbCitation](#reply-oncokbcitation)), `alterations` (list of [Reply_Text](#reply-text)), `approved_indications` (list of [Reply_Text](#reply-text)), `citations` (list of [Reply_Pmid](#reply-pmid)), `description` (string or null), `drugs` (list of [Reply_OncokbConcept](#reply-oncokbconcept)), `excluded_tumor_types` (list of [Reply_OncokbConcept](#reply-oncokbconcept)), `fda_level` ([Reply_Term](#reply-term) or null), `field` (string), `level` ([Reply_Term](#reply-term) or null), `ordinal` (integer (uint32)), `statement` ([Reply_Statement](#reply-statement) or null), `statement_unmapped` (boolean), `tumor_type` ([Reply_OncokbConcept](#reply-oncokbconcept) or null).

### Reply_OpenAccessStatus

Open-access status supplied by the paper-link provider.

Type: string.

### Reply_PageInfo

Counts for one search page, after BioMCP removes duplicates and applies filters.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | How many results come before this page. |
| `pool` | [Reply_PoolInfo](#reply-poolinfo) or null | The stored search this page comes from, when BioMCP kept one. Optional. |
| `returned` | integer (uint32) | How many results this page holds. |
| `total` | integer (uint64) | How many matching records the search holds in all. |
| `truncated` | boolean or null | True when the source had more matches than BioMCP fetched. Optional. |

### Reply_PartialDate

A Gregorian date preserving year, month or day precision. The parser additionally validates the calendar day, including leap years.

Type: string.

### Reply_PathwayEventRef

An event object or unresolved integer reference; never the parent's inferred ID.

One of these forms:

- `kind` is `"database"`: Native unresolved integer database reference. Fields: `db_id` ([Reply_Identifier_NonZeroU64](#reply-identifier-nonzerou64)).
- `kind` is `"object"`: Native event object; fields are absent only when omitted by the source. Fields: `db_id` ([Reply_Identifier_NonZeroU64](#reply-identifier-nonzerou64)), `id` ([Reply_ReactomeId](#reply-reactomeid) or null), `name` ([Reply_Text](#reply-text) or null), `schema_class` ([Reply_Text](#reply-text) or null), `source_url` ([Reply_Text](#reply-text) or null), `version` ([Reply_Text](#reply-text) or null).

### Reply_PathwayEvents

Independent source-ordered page of whole native references.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [Reply_PathwayHeader](#reply-pathwayheader) | Exact source pathway owning this relation. |
| `items` | list of [Reply_PathwayEventRef](#reply-pathwayeventref) | Whole native references; unresolved identities remain visible. Optional. |
| `page` | [Reply_PathwayPageInfo](#reply-pathwaypageinfo) | Relation-specific count provenance and continuation. |

### Reply_PathwayGenes

Independent source-ordered distinct membership references and their observed counts.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [Reply_PathwayHeader](#reply-pathwayheader) | Exact source pathway owning this relation. |
| `items` | list of [Reply_PathwayMember](#reply-pathwaymember) | Distinct namespaced source references in first occurrence order. Optional. |
| `page` | [Reply_PathwayPageInfo](#reply-pathwaypageinfo) | Distinct-reference total and continuation. |
| `physical_entities` | integer (uint64) | Every Reactome `PhysicalEntity` of the pathway, as Reactome lists them. They are not gene counts. |
| `reference_occurrences` | integer (uint64) | All reference occurrences across physical entities before distinct-reference grouping. |

### Reply_PathwayHeader

Small selectable native pathway identity, without its potentially long description.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_PathwayId](#reply-pathwayid) | Checked source-owned identity. |
| `name` | [Reply_Text](#reply-text) or null | Whole native name. Optional. |
| `revision` | [Reply_Text](#reply-text) or null | WikiPathways revision, separate from identity. Optional. |
| `species` | [Reply_PathwaySpecies](#reply-pathwayspecies) | Explicit native human species. |
| `version` | [Reply_Text](#reply-text) or null | Whole native Reactome stIdVersion, separate from stable identity. Optional. |

### Reply_PathwayId

Source-owned pathway identity; equal names never merge sources.

One of these forms:

- `namespace` is `"reactome"`: Human Reactome stable identity without a version suffix. Fields: `id` ([Reply_ReactomeId](#reply-reactomeid)).
- `namespace` is `"wikipathways"`: WikiPathways identity without a revision suffix. Fields: `id` ([Reply_WikiPathwaysId](#reply-wikipathwaysid)).

### Reply_PathwayMember

One source membership reference, without name-based gene inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [Reply_HgncId](#reply-hgncid) or null | Exact resolved gene identifier, absent when no checked join exists. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Native label, never interpreted as a gene symbol. Optional. |
| `node_id` | [Reply_Text](#reply-text) or null | Native data-node identity; repeated references remain separate nodes. Optional. |
| `participants` | list of [Reply_PathwayParticipant](#reply-pathwayparticipant) or null | Complete physical contexts, disclosed only by the explicit details view. Optional. |
| `physical_context_count` | integer (uint64) | Number of source physical entity contexts carrying this exact reference. |
| `reference` | [Reply_PathwaySourceRef](#reply-pathwaysourceref) | Namespaced native reference. |
| `schema_class` | [Reply_Text](#reply-text) or null | Native reference schema class. Optional. |
| `source_url` | [Reply_Text](#reply-text) or null | Native reference URL, including therapeutic references without a drug action. Optional. |

### Reply_PathwayPageInfo

Native page position and explicit count provenance; a short admitted page can continue.

| Field | Type | Meaning |
| --- | --- | --- |
| `next_offset` | integer or null | Next native position, independent of returned rows; absent only at the end. Optional. |
| `offset` | integer (uint32) | Native source or retained relation offset. |
| `pool` | [Reply_PoolInfo](#reply-poolinfo) or null | Snapshot identity and lifetime when retained. Optional. |
| `returned` | integer (uint32) | Whole admitted rows on this page. |
| `total` | integer (uint64) | Source-reported or observed relation total, as qualified by total_kind. |
| `total_kind` | [Reply_PathwayTotalKind](#reply-pathwaytotalkind) | Whether the total comes from the source or this retained collection. |

### Reply_PathwayParticipant

One native physical [entity](/concepts/progressive-disclosure/) carrying a membership reference.

| Field | Type | Meaning |
| --- | --- | --- |
| `db_id` | [Reply_Identifier_NonZeroU64](#reply-identifier-nonzerou64) | Native physical entity database identity, distinct from reference identity. |
| `name` | [Reply_Text](#reply-text) or null | Whole physical entity label, including source-stated modification context. Optional. |
| `schema_class` | [Reply_Text](#reply-text) or null | Native physical entity schema class, separate from reference schema class. Optional. |

### Reply_PathwaySourceRef

An open source namespace paired with its complete native identifier.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](#reply-text) | Complete namespace-local identifier, including native isoform suffixes. |
| `namespace` | [Reply_IdentifierNamespace](#reply-identifiernamespace) | Native namespace; never discarded during a gene join. |

### Reply_PathwaySpecies

Source-stated human species; no species inference from a title.

Values: `Homo sapiens`.

### Reply_PathwayTotalKind

Meaning of the collection total, independent of admitted rows on a page.

Values: `source`, `observed`, `distinct-references`, `data-nodes`.

### Reply_PdfText

One small contiguous source-text window, without summary or interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Zero-based mechanical text-fragment offset. |
| `pages` | integer or null | Native PDF pages when readable. Optional. |
| `state` | [Reply_PdfTextState](#reply-pdftextstate) | Extraction outcome; no-text includes scanned PDFs and does not imply OCR. |
| `text` | string or null | Verbatim extracted text in document order; omitted when unavailable. Optional. |
| `total` | integer (uint32) | Number of fragments in the readable document. |
| `url` | [Reply_Text](#reply-text) | Original FDA document URL. |

### Reply_PdfTextState

Bounded PDF text reader outcome.

Values: `text`, `no-text`, `too-large`.

### Reply_PgxAnnotationKind

Independent ClinPGx collection kinds, never merged.

| Value | Meaning |
| --- | --- |
| `clinical` | Clinical annotation min tier. |
| `guideline` | Guideline references. |
| `label` | Label references. |

### Reply_PgxCrossReference

Source-native PgxCrossReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `_url` | string or null | The source's own `_url` value, unchanged. Optional. |
| `id` | [Reply_Identifier_int64](#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `name` | string or null | Native `name` when supplied. Optional. |
| `resource` | string or null | The source's own `resource` value, unchanged. Optional. |
| `resourceId` | string or null | The source's own `resourceId` value, unchanged. Optional. |

### Reply_PgxDiplotypeExpression

Unchanged declared source diplotype expression; no allele sorting, phase or score calculation.

Type: string.

### Reply_PgxDiplotypeReference

Native location diplotype reference; its numeric ID is not a ClinPGx accession.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Identifier_int64](#reply-identifier-int64) | Native `id`; no expression, phase or phenotype is inferred. |

### Reply_PgxEvidenceTerm

Source-native PgxEvidenceTerm fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Identifier_int64](#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `resource` | string or null | The source's own `resource` value, unchanged. Optional. |
| `synonyms` | list of [Reply_PgxUnobserved](#reply-pgxunobserved) or null | Native `synonyms` when supplied. Optional. |
| `term` | string or null | The source's own `term` value, unchanged. Optional. |
| `termId` | string or null | The source's own `termId` value, unchanged. Optional. |
| `valid` | boolean or null | Native `valid` when supplied. Optional. |

### Reply_PgxGet

PGx card or section. No phenotype, phase or dose is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `diplotype` | [Reply_PgxDiplotypeExpression](#reply-pgxdiplotypeexpression) or null | Unchanged declared expression required for the diplotype section only. Optional. |
| `drug_id` | [Reply_Curie](#reply-curie) or null | Exact published CPIC drug key; recommendations only. Optional. |
| `gene` | [Reply_GeneSymbol](#reply-genesymbol) | Explicit source gene symbol. |
| `kind` | [Reply_PgxAnnotationKind](#reply-pgxannotationkind) or null | One ClinPGx collection; clinical is the default only on annotations. Optional. |
| `section` | [Reply_PgxSection](#reply-pgxsection) or null | One named section; omitted means the CPIC gene card. Optional. |
| `section_limit` | integer (uint32) | Rows per page: three clinical, five elsewhere. Optional. |
| `section_offset` | integer (uint32) | Offset within the selected section. Optional. |
| `snapshot` | [Reply_Text](#reply-text) or null | Retained collection for local continuation. Optional. |
| `source` | [Reply_PgxSource](#reply-pgxsource) | Required source, checked against the section. |

### Reply_PgxHeader

Source and declared gene context; no chosen pair or patient result.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [Reply_GeneSymbol](#reply-genesymbol) | Declared source symbol. |
| `source` | [Reply_PgxSource](#reply-pgxsource) | Explicit source. |

### Reply_PgxLocation

Source-native PgxLocation fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `buildVersion` | string or null | Native `buildVersion` when supplied. Optional. |
| `chromosomeId` | string or null | Native `chromosomeId` when supplied. Optional. |
| `chromosomeName` | string or null | Native `chromosomeName` when supplied. Optional. |
| `copyNumber` | string or null | The source's own `copyNumber` value, unchanged. Optional. |
| `diplotypes` | list of [Reply_PgxDiplotypeReference](#reply-pgxdiplotypereference) | The source's own `diplotypes` value, unchanged. Optional. |
| `displayName` | string or null | The source's own `displayName` value, unchanged. Optional. |
| `genes` | list of [Reply_PgxObjectReference](#reply-pgxobjectreference) | The source's own `genes` value, unchanged. Optional. |
| `gpPosition` | integer or null | The source's own `gpPosition` value, unchanged. Optional. |
| `haplotypes` | list of [Reply_PgxObjectReference](#reply-pgxobjectreference) | The source's own `haplotypes` value, unchanged. Optional. |
| `id` | [Reply_Identifier_int64](#reply-identifier-int64) | The source's own `id` value, unchanged. |
| `linkedObjects` | list of [Reply_PgxObjectReference](#reply-pgxobjectreference) | The source's own `linkedObjects` value, unchanged. Optional. |
| `refSeqCrossReference` | [Reply_PgxCrossReference](#reply-pgxcrossreference) or null | Native `refSeqCrossReference` when supplied. Optional. |
| `refSeqPosition` | integer or null | The source's own `refSeqPosition` value, unchanged. Optional. |
| `rsid` | string or null | Native `rsid` when supplied. Optional. |
| `tagGene` | boolean or null | The source's own `tagGene` value, unchanged. Optional. |
| `type` | string or null | The source's own `type` value, unchanged. Optional. |
| `variant` | [Reply_PgxObjectReference](#reply-pgxobjectreference) or null | Native `variant` when supplied. Optional. |

### Reply_PgxObjectReference

Source-native PgxObjectReference fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_ClinPgxAccession](#reply-clinpgxaccession) | The source's own `id` value, unchanged. |
| `name` | string or null | Native `name` when supplied. Optional. |
| `objCls` | string or null | Native `objCls` when supplied. Optional. |
| `symbol` | string or null | Native `symbol` when supplied. Optional. |

### Reply_PgxPageInfo

Independent collection scope; observed counts never imply native source totals.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Offset in this collection. |
| `returned` | integer (uint32) | Number of whole returned rows. |
| `total` | integer (uint64) | Exact source total or received bounded collection length. |
| `total_kind` | [Reply_PgxTotalKind](#reply-pgxtotalkind) | Provenance of total. |

### Reply_PgxSection

One independent named section.

| Value | Meaning |
| --- | --- |
| `recommendations` | Complete native recommendation contexts. |
| `guidelines` | Native guideline references. |
| `diplotype` | Declared source diplotype lookup. |
| `annotations` | One ClinPGx annotation kind. |

### Reply_PgxSource

Explicit independent source.

| Value | Meaning |
| --- | --- |
| `cpic` | CPIC source-native records. |
| `clinpgx` | ClinPGx annotation records. |

### Reply_PgxTotalKind

Meaning of the stated collection total.

| Value | Meaning |
| --- | --- |
| `source` | The source returned an exact total. |
| `observed` | Length of a bounded received collection. |

### Reply_PgxUnobserved

No element shape was captured for these empty arrays. Populated values require new source proof.

Type: any.

### Reply_PhenotypeGet

Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_HpId](#reply-hpid) | Exact HP identifier. |
| `section` | [Reply_PhenotypeSection](#reply-phenotypesection) | Absent means the source identity card. Optional. |
| `section_limit` | integer (uint32) | Default five text/link rows or one complete association row. Optional. |
| `section_offset` | integer (uint32) | Native section row offset. Optional. |
| `snapshot` | [Reply_Text](#reply-text) | Retained exact source reply for local text and link pages; unsupported for diseases and genes. Optional. |
| `source` | [Reply_PhenotypeSource](#reply-phenotypesource) | HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. Optional. |

### Reply_PhenotypeSection

A named source term section.

| Value | Meaning |
| --- | --- |
| `synonyms` | Native synonyms. |
| `xrefs` | Native cross references. |
| `hierarchy` | Native direct parent and child links. |
| `diseases` | Native disease association rows. |
| `genes` | Native human gene association rows. |

### Reply_PhenotypeSource

One independent term publisher.

| Value | Meaning |
| --- | --- |
| `hpo` | Direct HPO term metadata. |
| `monarch` | Monarch term search and associations. |

### Reply_PmaNumber

Native FDA PMA base application number.

Type: string.

### Reply_PmaSupplement

Explicit native PMA supplement selector; no invented base alias.

Type: string.

### Reply_PmaSupplementCell

Native PMA supplement cell; empty denotes base and never becomes S000.

Type: string.

### Reply_Pmcid

Canonical PMC identifier, with an explicit namespace.

Type: string.

### Reply_Pmid

Positive PubMed identifier.

Type: string.

### Reply_PoolInfo

A search BioMCP stored so later pages read the same results: its ID, size, and when it was made and expires.

| Field | Type | Meaning |
| --- | --- | --- |
| `created` | integer (uint64) | When the stored search was made, in Unix seconds. |
| `expires` | integer (uint64) | When the stored search expires, in Unix seconds. |
| `id` | [Reply_Text](#reply-text) | The stored search's ID. Pass it back to BioMCP as it is. |
| `size` | integer (uint32) | How many records the stored search holds. |

### Reply_PopulationDataType

Dataset sequencing sample set.

Values: `exomes`, `genomes`, `joint`.

### Reply_PopulationFrequency

One checked stratum of one source dataset release.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_count` | integer or null | Optional. |
| `allele_frequency` | [Reply_Fraction](#reply-fraction) or null | Optional. |
| `allele_number` | integer or null | Optional. |
| `ancestry_group` | [Reply_Text](#reply-text) or null | Optional. |
| `attribution` | [Reply_Attribution](#reply-attribution) |  |
| `build` | [Reply_Build](#reply-build) |  |
| `data_type` | [Reply_PopulationDataType](#reply-populationdatatype) or null | Optional. |
| `dataset` | [Reply_Text](#reply-text) |  |
| `dataset_version` | [Reply_Text](#reply-text) |  |
| `faf95` | [Reply_Fraction](#reply-fraction) or null | Optional. |
| `filters` | list of [Reply_Text](#reply-text) | Optional. |
| `genotype_filter` | [Reply_GenotypeFilter](#reply-genotypefilter) or null | Optional. |
| `grpmax_group` | [Reply_Text](#reply-text) or null | Optional. |
| `hemizygote_count` | integer or null | Optional. |
| `heteroplasmic_count` | integer or null | Optional. |
| `heteroplasmic_frequency` | [Reply_Fraction](#reply-fraction) or null | Optional. |
| `heterozygote_count` | integer or null | Optional. |
| `homoplasmic_count` | integer or null | Optional. |
| `homoplasmic_frequency` | [Reply_Fraction](#reply-fraction) or null | Optional. |
| `homozygote_count` | integer or null | Optional. |
| `sex` | [Reply_PopulationSex](#reply-populationsex) or null | Optional. |
| `subject` | [Reply_VariantRef](#reply-variantref) |  |
| `subset` | [Reply_Text](#reply-text) or null | Optional. |

### Reply_PopulationSex

Sex stratum stated by a population source.

Values: `xx`, `xy`.

### Reply_Position1

A positive one-based position, without an assembly or biological meaning.

Type: integer (uint32).

### Reply_Predicate

VA-Spec 1.1 proposition relationships in native snake_case.

Type: string.

### Reply_PredictorContext

Unchanged native context values associated with a source field path.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [Reply_Text](#reply-text) | Native source field path. |
| `values` | list of [Reply_Text](#reply-text) | Every native value in source array order. Optional. |

### Reply_PredictorScore

One unmodified source score, never a locally blended assessment.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Source-owned record and dataset release. |
| `gene` | [Reply_GeneRef](#reply-generef) or null | Source gene qualifier. Optional. |
| `label` | [Reply_Text](#reply-text) or null | Source label without code expansion or interpretation. Optional. |
| `method` | [Reply_Text](#reply-text) | Published component field path. |
| `score` | [Reply_FiniteNumber](#reply-finitenumber) | Source number, including zero. |
| `subject` | [Reply_VariantRef](#reply-variantref) | Exact source-qualified subject. |
| `transcript` | [Reply_TranscriptId](#reply-transcriptid) or null | Source transcript qualifier. Optional. |

### Reply_Problem

A field that could not be accepted from a source record.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [Reply_ProblemCode](#reply-problemcode) | Why the field value was not accepted. |
| `field` | string | The name of the rejected field. |
| `record` | [Reply_Text](#reply-text) or null | The record's identifier, when known. Optional. |
| `source` | [Reply_Source](#reply-source) | The source that supplied the record. |

### Reply_ProblemCode

Why a field value was rejected.

Type: string.

### Reply_ProfileExpression

Exact source text together with its parsed Boolean tree.

| Field | Type | Meaning |
| --- | --- | --- |
| `source` | [Reply_Text](#reply-text) |  |
| `tree` | [Reply_ProfileNode](#reply-profilenode) |  |

### Reply_ProfileNode

Source-stated Boolean relations. No node implies same-allele phase.

One of these forms:

- `operator` is `"member"`: One source-native variant reference. Fields: `record` ([Reply_Curie](#reply-curie)).
- `operator` is `"and"`: Every operand is required by the source expression. Fields: `operands` (list of [Reply_ProfileNode](#reply-profilenode)).
- `operator` is `"or"`: Alternatives in the source expression. Fields: `operands` (list of [Reply_ProfileNode](#reply-profilenode)).
- `operator` is `"not"`: Source negation of one member. Fields: `operand` ([Reply_ProfileNode](#reply-profilenode)).

### Reply_Proposition

Domain of a source claim.

Type: string.

### Reply_Protein_UniProtIsoform

A reviewed human protein listed by the authoritative gene record.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtIsoform](#reply-uniprotisoform) | Checked primary accession. |
| `function` | list of [Reply_Text](#reply-text) | Whole source function paragraphs, omitted when absent. Optional. |
| `length` | integer or null | Sequence length in amino acids. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Recommended protein name. Optional. |
| `subcellular_locations` | list of [Reply_Text](#reply-text) | Source subcellular location names. Optional. |

### Reply_ProteinAudit

Native source audit metadata, independent from local retrieval metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `entry_version` | integer or null | Native entry version, never an isoform version. Optional. |
| `first_public` | [Reply_PartialDate](#reply-partialdate) or null | Native first-public date. Optional. |
| `last_annotation_update` | [Reply_PartialDate](#reply-partialdate) or null | Native annotation update date. Optional. |
| `last_sequence_update` | [Reply_PartialDate](#reply-partialdate) or null | Native sequence update date. Optional. |
| `release` | [Reply_Text](#reply-text) or null | Native release when supplied. Optional. |
| `sequence_version` | integer or null | Native sequence version when supplied. Optional. |

### Reply_ProteinComplexes

Whole native rows with the independent unfiltered source count.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinComplexHeader](#reply-proteincomplexheader) | Bounded retained rows in source order. Optional. |
| `native_total` | integer (uint64) | Native source total before type selection or exact participant admission. |

### Reply_ProteinComplexHeader

Exact source complex header admitted by a native participant identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_ComplexPortalId](#reply-complexportalid) | Native complex accession. |
| `matched_participant` | [Reply_Text](#reply-text) | Exact native participant identifier supporting admission. |
| `name` | [Reply_Text](#reply-text) or null | Native source name. Optional. |
| `participant_total` | integer (uint64) | Actual participants in this exact complex, never a complex search total. |
| `predicted` | boolean or null | Literal native predictedComplex flag. Optional. |
| `taxon` | integer (uint32) | Source taxonomy identifier. |
| `url` | [Reply_Text](#reply-text) | Native complex record URL. |

### Reply_ProteinCoordinateConvention

Coordinate convention on the native feature before conversion.

Values: `one_based_inclusive`.

### Reply_ProteinDomainFragment

One native match fragment after coordinate conversion.

| Field | Type | Meaning |
| --- | --- | --- |
| `dc_status` | [Reply_Text](#reply-text) or null | Literal discontinuity status supplied by InterPro. Optional. |
| `span` | [Reply_UncertainSpan0](#reply-uncertainspan0) | Converted interbase span with uncertainty retained. |

### Reply_ProteinDomainLocation

One native InterPro location group retaining all discontinuous fragments.

| Field | Type | Meaning |
| --- | --- | --- |
| `fragments` | list of [Reply_ProteinDomainFragment](#reply-proteindomainfragment) | All native fragments; never reduced to a min/max hull. Optional. |

### Reply_ProteinDomainMatch

One native integrated InterPro match; not every entry type is a domain.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Source record and release facts. |
| `entry_type` | [Reply_Text](#reply-text) or null | Literal source entry type, including family, site and repeat. Optional. |
| `id` | [Reply_InterProId](#reply-interproid) | Integrated entry accession, distinct from member signatures. |
| `locations` | list of [Reply_ProteinDomainLocation](#reply-proteindomainlocation) | Every native location group in source order. Optional. |
| `member_signatures` | list of [Reply_ProteinMemberSignature](#reply-proteinmembersignature) | Source member signatures remain distinct from integrated entries. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Native integrated name. Optional. |
| `protein` | [Reply_UniProtIsoform](#reply-uniprotisoform) | Exact native protein identity. |
| `protein_length` | integer or null | Source protein length when supplied. Optional. |
| `taxon` | integer or null | Source taxon when supplied. Optional. |

### Reply_ProteinDomains

Whole native rows with the independent unfiltered source count.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinDomainMatch](#reply-proteindomainmatch) | Bounded retained rows in source order. Optional. |
| `native_total` | integer (uint64) | Native source total before type selection or exact participant admission. |

### Reply_ProteinFeature

UniProt feature with source attribution and coordinate provenance.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Source record and release facts. |
| `feature` | [Reply_UniProtFeature](#reply-uniprotfeature) | Shared feature, including native label and residues. |
| `source_coordinates` | [Reply_ProteinCoordinateConvention](#reply-proteincoordinateconvention) | Convention before mathematical conversion. |

### Reply_ProteinFunction

One whole source function paragraph and its own evidence.

| Field | Type | Meaning |
| --- | --- | --- |
| `evidence` | list of [Reply_Text](#reply-text) | All evidence references supplied for this text. Optional. |
| `molecule` | [Reply_Text](#reply-text) or null | Native molecule qualifier; never attributed to another isoform. Optional. |
| `text` | [Reply_Text](#reply-text) | Whole native source text, without clinical interpretation. |

### Reply_ProteinFunctionResult

Whole source function paragraphs and their own evidence.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ProteinFunction](#reply-proteinfunction) | Native paragraphs in source order, without inferred interpretation. Optional. |

### Reply_ProteinGet

Read source protein cards; a symbol returns candidates without choosing a protein. Sections disclose native facts with independent row bounds; function keeps all paragraphs. No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [Reply_ProteinInput](#reply-proteininput) | Source accessions or gene symbols, preserving caller order. |
| `section` | list of [Reply_ProteinSection](#reply-proteinsection) | Select native sections or all bounded first pages; function keeps one whole text record. Optional. |
| `section_limit` | integer (uint32) | First-page row limit; function keeps one whole native text record and does not page its paragraphs. Optional. |

### Reply_ProteinHeader

Native entry metadata wrapping the same Protein identity used by the gene slice.

| Field | Type | Meaning |
| --- | --- | --- |
| `entry_id` | [Reply_Text](#reply-text) or null | Native source entry name. Optional. |
| `gene_ids` | list of [Reply_Curie](#reply-curie) | Only identifiers explicitly supplied by the source. Optional. |
| `gene_names` | list of [Reply_Text](#reply-text) | Display names only; never identifier joins. Optional. |
| `protein` | [Reply_Protein_UniProtIsoform](#reply-protein-uniprotisoform) | Shared gene/protein identity fields; never independently restated. |
| `reviewed` | boolean | Native reviewed status. |
| `taxon` | integer (uint32) | Native source taxonomy identifier. |
| `url` | [Reply_Text](#reply-text) | Source entry URL. |

### Reply_ProteinInput

Exact source accession or a checked display gene symbol needing later resolution. The schema describes the string surface; deserialization checks either identity grammar.

Type: string.

### Reply_ProteinIsoform

One source alternative-product row; all stated identifiers stay together.

| Field | Type | Meaning |
| --- | --- | --- |
| `direct_sequence` | [Reply_ProteinSequenceAvailability](#reply-proteinsequenceavailability) | Explicit direct-sequence retrieval state, independent of descriptive status. |
| `ids` | list of [Reply_UniProtIsoform](#reply-uniprotisoform) | All source-stated isoform identifiers, including External base identities. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Source isoform name. Optional. |
| `sequence_ids` | list of [Reply_Text](#reply-text) | Source VSP sequence identifiers. Optional. |
| `sequence_status` | [Reply_ProteinIsoformStatus](#reply-proteinisoformstatus) or null | Native sequence status; no sequence is inferred from it. Optional. |
| `synonyms` | list of [Reply_Text](#reply-text) | Every source synonym. Optional. |

### Reply_ProteinIsoformStatus

Native UniProt alternative-product sequence status.

Values: `Displayed`, `Described`, `External`.

### Reply_ProteinMemberSignature

A member database signature, not an integrated IPR entry.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](#reply-text) | Native signature accession. |
| `name` | [Reply_Text](#reply-text) or null | Native member signature name. Optional. |
| `namespace` | [Reply_Text](#reply-text) | Native member database name. |

### Reply_ProteinQuery

A validated canonical ProteinQuery string.

Type: string.

### Reply_ProteinRelationQuery

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtIsoform](#reply-uniprotisoform) | Exact source protein sequence identity. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Reply_Text](#reply-text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |

### Reply_ProteinSection

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

### Reply_ProteinSectionPage

Bounded card rows and the size of their complete UniProt collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of T | Whole native rows in source order. |
| `total` | integer (uint64) | Number of source rows before card bounding. |

### Reply_ProteinSequence

A directly returned sequence with exact returned identity and audit facts.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtIsoform](#reply-uniprotisoform) | Exact returned accession; never replaced by the requested base entry. |
| `audit` | [Reply_ProteinAudit](#reply-proteinaudit) | Native audit facts for this record. |
| `crc64` | [Reply_Text](#reply-text) or null | Native checksum when supplied. Optional. |
| `length` | integer (uint32) | Native sequence length. |
| `md5` | [Reply_Text](#reply-text) or null | Native checksum when supplied. Optional. |
| `sequence` | [Reply_AminoAcids](#reply-aminoacids) | Source residues, never constructed from another entry. |

### Reply_ProteinSequenceAvailability

Direct sequence availability is a retrieval fact, never constructed from another entry.

Values: `available`, `not_available`, `not_checked`.

### Reply_ProteinSequenceResult

One directly returned source sequence, with exact identity and audit metadata.

| Field | Type | Meaning |
| --- | --- | --- |
| `sequence` | [Reply_ProteinSequence](#reply-proteinsequence) | Native sequence record; never constructed from another entry. |

### Reply_ProteinSite

A residue or residue interval, without a variant assertion.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | [Reply_Position1](#reply-position1) or null | Last one-based residue, if a range. Optional. |
| `gene` | [Reply_SymbolGeneRef](#reply-symbolgeneref) | Source gene identity. |
| `position` | [Reply_Position1](#reply-position1) | First one-based residue. |
| `protein` | [Reply_Accession](#reply-accession) or null | Source protein accession. Optional. |
| `residue` | [Reply_Text](#reply-text) or null | Source residue label, if stated. Optional. |
| `transcript` | [Reply_TranscriptReference](#reply-transcriptreference) or null | Source transcript, with version status kept. Optional. |

### Reply_ProteinStructureProperty

A native cross-reference property pair.

| Field | Type | Meaning |
| --- | --- | --- |
| `key` | [Reply_Text](#reply-text) | Native property key. |
| `value` | [Reply_Text](#reply-text) | Native property value, without interpreting residue coverage. |

### Reply_ProteinStructureRef

A source structure cross-reference link; no residue coverage claim.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](#reply-text) | Native cross-reference identifier. |
| `namespace` | [Reply_Text](#reply-text) | Native cross-reference database. |
| `properties` | list of [Reply_ProteinStructureProperty](#reply-proteinstructureproperty) | All native property pairs, including method, resolution and chains. Optional. |
| `url` | [Reply_Text](#reply-text) or null | Publisher link; no structure file is retrieved. Optional. |

### Reply_PublicationType

Clinically meaningful PubMed publication types.

Type: string.

### Reply_ReactomeId

A checked human Reactome stable pathway identifier.

Type: string.

### Reply_Reason

Why a source could not supply data.

Type: string.

### Reply_Region

A numbered exon or intron with its stated total.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Reply_RegionKind](#reply-regionkind) |  |
| `number` | [Reply_Position1](#reply-position1) |  |
| `total` | [Reply_Position1](#reply-position1) |  |

### Reply_RegionKind

An exon or intron in a source transcript annotation.

| Value | Meaning |
| --- | --- |
| `exon` | Exon in transcript order. |
| `intron` | Intron in transcript order. |

### Reply_RegionLevel

Values: `region`.

### Reply_RegionSubjectWire

| Field | Type | Meaning |
| --- | --- | --- |
| `interval` | [Reply_GenomicInterval](#reply-genomicinterval) |  |
| `label` | [Reply_Text](#reply-text) |  |
| `level` | [Reply_RegionLevel](#reply-regionlevel) |  |
| `record` | [Reply_Text](#reply-text) or null | Optional. |

### Reply_ReportId

Opaque published digits with source carried separately, never a prefixed wrapper.

Type: string.

### Reply_Representation

One source's own expression, assembly and optional source-stated interbase allele.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele` | [Reply_Allele](#reply-allele) or null | Source-supplied allele, without local normalization or equivalence inference. Optional. |
| `build` | [Reply_Build](#reply-build) or null | Assembly stated by the source, omitted when absent. Optional. |
| `expression` | [Reply_Expression](#reply-expression) | Published notation kept unchanged. |
| `source` | [Reply_Text](#reply-text) | Source asserting this form. |

### Reply_RsId

A dbSNP site identifier, never an allele join key.

Type: string.

### Reply_ScientificPValue

Exact normalized scientific p value; no floating-point conversion is used.

| Field | Type | Meaning |
| --- | --- | --- |
| `exponent` | integer (int64) | Source base-ten exponent; values below the double range remain exact. |
| `mantissa` | string | Source mantissa spelling, including significant trailing zeros. |

### Reply_Section_DrugRecords_DrugFdaSubmission

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugRecords_DrugFdaSubmission](#reply-drugrecords-drugfdasubmission)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_GnomadCounts

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_GnomadCounts](#reply-gnomadcounts)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_ProteinSectionPage

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` (T), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantAlphaGenome

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantAlphaGenome](#reply-variantalphagenome)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantAssertions

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantAssertions](#reply-variantassertions)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantCivic

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantCivic](#reply-variantcivic)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantConservation

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantConservation](#reply-variantconservation)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantCosmic

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantCosmic](#reply-variantcosmic)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantGwas

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantGwas](#reply-variantgwas)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantHotspots

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantHotspots](#reply-varianthotspots)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantOncokb

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantOncokb](#reply-variantoncokb)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantPopulation

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantPopulation](#reply-variantpopulation)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_VariantPredictors

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_VariantPredictors](#reply-variantpredictors)), `source` ([Reply_Source](#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](#reply-sourcerecord) or null), `source` ([Reply_Source](#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](#reply-reason)), `source` ([Reply_Source](#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_SemanticScholarAuthorId

A checked source-local Semantic Scholar author identifier, never a person identity.

Type: string.

### Reply_SemanticScholarId

Semantic Scholar's stable 40-character hexadecimal paper identifier.

Type: string.

### Reply_Sequence

A literal IUPAC DNA sequence; empty sequences express insertions or deletions.

Type: string.

### Reply_SequenceLocation

An interbase interval on one versioned sequence accession.

| Field | Type | Meaning |
| --- | --- | --- |
| `sequence` | [Reply_Accession](#reply-accession) | Versioned source sequence, without an inferred genome build. |
| `span` | [Reply_Span0](#reply-span0) | Zero-based half-open bounds. |

### Reply_SoCode

Values: `SO:0001819`, `SO:0001624`, `SO:0001574`, `SO:0001575`, `SO:0001630`, `SO:0001627`, `SO:0001623`, `SO:0001587`, `SO:0001583`, `SO:0001822`, `SO:0001821`, `SO:0001589`, `SO:0001578`, `SO:0001619`, `SO:1000002`, `SO:0000159`, `SO:0000667`, `SO:1000035`, `SO:1000036`, `SO:1000032`, `SO:0002073`.

### Reply_Source

A source of BioMCP data, identified by its canonical short name.

Type: string.

### Reply_SourceDateTime

Native timestamp with the existing explicit timezone disclosure.

| Field | Type | Meaning |
| --- | --- | --- |
| `timezone` | [Reply_SourceTimestampTimezone](#reply-sourcetimestamptimezone) | Whether the native timestamp states its timezone. |
| `value` | [Reply_SourceDateTimeValue](#reply-sourcedatetimevalue) | Unchanged checked source text. |

### Reply_SourceDateTimeValue

Checked source timestamp preserving fractional precision and explicit or missing timezone. The parser additionally validates calendar days and zoned UTC leap-second positions.

Type: string.

### Reply_SourceNotice

A source notice or request recovery step.

One of these forms:

- `kind` is `"drug-name-search"`: Source-stated names used in a trial or article search. Fields: `input` ([Reply_Text](#reply-text)), `message` (string), `names` (list of [Reply_Text](#reply-text)).
- `kind` is `"request-guidance"`: Local request limits, source observations and recovery guidance. Fields: `data_date` (string or null), `message` (string), `retry_after_seconds` (integer or null).
- `kind` is `"condition-word-mismatch"`: Registered conditions lack the literal words requested in a condition search. Fields: `message` (string).
- `kind` is `"cache-problem"`: A checked provider answer survived a local cache failure. Fields: `reason` ([Reply_CacheProblem](#reply-cacheproblem)).
- `kind` is `"graph-truncated"`: The source still has graph pages after the bounded fetch window. Fields: `fallback` ([Reply_Text](#reply-text)).
- `kind` is `"litvar-rsid-hint"`: An empty LitVar lookup can be repeated with a caller-supplied rsID. Fields: `message` (string).
- `kind` is `"pubmed-warning"`: PubMed supplied a named ESearch warning without changing its phrases. Fields: `name` (string), `phrases` (list of string).
- `kind` is `"not-found-words"`: PubMed did not index these words. Fields: `words` (list of [Reply_Text](#reply-text)).

### Reply_SourceOutcome

One source's outcome with source notices and request guidance.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer or null | How many records matched at the source. Optional. |
| `notices` | list of [Reply_SourceNotice](#reply-sourcenotice) | Source notices and request recovery guidance. Optional. |
| `reason` | [Reply_Reason](#reply-reason) or null | Why the source failed, or why BioMCP answered with older stored data. Optional. |
| `retrieved_count` | integer or null | Number of retrieved identities. Optional. |
| `source` | [Reply_Source](#reply-source) | The source BioMCP asked. |
| `state` | [Reply_SourceState](#reply-sourcestate) | What happened: `ok`, `stale`, `syncing`, `failed` or `skipped`. |
| `term` | [Reply_Text](#reply-text) or null | The search term BioMCP sent to the source. Optional. |
| `terms` | string | The name of the publisher's terms. |
| `terms_url` | string (uri) | The publisher's page with those terms. |
| `use` | [Reply_Use](#reply-use) | The source's use group: `open`, `attribution`, `noncommercial`, `licensed` or `per-record`. The publisher's terms decide what you may do with the data. |

### Reply_SourceRecord

The source's own entry for an entity.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](#reply-text) | Native source identifier. |
| `namespace` | [Reply_IdentifierNamespace](#reply-identifiernamespace) | Native authority; unknown means the source did not state one. |
| `url` | [Reply_Text](#reply-text) or null | Link to that entry. Optional. |

### Reply_SourceState

State of a source actually asked by the command.

Values: `ok`, `stale`, `syncing`, `failed`, `skipped`.

### Reply_SourceText

Unchanged free source text, including empty values and surrounding whitespace.

Type: string.

### Reply_SourceTimestamp

Checked source timestamp with its stated precision and offset.

| Field | Type | Meaning |
| --- | --- | --- |
| `timezone` | [Reply_SourceTimestampTimezone](#reply-sourcetimestamptimezone) | Whether the native timestamp includes a timezone. |
| `value` | [Reply_SourceTimestampValue](#reply-sourcetimestampvalue) | Checked RFC 3339 source timestamp. |

### Reply_SourceTimestampTimezone

Availability of timezone information in a native timestamp.

Values: `in-value`, `source-gave-none`.

### Reply_SourceTimestampValue

RFC 3339 source text preserving offset and precision. The parser additionally validates the calendar day and UTC month-end position of leap seconds. Years follow PartialDate's 0001–9999 range.

Type: string.

### Reply_SourceUpdated

Source update metadata preserving the date or timestamp supplied by its publisher.

One of these forms:

- [Reply_PartialDate](#reply-partialdate): A source date keeps its year, month or day precision.
- [Reply_SourceTimestamp](#reply-sourcetimestamp): A source timestamp keeps its stated fractional precision and offset.

### Reply_Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | integer (uint32) |  |
| `start` | integer (uint32) |  |

### Reply_SponsorType

Registry funding category facet.

Values: `nih`, `fed`, `industry`, `other`.

### Reply_Statement

One source claim, kept in its own framework and context.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_origin` | [Reply_Term](#reply-term) or null | Source-stated origin. Optional. |
| `attribution` | [Reply_Attribution](#reply-attribution) | One source record and its provenance. |
| `citations` | list of [Reply_Pmid](#reply-pmid) | Source publications. Optional. |
| `classification` | [Reply_Term](#reply-term) | Whole source classification label. |
| `condition` | [Reply_Concept](#reply-concept) or null | Source disease or tumor context. Optional. |
| `criteria` | list of [Reply_Text](#reply-text) | Source criterion labels in source order. Optional. |
| `criteria_not_met` | list of [Reply_Text](#reply-text) | Criteria assessed and not met. Optional. |
| `direction` | [Reply_Direction](#reply-direction) or null | Source-stated assessment direction. Optional. |
| `gene_context` | [Reply_GeneRef](#reply-generef) or null | Source gene context. Optional. |
| `method` | [Reply_Method](#reply-method) or null | Source method. Optional. |
| `mode_of_inheritance` | [Reply_Term](#reply-term) or null | Source inheritance mode. Optional. |
| `predicate` | [Reply_Predicate](#reply-predicate) or null | Source-stated relationship. Optional. |
| `proposition` | [Reply_Proposition](#reply-proposition) | Claim domain. |
| `status` | [Reply_Term](#reply-term) or null | Source record status. Optional. |
| `strength` | [Reply_Term](#reply-term) or null | Source evidence level or review status. Optional. |
| `subject` | [Reply_Subject](#reply-subject) | Source-owned subject reference. |
| `therapies` | list of [Reply_Concept](#reply-concept) | Source therapies in their native order. Optional. |

### Reply_StatisticEstimate

Source estimate and uncertainty without computed quantities.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer or null | Native count; no inferred denominator. Optional. |
| `lower_ci` | [Reply_WithUnit_FiniteNumber_Text](#reply-withunit-finitenumber-text) or null | Native lower confidence bound. Optional. |
| `standard_error` | [Reply_WithUnit_FiniteNumber_Text](#reply-withunit-finitenumber-text) or null | Native standard error. Optional. |
| `upper_ci` | [Reply_WithUnit_FiniteNumber_Text](#reply-withunit-finitenumber-text) or null | Native upper confidence bound. Optional. |
| `value` | [Reply_WithUnit_FiniteNumber_Text](#reply-withunit-finitenumber-text) or null | Native estimate. Optional. |

### Reply_StatisticSelector

One source-native selector without a local interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [Reply_Text](#reply-text) | Source selector code. |
| `label` | [Reply_Text](#reply-text) or null | Publisher label when supplied. Optional. |
| `name` | [Reply_Text](#reply-text) | Source field name. |

### Reply_Status

Overall command outcome.

Values: `complete`, `partial`, `absent`, `failed`.

### Reply_Step

A command you can run next. `args` is the command without the leading `biomcp`; `kind` says what it does.

| Field | Type | Meaning |
| --- | --- | --- |
| `args` | list of string | The command's words without the leading `biomcp`. The same list works as the MCP tool's arguments. Optional. |
| `kind` | [Reply_StepKind](#reply-stepkind) | What the command does, such as `page` or `section`. |

### Reply_StepKind

What a next command does.

Values: `restart`, `section`, `sort`, `page`, `fresh`, `disambiguate`, `pivot`, `sync`, `key`, `configure`.

### Reply_Strand

Source-stated forward or reverse strand.

Type: string.

### Reply_StudyBlobSha

GitHub repository blob revision, not a data-content checksum.

Type: string.

### Reply_StudyFile

Publisher file reference; repository blob bytes never mean data-content bytes.

| Field | Type | Meaning |
| --- | --- | --- |
| `downloadUrl` | [Reply_Text](#reply-text) or null | Native download link, absent for directory entries. Optional. |
| `htmlUrl` | [Reply_Text](#reply-text) | Publisher browser link. |
| `kind` | [Reply_StudyFileKind](#reply-studyfilekind) | Native file or directory kind. |
| `name` | [Reply_Text](#reply-text) | Native repository entry name. |
| `path` | [Reply_Text](#reply-text) | Exact native repository path. |
| `repositoryBlobSize` | integer (uint64) | GitHub size in repository blob bytes; LFS content size is unknown. |
| `sha` | [Reply_StudyBlobSha](#reply-studyblobsha) | Repository blob or tree revision, not a data-content checksum. |

### Reply_StudyFileKind

Native GitHub contents entry kind.

Values: `file`, `dir`.

### Reply_StudyFiles

One native repository directory page; blob sizes do not describe data content.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer or null | Directory acquisition wall time in Unix seconds. Optional. |
| `items` | list of [Reply_StudyFile](#reply-studyfile) | Source-ordered repository file references. Optional. |
| `license_notice` | [Reply_Text](#reply-text) or null | Native LICENSE publisher link, independent of the current file page. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Bounds within the complete received directory. |
| `study_id` | [Reply_StudyId](#reply-studyid) | Exact study directory scope. |

### Reply_StudyGet

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [Reply_Text](#reply-text) or null | Exact cBioPortal HUGO gene symbol or Entrez ID; required for mutations. Optional. |
| `id` | [Reply_StudyId](#reply-studyid) | Exact source study ID; no default or repair. |
| `molecular_profile` | [Reply_StudyProfileId](#reply-studyprofileid) or null | Exact mutation profile ID; omitted selects the study's first mutation profile. Optional. |
| `section` | [Reply_StudySection](#reply-studysection) or null | Omit for the card; all returns bounded first pages only. Optional. |
| `section_limit` | integer (uint32) | Maximum records in each selected section. Optional. |
| `section_offset` | integer (uint32) | Offset within one named section; all requires zero. Optional. |

### Reply_StudyHeader

Compact source study identity; source codes and counts retain their own scope.

| Field | Type | Meaning |
| --- | --- | --- |
| `allSampleCount` | integer or null | Native allSampleCount, never a patient or profiled-gene denominator. Optional. |
| `cancerTypeId` | [Reply_CbioportalCancerTypeId](#reply-cbioportalcancertypeid) or null | cBioPortal namespace only; no ontology inference. Optional. |
| `citation` | string or null | Native citation, absent for unpublished records. Optional. |
| `name` | [Reply_Text](#reply-text) or null | Native study name. Optional. |
| `pmids` | list of [Reply_Pmid](#reply-pmid) or null | Every validated source PMID in native order, including repetitions; absent remains absent. Optional. |
| `referenceGenome` | [Reply_Text](#reply-text) or null | Native study metadata label, never a coordinate assembly. Optional. |
| `studyId` | [Reply_StudyId](#reply-studyid) | Exact case-sensitive source identity. |

### Reply_StudyId

Opaque case-sensitive cBioPortal study identity, without repair.

Type: string.

### Reply_StudyMutation

A source mutation observation, without clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `mutation_type` | [Reply_Text](#reply-text) or null | Native mutation class. Optional. |
| `protein_change` | [Reply_Text](#reply-text) or null | Source protein change, without normalization. Optional. |
| `sample_id` | [Reply_Text](#reply-text) | Native sample ID within this study. |

### Reply_StudyMutations

Source mutation rows and the explicit gene/profile scope used to obtain them.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer or null | Source collection acquisition time. Optional. |
| `entrez_gene_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) | Exact cBioPortal gene identifier. |
| `items` | list of [Reply_StudyMutation](#reply-studymutation) | Source mutation observations in source order. |
| `molecular_profile_id` | [Reply_StudyProfileId](#reply-studyprofileid) | Selected mutation profile, including an automatically chosen default. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Bounds within the received collection. |
| `study_id` | [Reply_StudyId](#reply-studyid) | Exact source study ID. |

### Reply_StudyProfile

Molecular profile reference with exact study membership.

| Field | Type | Meaning |
| --- | --- | --- |
| `datatype` | [Reply_Text](#reply-text) or null | Extensible native data-type label. Optional. |
| `description` | string or null | Source description. Optional. |
| `genericAssayType` | [Reply_Text](#reply-text) or null | Native generic assay type, absent outside those profiles. Optional. |
| `molecularAlterationType` | [Reply_Text](#reply-text) or null | Extensible source alteration-type label. Optional. |
| `molecularProfileId` | [Reply_StudyProfileId](#reply-studyprofileid) | Exact native profile ID; never constructed from a study ID. |
| `name` | string or null | Source profile name. Optional. |
| `patientLevel` | boolean or null | Native patient-level flag, not a unit inferred for other records. Optional. |
| `showProfileInAnalysisTab` | boolean or null | Native profile display setting. Optional. |
| `studyId` | [Reply_StudyId](#reply-studyid) | Native profile study scope. |

### Reply_StudyProfileId

Exact cBioPortal molecular profile identity.

Type: string.

### Reply_StudyProfiles

One independently acquired, bounded native metadata collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer or null | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. Optional. |
| `items` | list of [Reply_StudyProfile](#reply-studyprofile) | Complete ordered metadata records, never member sample IDs. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Bounds and source-stated total for this collection. |
| `study_id` | [Reply_StudyId](#reply-studyid) | Exact study scope of this collection. |

### Reply_StudySample

A source sample identity and type, without patient or clinical attributes.

| Field | Type | Meaning |
| --- | --- | --- |
| `sample_id` | [Reply_Text](#reply-text) | Native sample ID within this study. |
| `sample_type` | [Reply_Text](#reply-text) or null | Native sample type, when supplied. Optional. |

### Reply_StudySampleList

Sample-list metadata without a representation for member sample IDs.

| Field | Type | Meaning |
| --- | --- | --- |
| `category` | [Reply_Text](#reply-text) or null | Extensible native list category. Optional. |
| `description` | string or null | Native list description. Optional. |
| `name` | string or null | Native list name. Optional. |
| `sampleCount` | integer or null | Stated source sampleCount when supplied, never computed from member IDs. Optional. |
| `sampleListId` | [Reply_StudySampleListId](#reply-studysamplelistid) | Exact native list ID. |
| `studyId` | [Reply_StudyId](#reply-studyid) | Native study scope. |

### Reply_StudySampleListId

Exact cBioPortal sample-list identity.

Type: string.

### Reply_StudySampleLists

One independently acquired, bounded native metadata collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer or null | Live collection acquisition wall time in Unix seconds, distinct from catalog acquisition. Optional. |
| `items` | list of [Reply_StudySampleList](#reply-studysamplelist) | Complete ordered metadata records, never member sample IDs. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Bounds and source-stated total for this collection. |
| `study_id` | [Reply_StudyId](#reply-studyid) | Exact study scope of this collection. |

### Reply_StudySamples

Source sample identities and types from one complete received collection.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer or null | Source collection acquisition time. Optional. |
| `items` | list of [Reply_StudySample](#reply-studysample) | Source sample identities and types in native order. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Bounds within the received collection. |
| `study_id` | [Reply_StudyId](#reply-studyid) | Exact native study scope. |

### Reply_StudySection

Supported bounded study metadata sections; gene counts are deferred.

| Value | Meaning |
| --- | --- |
| `profiles` | Five exact native molecular profile references. |
| `sample-lists` | Five sample-list metadata records, without member sample IDs. |
| `mutations` | Source mutation rows for the required gene and selected mutation profile. |
| `samples` | Source sample identities and types only. |
| `files` | Five repository file references; no automatic download. |
| `all` | Bounded first pages; mutations are included only when gene is supplied. |

### Reply_StudySnapshot

Metadata catalog revision and acquisition time; never a biological release date.

| Field | Type | Meaning |
| --- | --- | --- |
| `acquired` | integer (uint64) | Catalog acquisition wall time in Unix seconds. |
| `revision` | [Reply_Text](#reply-text) | Immutable bundle revision. |

### Reply_StudyType

Source-native registry value.

Values: `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS`.

### Reply_Subject

One of these forms:

- `level` is `"categorical"`: no description. Fields: `category` ([Reply_Category](#reply-category)), `edit` ([Reply_Coding](#reply-coding) or null), `gene` ([Reply_GeneRef](#reply-generef)), `label` ([Reply_Text](#reply-text)), `partner` ([Reply_GeneRef](#reply-generef) or null), `region` ([Reply_Region](#reply-region) or null), `site` ([Reply_ProteinSite](#reply-proteinsite) or null).
- `level` is `"allele"`: Exact allele identifier or reference-qualified notation. Fields: `hgvs` ([Reply_HgvsText](#reply-hgvstext) or null), `id` ([Reply_VariantId](#reply-variantid) or null).
- `level` is `"protein"`: Protein-level assertion, without genomic conversion. Fields: `gene` ([Reply_GeneRef](#reply-generef)), `hgvs` ([Reply_HgvsText](#reply-hgvstext) or null), `label` ([Reply_Text](#reply-text) or null).
- `level` is `"haplotype"`: Native ClinVar haplotype and its complete source member identities. Fields: `accession` ([Reply_VcvId](#reply-vcvid)), `id` ([Reply_ClinVarVariationId](#reply-clinvarvariationid)), `kind` ([Reply_ClinvarHaplotypeKind](#reply-clinvarhaplotypekind)), `members` (list of [Reply_ClinvarHaplotypeMember](#reply-clinvarhaplotypemember)), `number_of_chromosomes` ([Reply_Text](#reply-text)), `version` ([Reply_Text](#reply-text)).
- `level` is `"source"`: Native source variant with optional source-stated forms. Fields: `feature` ([Reply_Text](#reply-text)), `feature_record` ([Reply_Curie](#reply-curie) or null), `forms` (list of object or object), `record` ([Reply_Curie](#reply-curie)).
- `level` is `"profile"`: Compound source profile, without inferred allele identity or phase. Fields: `expression` ([Reply_ProfileExpression](#reply-profileexpression) or null), `label` ([Reply_Text](#reply-text)), `members` (list of object or object or object), `record` ([Reply_Text](#reply-text)).
- `level` is `"gene"`: no description. Fields: `ensembl_gene_id` (string or null), `hgnc_id` ([Reply_HgncId](#reply-hgncid) or null), `label` ([Reply_GeneSymbol](#reply-genesymbol)), `ncbi_gene_id` ([Reply_NcbiGeneId](#reply-ncbigeneid) or null).
- `level` is `"region"`: no description. Fields: `interval` ([Reply_GenomicInterval](#reply-genomicinterval)), `label` ([Reply_Text](#reply-text)), `record` ([Reply_Text](#reply-text) or null).

### Reply_SubjectQualification

Provenance of the subject's accession qualification, without allele normalization.

Values: `adapter-derived-reference`.

### Reply_Summary

A whole source-provided summary with attribution and stated date precision.

| Field | Type | Meaning |
| --- | --- | --- |
| `dated` | [Reply_PartialDate](#reply-partialdate) or null | Date stated by the source, if any. Optional. |
| `source` | [Reply_SummarySource](#reply-summarysource) | The summary's attribution, independent of the fetching provider. |
| `text` | [Reply_Text](#reply-text) | Entire summary text. |

### Reply_SummarySource

The attribution on a source-provided summary, independent of fetch providers.

Type: string.

### Reply_SymbolGeneRef

One gene label and the publisher identifiers known for it.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl_gene_id` | string or null | Optional. |
| `hgnc_id` | [Reply_HgncId](#reply-hgncid) or null | Optional. |
| `label` | [Reply_GeneSymbol](#reply-genesymbol) |  |
| `ncbi_gene_id` | [Reply_NcbiGeneId](#reply-ncbigeneid) or null | Optional. |

### Reply_SyncQuery

Explicit source synchronization selector.

| Field | Type | Meaning |
| --- | --- | --- |
| `source` | [Reply_Source](#reply-source) | Source whose complete identity data is synchronized. |

### Reply_Term

A source label with explicitly supported codings in source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `codings` | list of [Reply_Coding](#reply-coding) | Documented framework codings. Optional. |
| `label` | [Reply_Text](#reply-text) | Original label. |

### Reply_Text

Bounded nonempty text with surrounding Unicode whitespace removed.

Type: string.

### Reply_TranscriptId

A transcript reference in versioned Accession or unversioned AccessionBase form.

Type: string.

### Reply_TranscriptReference

A transcript reference in versioned Accession or unversioned AccessionBase form.

Type: string.

### Reply_TrialConditionResolution

Exact condition identity used for one trial request, without eligibility inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `alias` | [Reply_DiseaseRelation](#reply-diseaserelation) or null | Native relation proving a requested alias, when required. Optional. |
| `attribution` | [Reply_Attribution](#reply-attribution) or null | Resolver record provenance, independent of the trial provider. Optional. |
| `components` | list of [Reply_ComponentLabel](#reply-componentlabel) | Admitted resolver components and their own terms. Optional. |
| `concept` | [Reply_DiseaseConcept](#reply-diseaseconcept) or null | Returned concept with its unchanged native primary label. Optional. |
| `input` | [Reply_Curie](#reply-curie) | Caller-selected CURIE, never a source-name search. |
| `ncit_candidates` | list of [Reply_DiseaseRelation](#reply-diseaserelation) | Native NCIt mapping rows; duplicate targets keep their independent provenance. Optional. |
| `resolver` | [Reply_Source](#reply-source) or null | Named exact resolver; absent for explicit NCIt input. Optional. |
| `selected_ncit` | [Reply_Curie](#reply-curie) or null | Explicit or unique source-provided NCIt identifier actually sent to NCI. Optional. |

### Reply_TrialDocument

A posted source document; a URL never comes from caller-supplied filename text.

| Field | Type | Meaning |
| --- | --- | --- |
| `date` | [Reply_PartialDate](#reply-partialdate) or null | Source document date. Optional. |
| `filename` | [Reply_Text](#reply-text) | Exact advertised portable source filename. |
| `has_icf` | boolean or null | Source says this contains an informed consent form. Optional. |
| `has_protocol` | boolean or null | Source says this contains a protocol. Optional. |
| `has_sap` | boolean or null | Source says this contains a statistical analysis plan. Optional. |
| `kind` | string or null | Native document type abbreviation. Optional. |
| `label` | string or null | Source label. Optional. |
| `license` | [Reply_License](#reply-license) | No blanket reuse license is inferred from registry availability. |
| `size` | integer or null | Source byte count, when supplied. Optional. |
| `uploaded` | [Reply_TrialUploadTime](#reply-trialuploadtime) or null | Original upload timestamp, without guessing a timezone. Optional. |
| `url` | [Reply_Text](#reply-text) | Fixed registry CDN URL for this manifest entry. |

### Reply_TrialGet

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [Reply_NctId](#reply-nctid) | One to twenty exact NCT identifiers, in caller order. |
| `limit` | integer (uint32) | Whole-record count per source collection page; text fragments follow the text budget. Optional. |
| `offset` | integer (uint32) | Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. Optional. |
| `pool` | [Reply_Text](#reply-text) | Checked stored source record, supplied by a continuation. Optional. |
| `section` | list of [Reply_TrialSection](#reply-trialsection) | Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported. Optional. |
| `source` | [Reply_TrialSource](#reply-trialsource) | Explicit provider; defaults to ClinicalTrials.gov and never falls back. Optional. |

### Reply_TrialPhase

Source-native registry value.

Values: `EARLY_PHASE1`, `PHASE1`, `PHASE2`, `PHASE3`, `PHASE4`, `NA`.

### Reply_TrialPhaseFilter

Values: `early-1`, `1`, `2`, `3`, `4`, `not-applicable`, `1/2`, `2/3`, `NA`, `0`, `I`, `I_II`, `II`, `II_III`, `III`, `IV`.

### Reply_TrialRecords_NciDiseaseRef

One page of whole records, in the source's order.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_TrialGet](#reply-trialget) or null | The request that reads the next records. Missing when there are no more. Optional. |
| `items` | list of [Reply_NciDiseaseRef](#reply-ncidiseaseref) | Whole records, in the source's order. Optional. |
| `offset` | integer (uint32) | How many of the source's records come before this page. |
| `total` | integer (uint32) | How many records the source's list holds. |

### Reply_TrialSearch

Search ClinicalTrials.gov or keyed NCI with native counts. NCI --facility matches exact organization names; a listed site is not necessarily enrolling.

| Field | Type | Meaning |
| --- | --- | --- |
| `age` | integer (uint32) | Age in years. ClinicalTrials.gov only; unstated source bounds count as no limit. Optional. |
| `biomarker_code` | [Reply_NcitId](#reply-ncitid) | NCI NCIt code. Exact NCIt biomarker concept listed by NCI in trial criteria; NCI source only. This listing does not establish biomarker positivity or patient eligibility. Optional. |
| `biomarker_use` | [Reply_NciBiomarkerUse](#reply-ncibiomarkeruse) | NCI criterion. Filter by NCI's native inclusion or exclusion criterion. Check each result's biomarker entry and reported problems. Optional. |
| `condition` | list of [Reply_Text](#reply-text) | Disease; NCI needs NCIt. Conditions joined with OR. NCI needs NCIt codes such as C3224; use --condition-id for MONDO or DOID identifiers. Optional. |
| `condition_id` | [Reply_Curie](#reply-curie) | MONDO/DOID/NCIT:C. Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. Optional. |
| `count_only` | boolean | Total. Source total only; no returned study rows. Optional. |
| `country` | list of `"Afghanistan"` or `"Aland Islands"` or `"Albania"` or `"Algeria"` or `"American Samoa"` or `"Andorra"` or `"Angola"` or `"Anguilla"` or `"Antarctica"` or `"Antigua and Barbuda"` or `"Argentina"` or `"Armenia"` or `"Aruba"` or `"Australia"` or `"Austria"` or `"Azerbaijan"` or `"Bahrain"` or `"Bangladesh"` or `"Barbados"` or `"Belarus"` or `"Belgium"` or `"Belize"` or `"Benin"` or `"Bermuda"` or `"Bhutan"` or `"Bolivia"` or `"Bonaire, Saint Eustatius and Saba "` or `"Bosnia and Herzegovina"` or `"Botswana"` or `"Brazil"` or `"Brunei"` or `"Bulgaria"` or `"Burkina Faso"` or `"Burma"` or `"Burundi"` or `"Cabo Verde"` or `"Cambodia"` or `"Cameroon"` or `"Canada"` or `"Cayman Islands"` or `"Central African Republic"` or `"Chad"` or `"Chile"` or `"China"` or `"Christmas Island"` or `"Colombia"` or `"Comoros"` or `"Costa Rica"` or `"Croatia"` or `"Cuba"` or `"Curacao"` or `"Cyprus"` or `"Czechia"` or `"Côte d’Ivoire"` or `"Democratic Republic of the Congo"` or `"Denmark"` or `"Djibouti"` or `"Dominica"` or `"Dominican Republic"` or `"Ecuador"` or `"Egypt"` or `"El Salvador"` or `"Equatorial Guinea"` or `"Eritrea"` or `"Estonia"` or `"Eswatini"` or `"Ethiopia"` or `"Faroe Islands"` or `"Federal Republic of Yugoslavia"` or `"Fiji"` or `"Finland"` or `"France"` or `"French Guiana"` or `"French Polynesia"` or `"French Southern and Antarctic Lands"` or `"Gabon"` or `"Georgia"` or `"Germany"` or `"Ghana"` or `"Gibraltar"` or `"Greece"` or `"Greenland"` or `"Grenada"` or `"Guadeloupe"` or `"Guam"` or `"Guatemala"` or `"Guinea"` or `"Guinea-Bissau"` or `"Guyana"` or `"Haiti"` or `"Holy See"` or `"Honduras"` or `"Hong Kong"` or `"Hungary"` or `"Iceland"` or `"India"` or `"Indonesia"` or `"Iran"` or `"Iraq"` or `"Ireland"` or `"Israel"` or `"Italy"` or `"Jamaica"` or `"Japan"` or `"Jersey"` or `"Jordan"` or `"Kazakhstan"` or `"Kenya"` or `"Kiribati"` or `"Kosovo"` or `"Kuwait"` or `"Kyrgyzstan"` or `"Laos"` or `"Latvia"` or `"Lebanon"` or `"Lesotho"` or `"Liberia"` or `"Libya"` or `"Liechtenstein"` or `"Lithuania"` or `"Luxembourg"` or `"Macau"` or `"Madagascar"` or `"Malawi"` or `"Malaysia"` or `"Maldives"` or `"Mali"` or `"Malta"` or `"Martinique"` or `"Mauritania"` or `"Mauritius"` or `"Mayotte"` or `"Mexico"` or `"Micronesia"` or `"Moldova"` or `"Monaco"` or `"Mongolia"` or `"Montenegro"` or `"Montserrat"` or `"Morocco"` or `"Mozambique"` or `"Namibia"` or `"Nepal"` or `"Netherlands"` or `"Netherlands Antilles"` or `"New Caledonia"` or `"New Zealand"` or `"Nicaragua"` or `"Niger"` or `"Nigeria"` or `"Niue"` or `"North Korea"` or `"North Macedonia"` or `"Northern Mariana Islands"` or `"Norway"` or `"Oman"` or `"Pakistan"` or `"Palestinian Territories"` or `"Panama"` or `"Papua New Guinea"` or `"Paraguay"` or `"Peru"` or `"Philippines"` or `"Poland"` or `"Portugal"` or `"Puerto Rico"` or `"Qatar"` or `"Republic of the Congo"` or `"Reunion"` or `"Romania"` or `"Russia"` or `"Rwanda"` or `"Saint Kitts and Nevis"` or `"Saint Lucia"` or `"Saint Martin"` or `"Saint Vincent and the Grenadines"` or `"Samoa"` or `"San Marino"` or `"Saudi Arabia"` or `"Senegal"` or `"Serbia"` or `"Serbia and Montenegro"` or `"Seychelles"` or `"Sierra Leone"` or `"Singapore"` or `"Slovakia"` or `"Slovenia"` or `"Solomon Islands"` or `"Somalia"` or `"South Africa"` or `"South Korea"` or `"South Sudan"` or `"Spain"` or `"Sri Lanka"` or `"Sudan"` or `"Suriname"` or `"Sweden"` or `"Switzerland"` or `"Syria"` or `"Taiwan"` or `"Tajikistan"` or `"Tanzania"` or `"Thailand"` or `"The Bahamas"` or `"The Gambia"` or `"Timor-Leste"` or `"Togo"` or `"Trinidad and Tobago"` or `"Tunisia"` or `"Turkey (Türkiye)"` or `"Turkmenistan"` or `"Uganda"` or `"Ukraine"` or `"United Arab Emirates"` or `"United Kingdom"` or `"United States"` or `"United States Minor Outlying Islands"` or `"Uruguay"` or `"Uzbekistan"` or `"Vanuatu"` or `"Venezuela"` or `"Vietnam"` or `"Virgin Islands"` or `"Yemen"` or `"Zambia"` or `"Zimbabwe"` | Country label. Exact ClinicalTrials.gov country label; repeat for OR. Trials must list a site in a named country. NCI uses its native country filter. Optional. |
| `criteria` | [Reply_Text](#reply-text) | Criteria text. ClinicalTrials.gov only: search exact registry eligibility text for discovery; no eligibility judgment. Optional. |
| `distance` | [Reply_Text](#reply-text) | Radius, mi or km. ClinicalTrials.gov only: source distance with explicit mi or km units. Optional. |
| `exact_condition` | boolean | Exact condition. Ask ClinicalTrials.gov to restrict matching to the registered condition phrase; its support is partial. Optional. |
| `exact_term` | boolean | Search typed treatment terms without resolving their other names. Optional. |
| `facility` | [Reply_Text](#reply-text) | Facility. Facility phrase; ClinicalTrials.gov broadens after zero exact matches. NCI matches exact names. Cannot combine with radius. Optional. |
| `gene` | [Reply_GeneInput](#reply-geneinput) | Gene symbol/ID. ClinicalTrials.gov only: resolve one checked gene for registry text discovery; no eligibility judgment. Optional. |
| `has_results` | boolean | Posted results. ClinicalTrials.gov only: require posted source results. Optional. |
| `id` | [Reply_NctId](#reply-nctid) | NCT identifier. ClinicalTrials.gov only: exact NCT identity in a conjunction with other filters. Optional. |
| `intervention` | list of [Reply_Text](#reply-text) | Treatment terms; OR. Intervention alternatives joined with OR; supported by ClinicalTrials.gov. Optional. |
| `keyword` | list of [Reply_Text](#reply-text) | Text; OR. Repeat for ClinicalTrials.gov general text alternatives joined with OR; different filters combine with AND. NCI accepts one keyword. Optional. |
| `lat` | [Reply_Text](#reply-text) | Latitude degrees. ClinicalTrials.gov only: radius center latitude in degrees. Optional. |
| `limit` | integer (uint32) | Page size. Headers per page; NCI applies its own lower publisher limit. Optional. |
| `lon` | [Reply_Text](#reply-text) | Longitude degrees. ClinicalTrials.gov only: radius center longitude in degrees. Optional. |
| `next_page` | [Reply_Text](#reply-text) | Page token. Opaque registry continuation; keep filters unchanged. Optional. |
| `offset` | integer (uint32) | Page offset. Zero-based offset of the first row in this continuation page. Optional. |
| `phase` | [Reply_TrialPhaseFilter](#reply-trialphasefilter) | Native phase. Exact source phase. ClinicalTrials.gov: 1, 2, 3, 4, early-1, not-applicable, 1/2, 2/3. NCI: NA, 0, I, I_II, II, II_III, III, IV. Optional. |
| `query` | [Reply_Text](#reply-text) | Trial text. General registry discovery text. Optional. |
| `sex` | [Reply_TrialSex](#reply-trialsex) | ClinicalTrials.gov only: registry sex facet. Optional. |
| `source` | [Reply_TrialSource](#reply-trialsource) | clinicaltrials/nci. Explicit provider; defaults to ClinicalTrials.gov and never falls back. Optional. |
| `sponsor` | [Reply_Text](#reply-text) | Sponsor. ClinicalTrials.gov only: sponsor name; broadens after zero exact matches. Use --sponsor-type for the funding category. Optional. |
| `sponsor_type` | [Reply_SponsorType](#reply-sponsortype) | Funding class. ClinicalTrials.gov only: registry sponsor funding category; use --sponsor for a sponsor name. Optional. |
| `status` | [Reply_TrialStatusFilter](#reply-trialstatusfilter) | Native status. Exact source status. NCI: Active, Administratively Complete, Approved, Closed to Accrual, Closed to Accrual and Intervention, Complete, INACTIVE, Enrolling by Invitation, Temporarily Closed to Accrual, Temporarily Closed to Accrual and Intervention, Withdrawn. ClinicalTrials.gov uses registry labels. Optional. |
| `study_type` | [Reply_StudyType](#reply-studytype) | Study type. ClinicalTrials.gov only: exact registry study type. Optional. |
| `updated_since` | [Reply_PartialDate](#reply-partialdate) | First date. ClinicalTrials.gov only: first included registry update date. Optional. |
| `updated_until` | [Reply_PartialDate](#reply-partialdate) | Last date. ClinicalTrials.gov only: last included registry update date. Optional. |
| `variant` | [Reply_Text](#reply-text) | Variant text. ClinicalTrials.gov only: literal variant text for registry discovery; no eligibility judgment. Optional. |

### Reply_TrialSection

Available source protocol sections.

| Value | Meaning |
| --- | --- |
| `eligibility` | Intact criteria and structured age/sex fields. |
| `outcomes` | Primary, secondary and other outcome records. |
| `results` | Posted outcome values, analyses and adverse-event tables (ClinicalTrials.gov). |
| `arms` | Arms and interventions with original relationships. |
| `locations` | Whole source sites with their attached contacts. |
| `contacts` | Central contacts and overall officials in their original roles. |
| `references` | Registry publication identities and native reference types. |
| `documents` | Posted document metadata and source URLs. |
| `all` | Every implemented trial section, each with independent continuations. |
| `diseases` | Native NCI disease concepts, including source enrichment metadata. |
| `summary` | Complete NCI summary, paged by original character offsets. |

### Reply_TrialSex

Source-native registry value.

Values: `ALL`, `FEMALE`, `MALE`.

### Reply_TrialSource

Independent trial providers; results are never folded or used as fallbacks.

Values: `ctgov`, `nci`.

### Reply_TrialStatus

Publisher Status enum for OverallStatus from &lt;https://clinicaltrials.gov/api/v2/studies/enums&gt;.

Values: `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `TEMPORARILY_NOT_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN`.

### Reply_TrialStatusFilter

Values: `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `TEMPORARILY_NOT_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN`, `Active`, `Administratively Complete`, `Approved`, `Closed to Accrual`, `Closed to Accrual and Intervention`, `Complete`, `INACTIVE`, `Enrolling by Invitation`, `Temporarily Closed to Accrual`, `Temporarily Closed to Accrual and Intervention`, `Withdrawn`.

### Reply_TrialUploadTime

Native ClinicalTrials.gov DateTimeMinutes without timezone. The parser additionally validates the calendar day, including leap years.

Type: string.

### Reply_UncertainSpan0

Zero-based interbase span retaining uncertain boundary ranges.

| Field | Type | Meaning |
| --- | --- | --- |
| `end` | [Reply_Bound](#reply-bound) |  |
| `start` | [Reply_Bound](#reply-bound) |  |

### Reply_UniProtAcc

A whole UniProt base accession, without an isoform suffix.

Type: string.

### Reply_UniProtFeature

Shared native feature contract without source label remapping.

| Field | Type | Meaning |
| --- | --- | --- |
| `alternative` | list of [Reply_AminoAcids](#reply-aminoacids) | Every stated alternative, without translation. Optional. |
| `description` | [Reply_Text](#reply-text) or null | Complete source description. Optional. |
| `evidence` | list of [Reply_Text](#reply-text) | All original source evidence references. Optional. |
| `id` | [Reply_Text](#reply-text) or null | Native feature identifier. Optional. |
| `kind` | [Reply_Term](#reply-term) | Source feature label; coding only for documented exact feature types. |
| `original` | [Reply_AminoAcids](#reply-aminoacids) or null | Source original residues, when supplied. Optional. |
| `protein` | [Reply_UniProtIsoform](#reply-uniprotisoform) | Exact source sequence identity. |
| `span` | [Reply_UncertainSpan0](#reply-uncertainspan0) | Zero-based interbase coordinates retaining uncertain ranges. |

### Reply_UniProtIsoform

A stated UniProt accession with an optional positive isoform number; never imputes -1.

Type: string.

### Reply_UnmappedAssertion

Source conclusion and provenance retained without a shared statement.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](#reply-attribution) | Original record provenance. |
| `classification` | [Reply_Text](#reply-text) or null | Unchanged source label, when valid and supplied. Optional. |
| `conditions` | list of [Reply_AssertionCondition](#reply-assertioncondition) | Native conditions, including identifiers without supplied labels. Optional. |
| `review_status` | [Reply_Text](#reply-text) or null | Native review status. Optional. |

### Reply_Use

The source's use group. It describes the source, not the rights in any one record.

Values: `open`, `attribution`, `noncommercial`, `licensed`, `per-record`.

### Reply_VaersSection

One independent CDC WONDER grouping.

| Value | Meaning |
| --- | --- |
| `symptoms` | Native reported symptom rows. |
| `seriousness` | Reports classified by native seriousness. |
| `age` | Native age groups. |

### Reply_Variant

Variant entity with source-stated identities, forms and genes; no disclosure policy.

| Field | Type | Meaning |
| --- | --- | --- |
| `genes` | list of [Reply_GeneRef](#reply-generef) | Genes stated by the sources. Optional. |
| `ids` | list of [Reply_VariantId](#reply-variantid) | All source identifiers, without inferred aliases. Optional. |
| `representations` | list of [Reply_Representation](#reply-representation) | Every source-qualified representation. Optional. |

### Reply_VariantAlphaGenome

Paged native scorer results with explicit request provenance.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](#reply-build) | Source-resolved assembly used for the request. |
| `requested_interval` | [Reply_AlphaGenomeInterval](#reply-alphagenomeinterval) | Exact requested interval, never claimed as a returned interval. |
| `requested_model_version` | string | Checkpoint field sent; empty means the publisher default was requested. |
| `scorers` | list of [Reply_AlphaGenomeScorerPage](#reply-alphagenomescorerpage) | Native scorer groups associated only by requested position. Optional. |
| `variant` | [Reply_AlphaGenomeVariant](#reply-alphagenomevariant) | Exact request identity, checked against native metadata. |

### Reply_VariantAssertion

A native record/domain pair, never a vote or merged classification.

| Field | Type | Meaning |
| --- | --- | --- |
| `detail` | [Reply_AssertionDetail](#reply-assertiondetail) or null | Original source detail, returned only for an explicitly selected record. Optional. |
| `domain` | [Reply_AssertionDomain](#reply-assertiondomain) | Source classification domain. |
| `kind` | [Reply_AssertionKind](#reply-assertionkind) | Aggregate/submission/document context. |
| `statement` | [Reply_Statement](#reply-statement) or null | Supported mapping into the common model. Optional. |
| `statement_unmapped` | boolean | The native conclusion has no common-model statement. Optional. |
| `unmapped` | [Reply_UnmappedAssertion](#reply-unmappedassertion) or null | Native assertion whose category the common model cannot represent. Optional. |

### Reply_VariantAssertions

One source-ordered page, with native record and classification boundaries intact.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertions` | list of [Reply_VariantAssertion](#reply-variantassertion) | Source assertion rows in native order. Optional. |
| `next` | [Reply_VariantAssertionsGet](#reply-variantassertionsget) or null | Typed continuation preserving source and selectors. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Page bounds over native rows. |

### Reply_VariantAssertionsGet

Page assertions from one explicitly selected evidence source.

| Field | Type | Meaning |
| --- | --- | --- |
| `assertion` | [Reply_Text](#reply-text) or null | Native assertion accession or document ID. Optional. |
| `build` | [Reply_Build](#reply-build) or null | Identity lookup assembly; omitted uses the shared GRCh38 default. Optional. |
| `input` | [Reply_VariantLookupInput](#reply-variantlookupinput) | Supported variant ID; resolves one ClinVar Variation ID or ERepo CAid through the shared resolver. |
| `limit` | integer (uint32) | Assertions per page; omitted uses the native source default. Optional. |
| `offset` | integer (uint32) | Assertions to skip in source order. Optional. |
| `source` | [Reply_AssertionSource](#reply-assertionsource) | One native assertion source; shared identity conversion precedes source access when needed. |
| `version` | [Reply_Text](#reply-text) or null | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. Optional. |

### Reply_VariantCivic

CIViC molecular profiles with their original membership and separate source claims.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | [Reply_VariantGet](#reply-variantget) or null | Continue this section without changing the input or source selectors. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Disclosure bounds for this section. |
| `profiles` | list of [Reply_CivicProfile_VariantGet](#reply-civicprofile-variantget) | Source profiles in native order; name candidates carry no attached statements. Optional. |

### Reply_VariantComponent

Metadata for a component carried by an aggregator.

| Field | Type | Meaning |
| --- | --- | --- |
| `aggregator_build` | [Reply_Text](#reply-text) or null | Aggregator build version. Optional. |
| `aggregator_built` | [Reply_SourceDateTimeValue](#reply-sourcedatetimevalue) or null | Aggregator build timestamp, distinct from a component release. Optional. |
| `license` | [Reply_Text](#reply-text) or null | Component license URL, without any inferred grant. Optional. |
| `name` | [Reply_Text](#reply-text) | Component name from aggregator metadata. |
| `note` | [Reply_Text](#reply-text) or null | Whole source snapshot warning. Optional. |
| `terms` | [Reply_ComponentTerms](#reply-componentterms) | Independently reviewed publisher terms; no blanket aggregator grant. |
| `uploaded` | [Reply_SourceDateTimeValue](#reply-sourcedatetimevalue) or null | Component upload timestamp when the source states one. Optional. |
| `uploads` | list of [Reply_ComponentUpload](#reply-componentupload) | Source-named component upload timestamps when the publisher supplies a map. Optional. |
| `version` | [Reply_Text](#reply-text) or null | Component version; omitted when the source reports none. Optional. |

### Reply_VariantConservation

Source conservation measurements; no local pathogenicity inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_VariantComponent](#reply-variantcomponent) | Independent component versions and licenses. Optional. |
| `context` | list of [Reply_PredictorContext](#reply-predictorcontext) | Original context arrays, without inferred cross-field alignment. Optional. |
| `scores` | list of [Reply_PredictorScore](#reply-predictorscore) | Every retained source measurement in declared field order. Optional. |
| `subject_qualification` | [Reply_SubjectQualification](#reply-subjectqualification) | How the source chromosome label was reference-qualified. |

### Reply_VariantCosmic

Historical COSMIC fields carried by MyVariant, without direct COSMIC access.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_VariantComponent](#reply-variantcomponent) | Snapshot version, dates, warning and terms. Optional. |
| `next` | [Reply_VariantGet](#reply-variantget) or null | Typed next page for the same identity and section selection. Optional. |
| `records` | list of [Reply_CosmicRecord](#reply-cosmicrecord) | Native historical records in source order. Optional. |
| `total` | integer (uint64) | Number of native historical fragments before paging. |

### Reply_VariantGet

Read an rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates such as 7-140753336-A-T through the shared resolver. MyVariant identity lookups default to GRCh38; --build overrides it. Chromosome genomic HGVS requires --build. Use search variant --gene BRAF --protein V600E or search variant --gene PTEN --coding c.333G&gt;T for gene shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](#reply-build) or null | Assembly MyVariant identity assembly defaults to GRCh38; chromosome genomic HGVS requires an explicit build. Optional. |
| `civic_cursor` | [Reply_Text](#reply-text) or null | Cursor Native CIViC cursor, retained unchanged. Optional. |
| `civic_kind` | [Reply_CivicClaimKind](#reply-civicclaimkind) or null | Claims Native CIViC claim collection within the selected profile. Optional. |
| `civic_profile` | integer or null | Selected CIViC molecular profile for native claim paging. Optional. |
| `evidence_level` | list of [Reply_CivicEvidenceLevel](#reply-civicevidencelevel) | A–E Native CIViC evidence levels; repeat to include several levels. Optional. |
| `evidence_status` | [Reply_CivicEvidenceStatus](#reply-civicevidencestatus) or null | Status Native CIViC evidence status; omitted preserves the source's existing default. Optional. |
| `input` | [Reply_VariantLookupInput](#reply-variantlookupinput) | rs/CA/clinvar:N/VCV/HGVS/VCF rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates. |
| `limit` | integer (uint32) | Candidate rows per page; more remain reachable through typed next commands. Optional. |
| `offset` | integer (uint32) | Candidate rows to skip in source order. Optional. |
| `resolver` | [Reply_VariantResolver](#reply-variantresolver) | Resolver. Resolver for reference-qualified notation; never repairs the input locally. Optional. |
| `section` | list of [Reply_VariantSection](#reply-variantsection) | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. Optional. |
| `section_limit` | integer (uint32) | Rows per named source section page; omitted uses the native source default. Optional. |
| `section_offset` | integer (uint32) | Source rows to skip independently of identity candidate paging. Optional. |
| `tumor_type` | [Reply_Text](#reply-text) or null | Tumor Explicit tumor context for keyed OncoKB annotation. Optional. |

### Reply_VariantGwas

One page of exact rsID catalog associations.

| Field | Type | Meaning |
| --- | --- | --- |
| `associations` | list of [Reply_GwasAssociation](#reply-gwasassociation) | Native source associations, without inferred clinical meaning. Optional. |
| `continuation` | [Reply_VariantGet](#reply-variantget) or null | Continue this section using the same rsID and source selectors. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Exact source page bounds and total. |
| `rs_id` | [Reply_VariantId](#reply-variantid) | Exact source rsID. |

### Reply_VariantHeader

Consumer identity header with an exact source selection and compact annotation entity.

| Field | Type | Meaning |
| --- | --- | --- |
| `component_terms` | list of [Reply_ComponentLabel](#reply-componentlabel) | Terms for carried component identifiers on this consumer header. Optional. |
| `identity` | [Reply_Variant](#reply-variant) | Source-qualified variant entity. |
| `myvariant` | [Reply_Variant](#reply-variant) or null | Compact MyVariant annotations, attributed by their field name. Optional. |
| `record` | [Reply_SourceRecord](#reply-sourcerecord) | Exact source record, including source keys that are expressions rather than identifiers. |
| `requested` | [Reply_VariantId](#reply-variantid) or null | Checked caller ID used by a source-only section, without asserting resolved identity. Optional. |
| `sections` | [Reply_VariantSections](#reply-variantsections) or null | Explicitly requested source sections for this exact candidate. Optional. |
| `source` | [Reply_Source](#reply-source) | Source responsible for this identity. |
| `transcripts` | list of [Reply_VariantTranscript](#reply-varianttranscript) | Source-paired transcript identifiers and unchanged coding and protein notation. Optional. |

### Reply_VariantHotspots

Source residue recurrence counts without allele-frequency or clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `next` | [Reply_VariantGet](#reply-variantget) or null | Continue this section without changing identity selectors. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Disclosure bounds for this section. |
| `reason` | [Reply_Reason](#reply-reason) or null | Source rows remain candidates when no transcript/consequence join exists. Optional. |
| `records` | list of [Reply_HotspotRecord](#reply-hotspotrecord) | Native residue records in source order. Optional. |

### Reply_VariantId

Published variant-level identifiers, with each identifier's level retained.

Type: string.

### Reply_VariantLookupInput

Accepted forms: rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS, chromosome genomic HGVS, or VCF-style chromosome-position-reference-alternate coordinates (SNVs, deletions, insertions and delins), defaulting to GRCh38. Reference bases are validated by the source.

Type: string.

### Reply_VariantOncokb

Keyed OncoKB annotation, preserving the caller's assembly and tumor context.

| Field | Type | Meaning |
| --- | --- | --- |
| `allele_exist` | boolean or null | Native alternate allele curation flag. Optional. |
| `alteration` | [Reply_Text](#reply-text) or null | Source-resolved protein alteration, independent of the requested genomic identity. Optional. |
| `build` | [Reply_Build](#reply-build) | Explicit caller assembly, checked against the response query. |
| `data_version` | [Reply_Text](#reply-text) or null | Native dataset version. Optional. |
| `gene_exist` | boolean or null | Source gene curation flag. Optional. |
| `hgvsg` | [Reply_Text](#reply-text) | Exact genomic query sent to OncoKB. |
| `hotspot` | boolean or null | Source hotspot flag, without an actionability conclusion. Optional. |
| `hugo_symbol` | [Reply_GeneSymbol](#reply-genesymbol) or null | Source-resolved gene, without a genomic or protein conversion. Optional. |
| `last_update` | [Reply_Text](#reply-text) or null | Native release date, kept in its published spelling. Optional. |
| `next` | [Reply_VariantGet](#reply-variantget) or null | Continue without changing identity, build, tumor context or source. Optional. |
| `page` | [Reply_PageInfo](#reply-pageinfo) | Disclosure bounds over the records, independent of identity paging. |
| `records` | list of [Reply_OncokbVariantRecord](#reply-oncokbvariantrecord) | Source claims and text in native field order. Optional. |
| `tumor_type` | [Reply_Text](#reply-text) | Explicit caller tumor type, checked against the response query. |
| `variant_exist` | boolean or null | Native exact variant curation flag; never a clinical conclusion. Optional. |
| `vus` | boolean or null | Source investigated-but-unknown flag, never used to create a classification. Optional. |

### Reply_VariantPopulation

Bounded population summary with independent optional sequencing datasets.

| Field | Type | Meaning |
| --- | --- | --- |
| `build` | [Reply_Build](#reply-build) | Source dataset assembly. |
| `dataset` | [Reply_Text](#reply-text) | Source dataset selector. |
| `dataset_version` | [Reply_Text](#reply-text) or null | Release shared by independently checked exome and genome totals in this response. Optional. |
| `exome` | [Reply_GnomadCounts](#reply-gnomadcounts) or null | Exome observation; omitted when the source dataset is null. Optional. |
| `genome` | [Reply_GnomadCounts](#reply-gnomadcounts) or null | Genome observation; omitted when the source dataset is null. Optional. |
| `joint` | [Reply_Section_GnomadCounts](#reply-section-gnomadcounts) | Independent source joint totals, or an explicit empty state when none were supplied. Optional. |
| `variant` | [Reply_Text](#reply-text) | Source allele identifier. |

### Reply_VariantPredictors

Source prediction scores and the terms of their individual components.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_VariantComponent](#reply-variantcomponent) | Independent component versions and licenses. Optional. |
| `context` | list of [Reply_PredictorContext](#reply-predictorcontext) | Original context arrays, without inferred cross-field alignment. Optional. |
| `scores` | list of [Reply_PredictorScore](#reply-predictorscore) | Every retained source score in declared field order. Optional. |
| `subject_qualification` | [Reply_SubjectQualification](#reply-subjectqualification) | How the source chromosome label was reference-qualified. |

### Reply_VariantRef

Checked source subject reference; at least one source identity or label is required.

One of these forms:

- `level` is `"categorical"`: no description. Fields: `category` ([Reply_Category](#reply-category)), `edit` ([Reply_Coding](#reply-coding) or null), `gene` ([Reply_GeneRef](#reply-generef)), `label` ([Reply_Text](#reply-text)), `partner` ([Reply_GeneRef](#reply-generef) or null), `region` ([Reply_Region](#reply-region) or null), `site` ([Reply_ProteinSite](#reply-proteinsite) or null).
- `level` is `"allele"`: Exact allele identifier or reference-qualified notation. Fields: `hgvs` ([Reply_HgvsText](#reply-hgvstext) or null), `id` ([Reply_VariantId](#reply-variantid) or null).
- `level` is `"protein"`: Protein-level assertion, without genomic conversion. Fields: `gene` ([Reply_GeneRef](#reply-generef)), `hgvs` ([Reply_HgvsText](#reply-hgvstext) or null), `label` ([Reply_Text](#reply-text) or null).
- `level` is `"haplotype"`: Native ClinVar haplotype and its complete source member identities. Fields: `accession` ([Reply_VcvId](#reply-vcvid)), `id` ([Reply_ClinVarVariationId](#reply-clinvarvariationid)), `kind` ([Reply_ClinvarHaplotypeKind](#reply-clinvarhaplotypekind)), `members` (list of [Reply_ClinvarHaplotypeMember](#reply-clinvarhaplotypemember)), `number_of_chromosomes` ([Reply_Text](#reply-text)), `version` ([Reply_Text](#reply-text)).
- `level` is `"source"`: Native source variant with optional source-stated forms. Fields: `feature` ([Reply_Text](#reply-text)), `feature_record` ([Reply_Curie](#reply-curie) or null), `forms` (list of object or object), `record` ([Reply_Curie](#reply-curie)).
- `level` is `"profile"`: Compound source profile, without inferred allele identity or phase. Fields: `expression` ([Reply_ProfileExpression](#reply-profileexpression) or null), `label` ([Reply_Text](#reply-text)), `members` (list of object or object or object), `record` ([Reply_Text](#reply-text)).

### Reply_VariantResolution

Identity resolution preserves candidates and source validation outcomes.

Values: `unresolved`, `resolved`, `candidates`, `absent`, `rejected`, `unavailable`.

### Reply_VariantResolver

Explicit source service for reference-qualified notation.

Values: `car`, `mutalyzer`, `variantvalidator`.

### Reply_VariantSection

Named variant source sections, without automatic expansion of identity cards.

Type: string.

### Reply_VariantSections

Source-specific sections, absent unless explicitly selected.

| Field | Type | Meaning |
| --- | --- | --- |
| `alphagenome` | [Reply_Section_VariantAlphaGenome](#reply-section-variantalphagenome) or null | Explicit keyed AlphaGenome numeric scorer result. Optional. |
| `civic` | [Reply_Section_VariantCivic](#reply-section-variantcivic) or null | Explicit civic source result. Optional. |
| `clinvar` | [Reply_Section_VariantAssertions](#reply-section-variantassertions) or null | ClinVar native assertion rows. Optional. |
| `conservation` | [Reply_Section_VariantConservation](#reply-section-variantconservation) or null | Native conservation scores. Optional. |
| `cosmic` | [Reply_Section_VariantCosmic](#reply-section-variantcosmic) or null | Historical MyVariant-associated COSMIC fragments. Optional. |
| `erepo` | [Reply_Section_VariantAssertions](#reply-section-variantassertions) or null | ClinGen native assertion rows. Optional. |
| `gwas` | [Reply_Section_VariantGwas](#reply-section-variantgwas) or null | Explicit GWAS Catalog source result. Optional. |
| `hotspots` | [Reply_Section_VariantHotspots](#reply-section-varianthotspots) or null | Explicit hotspots source result. Optional. |
| `oncokb` | [Reply_Section_VariantOncokb](#reply-section-variantoncokb) or null | Explicit keyed OncoKB source result. Optional. |
| `population` | [Reply_Section_VariantPopulation](#reply-section-variantpopulation) or null | Explicit population source result. Optional. |
| `predict` | [Reply_Section_VariantPredictors](#reply-section-variantpredictors) or null | Native prediction scores. Optional. |

### Reply_VariantTranscript

A source-paired transcript annotation, without inferred transcript equivalence.

| Field | Type | Meaning |
| --- | --- | --- |
| `component` | [Reply_Text](#reply-text) | Native component and field carrying the paired annotation. |
| `gene` | [Reply_Text](#reply-text) or null | Source-stated gene symbol. Optional. |
| `hgvs_c` | [Reply_Text](#reply-text) or null | Source transcript notation from the coding field, including native n. or r. forms. Optional. |
| `hgvs_p` | [Reply_Text](#reply-text) or null | Source protein notation, never translated from coding notation. Optional. |
| `matched` | boolean | Whether this row's source notation matched the declared caller notation. |
| `source` | [Reply_Source](#reply-source) | Provider carrying this annotation. |
| `transcript` | [Reply_Text](#reply-text) | Transcript identifier exactly as published, including its source version. |

### Reply_VcvId

A ClinVar variation accession, possibly a haplotype or compound.

Type: string.

### Reply_WhoProductId

WHO product listing identity, independent of product codes.

Type: string.

### Reply_WikiPathwaysId

A checked WikiPathways identity without its revision.

Type: string.

### Reply_WithheldArticleText

Why article text was withheld, with a public reading destination.

| Field | Type | Meaning |
| --- | --- | --- |
| `reason` | [Reply_Reason](#reply-reason) | `disabled` when the use policy refuses the record's text; `unavailable` when its license could not be checked, so body text was left out. |
| `url` | [Reply_Text](#reply-text) or null | Public record where the caller can read the paper. Optional. |

### Reply_WithUnit_FiniteNumber_Text

A source number with its explicitly stated unit; missing units stay absent.

| Field | Type | Meaning |
| --- | --- | --- |
| `unit` | [Reply_Text](#reply-text) or null | Publisher unit or code, without conversion or inference. Optional. |
| `value` | [Reply_FiniteNumber](#reply-finitenumber) | Unchanged numeric source value. |

## Next

[Reply schemas](/reference/schemas/) lists every reply and the commands that return it. [Results](/results/) explains how to read a reply from the command line.
