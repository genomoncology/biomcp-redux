---
title: Reply of biomcp get pgx guidelines section (biomcp.pgx_guidelines/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx guidelines:
  a real example and the fields of schema biomcp.pgx_guidelines/1."
goal: "Parse the biomcp.pgx_guidelines/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_guidelines/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx guidelines`](/reference/commands/get-pgx/) replies with `schema` set to `biomcp.pgx_guidelines/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx guidelines --gene DPYD --source cpic
```

`biomcp schema biomcp.pgx_guidelines/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx guidelines --gene DPYD --source cpic` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pgx_guidelines/1",
  "status": "complete",
  "query": {
    "gene": "DPYD",
    "source": "cpic",
    "section": "guidelines",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "header": {"source": "cpic", "gene": "DPYD"},
    "items": [
      {
        "guideline_name": "DPYD and Fluoropyrimidines",
        "guideline_url": "https://www.clinpgx.org/guideline/PA166251462",
        "drugs": ["capecitabine", "fluorouracil", "tegafur"],
        "genes": [
          {
            "url": "https://cpicpgx.org/gene/dpyd/",
            "symbol": "DPYD"
          }
        ]
      }
    ],
    "page": {
      "offset": 0,
      "returned": 1,
      "total": 1,
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
  ]
}
```

## Fields

`result` holds [PgxGuidelines](#pgxguidelines). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxGuidelines

Independent source-ordered page of complete native records.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | yes | Source and selected gene. |
| `items` | list of [PgxGuidelineReference](#pgxguidelinereference) | yes | Complete native rows without duplicate summaries. |
| `page` | [PgxPageInfo](/reference/schemas/types/#pgxpageinfo) | yes | Meaning and bounds of this collection. |

### PgxGuidelineGene

Source-native PgxGuidelineGene fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `symbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | yes | The source's own `symbol` value, unchanged. |
| `url` | string | yes | The source's own `url` value, unchanged. |

### PgxGuidelineReference

Source-native PgxGuidelineReference fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `drugs` | list of string | yes | The source's own `drugs` value, unchanged. |
| `genes` | list of [PgxGuidelineGene](#pgxguidelinegene) | yes | The source's own `genes` value, unchanged. |
| `guideline_name` | string | yes | The source's own `guideline_name` value, unchanged. |
| `guideline_url` | string | yes | The source's own `guideline_url` value, unchanged. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments.
