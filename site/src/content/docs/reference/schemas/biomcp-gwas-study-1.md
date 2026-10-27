---
title: Reply of biomcp get gwas (biomcp.gwas_study/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: a real
  example and the fields of schema biomcp.gwas_study/1."
goal: "Parse the biomcp.gwas_study/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas_study/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/) replies with `schema` set to `biomcp.gwas_study/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get gwas GCST000035
```

`biomcp schema biomcp.gwas_study/1` prints the full JSON Schema.

## Example reply

`biomcp get gwas GCST000035` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.gwas_study/1",
  "status": "complete",
  "query": {
    "kind": "study",
    "id": "GCST000035",
    "section_limit": 3,
    "section_offset": 0
  },
  "result": {
    "id": "GCST000035",
    "publication": "17529967",
    "reported_traits": ["Breast cancer"],
    "ontology_traits": [
      {
        "id": "MONDO_0004989",
        "label": "breast carcinoma"
      }
    ],
    "initial_sample_description": "390 European ancestry cases, 364 European…",
    "replication_sample_description": "4,364 East Asian ancestry cases, 24,17…",
    "discovery_ancestries": ["754 European (U.K.)"],
    "replication_ancestries": [
      "48565 European (Australia, Denmark, Finland, Germany, NR, Netherlands,…",
      "7928 East Asian (NR, Republic of Korea, Taiwan, Thailand)"
    ],
    "genotyping_technologies": ["Genome-wide genotyping array"],
    "snp_count": 205586,
    "platforms": "Perlegen [205586]",
    "array_manufacturers": ["Perlegen"],
    "gxe": false,
    "gxg": false,
    "imputed": false,
    "pooled": false,
    "user_requested": false,
    "summary_statistics_available": false,
    "summary_statistics_url": "NA",
    "license_url": "https://www.ebi.ac.uk/about/terms-of-use/"
  },
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
      "kind": "section",
      "args": [
        "get", "gwas", "GCST000035", "associations", "--section-limit", "3",
        "--section-offset", "0"
      ]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [GwasStudy](#gwasstudy). `query` holds [GwasGet](/reference/schemas/types/#gwasget), a shared type. `query` repeats the request as BioMCP read it.

### GwasStudy

Native study card; independent source descriptions are never reconciled.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `array_manufacturers` | list of [Text](/reference/schemas/types/#text) or null | no | Native array manufacturer labels. |
| `discovery_ancestries` | list of [Text](/reference/schemas/types/#text) | yes | Native discovery ancestry summaries. |
| `genotyping_technologies` | list of [Text](/reference/schemas/types/#text) | yes | Source technologies. |
| `gxe` | boolean or null | no | Native gene-environment interaction flag. |
| `gxg` | boolean or null | no | Native gene-gene interaction flag. |
| `id` | [GwasStudyId](/reference/schemas/types/#gwasstudyid) | yes | Exact accession. |
| `imputed` | boolean or null | no | Native imputation flag. |
| `initial_sample_description` | [Text](/reference/schemas/types/#text) or null | no | Whole discovery sample prose. |
| `license_url` | [Text](/reference/schemas/types/#text) or null | no | Native terms link without an inferred reuse grant. |
| `ontology_traits` | list of [GwasTrait](/reference/schemas/types/#gwastrait) | yes | Every native mapped ontology label and ID. |
| `platforms` | [Text](/reference/schemas/types/#text) or null | no | Native platform description. |
| `pooled` | boolean or null | no | Native pooled study flag. |
| `publication` | [Pmid](/reference/schemas/types/#pmid) or null | no | Source publication identity. |
| `replication_ancestries` | list of [Text](/reference/schemas/types/#text) | yes | Native replication ancestry summaries. |
| `replication_sample_description` | [Text](/reference/schemas/types/#text) or null | no | Whole replication sample prose. |
| `reported_traits` | list of [Text](/reference/schemas/types/#text) | yes | Native reported disease traits. |
| `snp_count` | integer or null | no | Source SNP count; missing differs from zero. |
| `summary_statistics_available` | boolean or null | no | Source availability flag; grants no download or reuse permission. |
| `summary_statistics_url` | [Text](/reference/schemas/types/#text) or null | no | Native summary statistics link; not a local download. |
| `user_requested` | boolean or null | no | Native user-requested study flag. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments.
