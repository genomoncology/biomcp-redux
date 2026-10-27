---
title: Reply of biomcp get gwas loci section (biomcp.gwas_loci/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas loci: a
  real example and the fields of schema biomcp.gwas_loci/1."
goal: "Parse the biomcp.gwas_loci/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_loci/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas loci`](/reference/commands/get-gwas/) replies with `schema` set to `biomcp.gwas_loci/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get gwas association:6114 loci
```

`biomcp schema biomcp.gwas_loci/1` prints the full JSON Schema.

## Example reply

`biomcp get gwas association:6114 loci` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.gwas_loci/1",
  "status": "complete",
  "query": {
    "kind": "association",
    "id": "association:6114",
    "section": "loci",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "association": "association:6114",
    "items": [
      {
        "description": "Single variant",
        "strongest_risk_alleles": ["rs3817198-C"],
        "author_reported_genes": [
          {
            "name": "LSP1",
            "ensembl_ids": ["ENSG00000130592"],
            "entrez_ids": ["4046"]
          }
        ],
        "source_url": "https://www.ebi.ac.uk/gwas/rest/api/v2/associations/611…"
      }
    ]
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "gwas",
      "use": "restricted",
      "terms": "GWAS Catalog terms and dataset rights",
      "terms_url": "https://www.ebi.ac.uk/about/terms-of-use/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "pivot",
      "args": ["get", "gene", "ENSG00000130592"]
    },
    {"kind": "pivot", "args": ["get", "gene", "4046"]}
  ]
}
```

## Fields

`result` holds [GwasLoci](#gwasloci). `query` holds [GwasGet](/reference/schemas/types/#gwasget), a shared type. `query` repeats the request as BioMCP read it.

### GwasLoci

One bounded locus section for an association.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `association` | [GwasAssociationId](/reference/schemas/types/#gwasassociationid) | yes | Exact owning association. |
| `continuation` | [GwasGet](/reference/schemas/types/#gwasget) | no | Same-resource relation continuation. |
| `items` | list of [GwasLocus](#gwaslocus) | yes | Native locus rows. |

### GwasLocus

One native locus with independent reported genes and risk-allele text.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `author_reported_genes` | list of [GwasReportedGene](#gwasreportedgene) | yes | Reported source genes; never inferred from names. |
| `description` | [Text](/reference/schemas/types/#text) or null | no | Complete source locus description. |
| `source_url` | [Text](/reference/schemas/types/#text) or null | no | Native locus self link. |
| `strongest_risk_alleles` | list of [Text](/reference/schemas/types/#text) | yes | Source strongest-risk-allele strings. |

### GwasReportedGene

Source reported gene identity, without a causal-gene interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `ensembl_ids` | list of [Text](/reference/schemas/types/#text) | yes | Exact source Ensembl identifiers. |
| `entrez_ids` | list of [Text](/reference/schemas/types/#text) | yes | Exact source Entrez identifiers. |
| `name` | [Text](/reference/schemas/types/#text) or null | no | Native source gene name. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments.
