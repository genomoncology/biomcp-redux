---
title: biomcp.variant_page/6
description: "The BioMCP 1.0.0 reply schema biomcp.variant_page/6: its envelope
  fields, its result type VariantPage and the commands that return it."
goal: Look up the fields of a biomcp.variant_page/6 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_page/6
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_page/6`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_page/6`.

These commands return it: [`biomcp search variant`](/reference/commands/search-variant/).

Calls that returned it:

- `biomcp search variant --gene KRAS --protein G12C` ([`biomcp search variant`](/reference/commands/search-variant/))
- `biomcp search variant --gene EGFR --protein L858R --build GRCh37` ([`biomcp search variant`](/reference/commands/search-variant/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [VariantSearch](#query-variantsearch) | yes | Parsed query. |
| `result` | [VariantPage](#result-variantpage) or null | no | Answer when available. |
| `schema` | "biomcp.variant_page/6" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: VariantPage

One native MyVariant candidate page; its total and offset are in the shared envelope.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Assembly sent to the source, never inferred from a candidate label. |
| `filter_terms` | [ComponentTerms](#componentterms) or null | no | Independent publisher terms for named ClinVar filters, present only when requested. |
| `items` | array of [VariantHeader](#variantheader) | yes | Native source order, preserving every candidate including repeated source identities. |
| `selector_terms` | [ComponentTerms](#componentterms) or null | no | Independent publisher terms for the declared dbNSFP gene/protein selector. |
| `source_query` | string | yes | Exact query sent to the named source, without rewriting raw query text. |

## Query: VariantSearch

Discover source candidates using one declared selector, without interpreting free text.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) | no | Optional caller assembly sent to MyVariant; regions require it explicitly. |
| `condition` | [Text](#text) | no | Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. |
| `gene` | [GeneSymbol](#genesymbol) | no | Exact checked gene symbol for the fixed dbNSFP gene/protein field query; obeys component terms switches. |
| `has` | [VariantSearchField](#variantsearchfield) | no | Require the selected component's identifier field. Only source-proven mappings are offered. |
| `limit` | integer (uint32) | no | Native candidates per page, default 10, maximum 50; all candidates remain candidates. |
| `offset` | integer (uint32) | no | Native offset within this command's bounded result window; larger native totals remain visible. |
| `protein` | [ProteinQuery](#proteinquery) | no | Explicit substitution using checked one-letter or three-letter amino-acid codes. |
| `query` | [MyVariantQuery](#myvariantquery) | no | Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. |
| `region` | [VariantRegion](#variantregion) | no | Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. |
| `review_status` | [Text](#text) | no | Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. |
| `source` | [VariantSearchSource](#variantsearchsource) | no | Explicit source language; required for a raw query. |

## Types

Every other type this schema uses.

### Accession

Type: string.

### AcmgAmp2015Code

Checked AcmgAmp2015 codes.

Values: `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### Allele

Source-stated interbase replacement; equality never claims normalized equivalence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alternate` | [Sequence](#sequence) | yes |  |
| `location` | [SequenceLocation](#sequencelocation) | yes |  |
| `reference` | [Sequence](#sequence) | yes |  |

### AmpAscoCap2017LevelCode

Checked AmpAscoCap2017Level codes.

Values: `level_a`, `level_b`, `level_c`, `level_d`.

### AmpAscoCap2017TierCode

Checked AmpAscoCap2017Tier codes.

Values: `tier_i`, `tier_ii`, `tier_iii`, `tier_iv`.

### AssertionAssessment

Source assessment labels and identifiers remain independent.

| Field | Type | Required | Description |
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

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [Text](#text) or null | no | Native accession text, preserved even when no CURIE namespace is recognized. |
| `database` | [Text](#text) or null | no | Native database name without inventing a CURIE prefix. |
| `id` | [Curie](#curie) or null | no | Identifier exactly supplied by the source. |
| `label` | [Text](#text) or null | no | Label only when supplied. |

### AssertionConditionRelationship

A source trait relationship without assigning a clinical interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `related_conditions` | array of [AssertionCondition](#assertioncondition) | yes | Conditions named by the related source trait. |
| `relationship_type` | [Text](#text) or null | no | Relationship type exactly supplied by the source. |
| `source_conditions` | array of [AssertionCondition](#assertioncondition) | yes | Conditions on the source trait that owns this relationship. |

### AssertionDetail

Source details that do not fit the compact shared statement.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `approved` | [PartialDate](#partialdate) or null | no | Source approval date, distinct from evaluation and publication. |
| `citations` | array of [Pmid](#pmid) | no | Source citations, including assertion-method citations. |
| `comments` | array of [Text](#text) | no | Native classification comments, without extracting clinical claims. |
| `condition_relationships` | array of [AssertionConditionRelationship](#assertionconditionrelationship) | no | Native trait relationships, kept separate from assertion conditions. |
| `conditions` | array of [AssertionCondition](#assertioncondition) | no | All source conditions; ID-only conditions stay ID-only. |
| `document` | [AssertionDocument](#assertiondocument) or null | no | Source document facts for an explicitly selected historical version. |
| `impact_assertion_type` | [Text](#text) or null | no | Native somatic impact assertion type, without deriving a predicate. |
| `impact_clinical_significance` | [Text](#text) or null | no | Native somatic significance, independent of the tier label. |
| `impact_therapies` | [Text](#text) or null | no | Native therapy text, without splitting or assigning it to a predicate. |
| `methods` | array of [Text](#text) | no | Assertion method labels, not inferred from clinical testing. |
| `origins` | array of [Text](#text) | no | Source origin labels without reducing multiple observations. |
| `published` | [PartialDate](#partialdate) or null | no | Source publication date; never an inferred evaluation date. |
| `summary` | [Text](#text) or null | no | Source prose, kept separately from structured criteria. |
| `unmet_criteria` | array of [Text](#text) | no | Criteria explicitly marked unmet. |
| `versions` | array of [Text](#text) | no | Native document versions offered by the source. |

### AssertionDocument

Native SEPIO facts kept separate from common interpretation fields.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assessments` | array of [AssertionAssessment](#assertionassessment) | no | Assessments directly supporting the interpretation. |
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

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `evaluated` | [PartialDate](#partialdate) or null | no | Source evaluation date. |
| `record` | [Text](#text) or null | no | Native assertion record. |
| `release` | [Text](#text) or null | no | Dataset release. |
| `source` | [Text](#text) | yes | Source responsible for the claim. |
| `submitter` | [Text](#text) or null | no | Source submitter or panel. |
| `url` | [Text](#text) or null | no | Source record link. |
| `version` | [Text](#text) or null | no | Record version. |

### Build

The assembly stated by the source; no default is inferred.

Values: `GRCh37`, `GRCh38`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### CivicClaimKind

CIViC native claim collection.

Values: `evidence`, `assertions`.

### CivicClaimPage

Native stream bounds; evidence and assertions have separate cursors.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | no | Continue the same source candidate, profile and claim stream. |
| `returned` | integer (uint32) | yes | Number of native rows displayed in this stream. |
| `total` | integer (uint64) | yes | Native total for this stream. |

### CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

### CivicMember

A native CIViC member, including members without a common-model subject.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `feature` | [Text](#text) | yes | Native feature label, including fusion features. |
| `hgvs_descriptions` | array of [Text](#text) | no | Every native notation, without selecting a transcript or repairing a spelling. |
| `label` | [Text](#text) | yes | Whole source variant label. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native variant identity and link. |
| `subject` | [VariantRef](#variantref) or null | no | Source-qualified subject when supported by the common model. |

### CivicMembership

Identity strength of a CIViC profile lookup.

Values: `exact`, `candidate`.

### CivicNativeClaim

Source facts retained without inventing a common-model subject or proposition.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele_origin` | [Term](#term) or null | no | Native origin, when stated. |
| `attribution` | [Attribution](#attribution) | yes | Original source provenance. |
| `citations` | array of [Pmid](#pmid) | no | Native publication identities. |
| `classification` | [Term](#term) | yes | Whole native classification. |
| `condition` | [Concept](#concept) or null | no | Source disease context. |
| `strength` | [Term](#term) or null | no | Native level, when present. |
| `therapies` | array of [Concept](#concept) | no | Native therapies in source order. |

### CivicProfile

One source molecular profile, preserving compound expressions.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertion_count` | integer (uint64) | yes | Native assertion count, independent of the displayed page. |
| `assertion_page` | [CivicClaimPage](#civicclaimpage) | yes | Native assertion stream and its independent continuation. |
| `compound` | boolean | yes | Native complex-profile flag; a member match does not satisfy the whole profile. |
| `evidence_count` | integer (uint64) | yes | Native evidence count, independent of the displayed page. |
| `evidence_page` | [CivicClaimPage](#civicclaimpage) | yes | Native evidence stream and its independent continuation. |
| `label` | [Text](#text) | yes | Whole source profile label, including its logical expression. |
| `members` | array of [CivicMember](#civicmember) | yes | Every source member reference, without collapsing compound profiles. |
| `membership` | [CivicMembership](#civicmembership) | yes | Whether a source member was joined to the requested exact identity. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native profile identity and link. |
| `statements` | array of [CivicStatement](#civicstatement) | yes | Source claims; a compound claim retains a profile subject. |

### CivicStatement

One CIViC claim with native labels that exceed the common statement model.

| Field | Type | Required | Description |
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

### Coding

A code in one explicitly named source framework.

One of:

- object: OncoKB oncogenicity vocabulary.
- object: OncoKB evidence levels.
- object: CIViC evidence level.
- object: acmg_amp_2015 vocabulary.
- object: clingen_low_penetrance_2024 vocabulary.
- object: amp_asco_cap_2017_tier vocabulary.
- object: amp_asco_cap_2017_level vocabulary.
- object: clingen_cgc_vicc_2022 vocabulary.

### ComponentLabel

Terms attached to a named carried component on a consumer response.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `component` | [Text](#text) | yes | Native component name. |
| `terms` | [ComponentTerms](#componentterms) | yes | Canonical publisher terms. |

### ComponentTerms

Terms attached to one independently licensed source component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | [Text](#text) | yes | Publisher terms name. |
| `url` | [Text](#text) | yes | Publisher terms page. |
| `usage` | [Use](#use) | yes | Publisher use category. |

### ComponentUpload

One source-named component upload timestamp.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `date` | [Text](#text) | yes | Unchanged source timestamp. |
| `name` | [Text](#text) | yes | Source subcollection name. |

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) or null | no | Source-stated concept identifier. |
| `label` | [Text](#text) | yes | Original concept label. |

### CosmicCoordinates

Historical coordinates exactly as carried by the component.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | integer (uint64) | yes | Source end value, without coordinate conversion. |
| `start` | integer (uint64) | yes | Source start value, without coordinate conversion. |

### CosmicIdentity

Relationship between a carried historical fragment and its aggregator record.

Values: `unverified`.

### CosmicRecord

One source record from MyVariant's historical COSMIC copy.

| Field | Type | Required | Description |
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
| `tumor_site` | array of [Text](#text) | no | Every source tumor-site label in native order. |

### Curie

Type: string.

### Direction

Assessment direction stated by the source.

Values: `supports`, `disputes`, `neutral`.

### EnsemblGeneId

Type: string.

### Expression

Source expression checked for its declared published form, without normalization.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `syntax` | [ExpressionSyntax](#expressionsyntax) | yes |  |
| `syntax_version` | [Text](#text) or null | no |  |
| `value` | [Text](#text) | yes |  |

### ExpressionSyntax

Published syntax label for a source-preserved expression.

Values: `hgvs.g`, `hgvs.c`, `hgvs.n`, `hgvs.r`, `hgvs.p`, `hgvs.m`, `hgvs.o`, `spdi`, `gnomad`, `vcf`, `iscn`.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

### GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ensembl_gene_id` | [EnsemblGeneId](#ensemblgeneid) or null | no |  |
| `hgnc_id` | [HgncId](#hgncid) or null | no |  |
| `label` | [Text](#text) | yes |  |
| `ncbi_gene_id` | [NcbiGeneId](#ncbigeneid) or null | no |  |

### GeneSymbol

Type: string.

### GnomadCounts

Native gnomAD counts retained when a dataset release cannot be established.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ac` | integer (uint64) | yes | Native allele count, including reported zero. |
| `an` | integer (uint64) | yes | Native allele denominator. |
| `ancestry_group` | [Text](#text) or null | no | Source ancestry code decoded only by the documented source grammar. |
| `data_type` | [PopulationDataType](#populationdatatype) | yes | Source sequencing sample set. |
| `filters` | array of [Text](#text) | no | Native source filters, never converted to a classification. |
| `frequency` | [PopulationFrequency](#populationfrequency) or null | no | Checked total from a release supplied alongside matching source counts. |
| `group_id` | [Text](#text) or null | no | Complete native group ID, including its source sex suffix when present. |
| `sex` | [PopulationSex](#populationsex) or null | no | Source sex stratum decoded only by the documented source grammar. |

### GwasAssociation

Source association with unchanged trait and effect labels.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alleles` | array of [GwasEffectAllele](#gwaseffectallele) | yes | Every source effect allele paired with its rsID. |
| `author` | [Text](#text) or null | no | Source author attribution, without local publication inference. |
| `beta` | [Text](#text) or null | no | Source beta text, including units and direction. |
| `ci_lower` | [FiniteNumber](#finitenumber) or null | no | Source lower effect interval bound. |
| `ci_upper` | [FiniteNumber](#finitenumber) or null | no | Source upper effect interval bound. |
| `exact_p_value` | [ScientificPValue](#scientificpvalue) or null | no | Authoritative exact source probability, including double underflow. |
| `id` | integer (uint64) | yes | Native association identity. |
| `interval` | [Text](#text) or null | no | Source effect interval. |
| `locations` | array of [Text](#text) | no | Native mapping location strings; no assumed assembly or coordinate conversion. |
| `mapped_genes` | array of [Text](#text) | no | Original native mapped gene context; reported genes remain in loci. |
| `odds_ratio` | [FiniteNumber](#finitenumber) or null | no | Source odds ratio. |
| `ontology_traits` | array of [GwasTrait](#gwastrait) | no | Source mapped trait identifiers and whole labels, without clinical inference. |
| `p_value` | [FiniteNumber](#finitenumber) | yes | Reported p value. |
| `publication` | [Text](#text) or null | no | Source publication identifier. |
| `risk_frequency` | [Text](#text) or null | no | Source allele frequency text, without interpreting NR. |
| `study` | [Text](#text) | yes | Study accession. |
| `traits` | array of [Text](#text) | yes | Source reported traits. |

### GwasEffectAllele

Catalog effect allele paired with its exact rsID.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele` | [Text](#text) | yes | Source allele text. |
| `rs_id` | [Text](#text) | yes | Source rsID. |

### GwasTrait

Source ontology mapping as supplied by GWAS Catalog.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Text](#text) | yes | Exact source ontology identifier spelling. |
| `label` | [Text](#text) | yes | Whole source trait label. |

### HgncId

Type: string.

### HgvsText

Type: string.

### HotspotAlternate

One source amino-acid alternate count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `amino_acid` | [Text](#text) | yes | Source alternate amino-acid spelling. |
| `count` | integer (uint64) | yes | Source count, without deriving a frequency. |

### HotspotMembership

Source transcript and consequence join state.

Values: `exact`, `candidate`.

### HotspotRecord

Counts for one source transcript and residue label.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alternates` | array of [HotspotAlternate](#hotspotalternate) | yes | Source alternate amino acids and their counts. |
| `gene` | [GeneSymbol](#genesymbol) | yes | Gene as stated by the source. |
| `membership` | [HotspotMembership](#hotspotmembership) | yes | Whether transcript and consequence were linked by the identity source. |
| `residue` | [Text](#text) | yes | Native residue label; no genomic allele is inferred. |
| `transcript` | [Text](#text) or null | no | Native transcript spelling, without an inferred accession version. |
| `tumor_count` | integer (uint64) | yes | Tumors reported for this residue by the source. |

### LeafRef

One of:

- object
- object

### MyVariantQuery

Type: string.

### NcbiGeneId

Type: string.

### OncokbCitation

One native non-PubMed source citation.

| Field | Type | Required | Description |
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

One of:

- object: A complete source statement with its original label.
- object: Native non-PubMed citation, preserving source text and link.
- object: Native narrative, split only at checked character boundaries.
- object: A source label without a representable source protein subject.
- object: Native therapeutic, diagnostic or prognostic implication.

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

### PopulationDataType

Dataset sequencing sample set.

Values: `exomes`, `genomes`, `joint`.

### PopulationFrequency

One checked stratum of one source dataset release.

| Field | Type | Required | Description |
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
| `filters` | array of [Text](#text) | no |  |
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

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `field` | [Text](#text) | yes | Native source field path. |
| `values` | array of [Text](#text) | yes | Every native value in source array order. |

### PredictorScore

One unmodified source score, never a locally blended assessment.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source-owned record and dataset release. |
| `label` | [Text](#text) or null | no | Source label without code expansion or interpretation. |
| `method` | [Text](#text) | yes | Published component field path. |
| `score` | [FiniteNumber](#finitenumber) | yes | Source number, including zero. |
| `subject` | [VariantRef](#variantref) | yes | Exact source-qualified subject. |

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

### Proposition

Domain of a source claim.

Values: `pathogenicity`, `oncogenicity`, `clinical_significance`, `therapeutic_response`, `diagnostic`, `prognostic`, `functional_impact`.

### ProteinQuery

Type: string.

### Reason

Why a source could not supply data.

Values: `not_found`, `not_open`, `not_pdf`, `unsupported`, `too_large`, `rate_limited`, `unavailable`, `tls`, `proxy`, `rejected`, `bad_reply`, `deadline_exceeded`, `offline`, `no_key`, `disabled`, `replay_miss`, `syncing`, `no_join_id`.

### Representation

One source's own expression, assembly and optional source-stated interbase allele.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele` | [Allele](#allele) or null | no | Source-supplied allele, without local normalization or equivalence inference. |
| `build` | [Build](#build) or null | no | Assembly stated by the source, omitted when absent. |
| `expression` | [Expression](#expression) | yes | Published notation kept unchanged. |
| `source` | [Text](#text) | yes | Source asserting this form. |

### ScientificPValue

Exact normalized scientific p value; no floating-point conversion is used.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `exponent` | integer (int64) | yes | Source base-ten exponent; values below the double range remain exact. |
| `mantissa` | string | yes | Source mantissa spelling, including significant trailing zeros. |

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

### Section8

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Section9

A requested section has exactly one state.

One of:

- object: The source returned data.
- object: The source answered with no data.
- object: A requested source failed.
- object: User settings prevented the source request.
- object: Entity facts rule this section out.

### Sequence

Type: string.

### SequenceLocation

An interbase interval on one versioned sequence accession.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `sequence` | [Accession](#accession) | yes | Versioned source sequence, without an inferred genome build. |
| `span` | [Span0](#span0) | yes | Zero-based half-open bounds. |

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

### Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | integer (uint32) | yes |  |
| `start` | integer (uint32) | yes |  |

### Statement

One source claim, kept in its own framework and context.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `allele_origin` | [Term](#term) or null | no | Source-stated origin. |
| `attribution` | [Attribution](#attribution) | yes | One source record and its provenance. |
| `citations` | array of [Pmid](#pmid) | no | Source publications. |
| `classification` | [Term](#term) | yes | Whole source classification label. |
| `condition` | [Concept](#concept) or null | no | Source disease or tumor context. |
| `criteria` | array of [Text](#text) | no | Source criterion labels in source order. |
| `direction` | [Direction](#direction) or null | no | Source-stated assessment direction. |
| `gene_context` | [GeneRef](#generef) or null | no | Source gene context. |
| `predicate` | [Predicate](#predicate) or null | no | Source-stated relationship. |
| `proposition` | [Proposition](#proposition) | yes | Claim domain. |
| `strength` | [Term](#term) or null | no | Source evidence level or review status. |
| `subject` | [VariantRef](#variantref) | yes | Source-owned subject reference. |
| `therapies` | array of [Concept](#concept) | no | Source therapies in their native order. |

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

### SubjectQualification

Provenance of the subject's accession qualification, without allele normalization.

Values: `adapter_derived_reference`.

### Term

A source label with optional explicitly supported coding.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `coding` | [Coding](#coding) or null | no | Documented framework coding. |
| `label` | [Text](#text) | yes | Original label. |

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### UnmappedAssertion

Source classification retained when its category has no approved shared proposition.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Original record provenance. |
| `classification` | [Text](#text) | yes | Unchanged source label. |
| `conditions` | array of [AssertionCondition](#assertioncondition) | no | Native conditions, including identifiers without supplied labels. |
| `review_status` | [Text](#text) or null | no | Native review status. |

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

### Variant

Variant entity with source-stated identities, forms and genes; no disclosure policy.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `genes` | array of [GeneRef](#generef) | no | Genes stated by the sources. |
| `ids` | array of [VariantId](#variantid) | no | All source identifiers, without inferred aliases. |
| `representations` | array of [Representation](#representation) | no | Every source-qualified representation. |

### VariantAssertion

A native record/domain pair, never a vote or merged classification.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `detail` | [AssertionDetail](#assertiondetail) or null | no | Original source detail, returned only for an explicitly selected record. |
| `domain` | [AssertionDomain](#assertiondomain) | yes | Source classification domain. |
| `kind` | [AssertionKind](#assertionkind) | yes | Aggregate/submission/document context. |
| `statement` | [Statement](#statement) or null | no | Supported mapping into the common model. |
| `statement_unmapped` | boolean | no | Native category has no approved common-model proposition. |
| `unmapped` | [UnmappedAssertion](#unmappedassertion) or null | no | Native assertion whose category the common model cannot represent. |

### VariantAssertions

One source-ordered page, with native record and classification boundaries intact.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertions` | array of [VariantAssertion](#variantassertion) | yes | Source assertion rows in native order. |
| `next` | [VariantAssertionsGet](#variantassertionsget) or null | no | Typed continuation preserving source and selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Page bounds over native rows. |

### VariantAssertionsGet

Page assertions from one explicitly selected evidence source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertion` | [Text](#text) or null | no | Native assertion accession or document ID. |
| `input` | [VariantInput](#variantinput) | yes | Checked Variation ID for ClinVar, or CAid for ERepo. |
| `limit` | integer (uint32) | no | Assertions per page; omitted uses the native source default. |
| `offset` | integer (uint32) | no | Assertions to skip in source order. |
| `source` | [AssertionSource](#assertionsource) | yes | One native source; no fallback or text search. |
| `version` | [Text](#text) or null | no | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. |

### VariantCivic

CIViC molecular profiles with their original membership and separate source claims.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | no | Continue this section without changing the input or source selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Disclosure bounds for this section. |
| `profiles` | array of [CivicProfile](#civicprofile) | yes | Source profiles in native order; name candidates carry no attached statements. |

### VariantComponent

Metadata for a component carried by an aggregator.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `aggregator_build` | [Text](#text) | yes | Aggregator build version. |
| `aggregator_built` | [Text](#text) | yes | Aggregator build timestamp, distinct from a component release. |
| `license` | [Text](#text) | yes | Component license URL, without any inferred grant. |
| `name` | [Text](#text) | yes | Component name from aggregator metadata. |
| `note` | [Text](#text) or null | no | Whole source snapshot warning. |
| `terms` | [ComponentTerms](#componentterms) | yes | Independently reviewed publisher terms; no blanket aggregator grant. |
| `uploaded` | [Text](#text) or null | no | Component upload timestamp when the source states one. |
| `uploads` | array of [ComponentUpload](#componentupload) | no | Source-named component upload timestamps when the publisher supplies a map. |
| `version` | [Text](#text) or null | no | Component version; omitted when the source reports none. |

### VariantConservation

Source conservation measurements; no local pathogenicity inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [VariantComponent](#variantcomponent) | yes | Independent component versions and licenses. |
| `context` | array of [PredictorContext](#predictorcontext) | no | Original context arrays, without inferred cross-field alignment. |
| `scores` | array of [PredictorScore](#predictorscore) | yes | Every retained source measurement in declared field order. |
| `subject_qualification` | [SubjectQualification](#subjectqualification) | yes | How the source chromosome label was reference-qualified. |

### VariantCosmic

Historical COSMIC fields carried by MyVariant, without direct COSMIC access.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [VariantComponent](#variantcomponent) | yes | Snapshot version, dates, warning and terms. |
| `next` | [VariantGet](#variantget) or null | no | Typed next page for the same identity and section selection. |
| `records` | array of [CosmicRecord](#cosmicrecord) | yes | Native historical records in source order. |
| `total` | integer (uint64) | yes | Number of native historical fragments before paging. |

### VariantGet

Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. Use search variant --gene BRAF --protein V600E for shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Required for chromosome notation; no assembly is guessed. |
| `civic_cursor` | [Text](#text) or null | no | Native CIViC cursor, retained unchanged. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | no | Native CIViC claim collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected CIViC molecular profile for native claim paging. |
| `input` | [VariantInput](#variantinput) | yes | Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS. |
| `limit` | integer (uint32) | no | Candidate rows per page; more remain reachable through typed next commands. |
| `offset` | integer (uint32) | no | Candidate rows to skip in source order. |
| `resolver` | [VariantResolver](#variantresolver) | no | Resolver for reference-qualified notation; never repairs the input locally. |
| `section` | array of [VariantSection](#variantsection) | no | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. |
| `section_limit` | integer (uint32) | no | Rows per named source section page; omitted uses the native source default. |
| `section_offset` | integer (uint32) | no | Source rows to skip independently of identity candidate paging. |
| `tumor_type` | [Text](#text) or null | no | Explicit tumor context for keyed OncoKB annotation. |

### VariantGwas

One page of exact rsID catalog associations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `associations` | array of [GwasAssociation](#gwasassociation) | yes | Native source associations, without inferred clinical meaning. |
| `continuation` | [VariantGet](#variantget) or null | no | Continue this section using the same rsID and source selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Exact source page bounds and total. |
| `rs_id` | [VariantId](#variantid) | yes | Exact source rsID. |

### VariantHeader

Consumer identity header with an exact source selection and compact annotation entity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `component_terms` | array of [ComponentLabel](#componentlabel) | no | Terms for carried component identifiers on this consumer header. |
| `identity` | [Variant](#variant) | yes | Source-qualified variant entity. |
| `myvariant` | [Variant](#variant) or null | no | Compact MyVariant annotations, attributed by their field name. |
| `record` | [SourceRecord](#sourcerecord) | yes | Exact source record, including source keys that are expressions rather than identifiers. |
| `requested` | [VariantId](#variantid) or null | no | Checked caller ID used by a source-only section, without asserting resolved identity. |
| `sections` | [VariantSections](#variantsections) or null | no | Explicitly requested source sections for this exact candidate. |
| `source` | [Source](#source) | yes | Source responsible for this identity. |

### VariantHotspots

Source residue recurrence counts without allele-frequency or clinical interpretation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `next` | [VariantGet](#variantget) or null | no | Continue this section without changing identity selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Disclosure bounds for this section. |
| `reason` | [Reason](#reason) or null | no | Source rows remain candidates when no transcript/consequence join exists. |
| `records` | array of [HotspotRecord](#hotspotrecord) | yes | Native residue records in source order. |

### VariantId

Type: string.

### VariantInput

Type: string.

### VariantOncokb

Keyed OncoKB annotation, preserving the caller's assembly and tumor context.

| Field | Type | Required | Description |
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
| `records` | array of [OncokbVariantRecord](#oncokbvariantrecord) | yes | Source claims and text in native field order. |
| `tumor_type` | [Text](#text) | yes | Explicit caller tumor type, checked against the response query. |
| `variant_exist` | boolean | yes | Native exact variant curation flag; never a clinical conclusion. |
| `vus` | boolean or null | no | Source investigated-but-unknown flag, never used to create a classification. |

### VariantPopulation

Bounded population summary with independent optional sequencing datasets.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) | yes | Source dataset assembly. |
| `dataset` | [Text](#text) | yes | Source dataset selector. |
| `exome` | [GnomadCounts](#gnomadcounts) or null | no | Exome observation; omitted when the source dataset is null. |
| `genome` | [GnomadCounts](#gnomadcounts) or null | no | Genome observation; omitted when the source dataset is null. |
| `variant` | [Text](#text) | yes | Source allele identifier. |

### VariantPredictors

Source prediction scores and the terms of their individual components.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `components` | array of [VariantComponent](#variantcomponent) | yes | Independent component versions and licenses. |
| `context` | array of [PredictorContext](#predictorcontext) | no | Original context arrays, without inferred cross-field alignment. |
| `scores` | array of [PredictorScore](#predictorscore) | yes | Every retained source score in declared field order. |
| `subject_qualification` | [SubjectQualification](#subjectqualification) | yes | How the source chromosome label was reference-qualified. |

### VariantRef

Checked source subject reference; at least one source identity or label is required.

One of:

- object: Exact allele identifier or reference-qualified notation.
- object: Protein-level assertion, without genomic conversion.
- object: Compound source profile, without an inferred allele.

### VariantRegion

Type: string.

### VariantResolver

Explicit source service for reference-qualified notation.

Values: `car`, `mutalyzer`, `variantvalidator`.

### VariantSearchField

Source-proven component fields supported by the presence filter.

Values: `clinvar`.

### VariantSearchSource

The source-native query language exposed by variant search.

Values: `myvariant`.

### VariantSection

Named variant source sections, without automatic expansion of identity cards.

Values: `clinvar`, `erepo`, `population`, `gwas`, `civic`, `oncokb`, `hotspots`, `predict`, `conservation`, `cosmic`, `all`.

### VariantSections

Source-specific sections, absent unless explicitly selected.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic` | [Section8](#section8) or null | no | Explicit civic source result. |
| `clinvar` | [Section4](#section4) or null | no | ClinVar native assertion rows. |
| `conservation` | [Section2](#section2) or null | no | Native conservation scores. |
| `cosmic` | [Section3](#section3) or null | no | Historical MyVariant-associated COSMIC fragments. |
| `erepo` | [Section4](#section4) or null | no | ClinGen native assertion rows. |
| `gwas` | [Section6](#section6) or null | no | Explicit GWAS Catalog source result. |
| `hotspots` | [Section9](#section9) or null | no | Explicit hotspots source result. |
| `oncokb` | [Section7](#section7) or null | no | Explicit keyed OncoKB source result. |
| `population` | [Section5](#section5) or null | no | Explicit population source result. |
| `predict` | [Section](#section) or null | no | Native prediction scores. |

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_page/6` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
