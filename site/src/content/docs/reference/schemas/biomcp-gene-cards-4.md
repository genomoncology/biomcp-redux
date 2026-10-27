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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gene constraint`](/reference/commands/get-gene/) and [`biomcp get gene`](/reference/commands/get-gene/) reply with `schema` set to `biomcp.gene_cards/4`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get gene TP53 constraint
biomcp --disable-sources noncommercial,restricted get gene BRAF
```

`biomcp schema biomcp.gene_cards/4` prints the full JSON Schema.

## Example reply

`biomcp --disable-sources noncommercial,restricted get gene BRAF` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
        "answer": {
          "resolution": "resolved",
          "gene": {
            "hgnc_id": "HGNC:1097",
            "symbol": "BRAF",
            "entrez_id": "673",
            "ensembl_id": "ENSG00000157764",
            "status": "approved",
            "locus_group": "protein_coding",
            "locus_type": "gene_with_protein_product",
            "location": "7q34",
            "previous_symbols": [],
            "aliases": ["BRAF1", "BRAF-1"],
            "uniprot": ["P15056"],
            "omim": ["164757"],
            "mane": […],
            "spans": [],
            "name": "B-Raf proto-oncogene, serine/threonine kinase"
          },
          "match": {"kind": "symbol", "text": "BRAF"}
        },
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

