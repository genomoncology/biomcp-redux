---
title: Reply of biomcp get gene constraint section (biomcp.gene_cards/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gene constraint:
  a real example and the fields of schema biomcp.gene_cards/4."
goal: "Parse the biomcp.gene_cards/4 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene_cards/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gene constraint`](/reference/commands/get-gene/) replies with `schema` set to `biomcp.gene_cards/4`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get gene TP53 constraint
biomcp --disable-sources noncommercial,restricted get gene BRAF
```

`biomcp schema biomcp.gene_cards/4` prints the full JSON Schema.

## Example reply

`biomcp --disable-sources noncommercial,restricted get gene BRAF` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.gene_cards/4",
  "status": "partial",
  "query": {"input": ["BRAF"]},
  "result": {
    "items": [
      {
        "input": "BRAF",
        "status": "partial",
        "answer": {…},
        "more": [
          "protein", "civic", "clingen", "constraint", "expression", "pathways"
        ]
      }
    ]
  },
  "sources": [
    {
      "source": "hgnc",
      "use": "open",
      "terms": "CC0 1.0",
      "terms_url": "https://www.genenames.org/about/license/",
      "state": "ok"
    },
    … 2 more
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "--disable-sources", "noncommercial,restricted", "get", "gene",
        "HGNC:1097", "protein", "civic", "clingen",
        … 3 more
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [GeneCards](#genecards). `query` holds [GeneGet](#geneget). `query` repeats the request as BioMCP read it.

### GeneCards

Gene answers in input order.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [GeneItem](#geneitem) | Requested inputs and their answers. |

### GeneGet

Get one to twenty human gene inputs in their original order.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [GeneInput](/reference/schemas/types/#geneinput) | Gene symbols or checked identifiers, in input order. |
| `section` | list of [GeneSectionWord](#genesectionword) | Requested [section words](/concepts/progressive-disclosure/), preserving caller order and repetitions. Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs. Optional. |

### Other types in this reply

19 more types appear only in this reply: [AccessionBase](#accessionbase), [CancerPtpm](#cancerptpm), [CivicGene](#civicgene), [ClinGenGene](#clingengene), [ClinGenValidity](#clingenvalidity), [Constraint](#constraint), [GeneExpression](#geneexpression), [GeneItem](#geneitem), [GeneSection](#genesection), [GeneSectionWord](#genesectionword), [GeneSections](#genesections), [Oncokb](#oncokb), [OncokbCard](#oncokbcard), [OncokbGeneType](#oncokbgenetype), [OncokbTranscript](#oncokbtranscript), [PathwayRef](#pathwayref), [RnaAbundance](#rnaabundance), [Section](#section), [TissueNtpm](#tissuentpm). Open the list below to read their fields.

<details>
<summary>Fields of the 19 other types</summary>

### AccessionBase

Type: string.

### CancerPtpm

HPA cancer RNA abundance in protein-coding transcripts per million.

| Field | Type | Meaning |
| --- | --- | --- |
| `ptpm` | [RnaAbundance](#rnaabundance) | Finite nonnegative pTPM. |
| `tissue` | [Text](/reference/schemas/types/#text) | Exact source cancer tissue name. |

### CivicGene

CIViC's gene-level entry without evidence or variant lists.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | [Text](/reference/schemas/types/#text) or null | Complete source description, absent when the source has none. Optional. |
| `id` | integer (uint64) | CIViC feature identifier. |
| `url` | [Text](/reference/schemas/types/#text) | Link to the source entry. |
| `variant_count` | integer (uint64) | Number of curated variants reported by the source. |

### ClinGenGene

ClinGen gene validity and dosage assessments in published source order.

| Field | Type | Meaning |
| --- | --- | --- |
| `dosage_date` | [Text](/reference/schemas/types/#text) or null | Source dosage assessment date, preserving its original spelling. Optional. |
| `haploinsufficiency` | [Text](/reference/schemas/types/#text) or null | Source haploinsufficiency assessment, when a dosage record exists. Optional. |
| `triplosensitivity` | [Text](/reference/schemas/types/#text) or null | Source triplosensitivity assessment, when a dosage record exists. Optional. |
| `validity` | list of [ClinGenValidity](#clingenvalidity) | Whole gene–disease validity records in file order. |

### ClinGenValidity

One published ClinGen gene–disease validity record.

| Field | Type | Meaning |
| --- | --- | --- |
| `classification` | [Text](/reference/schemas/types/#text) | Source classification without inference. |
| `classification_date` | [Text](/reference/schemas/types/#text) | Source classification date, preserving its original spelling. |
| `disease` | [Text](/reference/schemas/types/#text) | Source disease label. |
| `expert_panel` | [Text](/reference/schemas/types/#text) | Source expert panel. |
| `mode_of_inheritance` | [Text](/reference/schemas/types/#text) | Source mode of inheritance. |
| `mondo_id` | [Text](/reference/schemas/types/#text) | Source MONDO identifier. |
| `report_url` | [Text](/reference/schemas/types/#text) | Source report URL. |

### Constraint

Source constraint metrics for the source's preferred transcript.

| Field | Type | Meaning |
| --- | --- | --- |
| `flags` | list of [Text](/reference/schemas/types/#text) | Source flags, omitted when empty. Optional. |
| `loeuf` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Upper confidence bound of the observed-to-expected loss-of-function ratio. |
| `mis_z` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Missense Z score. |
| `oe_lof` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Observed-to-expected loss-of-function ratio. |
| `pli` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Probability of loss-of-function intolerance. |
| `syn_z` | [FiniteNumber](/reference/schemas/types/#finitenumber) | Synonymous Z score. |
| `transcript` | [AccessionBase](#accessionbase) | Source MANE Select transcript, otherwise the source canonical transcript. |
| `version` | [Text](/reference/schemas/types/#text) | Source dataset family, not a minor release identifier. |

### GeneExpression

Gene-level HPA expression and subcellular annotations.

| Field | Type | Meaning |
| --- | --- | --- |
| `rna_cancer_ptpm` | list of [CancerPtpm](#cancerptpm) | Cancer-specific pTPM pairs. Optional. |
| `rna_cancer_specificity` | [Text](/reference/schemas/types/#text) or null | Source RNA cancer specificity. Optional. |
| `rna_tissue_distribution` | [Text](/reference/schemas/types/#text) or null | Source RNA tissue distribution. Optional. |
| `rna_tissue_ntpm` | list of [TissueNtpm](#tissuentpm) | Tissue-specific nTPM pairs. Optional. |
| `rna_tissue_specificity` | [Text](/reference/schemas/types/#text) or null | Source RNA tissue specificity. Optional. |
| `subcellular_additional` | list of [Text](/reference/schemas/types/#text) | Source additional subcellular locations. Optional. |
| `subcellular_main` | list of [Text](/reference/schemas/types/#text) | Source main subcellular locations. Optional. |

### GeneItem

Answer to one requested input.

| Field | Type | Meaning |
| --- | --- | --- |
| `answer` | [GeneResolution](/reference/schemas/types/#generesolution) or null | Absent only when identity lookup failed. Optional. |
| `input` | [GeneInput](/reference/schemas/types/#geneinput) | Original checked input. |
| `more` | list of [GeneSection](#genesection) | Applicable sections left out of this request. Optional. |
| `oncokb` | [OncokbCard](#oncokbcard) or null | Verified keyed source card annotation. Optional. |
| `sections` | [GeneSections](#genesections) or null | Requested sections, present only for a resolved identity. Optional. |
| `status` | [Status](/reference/schemas/types/#status) | This input's outcome. |

### GeneSection

A landed section that can be requested for a gene.

| Value | Meaning |
| --- | --- |
| `protein` | Reviewed human proteins. |
| `civic` | CIViC gene curation. |
| `clingen` | ClinGen validity and dosage. |
| `constraint` | Population constraint metrics. |
| `expression` | Human Protein Atlas expression. |
| `pathways` | Human Reactome pathways. |
| `oncokb` | Keyed OncoKB annotations. |

### GeneSectionWord

A section request word, including expansion to all landed sections.

| Value | Meaning |
| --- | --- |
| `protein` | Reviewed human proteins. |
| `civic` | CIViC gene curation. |
| `clingen` | ClinGen validity and dosage. |
| `constraint` | Population constraint metrics. |
| `expression` | Human Protein Atlas expression. |
| `pathways` | Human Reactome pathways. |
| `oncokb` | Keyed OncoKB annotations. |
| `all` | Every landed section. |

### GeneSections

Landed gene sections, each retaining its source and explicit state.

| Field | Type | Meaning |
| --- | --- | --- |
| `civic` | [Section](/reference/schemas/types/#section) of [CivicGene](#civicgene) or null | CIViC gene-level curation. Optional. |
| `clingen` | [Section](/reference/schemas/types/#section) of [ClinGenGene](#clingengene) or null | ClinGen validity and dosage assessments. Optional. |
| `constraint` | [Section](/reference/schemas/types/#section) of [Constraint](#constraint) or null | Population constraint reported by gnomAD. Optional. |
| `expression` | [Section](/reference/schemas/types/#section) of [GeneExpression](#geneexpression) or null | Human Protein Atlas expression annotations. Optional. |
| `oncokb` | [Section](/reference/schemas/types/#section) of [Oncokb](#oncokb) or null | Keyed source gene details. Optional. |
| `pathways` | [Section](/reference/schemas/types/#section) of [Capped](/reference/schemas/types/#capped) of [PathwayRef](#pathwayref) or null | Lowest-level human Reactome mappings. Optional. |
| `protein` | [Section](#section) or null | Reviewed human proteins listed by HGNC. Optional. |

### Oncokb

Whole OncoKB gene prose and independent transcript annotations.

| Field | Type | Meaning |
| --- | --- | --- |
| `background` | [Text](/reference/schemas/types/#text) or null | Whole source background. Optional. |
| `grch37` | [OncokbTranscript](#oncokbtranscript) | GRCh37 source transcripts. |
| `grch38` | [OncokbTranscript](#oncokbtranscript) | GRCh38 source transcripts. |
| `summary` | [Text](/reference/schemas/types/#text) or null | Whole source summary. Optional. |
| `version` | [Text](/reference/schemas/types/#text) or null | Verified data release version. Optional. |

### OncokbCard

Short source-stated annotation on a gene card.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene_type` | [OncokbGeneType](#oncokbgenetype) | Source gene classification. |
| `highest_resistance_level` | [Text](/reference/schemas/types/#text) or null | Highest resistance level, as stated by the source. Optional. |
| `highest_sensitive_level` | [Text](/reference/schemas/types/#text) or null | Highest sensitive level, as stated by the source. Optional. |
| `version` | [Text](/reference/schemas/types/#text) or null | Data release version, when verified. Optional. |

### OncokbGeneType

OncoKB's stated gene classification.

| Value | Meaning |
| --- | --- |
| `ONCOGENE_AND_TSG` | Both stated roles. |
| `ONCOGENE` | Source-stated oncogene. |
| `TSG` | Source-stated tumor suppressor. |
| `INSUFFICIENT_EVIDENCE` | Source states insufficient evidence. |
| `NEITHER` | Source states neither role. |

### OncokbTranscript

Source transcript annotation for one genome build.

| Field | Type | Meaning |
| --- | --- | --- |
| `isoform` | [Text](/reference/schemas/types/#text) or null | Source-stated isoform, kept separate from HGNC MANE. Optional. |
| `refseq` | [Text](/reference/schemas/types/#text) or null | Source-stated RefSeq transcript. Optional. |

### PathwayRef

A pathway reference without membership or hierarchy data.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [ReactomeId](/reference/schemas/types/#reactomeid) | Human stable identifier. |
| `name` | [Text](/reference/schemas/types/#text) | Whole source pathway name. |

### RnaAbundance

A finite nonnegative RNA abundance expressed in the field's stated unit.

Type: number (double).

### Section

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` (list of [Protein](/reference/schemas/types/#protein) of [UniProtAcc](/reference/schemas/types/#uniprotacc)), `source` ([Source](/reference/schemas/types/#source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([SourceRecord](/reference/schemas/types/#sourcerecord) or null), `source` ([Source](/reference/schemas/types/#source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reason](/reference/schemas/types/#reason)), `source` ([Source](/reference/schemas/types/#source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reason](/reference/schemas/types/#reason)), `source` ([Source](/reference/schemas/types/#source)).
- `state` is `"not_applicable"`: The section does not apply to this record.

### TissueNtpm

HPA tissue RNA abundance in normalized transcripts per million.

| Field | Type | Meaning |
| --- | --- | --- |
| `ntpm` | [RnaAbundance](#rnaabundance) | Finite nonnegative nTPM. |
| `tissue` | [Text](/reference/schemas/types/#text) | Exact source tissue name. |

</details>

## Next

[`biomcp get gene`](/reference/commands/get-gene/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
