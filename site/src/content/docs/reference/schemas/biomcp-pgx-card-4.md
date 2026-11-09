---
title: Reply of biomcp get pgx (biomcp.pgx-card/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get pgx: the fields
  of schema biomcp.pgx-card/4."
goal: "Parse the biomcp.pgx-card/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pgx-card/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get pgx`](/reference/commands/get-pgx/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.pgx-card/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pgx-card/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PgxCard](#reply_pgxcard). `query` holds [Reply_PgxGet](/reference/schemas/types/#reply_pgxget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_PgxCard

Compact exact source gene identity, not a selected gene-drug pair.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [Reply_PgxHeader](/reference/schemas/types/#reply_pgxheader) | Selected gene and source. |
| `identity` | [Reply_PgxGeneIdentity](#reply_pgxgeneidentity) | Native record version is not a dataset release. |

### Other types in this reply

One more type appear only in this reply: [Reply_PgxGeneIdentity](#reply_pgxgeneidentity). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_PgxGeneIdentity

Source-native PgxGeneIdentity fields; no clinical interpretation.

| Field | Type | Meaning |
| --- | --- | --- |
| `clinpgxid` | [Reply_ClinPgxAccession](/reference/schemas/types/#reply_clinpgxaccession) or null | The source's own `clinpgxid` value, unchanged. Optional. |
| `ensemblid` | [Reply_EnsemblGeneId](/reference/schemas/types/#reply_ensemblgeneid) or null | The source's own `ensemblid` value, unchanged. Optional. |
| `hgncid` | [Reply_HgncId](/reference/schemas/types/#reply_hgncid) or null | The source's own `hgncid` value, unchanged. Optional. |
| `lookupmethod` | string or null | The source's own `lookupmethod` value, unchanged. Optional. |
| `ncbiid` | [Reply_NcbiGeneId](/reference/schemas/types/#reply_ncbigeneid) or null | The source's own `ncbiid` value, unchanged. Optional. |
| `symbol` | [Reply_GeneSymbol](/reference/schemas/types/#reply_genesymbol) | The source's own `symbol` value, unchanged. |
| `version` | integer or null | The source's own `version` value, unchanged. Optional. |

</details>

## Next

[`biomcp get pgx`](/reference/commands/get-pgx/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
