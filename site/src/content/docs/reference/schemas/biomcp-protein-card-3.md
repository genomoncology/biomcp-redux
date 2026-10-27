---
title: biomcp.protein_card/3
description: "The BioMCP 1.0.0 reply schema biomcp.protein_card/3: its envelope
  fields, its result type ProteinCard and the commands that return it."
goal: Look up the fields of a biomcp.protein_card/3 reply.
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 reply schema `biomcp.protein_card/3`. Every reply names its schema in `schema`. Print this one as JSON Schema with `biomcp schema biomcp.protein_card/3`.

These commands return it: [`biomcp get protein`](/reference/commands/get-protein/).

Calls that returned it:

- `biomcp get protein P15056` ([`biomcp get protein`](/reference/commands/get-protein/))
- `biomcp get protein P00533 Q9UIF7 P38398-2` ([`biomcp get protein`](/reference/commands/get-protein/))

## Envelope

One versioned command response; schema identity comes only from its body type.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `local_error` | [CacheProblem](#cacheproblem) or null | no | Bounded local cache failure, independent of provider outcomes. |
| `next` | array of [Step](#step) | no | Available next commands. |
| `page` | [PageInfo](#pageinfo) or null | no | Search page metadata. |
| `problems` | array of [Problem](#problem) | no | Rejected source fields. |
| `query` | [ProteinGet](#query-proteinget) | yes | Parsed query. |
| `result` | [ProteinCard](#result-proteincard) or null | no | Answer when available. |
| `schema` | "biomcp.protein_card/3" | yes |  |
| `sources` | array of [SourceOutcome](#sourceoutcome) | no | Sources actually asked. |
| `status` | [Status](#status) | yes | Overall outcome. |

## Result: ProteinCard

Answers to requested inputs; ambiguous gene inputs retain all candidates.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinCardItem](#proteincarditem) | no | One result per input in caller order. |

## Query: ProteinGet

Read source protein cards; a symbol returns candidates without choosing a protein.
Sections disclose native facts with independent row bounds; function keeps all paragraphs.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | array of [ProteinInput](#proteininput) | yes | Source accessions or gene symbols, preserving caller order. |
| `section` | array of [ProteinSection](#proteinsection) | no | Select native sections or all bounded first pages; function keeps one whole text record. |
| `section_limit` | integer (uint32) | no | First-page row limit; function keeps one whole native text record and does not page its paragraphs. |

## Types

Every other type this schema uses.

### AcmgAmp2015Code

Checked AcmgAmp2015 codes.

Values: `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign`.

### AminoAcids

Type: string.

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

### Bound

A bounded source range; equal min and max denote an exact boundary.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `max` | integer or null | no |  |
| `min` | integer or null | no |  |

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

### ComplexPortalId

Type: string.

### Curie

Type: string.

### InterProId

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
| `accession` | [UniProtIsoform](#uniprotisoform) | yes | Checked primary accession. |
| `function` | array of [Text](#text) | no | Whole source function paragraphs, omitted when absent. |
| `length` | integer (uint32) | yes | Sequence length in amino acids. |
| `name` | [Text](#text) | yes | Recommended protein name. |
| `subcellular_locations` | array of [Text](#text) | no | Source subcellular location names. |

### ProteinAudit

Native source audit metadata, independent from local retrieval metadata.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `entry_version` | integer or null | no | Native entry version, never an isoform version. |
| `first_public` | [PartialDate](#partialdate) or null | no | Native first-public date. |
| `last_annotation_update` | [PartialDate](#partialdate) or null | no | Native annotation update date. |
| `last_sequence_update` | [PartialDate](#partialdate) or null | no | Native sequence update date. |
| `release` | [Text](#text) or null | no | Native release when supplied. |
| `sequence_version` | integer or null | no | Native sequence version when supplied. |

### ProteinCardItem

One requested identity and its source outcome, without first-hit selection.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `audit` | [ProteinAudit](#proteinaudit) or null | no | Native audit for the selected entry only. |
| `candidates` | array of [ProteinHeader](#proteinheader) | no | Competing exact source identities when a gene does not resolve uniquely. |
| `header` | [ProteinHeader](#proteinheader) or null | no | Exact resolved native header; absent for ambiguity or failure. |
| `input` | [ProteinInput](#proteininput) | yes | Original caller identity. |
| `more` | array of [ProteinSection](#proteinsection) | no | Supported sections not requested. |
| `sections` | [ProteinSections](#proteinsections) or null | no | Requested bounded sections only. |
| `status` | [Status](#status) | yes | Outcome of this input. |

### ProteinComplexHeader

Exact source complex header admitted by a native participant identity.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | [ComplexPortalId](#complexportalid) | yes | Native complex accession. |
| `matched_participant` | [Text](#text) | yes | Exact native participant identifier supporting admission. |
| `name` | [Text](#text) | yes | Native source name. |
| `participant_total` | integer (uint64) | yes | Actual participants in this exact complex, never a complex search total. |
| `predicted` | boolean | yes | Literal native predictedComplex flag. |
| `taxon` | integer (uint32) | yes | Source taxonomy identifier. |
| `url` | [Text](#text) | yes | Native complex record URL. |

### ProteinComplexes

Whole native rows with the independent unfiltered source count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinComplexHeader](#proteincomplexheader) | yes | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | yes | Native source total before type selection or exact participant admission. |

### ProteinCoordinateConvention

Coordinate convention on the native feature before conversion.

Values: `one_based_inclusive`.

### ProteinDomainFragment

One native match fragment after coordinate conversion.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `dc_status` | [Text](#text) or null | no | Literal discontinuity status supplied by InterPro. |
| `span` | [UncertainSpan0](#uncertainspan0) | yes | Converted interbase span with uncertainty retained. |

### ProteinDomainLocation

One native InterPro location group retaining all discontinuous fragments.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fragments` | array of [ProteinDomainFragment](#proteindomainfragment) | no | All native fragments; never reduced to a min/max hull. |

### ProteinDomainMatch

One native integrated InterPro match; not every entry type is a domain.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source record and release facts. |
| `entry_type` | [Text](#text) | yes | Literal source entry type, including family, site and repeat. |
| `id` | [InterProId](#interproid) | yes | Integrated entry accession, distinct from member signatures. |
| `locations` | array of [ProteinDomainLocation](#proteindomainlocation) | no | Every native location group in source order. |
| `member_signatures` | array of [ProteinMemberSignature](#proteinmembersignature) | no | Source member signatures remain distinct from integrated entries. |
| `name` | [Text](#text) | yes | Native integrated name. |
| `protein` | [UniProtIsoform](#uniprotisoform) | yes | Exact native protein identity. |
| `protein_length` | integer or null | no | Source protein length when supplied. |
| `taxon` | integer or null | no | Source taxon when supplied. |

### ProteinDomains

Whole native rows with the independent unfiltered source count.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinDomainMatch](#proteindomainmatch) | yes | Bounded retained rows in source order. |
| `native_total` | integer (uint64) | yes | Native source total before type selection or exact participant admission. |

### ProteinFeature

UniProt feature with source attribution and coordinate provenance.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `attribution` | [Attribution](#attribution) | yes | Source record and release facts. |
| `feature` | [UniProtFeature](#uniprotfeature) | yes | Shared feature, including native label and residues. |
| `source_coordinates` | [ProteinCoordinateConvention](#proteincoordinateconvention) | yes | Convention before mathematical conversion. |

### ProteinFeatures

Native UniProt feature rows, preserving exact labels and uncertain bounds.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinFeature](#proteinfeature) | yes | Whole native rows in source order; totals apply only to this collection. |

### ProteinFunction

One whole source function paragraph and its own evidence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `evidence` | array of [Text](#text) | no | All evidence references supplied for this text. |
| `molecule` | [Text](#text) or null | no | Native molecule qualifier; never attributed to another isoform. |
| `text` | [Text](#text) | yes | Whole native source text, without clinical interpretation. |

### ProteinFunctionResult

Whole source function paragraphs and their own evidence.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinFunction](#proteinfunction) | no | Native paragraphs in source order, without inferred interpretation. |

### ProteinHeader

Native entry metadata wrapping the same Protein identity used by the gene slice.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `entry_id` | [Text](#text) | yes | Native source entry name. |
| `gene_ids` | array of [Curie](#curie) | no | Only identifiers explicitly supplied by the source. |
| `gene_names` | array of [Text](#text) | no | Display names only; never identifier joins. |
| `protein` | [Protein](#protein) | yes | Shared gene/protein identity fields; never independently restated. |
| `reviewed` | boolean | yes | Native reviewed status. |
| `taxon` | integer (uint32) | yes | Native source taxonomy identifier. |
| `url` | [Text](#text) | yes | Source entry URL. |

### ProteinInput

Exact source accession or a checked display gene symbol needing later resolution.
The schema describes the string surface; deserialization checks either identity grammar.

Type: string.

### ProteinIsoform

One source alternative-product row; all stated identifiers stay together.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `direct_sequence` | [ProteinSequenceAvailability](#proteinsequenceavailability) | yes | Explicit direct-sequence retrieval state, independent of descriptive status. |
| `ids` | array of [UniProtIsoform](#uniprotisoform) | no | All source-stated isoform identifiers, including External base identities. |
| `name` | [Text](#text) | yes | Source isoform name. |
| `sequence_ids` | array of [Text](#text) | no | Source VSP sequence identifiers. |
| `sequence_status` | [ProteinIsoformStatus](#proteinisoformstatus) | yes | Native sequence status; no sequence is inferred from it. |
| `synonyms` | array of [Text](#text) | no | Every source synonym. |

### ProteinIsoformStatus

Native UniProt alternative-product sequence status.

Values: `Displayed`, `Described`, `External`.

### ProteinIsoforms

Alternative products with every stated identifier and explicit direct sequence state.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinIsoform](#proteinisoform) | yes | Whole native rows in source order; totals apply only to this collection. |

### ProteinMemberSignature

A member database signature, not an integrated IPR entry.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `accession` | [Text](#text) | yes | Native signature accession. |
| `database` | [Text](#text) | yes | Native member database name. |
| `name` | [Text](#text) or null | no | Native member signature name. |

### ProteinSection

Named protein sections; all expands to bounded first pages.

Values: `features`, `domains`, `isoforms`, `complexes`, `structures`, `function`, `sequence`, `all`.

### ProteinSections

Independent source section states; all discloses bounded first pages only.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `complexes` | [Section4](#section4) or null | no | Requested exact participant-admitted header page. |
| `domains` | [Section2](#section2) or null | no | Requested InterPro match page. |
| `features` | [Section](#section) or null | no | Requested UniProt feature page. |
| `function` | [Section6](#section6) or null | no | Requested whole native function text and evidence. |
| `isoforms` | [Section3](#section3) or null | no | Requested native alternative-product page. |
| `sequence` | [Section7](#section7) or null | no | Requested direct sequence; absent/unavailable states remain explicit. |
| `structures` | [Section5](#section5) or null | no | Requested structure-link page. |

### ProteinSequence

A directly returned sequence with exact returned identity and audit facts.

| Field | Type | Required | Description |
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

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `sequence` | [ProteinSequence](#proteinsequence) | yes | Native sequence record; never constructed from another entry. |

### ProteinStructureProperty

A native cross-reference property pair.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `key` | [Text](#text) | yes | Native property key. |
| `value` | [Text](#text) | yes | Native property value, without interpreting residue coverage. |

### ProteinStructureRef

A source structure cross-reference link; no residue coverage claim.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `database` | [Text](#text) | yes | Native cross-reference database. |
| `id` | [Text](#text) | yes | Native cross-reference identifier. |
| `properties` | array of [ProteinStructureProperty](#proteinstructureproperty) | no | All native property pairs, including method, resolution and chains. |
| `url` | [Text](#text) | yes | Publisher link; no structure file is retrieved. |

### ProteinStructures

Source structure links and native properties; no residue coverage claim.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | array of [ProteinStructureRef](#proteinstructureref) | yes | Whole native rows in source order; totals apply only to this collection. |

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

### Term

A source label with optional explicitly supported coding.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `coding` | [Coding](#coding) or null | no | Documented framework coding. |
| `label` | [Text](#text) | yes | Original label. |

### Text

Nonempty text with surrounding Unicode whitespace removed.

Type: string.

### UncertainSpan0

Zero-based interbase span retaining uncertain boundary ranges.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `end` | [Bound](#bound) | yes |  |
| `start` | [Bound](#bound) | yes |  |

### UniProtFeature

Shared native feature contract without source label remapping.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `alternative` | array of [AminoAcids](#aminoacids) | no | Every stated alternative, without translation. |
| `description` | [Text](#text) or null | no | Complete source description. |
| `evidence` | array of [Text](#text) | no | All original source evidence references. |
| `id` | [Text](#text) or null | no | Native feature identifier. |
| `kind` | [Term](#term) | yes | Source feature label; coding only for documented exact feature types. |
| `original` | [AminoAcids](#aminoacids) or null | no | Source original residues, when supplied. |
| `protein` | [UniProtIsoform](#uniprotisoform) | yes | Exact source sequence identity. |
| `span` | [UncertainSpan0](#uncertainspan0) | yes | Zero-based interbase coordinates retaining uncertain ranges. |

### UniProtIsoform

Type: string.

### Use

Reviewed source-use category, without inferring rights in individual records.

Values: `open`, `attribution`, `noncommercial`, `restricted`.

[Reply envelope](/reference/envelope/) gives the values of `status`, `sources[].state` and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.

This page is generated from `biomcp schema biomcp.protein_card/3` in BioMCP 1.0.0. Fix a wrong description in the schema, then regenerate.
