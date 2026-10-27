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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas loci`](/reference/commands/get-gwas/) replies with `schema` set to `biomcp.gwas_loci/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get gwas association:6114 loci
```

`biomcp schema biomcp.gwas_loci/1` prints the full JSON Schema.

## Example reply

`biomcp get gwas association:6114 loci` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
        "author_reported_genes": […],
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
    … 1 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [GwasLoci](#gwasloci). `query` holds [GwasGet](/reference/schemas/types/#gwasget), a shared type. `query` repeats the request as BioMCP read it.

### GwasLoci

One bounded locus section for an association.

| Field | Type | Meaning |
| --- | --- | --- |
| `association` | [GwasAssociationId](/reference/schemas/types/#gwasassociationid) | Exact owning association. |
| `continuation` | [GwasGet](/reference/schemas/types/#gwasget) | Same-resource relation continuation. Optional. |
| `items` | list of [GwasLocus](#gwaslocus) | Native locus rows. |

### GwasLocus

One native locus with independent reported genes and risk-allele text.

| Field | Type | Meaning |
| --- | --- | --- |
| `author_reported_genes` | list of [GwasReportedGene](#gwasreportedgene) | Reported source genes; never inferred from names. |
| `description` | [Text](/reference/schemas/types/#text) or null | Complete source locus description. Optional. |
| `source_url` | [Text](/reference/schemas/types/#text) or null | Native locus self link. Optional. |
| `strongest_risk_alleles` | list of [Text](/reference/schemas/types/#text) | Source strongest-risk-allele strings. |

### GwasReportedGene

Source reported gene identity, without a causal-gene interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `ensembl_ids` | list of [Text](/reference/schemas/types/#text) | Exact source Ensembl identifiers. |
| `entrez_ids` | list of [Text](/reference/schemas/types/#text) | Exact source Entrez identifiers. |
| `name` | [Text](/reference/schemas/types/#text) or null | Native source gene name. Optional. |

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
