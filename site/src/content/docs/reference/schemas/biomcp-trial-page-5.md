---
title: biomcp.trial_page/5
description: "The BioMCP 1.0.0 reply schema biomcp.trial_page/5: its envelope
  fields, its result type TrialPage and the commands that return it."
goal: Look up the fields of a biomcp.trial_page/5 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial_page/5
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.trial_page/5`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.trial_page/5`.

These commands return it: [`biomcp search trial`](/reference/commands/search-trial/).

Calls that returned it:

- `biomcp search trial --condition 'non-small cell lung cancer' --variant 'KRAS G12C' --status RECRUITING --limit 3` ([`biomcp search trial`](/reference/commands/search-trial/))
- `biomcp search trial --condition melanoma --has-results --count-only` ([`biomcp search trial`](/reference/commands/search-trial/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [TrialSearch](#query-trialsearch) | yes | Parsed query. |
| `result` | [TrialPage](#result-trialpage) or null | no | Answer when available. |
| `schema` | "biomcp.trial_page/5" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: TrialPage

Registry page preserving source order and exact request parameters.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | string | yes | Publisher attribution. |
| `condition_resolution` | [TrialConditionResolution](#trialconditionresolution) or null | no | Exact condition identity and source-provided mapping, absent for literal text. |
| `count_request` | array or null | no | Exact cached native count request when a continuation omits its total. |
| `gene_resolution` | [GeneResolution](#generesolution) or null | no | Unresolved gene identity, without a registry request. |
| `items` | array of [TrialHeader](#trialheader) | yes | Native study headers in registry order. |
| `request` | array of [TrialParameter](#trialparameter) | yes | Exact parameters sent to the study search endpoint. |
| `source_updated` | [Text](#text) or null | no | Registry's source processing timestamp, independently of retrieval time. |

## Query: TrialSearch

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

| Field | Type | Required | Description |
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

## Types

Every other type this schema uses.

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

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [Text](#text) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Chromosome

Type: string.

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

### Concept

One source concept with its optional ontology identifier.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) or null | no | Source-stated concept identifier. |
| `label` | [Text](#text) | yes | Original concept label. |

### Curie

Type: string.

### Cytoband

Type: string.

### DiseaseEndpoint

One native endpoint with an identifier and an optional source label.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [Curie](#curie) | yes | Exact source identifier. |
| `label` | [Text](#text) or null | no | Source label when supplied. |

### DiseaseRelation

A publisher relation with distinct endpoints and predicate.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Publisher record and version. |
| `description` | string or null | no | Unchanged native cross-reference description when supplied. |
| `object` | [DiseaseEndpoint](#diseaseendpoint) | yes | Native object. |
| `predicate` | [Curie](#curie) | yes | Publisher predicate, without equivalence promotion. |
| `subject` | [DiseaseEndpoint](#diseaseendpoint) | yes | Native subject. |

### EnsemblGeneId

Type: string.

### Gene

Human gene identity and whole source annotations, independent of response cards.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `aliases` | array of [Text](#text) | yes | All stated HGNC aliases. |
| `ensembl_id` | [EnsemblGeneId](#ensemblgeneid) or null | no | Ensembl gene identifier, if stated. |
| `entrez_id` | [NcbiGeneId](#ncbigeneid) or null | no | NCBI Gene identifier, if stated. |
| `hgnc_id` | [HgncId](#hgncid) | yes | HGNC identifier. |
| `location` | [Cytoband](#cytoband) or null | no | Single stated band, if available. |
| `locus_group` | [LocusGroup](#locusgroup) | yes | HGNC locus group. |
| `locus_type` | [LocusType](#locustype) or null | no | HGNC locus type, if stated and recognized. |
| `mane` | array of [Mane](#mane) | no | Versioned MANE Select transcript pair, if stated. |
| `name` | [Text](#text) | yes | Gene name. |
| `omim` | array of [OmimId](#omimid) | yes | All stated OMIM identifiers. |
| `previous_symbols` | array of [GeneSymbol](#genesymbol) | yes | All stated previous symbols. |
| `spans` | array of [GenomicInterval](#genomicinterval) | yes | Source intervals with explicit assembly and counting rule. |
| `status` | [GeneStatus](#genestatus) | yes | HGNC record status. |
| `summary` | [Summary](#summary) or null | no | Whole attributed summary, if stated. |
| `symbol` | [GeneSymbol](#genesymbol) | yes | Source-preserved symbol. |
| `uniprot` | array of [UniProtAcc](#uniprotacc) | yes | All stated UniProt base accessions. |

### GeneHeader

Enough gene identity to choose a search result.

| Field | Type | Required | Description |
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

### GeneResolution

Resolution outcomes preserve ambiguity and withdrawn identity.

One of:

- object: One resolved identity and its source match.
- object: Every candidate, with no silent selection.
- object: The original withdrawn identity and stated successors.
- object: No source identity matched.

### GeneStatus

HGNC approval status.

Values: `approved`, `withdrawn`.

### GeneSymbol

Type: string.

### GenomicInterval

An assembly-qualified zero-based half-open source interval.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `build` | [Build](#build) or null | no | Source-stated assembly, omitted when absent. |
| `chromosome` | [Chromosome](#chromosome) | yes | Canonical primary chromosome. |
| `span` | [Span0](#span0) | yes | Checked zero-based half-open bounds; an empty span remains valid. |
| `strand` | [Strand](#strand) or null | no | Source-stated strand, if available. |

### HgncId

Type: string.

### HgncStatus

HGNC's stated withdrawn-file status.

Values: `entry_withdrawn`, `merged_split`.

### LocusGroup

HGNC locus group, separate from a missing source value.

Values: `protein_coding`, `non_coding_rna`, `pseudogene`, `phenotype`, `other`.

### LocusType

HGNC locus type; Unknown is a source-stated value, distinct from absence.

Values: `gene_with_protein_product`, `rna_cluster`, `long_non_coding_rna`, `micro_rna`, `misc_rna`, `ribosomal_rna`, `small_nuclear_rna`, `small_nucleolar_rna`, `transfer_rna`, `vault_rna`, `y_rna`, `phenotype_only`, `pseudogene`, `complex_locus_constituent`, `endogenous_retrovirus`, `fragile_site`, `immunoglobulin_gene`, `immunoglobulin_pseudogene`, `readthrough`, `region`, `t_cell_receptor_gene`, `t_cell_receptor_pseudogene`, `transposable_element`, `unknown`, `virus_integration_site`.

### Mane

A source-stated MANE pair with versioned sequence accessions.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ensembl` | string | yes | Versioned Ensembl accession. |
| `refseq` | string | yes | Versioned RefSeq accession. |
| `status` | [ManeStatus](#manestatus) | yes | Source-stated transcript status. |

### ManeStatus

The source-stated MANE transcript status.

Values: `select`, `plus_clinical`.

### Match

A source-stated match, preserving the text that matched.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | [MatchKind](#matchkind) | yes | The match category. |
| `text` | [Text](#text) | yes | The source text that matched the input. |

### MatchKind

How a source matched a gene input.

Values: `symbol`, `previous_symbol`, `alias`, `ncbi_alias`, `hgnc_id`, `entrez_id`, `ensembl_id`, `band`.

### NcbiGeneId

Type: string.

### NctId

Type: string.

### OmimId

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

### PartialDate

Type: string.

### PhaseFilter

Explicit source phase selection, including the registry's combined phases.

Values: `early-1`, `1`, `2`, `3`, `4`, `not-applicable`, `1/2`, `2/3`.

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

### Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | integer (uint32) | yes |  |
| `start` | integer (uint32) | yes |  |

### SponsorType

Registry funding category facet.

Values: `nih`, `fed`, `industry`, `other`.

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

### Strand

Source-stated forward or reverse strand.

Values: `+`, `-`.

### StudyType

Source-native registry value.

Values: `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS`.

### Summary

A whole source-provided summary with attribution and stated date precision.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `dated` | [PartialDate](#partialdate) or null | no | Date stated by the source, if any. |
| `source` | [SummarySource](#summarysource) | yes | The summary's attribution, independent of the fetching provider. |
| `text` | [Text](#text) | yes | Entire summary text. |

### SummarySource

The attribution on a source-provided summary, independent of fetch providers.

Values: `refseq`.

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### TrialConditionResolution

Exact condition identity used for one trial request, without eligibility inference.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alias` | [DiseaseRelation](#diseaserelation) or null | no | Native relation proving a requested alias, when required. |
| `attribution` | [Attribution](#attribution) or null | no | Resolver record provenance, independent of the trial provider. |
| `components` | array of [ComponentLabel](#componentlabel) | no | Admitted resolver components and their own terms. |
| `concept` | [Concept](#concept) or null | no | Returned concept with its unchanged native primary label. |
| `input` | [Curie](#curie) | yes | Caller-selected CURIE, never a source-name search. |
| `ncit_candidates` | array of [DiseaseRelation](#diseaserelation) | no | Native NCIt mapping rows; duplicate targets keep their independent provenance. |
| `resolver` | [Source](#source) or null | no | Named exact resolver; absent for explicit NCIt input. |
| `selected_ncit` | [Curie](#curie) or null | no | Explicit or unique source-provided NCIt identifier actually sent to NCI. |

### TrialHeader

Small source-attributed trial discovery record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `conditions` | [Capped](#capped) | yes | Source condition names. |
| `interventions` | [Capped](#capped) | yes | Source intervention names. |
| `nct_id` | [NctId](#nctid) | yes | Exact registry identity. |
| `phases` | array of [TrialPhase](#trialphase) | yes | Source phases; absent phases remain absent. |
| `status` | [TrialStatus](#trialstatus) | yes | Registry's overall status, independent of site status. |
| `title` | [Text](#text) | yes | Complete brief title. |
| `updated` | [PartialDate](#partialdate) | yes | Last posted registry update. |
| `url` | [Text](#text) | yes | Publisher's canonical record URL. |

### TrialParameter

One exact source request parameter, in canonical name order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | string | yes | Native parameter name. |
| `value` | string | yes | Exact unencoded parameter value. |

### TrialPhase

Source-native registry value.

Values: `EARLY_PHASE1`, `PHASE1`, `PHASE2`, `PHASE3`, `PHASE4`, `NA`.

### TrialSex

Source-native registry value.

Values: `ALL`, `FEMALE`, `MALE`.

### TrialSource

Independent trial providers; results are never folded or used as fallbacks.

Values: `ctgov`, `nci`.

### TrialStatus

Source-native registry value.

Values: `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN`.

### UniProtAcc

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.trial_page/5` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
