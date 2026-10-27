---
title: Reply of biomcp get pgx (biomcp.pgx_card/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: a real
  example and the fields of schema biomcp.pgx_card/1."
goal: "Parse the biomcp.pgx_card/1 reply: see a real example and what each field
  means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) replies with `schema` set to `biomcp.pgx_card/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx --gene DPYD --source cpic
```

`biomcp schema biomcp.pgx_card/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx --gene DPYD --source cpic` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.pgx_card/1",
  "status": "complete",
  "query": {
    "gene": "DPYD",
    "source": "cpic",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "header": {"source": "cpic", "gene": "DPYD"},
    "identity": {
      "symbol": "DPYD",
      "hgncid": "HGNC:3012",
      "ncbiid": "282617",
      "ensemblid": "ENSG00000188641",
      "clinpgxid": "PA145",
      "lookupmethod": "ACTIVITY_SCORE",
      "version": 463
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
        "get", "pgx", "recommendations", "--gene", "DPYD", "--source", "cpic"
      ]
    },
    {
      "kind": "section",
      "args": [
        "get", "pgx", "guidelines", "--gene", "DPYD", "--source", "cpic"
      ]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [PgxCard](#pgxcard). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxCard

Compact exact source gene identity, not a selected gene-drug pair.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | yes | Selected gene and source. |
| `identity` | [PgxGeneIdentity](#pgxgeneidentity) | yes | Native record version is not a dataset release. |

### PgxGeneIdentity

Source-native PgxGeneIdentity fields; no clinical interpretation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `clinpgxid` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | yes | The source's own `clinpgxid` value, unchanged. |
| `ensemblid` | [EnsemblGeneId](/reference/schemas/types/#ensemblgeneid) | yes | The source's own `ensemblid` value, unchanged. |
| `hgncid` | [HgncId](/reference/schemas/types/#hgncid) | yes | The source's own `hgncid` value, unchanged. |
| `lookupmethod` | string | yes | The source's own `lookupmethod` value, unchanged. |
| `ncbiid` | [NcbiGeneId](/reference/schemas/types/#ncbigeneid) | yes | The source's own `ncbiid` value, unchanged. |
| `symbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | yes | The source's own `symbol` value, unchanged. |
| `version` | integer (int64) | yes | The source's own `version` value, unchanged. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments.
