---
title: Reply of biomcp search pgx (biomcp.pgx_page/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search pgx: a real
  example and the fields of schema biomcp.pgx_page/2."
goal: "Parse the biomcp.pgx_page/2 reply: see a real example and what each field
  means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search pgx`](/reference/commands/search-pgx/) replies with `schema` set to `biomcp.pgx_page/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search pgx --gene CYP2C19 --source cpic --limit 50
biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C
```

`biomcp schema biomcp.pgx_page/2` prints the full JSON Schema.

## Example reply

`biomcp search pgx --gene CYP2C19 --source cpic --limit 50` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pgx_page/2",
  "status": "complete",
  "query": {
    "gene": "CYP2C19",
    "source": "cpic",
    "limit": 50,
    "offset": 0
  },
  "result": {
    "header": {"source": "cpic", "gene": "CYP2C19"},
    "items": [
      {
        "pairid": 110080,
        "drugid": "RxNorm:704",
        "drugname": "amitriptyline",
        "genesymbol": "CYP2C19",
        "guidelinename": "CYP2D6, CYP2C19 and Tricyclic Antidepressants",
        "guidelineurl": "https://www.clinpgx.org/guideline/PA166251445",
        "cpiclevel": "A",
        "clinpgxlevel": "1A",
        "pgxtesting": null,
        "pmids": ["23486447", "27997040"],
        "usedforrecommendation": "Yes",
        "provisional": false
      },
      … 26 more
    ],
    "page": {
      "offset": 0,
      "returned": 27,
      "total": 27,
      "total_kind": "source"
    }
  },
  "sources": [
    {
      "source": "cpic",
      "use": "noncommercial",
      "terms": "ClinPGx data use policy; noncommercial access restriction; ca…",
      "terms_url": "https://www.clinpgx.org/page/dataUsagePolicy",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "get", "pgx", "--gene", "CYP2C19", "--source", "cpic"
      ]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [PgxPage](#pgxpage). `query` holds [PgxSearch](#pgxsearch). `query` repeats the request as BioMCP read it.

### PgxPage

Independent source-ordered page of complete native records.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | yes | Source and selected gene. |
| `items` | list of [PgxPair](#pgxpair) | yes | Complete native rows without duplicate summaries. |
| `page` | [PgxPageInfo](/reference/schemas/types/#pgxpageinfo) | yes | Meaning and bounds of this collection. |

### PgxSearch

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cpic_level` | [Text](/reference/schemas/types/#text) or null | no | Literal native level, including compound levels such as B/C. |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | yes | Explicit source gene symbol. |
| `limit` | integer (uint32) | no | Rows per native page. |
| `offset` | integer (uint32) | no | Native source offset. |
| `source` | `"cpic"` | yes | Required CPIC source; ClinPGx is not supported for search. |

### PgxPair

Source-native PgxPair fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `clinpgxlevel` | string or null | no | The source's own `clinpgxlevel` value, unchanged. |
| `cpiclevel` | string | yes | The source's own `cpiclevel` value, unchanged. |
| `drugid` | [Curie](/reference/schemas/types/#curie) | yes | The source's own `drugid` value, unchanged. |
| `drugname` | string | yes | The source's own `drugname` value, unchanged. |
| `genesymbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | yes | The source's own `genesymbol` value, unchanged. |
| `guidelinename` | string or null | no | The source's own `guidelinename` value, unchanged. |
| `guidelineurl` | string or null | no | The source's own `guidelineurl` value, unchanged. |
| `pairid` | integer (int64) | yes | The source's own `pairid` value, unchanged. |
| `pgxtesting` | string or null | no | The source's own `pgxtesting` value, unchanged. |
| `pmids` | list of [Pmid](/reference/schemas/types/#pmid) or null | no | The source's own `pmids` value, unchanged. |
| `provisional` | boolean | yes | The source's own `provisional` value, unchanged. |
| `usedforrecommendation` | string | yes | The source's own `usedforrecommendation` value, unchanged. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search pgx`](/reference/commands/search-pgx/) gives the command's arguments.
