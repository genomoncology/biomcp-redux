---
title: Reply of biomcp search protein (biomcp.protein_page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search protein: a
  real example and the fields of schema biomcp.protein_page/1."
goal: "Parse the biomcp.protein_page/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search protein`](/reference/commands/search-protein/) replies with `schema` set to `biomcp.protein_page/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search protein --gene BRAF
biomcp search protein EGFR
```

`biomcp schema biomcp.protein_page/1` prints the full JSON Schema.

## Example reply

`biomcp search protein --gene BRAF` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.protein_page/1",
  "status": "complete",
  "query": {
    "gene": "BRAF",
    "reviewed": true,
    "limit": 5,
    "offset": 0
  },
  "result": {
    "items": [
      {…}
    ]
  },
  "page": {
    "offset": 0,
    "returned": 1,
    "total": 1,
    "pool": {
      "id": "3b64f965eb83f262375ed7173f33d3c0",
      "size": 1,
      "created": 1793140312,
      "expires": 1793226712
    },
    "truncated": false
  },
  "sources": [
    {
      "source": "uniprot",
      "use": "attribution",
      "terms": "CC BY 4.0",
      "terms_url": "https://www.uniprot.org/help/license",
      "state": "ok"
    }
  ],
  "next": [
    {"kind": "disambiguate", "args": ["get", "protein", "P15056"]}
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ProteinPage](#proteinpage). `query` holds [ProteinSearch](#proteinsearch). `query` repeats the request as BioMCP read it.

### ProteinPage

Source-ordered human reviewed protein headers.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ProteinHeader](/reference/schemas/types/#proteinheader) | Whole native rows in source order; totals apply only to this collection. |

### ProteinSearch

Search five reviewed human UniProt headers by literal text or exact gene. No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | Exact native gene field, without identifier inference. Optional. |
| `limit` | integer (uint32) | Five rows by default; no more than 25. Optional. |
| `offset` | integer (uint32) | Source/retained result offset. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Exact retained collection identity. Optional. |
| `query` | [Text](/reference/schemas/types/#text) | Literal text, exclusive with gene. Optional. |
| `reviewed` | boolean | Reviewed human scope stays enabled; unreviewed search is deferred. Optional. |

## Next

[`biomcp search protein`](/reference/commands/search-protein/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
