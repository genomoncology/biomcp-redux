---
title: biomcp.gene_cards/4
description: "The BioMCP 1.0.0 reply schema biomcp.gene_cards/4: its envelope
  fields, its result type GeneCards and the commands that return it."
goal: Look up the fields of a biomcp.gene_cards/4 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene_cards/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.gene_cards/4`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.gene_cards/4`.

These commands return it: [`biomcp get gene`](/reference/commands/get-gene/).

Calls that returned it:

- `biomcp get gene TP53 constraint` ([`biomcp get gene`](/reference/commands/get-gene/))
- `biomcp --disable-sources noncommercial,restricted get gene BRAF`

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [GeneGet](#query-geneget) | yes | Parsed query. |
| `result` | [GeneCards](#result-genecards) or null | no | Answer when available. |
| `schema` | "biomcp.gene_cards/4" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: GeneCards

Gene answers in input order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [GeneItem](#geneitem) | yes | Requested inputs and their answers. |

## Query: GeneGet

Get one to twenty human gene inputs in their original order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of [GeneInput](#geneinput) | yes | Gene symbols or checked identifiers, in input order. |
| `section` | array of [GeneSectionWord](#genesectionword) | no | Requested section words, preserving caller order and repetitions. Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs. |

## Types

Every other type this schema uses.

### AccessionBase

Type: string.

### Build

The assembly stated by the source; no default is inferred.

Values: `GRCh37`, `GRCh38`.

### CacheProblem

A local cache failure that leaves the provider outcome intact.

Values: `busy`, `database`, `version`.

### CancerPtpm

HPA cancer RNA abundance in protein-coding transcripts per million.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ptpm` | [RnaAbundance](#rnaabundance) | yes | Finite nonnegative pTPM. |
| `tissue` | [Text](#text) | yes | Exact source cancer tissue name. |

### Capped

A bounded list with the total before its cap was applied.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [PathwayRef](#pathwayref) | yes | References in stable source order. |
| `total` | integer (uint64) | yes | Distinct references before capping. |

### Chromosome

Type: string.

### CivicGene

CIViC's gene-level entry without evidence or variant lists.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `description` | [Text](#text) or null | no | Complete source description, absent when the source has none. |
| `id` | integer (uint64) | yes | CIViC feature identifier. |
| `url` | [Text](#text) | yes | Link to the source entry. |
| `variant_count` | integer (uint64) | yes | Number of curated variants reported by the source. |

### ClinGenGene

ClinGen gene validity and dosage assessments in published source order.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `dosage_date` | [Text](#text) or null | no | Source dosage assessment date, preserving its original spelling. |
| `haploinsufficiency` | [Text](#text) or null | no | Source haploinsufficiency assessment, when a dosage record exists. |
| `triplosensitivity` | [Text](#text) or null | no | Source triplosensitivity assessment, when a dosage record exists. |
| `validity` | array of [ClinGenValidity](#clingenvalidity) | yes | Whole gene–disease validity records in file order. |

### ClinGenValidity

One published ClinGen gene–disease validity record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `classification` | [Text](#text) | yes | Source classification without inference. |
| `classification_date` | [Text](#text) | yes | Source classification date, preserving its original spelling. |
| `disease` | [Text](#text) | yes | Source disease label. |
| `expert_panel` | [Text](#text) | yes | Source expert panel. |
| `mode_of_inheritance` | [Text](#text) | yes | Source mode of inheritance. |
| `mondo_id` | [Text](#text) | yes | Source MONDO identifier. |
| `report_url` | [Text](#text) | yes | Source report URL. |

### Constraint

Source constraint metrics for the source's preferred transcript.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `flags` | array of [Text](#text) | no | Source flags, omitted when empty. |
| `loeuf` | [FiniteNumber](#finitenumber) | yes | Upper confidence bound of the observed-to-expected loss-of-function ratio. |
| `mis_z` | [FiniteNumber](#finitenumber) | yes | Missense Z score. |
| `oe_lof` | [FiniteNumber](#finitenumber) | yes | Observed-to-expected loss-of-function ratio. |
| `pli` | [FiniteNumber](#finitenumber) | yes | Probability of loss-of-function intolerance. |
| `syn_z` | [FiniteNumber](#finitenumber) | yes | Synonymous Z score. |
| `transcript` | [AccessionBase](#accessionbase) | yes | Source MANE Select transcript, otherwise the source canonical transcript. |
| `version` | [Text](#text) | yes | Source dataset family, not a minor release identifier. |

### Cytoband

Type: string.

### EnsemblGeneId

Type: string.

### FiniteNumber

A finite decimal that JSON can represent without losing its value to null.

Type: number (double).

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

### GeneExpression

Gene-level HPA expression and subcellular annotations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `rna_cancer_ptpm` | array of [CancerPtpm](#cancerptpm) | no | Cancer-specific pTPM pairs. |
| `rna_cancer_specificity` | [Text](#text) or null | no | Source RNA cancer specificity. |
| `rna_tissue_distribution` | [Text](#text) or null | no | Source RNA tissue distribution. |
| `rna_tissue_ntpm` | array of [TissueNtpm](#tissuentpm) | no | Tissue-specific nTPM pairs. |
| `rna_tissue_specificity` | [Text](#text) or null | no | Source RNA tissue specificity. |
| `subcellular_additional` | array of [Text](#text) | no | Source additional subcellular locations. |
| `subcellular_main` | array of [Text](#text) | no | Source main subcellular locations. |

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

### GeneItem

Answer to one requested input.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `answer` | [GeneResolution](#generesolution) or null | no | Absent only when identity lookup failed. |
| `input` | [GeneInput](#geneinput) | yes | Original checked input. |
| `more` | array of [GeneSection](#genesection) | no | Applicable sections left out of this request. |
| `oncokb` | [OncokbCard](#oncokbcard) or null | no | Verified keyed source card annotation. |
| `sections` | [GeneSections](#genesections) or null | no | Requested sections, present only for a resolved identity. |
| `status` | [Status](#status) | yes | This input's outcome. |

### GeneResolution

Resolution outcomes preserve ambiguity and withdrawn identity.

One of:

- object: One resolved identity and its source match.
- object: Every candidate, with no silent selection.
- object: The original withdrawn identity and stated successors.
- object: No source identity matched.

### GeneSection

A landed section that can be requested for a gene.

Values: `protein`, `civic`, `clingen`, `constraint`, `expression`, `pathways`, `oncokb`.

### GeneSectionWord

A section request word, including expansion to all landed sections.

Values: `protein`, `civic`, `clingen`, `constraint`, `expression`, `pathways`, `oncokb`, `all`.

### GeneSections

Landed gene sections, each retaining its source and explicit state.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `civic` | [Section2](#section2) or null | no | CIViC gene-level curation. |
| `clingen` | [Section3](#section3) or null | no | ClinGen validity and dosage assessments. |
| `constraint` | [Section4](#section4) or null | no | Population constraint reported by gnomAD. |
| `expression` | [Section5](#section5) or null | no | Human Protein Atlas expression annotations. |
| `oncokb` | [Section7](#section7) or null | no | Keyed source gene details. |
| `pathways` | [Section6](#section6) or null | no | Lowest-level human Reactome mappings. |
| `protein` | [Section](#section) or null | no | Reviewed human proteins listed by HGNC. |

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

### OmimId

Type: string.

### Oncokb

Whole OncoKB gene prose and independent transcript annotations.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `background` | [Text](#text) or null | no | Whole source background. |
| `grch37` | [OncokbTranscript](#oncokbtranscript) | yes | GRCh37 source transcripts. |
| `grch38` | [OncokbTranscript](#oncokbtranscript) | yes | GRCh38 source transcripts. |
| `summary` | [Text](#text) or null | no | Whole source summary. |
| `version` | [Text](#text) or null | no | Verified data release version. |

### OncokbCard

Short source-stated annotation on a gene card.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `gene_type` | [OncokbGeneType](#oncokbgenetype) | yes | Source gene classification. |
| `highest_resistance_level` | [Text](#text) or null | no | Highest resistance level, as stated by the source. |
| `highest_sensitive_level` | [Text](#text) or null | no | Highest sensitive level, as stated by the source. |
| `version` | [Text](#text) or null | no | Data release version, when verified. |

### OncokbGeneType

OncoKB's stated gene classification.

Values: `ONCOGENE_AND_TSG`, `ONCOGENE`, `TSG`, `INSUFFICIENT_EVIDENCE`, `NEITHER`.

### OncokbTranscript

Source transcript annotation for one genome build.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `isoform` | [Text](#text) or null | no | Source-stated isoform, kept separate from HGNC MANE. |
| `refseq` | [Text](#text) or null | no | Source-stated RefSeq transcript. |

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

### PathwayRef

A pathway reference without membership or hierarchy data.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [ReactomeId](#reactomeid) | yes | Human stable identifier. |
| `name` | [Text](#text) | yes | Whole source pathway name. |

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

### Protein

A reviewed human protein listed by the authoritative gene record.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [UniProtAcc](#uniprotacc) | yes | Checked primary accession. |
| `function` | array of [Text](#text) | no | Whole source function paragraphs, omitted when absent. |
| `length` | integer (uint32) | yes | Sequence length in amino acids. |
| `name` | [Text](#text) | yes | Recommended protein name. |
| `subcellular_locations` | array of [Text](#text) | no | Source subcellular location names. |

### ReactomeId

Type: string.

### Reason

Why a source could not supply data.

Values: `not_found`, `not_open`, `not_pdf`, `unsupported`, `too_large`, `rate_limited`, `unavailable`, `tls`, `proxy`, `rejected`, `bad_reply`, `deadline_exceeded`, `offline`, `no_key`, `disabled`, `replay_miss`, `syncing`, `no_join_id`.

### RnaAbundance

A finite nonnegative RNA abundance expressed in the field's stated unit.

Type: number (double).

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

### Span0

A zero-based half-open span with ordered bounds; empty spans are valid.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | integer (uint32) | yes |  |
| `start` | integer (uint32) | yes |  |

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

### TissueNtpm

HPA tissue RNA abundance in normalized transcripts per million.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `ntpm` | [RnaAbundance](#rnaabundance) | yes | Finite nonnegative nTPM. |
| `tissue` | [Text](#text) | yes | Exact source tissue name. |

### UniProtAcc

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.gene_cards/4` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
