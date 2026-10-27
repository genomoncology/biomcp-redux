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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search protein`](/reference/commands/search-protein/) replies with `schema` set to `biomcp.protein_page/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search protein --gene BRAF
biomcp search protein EGFR
```

`biomcp schema biomcp.protein_page/1` prints the full JSON Schema.

## Example reply

`biomcp search protein --gene BRAF` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "protein": {
          "accession": "P15056",
          "name": "Serine/threonine-protein kinase B-raf",
          "length": 766
        },
        "entry_id": "BRAF_HUMAN",
        "gene_names": ["BRAF", "BRAF1", "RAFB1"],
        "taxon": 9606,
        "reviewed": true,
        "url": "https://www.uniprot.org/uniprotkb/P15056/entry"
      }
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
    {
      "kind": "disambiguate",
      "args": ["get", "protein", "P15056"]
    }
  ]
}
```

## Fields

`result` holds [ProteinPage](#proteinpage). `query` holds [ProteinSearch](#proteinsearch). `query` repeats the request as BioMCP read it.

### ProteinPage

Source-ordered human reviewed protein headers.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ProteinHeader](/reference/schemas/types/#proteinheader) | yes | Whole native rows in source order; totals apply only to this collection. |

### ProteinSearch

Search five reviewed human UniProt headers by literal text or exact gene. No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `gene` | [GeneSymbol](/reference/schemas/types/#genesymbol) | no | Exact native gene field, without identifier inference. |
| `limit` | integer (uint32) | no | Five rows by default; no more than 25. |
| `offset` | integer (uint32) | no | Source/retained result offset. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Exact retained collection identity. |
| `query` | [Text](/reference/schemas/types/#text) | no | Literal text, exclusive with gene. |
| `reviewed` | boolean | no | Reviewed human scope stays enabled; unreviewed search is deferred. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search protein`](/reference/commands/search-protein/) gives the command's arguments.
