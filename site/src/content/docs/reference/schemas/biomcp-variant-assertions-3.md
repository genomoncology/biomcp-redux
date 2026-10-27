---
title: biomcp.variant_assertions/3
description: "The BioMCP 1.0.0 reply schema biomcp.variant_assertions/3: its
  envelope fields, its result type VariantAssertions and the commands that
  return it."
goal: Look up the fields of a biomcp.variant_assertions/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.variant_assertions/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.variant_assertions/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.variant_assertions/3`.

These commands return it: [`biomcp variant assertions`](/reference/commands/variant-assertions/).

Calls that returned it:

- `biomcp variant assertions clinvar:13961 --source clinvar --limit 50` ([`biomcp variant assertions`](/reference/commands/variant-assertions/))
- `biomcp variant assertions CA000434 --source erepo` ([`biomcp variant assertions`](/reference/commands/variant-assertions/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [VariantAssertionsGet](#query-variantassertionsget) | yes | Parsed query. |
| `result` | [VariantAssertions](#result-variantassertions) or null | no | Answer when available. |
| `schema` | "biomcp.variant_assertions/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: VariantAssertions

One source-ordered page, with native record and classification boundaries intact.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertions` | array of [VariantAssertion](#variantassertion) | yes | Source assertion rows in native order. |
| `next` | [VariantAssertionsGet](#query-variantassertionsget) or null | no | Typed continuation preserving source and selectors. |
| `page` | [PageInfo](#pageinfo) | yes | Page bounds over native rows. |

## Query: VariantAssertionsGet

Page assertions from one explicitly selected evidence source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertion` | [Text](#text) or null | no | Native assertion accession or document ID. |
| `input` | [VariantInput](#variantinput) | yes | Checked Variation ID for ClinVar, or CAid for ERepo. |
| `limit` | integer (uint32) | no | Assertions per page; omitted uses the native source default. |
| `offset` | integer (uint32) | no | Assertions to skip in source order. |
| `source` | [AssertionSource](#assertionsource) | yes | One native source; no fallback or text search. |
| `version` | [Text](#text) or null | no | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. |

## Types

Every other type this schema uses.

### AcmgAmp2015Code

Checked AcmgAmp2015 codes.

Values: `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign`.

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

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

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

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) or null | no | Source-stated concept identifier. |
| `label` | [Text](#text) | yes | Original concept label. |

### Curie

Type: string.

### Direction

Assessment direction stated by the source.

Values: `supports`, `disputes`, `neutral`.

### EnsemblGeneId

Type: string.

### GeneRef

A source-stated gene reference; its label never implies an identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ensembl_gene_id` | [EnsemblGeneId](#ensemblgeneid) or null | no |  |
| `hgnc_id` | [HgncId](#hgncid) or null | no |  |
| `label` | [Text](#text) | yes |  |
| `ncbi_gene_id` | [NcbiGeneId](#ncbigeneid) or null | no |  |

### HgncId

Type: string.

### HgvsText

Type: string.

### LeafRef

One of:

- object
- object

### NcbiGeneId

Type: string.

### OncokbLevelCode

Source-defined OncoKB vocabulary.

Values: `level_1`, `level_2`, `level_3a`, `level_3b`, `level_4`, `level_r1`, `level_r2`, `level_dx1`, `level_dx2`, `level_dx3`, `level_px1`, `level_px2`, `level_px3`.

### OncokbOncogenicityCode

Source-defined OncoKB vocabulary.

Values: `oncogenic`, `likely_oncogenic`, `likely_neutral`, `inconclusive`, `resistance`, `unknown`.

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

### Predicate

VA-Spec 1.1 proposition relationships in native snake_case.

Values: `is_causal_for`, `is_oncogenic_for`, `has_clinical_significance_for`, `predicts_sensitivity_to`, `predicts_resistance_to`, `is_diagnostic_inclusion_criterion_for`, `is_diagnostic_exclusion_criterion_for`, `associated_with_better_outcome_for`, `associated_with_worse_outcome_for`, `impacts_function_of`.

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

### VariantId

Type: string.

### VariantInput

Type: string.

### VariantRef

Checked source subject reference; at least one source identity or label is required.

One of:

- object: Exact allele identifier or reference-qualified notation.
- object: Protein-level assertion, without genomic conversion.
- object: Compound source profile, without an inferred allele.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.variant_assertions/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