`result` holds [GeneCards](#genecards). `query` holds [GeneGet](#geneget). `query` repeats the request as BioMCP read it.

### GeneCards

Gene answers in input order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [GeneItem](#geneitem) | yes | Requested inputs and their answers. |

### GeneGet

Get one to twenty human gene inputs in their original order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `input` | list of [GeneInput](/reference/schemas/types/#geneinput) | yes | Gene symbols or checked identifiers, in input order. |
| `section` | list of [GeneSectionWord](#genesectionword) | no | Requested [section words](/concepts/progressive-disclosure/), preserving caller order and repetitions. Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs. |

### AccessionBase

Type: string.

### CancerPtpm

HPA cancer RNA abundance in protein-coding transcripts per million.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ptpm` | [RnaAbundance](#rnaabundance) | yes | Finite nonnegative pTPM. |
| `tissue` | [Text](/reference/schemas/types/#text) | yes | Exact source cancer tissue name. |

### CivicGene

CIViC's gene-level entry without evidence or variant lists.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | [Text](/reference/schemas/types/#text) or null | no | Complete source description, absent when the source has none. |
| `id` | integer (uint64) | yes | CIViC feature identifier. |
| `url` | [Text](/reference/schemas/types/#text) | yes | Link to the source entry. |
| `variant_count` | integer (uint64) | yes | Number of curated variants reported by the source. |

### ClinGenGene

ClinGen gene validity and dosage assessments in published source order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `dosage_date` | [Text](/reference/schemas/types/#text) or null | no | Source dosage assessment date, preserving its original spelling. |
| `haploinsufficiency` | [Text](/reference/schemas/types/#text) or null | no | Source haploinsufficiency assessment, when a dosage record exists. |
| `triplosensitivity` | [Text](/reference/schemas/types/#text) or null | no | Source triplosensitivity assessment, when a dosage record exists. |
| `validity` | list of [ClinGenValidity](#clingenvalidity) | yes | Whole gene–disease validity records in file order. |

### ClinGenValidity

One published ClinGen gene–disease validity record.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `classification` | [Text](/reference/schemas/types/#text) | yes | Source classification without inference. |
| `classification_date` | [Text](/reference/schemas/types/#text) | yes | Source classification date, preserving its original spelling. |
| `disease` | [Text](/reference/schemas/types/#text) | yes | Source disease label. |
| `expert_panel` | [Text](/reference/schemas/types/#text) | yes | Source expert panel. |
| `mode_of_inheritance` | [Text](/reference/schemas/types/#text) | yes | Source mode of inheritance. |
| `mondo_id` | [Text](/reference/schemas/types/#text) | yes | Source MONDO identifier. |
| `report_url` | [Text](/reference/schemas/types/#text) | yes | Source report URL. |

### Constraint

Source constraint metrics for the source's preferred transcript.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `flags` | list of [Text](/reference/schemas/types/#text) | no | Source flags, omitted when empty. |
| `loeuf` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Upper confidence bound of the observed-to-expected loss-of-function ratio. |
| `mis_z` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Missense Z score. |
| `oe_lof` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Observed-to-expected loss-of-function ratio. |
| `pli` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Probability of loss-of-function intolerance. |
| `syn_z` | [FiniteNumber](/reference/schemas/types/#finitenumber) | yes | Synonymous Z score. |
| `transcript` | [AccessionBase](#accessionbase) | yes | Source MANE Select transcript, otherwise the source canonical transcript. |
| `version` | [Text](/reference/schemas/types/#text) | yes | Source dataset family, not a minor release identifier. |

### GeneExpression

Gene-level HPA expression and subcellular annotations.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `rna_cancer_ptpm` | list of [CancerPtpm](#cancerptpm) | no | Cancer-specific pTPM pairs. |
| `rna_cancer_specificity` | [Text](/reference/schemas/types/#text) or null | no | Source RNA cancer specificity. |
| `rna_tissue_distribution` | [Text](/reference/schemas/types/#text) or null | no | Source RNA tissue distribution. |
| `rna_tissue_ntpm` | list of [TissueNtpm](#tissuentpm) | no | Tissue-specific nTPM pairs. |
| `rna_tissue_specificity` | [Text](/reference/schemas/types/#text) or null | no | Source RNA tissue specificity. |
| `subcellular_additional` | list of [Text](/reference/schemas/types/#text) | no | Source additional subcellular locations. |
| `subcellular_main` | list of [Text](/reference/schemas/types/#text) | no | Source main subcellular locations. |

### GeneItem

Answer to one requested input.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `answer` | [GeneResolution](/reference/schemas/types/#generesolution) or null | no | Absent only when identity lookup failed. |
| `input` | [GeneInput](/reference/schemas/types/#geneinput) | yes | Original checked input. |
| `more` | list of [GeneSection](#genesection) | no | Applicable sections left out of this request. |
| `oncokb` | [OncokbCard](#oncokbcard) or null | no | Verified keyed source card annotation. |
| `sections` | [GeneSections](#genesections) or null | no | Requested sections, present only for a resolved identity. |
| `status` | [Status](/reference/schemas/types/#status) | yes | This input's outcome. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `civic` | [Section](/reference/schemas/types/#section) of [CivicGene](#civicgene) or null | no | CIViC gene-level curation. |
| `clingen` | [Section](/reference/schemas/types/#section) of [ClinGenGene](#clingengene) or null | no | ClinGen validity and dosage assessments. |
| `constraint` | [Section](/reference/schemas/types/#section) of [Constraint](#constraint) or null | no | Population constraint reported by gnomAD. |
| `expression` | [Section](/reference/schemas/types/#section) of [GeneExpression](#geneexpression) or null | no | Human Protein Atlas expression annotations. |
| `oncokb` | [Section](/reference/schemas/types/#section) of [Oncokb](#oncokb) or null | no | Keyed source gene details. |
| `pathways` | [Section](/reference/schemas/types/#section) of [Capped](/reference/schemas/types/#capped) of [PathwayRef](#pathwayref) or null | no | Lowest-level human Reactome mappings. |
| `protein` | [Section](#section) or null | no | Reviewed human proteins listed by HGNC. |

### Oncokb

Whole OncoKB gene prose and independent transcript annotations.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `background` | [Text](/reference/schemas/types/#text) or null | no | Whole source background. |
| `grch37` | [OncokbTranscript](#oncokbtranscript) | yes | GRCh37 source transcripts. |
| `grch38` | [OncokbTranscript](#oncokbtranscript) | yes | GRCh38 source transcripts. |
| `summary` | [Text](/reference/schemas/types/#text) or null | no | Whole source summary. |
| `version` | [Text](/reference/schemas/types/#text) or null | no | Verified data release version. |

### OncokbCard

Short source-stated annotation on a gene card.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `gene_type` | [OncokbGeneType](#oncokbgenetype) | yes | Source gene classification. |
| `highest_resistance_level` | [Text](/reference/schemas/types/#text) or null | no | Highest resistance level, as stated by the source. |
| `highest_sensitive_level` | [Text](/reference/schemas/types/#text) or null | no | Highest sensitive level, as stated by the source. |
| `version` | [Text](/reference/schemas/types/#text) or null | no | Data release version, when verified. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `isoform` | [Text](/reference/schemas/types/#text) or null | no | Source-stated isoform, kept separate from HGNC MANE. |
| `refseq` | [Text](/reference/schemas/types/#text) or null | no | Source-stated RefSeq transcript. |

### PathwayRef

A pathway reference without membership or hierarchy data.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [ReactomeId](/reference/schemas/types/#reactomeid) | yes | Human stable identifier. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Whole source pathway name. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ntpm` | [RnaAbundance](#rnaabundance) | yes | Finite nonnegative nTPM. |
| `tissue` | [Text](/reference/schemas/types/#text) | yes | Exact source tissue name. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get gene`](/reference/commands/get-gene/) gives the command's arguments.
