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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) replies with `schema` set to `biomcp.pgx_card/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get pgx --gene DPYD --source cpic
```

`biomcp schema biomcp.pgx_card/1` prints the full JSON Schema.

## Example reply

`biomcp get pgx --gene DPYD --source cpic` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
    … 2 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [PgxCard](#pgxcard). `query` holds [PgxGet](/reference/schemas/types/#pgxget), a shared type. `query` repeats the request as BioMCP read it.

### PgxCard

Compact exact source gene identity, not a selected gene-drug pair.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [PgxHeader](/reference/schemas/types/#pgxheader) | Selected gene and source. |
| `identity` | [PgxGeneIdentity](#pgxgeneidentity) | Native record version is not a dataset release. |

### Other types in this reply

One more type appear only in this reply: [PgxGeneIdentity](#pgxgeneidentity). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### PgxGeneIdentity

Source-native PgxGeneIdentity fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `clinpgxid` | [ClinPgxAccession](/reference/schemas/types/#clinpgxaccession) | The source's own `clinpgxid` value, unchanged. |
| `ensemblid` | [EnsemblGeneId](/reference/schemas/types/#ensemblgeneid) | The source's own `ensemblid` value, unchanged. |
| `hgncid` | [HgncId](/reference/schemas/types/#hgncid) | The source's own `hgncid` value, unchanged. |
| `lookupmethod` | string | The source's own `lookupmethod` value, unchanged. |
| `ncbiid` | [NcbiGeneId](/reference/schemas/types/#ncbigeneid) | The source's own `ncbiid` value, unchanged. |
| `symbol` | [GeneSymbol](/reference/schemas/types/#genesymbol) | The source's own `symbol` value, unchanged. |
| `version` | integer (int64) | The source's own `version` value, unchanged. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
