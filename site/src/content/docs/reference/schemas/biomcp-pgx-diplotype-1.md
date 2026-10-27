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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx diplotype`](/reference/commands/get-pgx/) and [`biomcp get pgx`](/reference/commands/get-pgx/) reply with `schema` set to `biomcp.pgx_diplotype/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx diplotype --gene CYP2C19 --source cpic --diplotype '*2/*2'
```

`biomcp schema biomcp.pgx_diplotype/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx diplotype --gene CYP2C19 --source cpic --diplotype '*2/*2'` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…}
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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PgxDiplotype](#pgxdiplotype). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxDiplotype

Every source-returned mapping for the exact declared expression.

| Field | Type | Meaning |
| --- | --- | --- |
| `diplotypes` | list of [PgxDiplotypeResult](#pgxdiplotyperesult) | Zero, one or multiple native mappings; never a selected patient phenotype. |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | Explicit gene and source. |

### PgxDiplotypeResult

Source-native PgxDiplotypeResult fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `activityvalue1` | string | The source's own `activityvalue1` value, unchanged. |
| `activityvalue2` | string | The source's own `activityvalue2` value, unchanged. |
| `consultationtext` | string | The source's own `consultationtext` value, unchanged. |
| `description` | string | The source's own `description` value, unchanged. |
| `diplotype` | string | The source's own `diplotype` value, unchanged. |
| `diplotypekey` | object | The source's own `diplotypekey` value, unchanged. |
| `ehrpriority` | string | The source's own `ehrpriority` value, unchanged. |
| `function1` | string | The source's own `function1` value, unchanged. |
| `function2` | string | The source's own `function2` value, unchanged. |
| `generesult` | string | The source's own `generesult` value, unchanged. |
| `genesymbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | The source's own `genesymbol` value, unchanged. |
| `lookupkey` | object | The source's own `lookupkey` value, unchanged. |
| `totalactivityscore` | string | The source's own `totalactivityscore` value, unchanged. |

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
