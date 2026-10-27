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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search pgx`](/reference/commands/search-pgx/) replies with `schema` set to `biomcp.pgx_page/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search pgx --gene CYP2C19 --source cpic --limit 50
biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C
```

`biomcp schema biomcp.pgx_page/2` prints the full JSON Schema.

## Example reply

`biomcp search pgx --gene CYP2C19 --source cpic --limit 50` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…},
      … 26 more
    ],
    "page": {"offset": 0, "returned": 27, "total": 27, "total_kind": "source"}
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
      "args": ["get", "pgx", "--gene", "CYP2C19", "--source", "cpic"]
    },
    … 1 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PgxPage](#pgxpage). `query` holds [PgxSearch](#pgxsearch). `query` repeats the request as BioMCP read it.

### PgxPage

Independent source-ordered page of complete native records.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | Source and selected gene. |
| `items` | list of [PgxPair](#pgxpair) | Complete native rows without duplicate summaries. |
| `page` | [PgxPageInfo](/reference/schemas/types/#pgxpageinfo) | Meaning and bounds of this collection. |

### PgxSearch

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `cpic_level` | [Text](/reference/schemas/types/#text) or null | Literal native level, including compound levels such as B/C. Optional. |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | Explicit source gene symbol. |
| `limit` | integer (uint32) | Rows per native page. Optional. |
| `offset` | integer (uint32) | Native source offset. Optional. |
| `source` | `"cpic"` | Required CPIC source; ClinPGx is not supported for search. |

### PgxPair

Source-native PgxPair fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `clinpgxlevel` | string or null | The source's own `clinpgxlevel` value, unchanged. Optional. |
| `cpiclevel` | string | The source's own `cpiclevel` value, unchanged. |
| `drugid` | [Curie](/reference/schemas/types/#curie) | The source's own `drugid` value, unchanged. |
| `drugname` | string | The source's own `drugname` value, unchanged. |
| `genesymbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | The source's own `genesymbol` value, unchanged. |
| `guidelinename` | string or null | The source's own `guidelinename` value, unchanged. Optional. |
| `guidelineurl` | string or null | The source's own `guidelineurl` value, unchanged. Optional. |
| `pairid` | integer (int64) | The source's own `pairid` value, unchanged. |
| `pgxtesting` | string or null | The source's own `pgxtesting` value, unchanged. Optional. |
| `pmids` | list of [Pmid](/reference/schemas/types/#pmid) or null | The source's own `pmids` value, unchanged. Optional. |
| `provisional` | boolean | The source's own `provisional` value, unchanged. |
| `usedforrecommendation` | string | The source's own `usedforrecommendation` value, unchanged. |

## Next

[`biomcp search pgx`](/reference/commands/search-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
