---
title: biomcp.drug_civic/4
description: "The BioMCP 1.0.0 reply schema biomcp.drug_civic/4: its envelope
  fields, its result type DrugCivic and the commands that return it."
goal: Look up the fields of a biomcp.drug_civic/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_civic/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.drug_civic/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.drug_civic/4`.

This call returns it.

- `biomcp get drug 2033 civic --source civic` ([`biomcp get drug`](/reference/commands/get-drug/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [DrugGet](#query-drugget) | yes | Parsed query. |
| `result` | [DrugCivic](#result-drugcivic) or null | no | Answer when available. |
| `schema` | "biomcp.drug_civic/4" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: DrugCivic

Complete CIViC therapy/profile context with independent native claim continuations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic` | [Section](#section) | yes | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `identity` | [DrugHeader](#drugheader) | yes | Selected native identity, checked independently of section facts. |

## Query: DrugGet

Exact native ID retrieval with an explicitly selected source.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic_cursor` | [Text](#text) or null | no | Native continuation cursor for that exact drug, profile and claim collection. |
| `civic_kind` | [CivicClaimKind](#civicclaimkind) or null | no | Independent native evidence or assertion collection within the selected profile. |
| `civic_profile` | integer or null | no | Selected native CIViC molecular profile for a drug-scoped claim stream. |
| `input` | [DrugId](#drugid) | yes | Published source-native identifier, never SOURCE:ID or a selected name. |
| `label_section` | [Text](#text) or null | no | Exact native label heading, required only for the label section. |
| `product_number` | [Text](#text) or null | no | Explicit native application product number, only for approvals. |
| `section` | array of [DrugSection](#drugsection) | no | Explicit named disclosures; empty means identity only. |
| `section_limit` | integer or null | no | Named section page size, refused on an identity-only card. |
| `section_offset` | integer or null | no | Named section continuation, refused on an identity-only card. |
| `source` | [DrugSource](#drugsource) | yes | Required source; there is no default exact-ID resolver. |

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
| `next` | [DrugGet](#query-drugget) or null | no | Continue the same source candidate, profile and claim stream. |
| `returned` | integer (uint32) | yes | Number of native rows displayed in this stream. |
| `total` | integer (uint64) | yes | Native total for this stream. |

### CivicEvidenceLevelCode

Source-defined CIViC evidence levels.

Values: `a`, `b`, `c`, `d`, `e`.

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

### DrugCivicClaim

One native claim with its complete source therapy set and molecular profile.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `claim` | [CivicStatement](#civicstatement) | yes | Existing source claim mapping, including unmapped native facts. |
| `oncokb_lookup` | [DrugOncokbLookup](#drugoncokblookup) or null | no | Scoped lookup of a source-listed genomic description, with no evidence transfer. |
| `oncokb_missing` | array of [DrugOncokbMissing](#drugoncokbmissing) | no | Missing source context prevents a keyed variant annotation next command. |
| `profile` | [DrugCivicProfile](#drugcivicprofile) | yes | Source molecular profile context; its member expression is never collapsed. |
| `therapies` | array of [DrugHeader](#drugheader) | yes | Complete native therapy identities, without cross-source equivalence. |
| `therapy_interaction_type` | [Text](#text) or null | no | Native combination interaction label when supplied. |

### DrugCivicMember

A drug claim member with its source-owned build, without altering variant responses.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `feature` | [Text](#text) | yes | Native feature label, including fusion features. |
| `hgvs_descriptions` | array of [Text](#text) | no | Every native notation, without selecting a transcript or repairing a spelling. |
| `label` | [Text](#text) | yes | Whole source variant label. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native variant identity and link. |
| `reference_build` | [Text](#text) or null | no | Native coordinates.referenceBuild spelling, including unsupported builds. |
| `subject` | [VariantRef](#variantref) or null | no | Source-qualified subject when supported by the common model. |

### DrugCivicPage

One native claim stream and its own continuation.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `page` | [CivicClaimPage](#civicclaimpage) | yes | Stream totals and continuation, independent of every other stream. |
| `records` | array of [DrugCivicClaim](#drugcivicclaim) | yes | Native claim order; each row carries the profile to which that claim belongs. |

### DrugCivicProfile

Profile context embedded in a therapy claim, with no invented profile totals.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `compound` | boolean | yes | Source complex-profile flag. |
| `label` | [Text](#text) | yes | Complete source expression. |
| `members` | array of [DrugCivicMember](#drugcivicmember) | yes | Every source member, including members without a common-model subject. |
| `record` | [SourceRecord](#sourcerecord) | yes | Native profile identity and link. |

### DrugCivicStreams

Independent accepted therapy claim collections, without profile count inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `assertions` | [DrugCivicPage](#drugcivicpage) or null | no | Assertions filtered independently by the selected native therapy and optional profile. |
| `evidence` | [DrugCivicPage](#drugcivicpage) or null | no | Evidence items filtered by the selected native therapy and optional profile. |

### DrugHeader

One source candidate, without authorization, targets, label text or interactions.

| Field | Type | Required | Description |
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

### DrugIdentityLevel

Identity level established by the source endpoint or record type, never inferred from ID shape.

Values: `ingredient`, `salt`, `clinical_formulation`, `branded_product`, `spl_set`, `spl_version`, `ndc_package`, `fda_application`, `fda_product`, `ema_authorization`, `ema_application`, `orphan_designation`, `unknown`.

### DrugOncokbLookup

Link one source member to a checked variant lookup in its claim's tumor context.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `label` | [Text](#text) | yes | Scope visible alongside the generated command. |
| `member` | [Text](#text) | yes | Native CIViC member ID to which the genomic description belongs. |
| `query` | [VariantGet](#variantget) | yes | Checked owning-entity request; input preserves the selected source HGVS. |

### DrugOncokbMissing

Context absent or insufficient for an exact source-owned annotation command.

Values: `gene`, `build`, `variant`, `tumor_type`.

### DrugSection

Explicit drug disclosures; identity cards never expand into these sections.

Values: `xrefs`, `targets`, `indications`, `civic`, `label`, `approvals`, `regulatory`, `interactions`.

### DrugSource

Values: `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter`.

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

### Section

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

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

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

### VariantResolver

Explicit source service for reference-qualified notation.

Values: `car`, `mutalyzer`, `variantvalidator`.

### VariantSection

Named variant source sections, without automatic expansion of identity cards.

Values: `clinvar`, `erepo`, `population`, `gwas`, `civic`, `oncokb`, `hotspots`, `predict`, `conservation`, `cosmic`, `all`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.drug_civic/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
