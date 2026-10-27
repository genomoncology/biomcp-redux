---
title: Reply of biomcp get pgx diplotype section (biomcp.pgx_diplotype/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx diplotype: a
  real example and the fields of schema biomcp.pgx_diplotype/1."
goal: "Parse the biomcp.pgx_diplotype/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_diplotype/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx diplotype`](/reference/commands/get-pgx/) and [`biomcp get pgx`](/reference/commands/get-pgx/) reply with `schema` set to `biomcp.pgx_diplotype/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx diplotype --gene CYP2C19 --source cpic --diplotype '*2/*2'
```

`biomcp schema biomcp.pgx_diplotype/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx diplotype --gene CYP2C19 --source cpic --diplotype '*2/*2'` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pgx_diplotype/1",
  "status": "complete",
  "query": {
    "gene": "CYP2C19",
    "source": "cpic",
    "section": "diplotype",
    "diplotype": "*2/*2",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "header": {"source": "cpic", "gene": "CYP2C19"},
    "diplotypes": [
      {
        "genesymbol": "CYP2C19",
        "diplotype": "*2/*2",
        "function1": "No function",
        "function2": "No function",
        "activityvalue1": "n/a",
        "activityvalue2": "n/a",
        "totalactivityscore": "n/a",
        "description": "An individual carrying two no function alleles",
        "diplotypekey": {"CYP2C19": {"*2": 2}},
        "generesult": "Poor Metabolizer",
        "ehrpriority": "Abnormal/Priority/High Risk",
        "consultationtext": "This result signifies that the patient has two c…",
        "lookupkey": {"CYP2C19": "Poor Metabolizer"}
      }
    ]
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
        "get", "pgx", "recommendations", "--gene", "CYP2C19", "--source", "cpic"
      ]
    }
  ]
}
```

## Fields

`result` holds [PgxDiplotype](#pgxdiplotype). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxDiplotype

Every source-returned mapping for the exact declared expression.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `diplotypes` | list of [PgxDiplotypeResult](#pgxdiplotyperesult) | yes | Zero, one or multiple native mappings; never a selected patient phenotype. |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | yes | Explicit gene and source. |

### PgxDiplotypeResult

Source-native PgxDiplotypeResult fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `activityvalue1` | string | yes | The source's own `activityvalue1` value, unchanged. |
| `activityvalue2` | string | yes | The source's own `activityvalue2` value, unchanged. |
| `consultationtext` | string | yes | The source's own `consultationtext` value, unchanged. |
| `description` | string | yes | The source's own `description` value, unchanged. |
| `diplotype` | string | yes | The source's own `diplotype` value, unchanged. |
| `diplotypekey` | object | yes | The source's own `diplotypekey` value, unchanged. |
| `ehrpriority` | string | yes | The source's own `ehrpriority` value, unchanged. |
| `function1` | string | yes | The source's own `function1` value, unchanged. |
| `function2` | string | yes | The source's own `function2` value, unchanged. |
| `generesult` | string | yes | The source's own `generesult` value, unchanged. |
| `genesymbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | yes | The source's own `genesymbol` value, unchanged. |
| `lookupkey` | object | yes | The source's own `lookupkey` value, unchanged. |
| `totalactivityscore` | string | yes | The source's own `totalactivityscore` value, unchanged. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments.
