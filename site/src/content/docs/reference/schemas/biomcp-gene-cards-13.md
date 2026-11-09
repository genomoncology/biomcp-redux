---
title: Reply of biomcp get gene (biomcp.gene-cards/13)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gene: the fields
  of schema biomcp.gene-cards/13."
goal: "Parse the biomcp.gene-cards/13 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene-cards/13
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gene`](/reference/commands/get-gene/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.gene-cards/13`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gene-cards/13` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GeneCards](#reply_genecards). `query` holds [Reply_GeneGet](#reply_geneget). `query` repeats the request as BioMCP read it.

### Reply_GeneCards

Gene answers in input order.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_GeneItem](#reply_geneitem) | Requested inputs and their answers. Optional. |

### Reply_GeneGet

Get one to twenty human gene inputs in their original order.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [Reply_GeneInput](/reference/schemas/types/#reply_geneinput) | Gene symbols or checked identifiers, in input order. |
| `section` | list of [Reply_GeneSectionWord](#reply_genesectionword) | Requested [section words](/concepts/progressive-disclosure/), preserving caller order and repetitions. Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs. Optional. |

### Other types in this reply

33 more types appear only in this reply: [Reply_AccessionBase](#reply_accessionbase), [Reply_CancerPtpm](#reply_cancerptpm), [Reply_Capped_PathwayRef](#reply_capped_pathwayref), [Reply_CivicGene](#reply_civicgene), [Reply_ClinGenGene](#reply_clingengene), [Reply_ClinGenValidity](#reply_clingenvalidity), [Reply_Constraint](#reply_constraint), [Reply_GenccGene](#reply_genccgene), [Reply_GenccSubmission](#reply_genccsubmission), [Reply_GeneExpression](#reply_geneexpression), [Reply_GeneItem](#reply_geneitem), [Reply_GeneSection](#reply_genesection), [Reply_GeneSectionWord](#reply_genesectionword), [Reply_GeneSections](#reply_genesections), [Reply_MondoId](#reply_mondoid), [Reply_Oncokb](#reply_oncokb), [Reply_OncokbCard](#reply_oncokbcard), [Reply_OncokbGeneType](#reply_oncokbgenetype), [Reply_OncokbTranscript](#reply_oncokbtranscript), [Reply_PathwayRef](#reply_pathwayref), [Reply_Protein_UniProtAcc](#reply_protein_uniprotacc), [Reply_RnaAbundance](#reply_rnaabundance), [Reply_RnaUnit](#reply_rnaunit), [Reply_Section_Array_of_Protein_UniProtAcc](#reply_section_array_of_protein_uniprotacc), [Reply_Section_Capped_PathwayRef](#reply_section_capped_pathwayref), [Reply_Section_CivicGene](#reply_section_civicgene), [Reply_Section_ClinGenGene](#reply_section_clingengene), [Reply_Section_Constraint](#reply_section_constraint), [Reply_Section_GenccGene](#reply_section_genccgene), [Reply_Section_GeneExpression](#reply_section_geneexpression), [Reply_Section_Oncokb](#reply_section_oncokb), [Reply_TissueNtpm](#reply_tissuentpm), [Reply_WithUnit_RnaAbundance_RnaUnit](#reply_withunit_rnaabundance_rnaunit). Open the list below to read their fields.

<details>
<summary>Fields of the 33 other types</summary>

### Reply_AccessionBase

An accession whose source states no version; never used for an allele location.

Type: string.

### Reply_CancerPtpm

HPA cancer RNA abundance in protein-coding transcripts per million.

| Field | Type | Meaning |
| --- | --- | --- |
| `ptpm` | [Reply_WithUnit_RnaAbundance_RnaUnit](#reply_withunit_rnaabundance_rnaunit) | Finite nonnegative pTPM. |
| `tissue` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact source cancer tissue name. |

### Reply_Capped_PathwayRef

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_PathwayRef](#reply_pathwayref) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_CivicGene

CIViC's gene-level entry without evidence or variant lists.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Complete source description, absent when the source has none. Optional. |
| `id` | [Reply_Identifier_uint64](/reference/schemas/types/#reply_identifier_uint64) | CIViC feature identifier. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Link to the source entry. Optional. |
| `variant_count` | integer or null | Number of curated variants reported by the source. Optional. |

### Reply_ClinGenGene

ClinGen gene validity and dosage assessments in published source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `dosage_date` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source dosage assessment date, preserving its original spelling. Optional. |
| `dosage_status` | [Reply_SourceOutcome](/reference/schemas/types/#reply_sourceoutcome) | Outcome of the dosage sensitivity source read. |
| `haploinsufficiency` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source haploinsufficiency assessment, when a dosage record exists. Optional. |
| `triplosensitivity` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source triplosensitivity assessment, when a dosage record exists. Optional. |
| `validity` | list of [Reply_ClinGenValidity](#reply_clingenvalidity) | Whole gene–disease validity records in file order. Optional. |
| `validity_status` | [Reply_SourceOutcome](/reference/schemas/types/#reply_sourceoutcome) | Outcome of the gene–disease validity source read. |

### Reply_ClinGenValidity

One published ClinGen gene–disease validity record.

| Field | Type | Meaning |
| --- | --- | --- |
| `classification` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source classification without inference. Optional. |
| `classification_date` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source classification date, preserving its original spelling. Optional. |
| `disease` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source disease label. Optional. |
| `expert_panel` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source expert panel. Optional. |
| `mode_of_inheritance` | [Reply_Text](/reference/schemas/types/#reply_text) | Source mode of inheritance. |
| `mondo_id` | [Reply_MondoId](#reply_mondoid) | Source MONDO identifier. |
| `report_url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source report URL. Optional. |

### Reply_Constraint

Source constraint metrics for the source's preferred transcript.

| Field | Type | Meaning |
| --- | --- | --- |
| `flags` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Source flags, omitted when empty. Optional. |
| `loeuf` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) or null | Upper confidence bound of the observed-to-expected loss-of-function ratio. Optional. |
| `mis_z` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) or null | Missense Z score. Optional. |
| `oe_lof` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) or null | Observed-to-expected loss-of-function ratio. Optional. |
| `pli` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) or null | Probability of loss-of-function intolerance. Optional. |
| `syn_z` | [Reply_FiniteNumber](/reference/schemas/types/#reply_finitenumber) or null | Synonymous Z score. Optional. |
| `transcript` | [Reply_AccessionBase](#reply_accessionbase) | Source MANE Select transcript, otherwise the source canonical transcript. |
| `version` | [Reply_Text](/reference/schemas/types/#reply_text) | Source dataset family, not a minor release identifier. |

### Reply_GenccGene

GenCC records in published order without merged classifications.

| Field | Type | Meaning |
| --- | --- | --- |
| `submissions` | list of [Reply_GenccSubmission](#reply_genccsubmission) | Individual submissions in source order. |

### Reply_GenccSubmission

One source submission; no clinical validity is inferred.

| Field | Type | Meaning |
| --- | --- | --- |
| `classification` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source classification title without inference. Optional. |
| `classification_id` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source classification identifier. Optional. |
| `disease` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source disease title. Optional. |
| `disease_id` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source disease identifier. Optional. |
| `evaluated_date` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source submitted_as_date, preserving its spelling. Optional. |
| `id` | [Reply_Text](/reference/schemas/types/#reply_text) | Source submission identifier. |
| `mode_of_inheritance` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source inheritance title. Optional. |
| `mode_of_inheritance_id` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source inheritance identifier. Optional. |
| `submitted_date` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source submitted_run_date, preserving its spelling. Optional. |
| `submitter` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source submitter title. Optional. |
| `submitter_id` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source submitter identifier. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply_text) | Source submission version. |

### Reply_GeneExpression

Gene-level HPA expression and subcellular annotations.

| Field | Type | Meaning |
| --- | --- | --- |
| `rna_cancer_ptpm` | list of [Reply_CancerPtpm](#reply_cancerptpm) | Cancer-specific pTPM pairs. Optional. |
| `rna_cancer_specificity` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source RNA cancer specificity. Optional. |
| `rna_tissue_distribution` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source RNA tissue distribution. Optional. |
| `rna_tissue_ntpm` | list of [Reply_TissueNtpm](#reply_tissuentpm) | Tissue-specific nTPM pairs. Optional. |
| `rna_tissue_specificity` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source RNA tissue specificity. Optional. |
| `subcellular_additional` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Source additional subcellular locations. Optional. |
| `subcellular_main` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Source main subcellular locations. Optional. |

### Reply_GeneItem

Answer to one requested input.

| Field | Type | Meaning |
| --- | --- | --- |
| `answer` | [Reply_GeneResolution](/reference/schemas/types/#reply_generesolution) or null | Absent only when identity lookup failed. Optional. |
| `input` | [Reply_GeneInput](/reference/schemas/types/#reply_geneinput) | Original checked input. |
| `more` | list of [Reply_GeneSection](#reply_genesection) | Applicable sections left out of this request. Optional. |
| `oncokb` | [Reply_OncokbCard](#reply_oncokbcard) or null | Verified keyed source card annotation. Optional. |
| `sections` | [Reply_GeneSections](#reply_genesections) or null | Requested sections, present only for a resolved identity. Optional. |
| `status` | [Reply_Status](/reference/schemas/types/#reply_status) | This input's outcome. |

### Reply_GeneSection

A landed section that can be requested for a gene.

Type: string.

### Reply_GeneSectionWord

A section request word, including expansion to all landed sections.

Type: string.

### Reply_GeneSections

Landed gene sections, each retaining its source and explicit state.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic` | [Reply_Section_CivicGene](#reply_section_civicgene) or null | CIViC gene-level curation. Optional. |
| `clingen` | [Reply_Section_ClinGenGene](#reply_section_clingengene) or null | ClinGen validity and dosage assessments. Optional. |
| `constraint` | [Reply_Section_Constraint](#reply_section_constraint) or null | Population constraint reported by gnomAD. Optional. |
| `expression` | [Reply_Section_GeneExpression](#reply_section_geneexpression) or null | Human Protein Atlas expression annotations. Optional. |
| `gencc` | [Reply_Section_GenccGene](#reply_section_genccgene) or null | GenCC assessments, retained separately for each submitter. Optional. |
| `oncokb` | [Reply_Section_Oncokb](#reply_section_oncokb) or null | Keyed source gene details. Optional. |
| `pathways` | [Reply_Section_Capped_PathwayRef](#reply_section_capped_pathwayref) or null | Lowest-level human Reactome mappings. Optional. |
| `protein` | [Reply_Section_Array_of_Protein_UniProtAcc](#reply_section_array_of_protein_uniprotacc) or null | Reviewed human proteins listed by HGNC. Optional. |

### Reply_MondoId

A publisher MONDO identifier with seven digits.

Type: string.

### Reply_Oncokb

Whole OncoKB gene prose and independent transcript annotations.

| Field | Type | Meaning |
| --- | --- | --- |
| `background` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Whole source background. Optional. |
| `grch37` | [Reply_OncokbTranscript](#reply_oncokbtranscript) | GRCh37 source transcripts. |
| `grch38` | [Reply_OncokbTranscript](#reply_oncokbtranscript) | GRCh38 source transcripts. |
| `summary` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Whole source summary. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Verified data release version. Optional. |

### Reply_OncokbCard

Short source-stated annotation on a gene card.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene_type` | [Reply_SourceText](/reference/schemas/types/#reply_sourcetext) or null | Exact native classification, including an unpublished future literal. Optional. |
| `gene_type_code` | [Reply_OncokbGeneType](#reply_oncokbgenetype) or null | Coding for the five published source literals only. Optional. |
| `highest_resistance_level` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Highest resistance level, as stated by the source. Optional. |
| `highest_sensitive_level` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Highest sensitive level, as stated by the source. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Data release version, when verified. Optional. |

### Reply_OncokbGeneType

OncoKB's stated gene classification.

Type: string.

### Reply_OncokbTranscript

Source transcript annotation for one genome build.

| Field | Type | Meaning |
| --- | --- | --- |
| `isoform` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source-stated isoform, kept separate from HGNC MANE. Optional. |
| `refseq` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source-stated RefSeq transcript. Optional. |

### Reply_PathwayRef

A pathway reference without membership or hierarchy data.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_ReactomeId](/reference/schemas/types/#reply_reactomeid) | Human stable identifier. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Whole source pathway name. Optional. |

### Reply_Protein_UniProtAcc

A reviewed human protein listed by the authoritative gene record.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [Reply_UniProtAcc](/reference/schemas/types/#reply_uniprotacc) | Checked primary accession. |
| `function` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Whole source function paragraphs, omitted when absent. Optional. |
| `length` | integer or null | Sequence length in amino acids. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Recommended protein name. Optional. |
| `subcellular_locations` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Source subcellular location names. Optional. |

### Reply_RnaAbundance

A finite nonnegative RNA abundance expressed in the field's stated unit.

Type: number (double).

### Reply_RnaUnit

Publisher RNA abundance units.

Values: `nTPM`, `pTPM`.

### Reply_Section_Array_of_Protein_UniProtAcc

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` (list of [Reply_Protein_UniProtAcc](#reply_protein_uniprotacc)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Capped_PathwayRef

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Capped_PathwayRef](#reply_capped_pathwayref)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_CivicGene

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_CivicGene](#reply_civicgene)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_ClinGenGene

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_ClinGenGene](#reply_clingengene)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Constraint

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Constraint](#reply_constraint)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_GenccGene

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_GenccGene](#reply_genccgene)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_GeneExpression

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_GeneExpression](#reply_geneexpression)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Oncokb

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Oncokb](#reply_oncokb)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_TissueNtpm

HPA tissue RNA abundance in normalized transcripts per million.

| Field | Type | Meaning |
| --- | --- | --- |
| `ntpm` | [Reply_WithUnit_RnaAbundance_RnaUnit](#reply_withunit_rnaabundance_rnaunit) | Finite nonnegative nTPM. |
| `tissue` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact source tissue name. |

### Reply_WithUnit_RnaAbundance_RnaUnit

A source number with its explicitly stated unit; missing units stay absent.

| Field | Type | Meaning |
| --- | --- | --- |
| `unit` | [Reply_RnaUnit](#reply_rnaunit) or null | Publisher unit or code, without conversion or inference. Optional. |
| `value` | [Reply_RnaAbundance](#reply_rnaabundance) | Unchanged numeric source value. |

</details>

## Next

[`biomcp get gene`](/reference/commands/get-gene/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
