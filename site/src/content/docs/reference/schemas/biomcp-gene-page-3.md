---
title: Reply of biomcp search gene (biomcp.gene_page/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search gene: a real
  example and the fields of schema biomcp.gene_page/3."
goal: "Parse the biomcp.gene_page/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gene_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search gene`](/reference/commands/search-gene/) replies with `schema` set to `biomcp.gene_page/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search gene BRAF --limit 3
biomcp search gene KRAS
```

`biomcp schema biomcp.gene_page/3` prints the full JSON Schema.

## Example reply

`biomcp search gene BRAF --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.gene_page/3",
  "status": "complete",
  "query": {"query": "BRAF", "limit": 3, "offset": 0},
  "result": {
    "items": [
      {
        "hgnc_id": "HGNC:1097",
        "symbol": "BRAF",
        "locus_group": "protein_coding",
        "location": "7q34",
        "status": "approved",
        "name": "B-Raf proto-oncogene, serine/threonine kinase",
        "match": {"kind": "symbol", "text": "BRAF"}
      },
      … 2 more
    ]
  },
  "page": {"offset": 0, "returned": 3, "total": 3},
  "sources": [
    {
      "source": "hgnc",
      "use": "open",
      "terms": "CC0 1.0",
      "terms_url": "https://www.genenames.org/about/license/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "gene", "HGNC:1097"]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [GenePage](#genepage). `query` holds [GeneSearch](#genesearch). `query` repeats the request as BioMCP read it.

### GenePage

One page of gene headers.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [GeneHeader](/reference/schemas/types/#geneheader) | Headers after ranking and paging. |

### GeneSearch

Search genes by text, cytoband, or their intersection.

| Field | Type | Meaning |
| --- | --- | --- |
| `band` | [Cytoband](/reference/schemas/types/#cytoband) | A cytoband prefix. Optional. |
| `limit` | integer (uint32) | Number of results per page. Optional. |
| `locus_group` | [LocusGroup](/reference/schemas/types/#locusgroup) | Optional HGNC locus group intersected with the selector. Optional. |
| `offset` | integer (uint32) | Number of results to skip. Optional. |
| `query` | [Text](/reference/schemas/types/#text) | Words matched against symbols, previous symbols, aliases and names. Optional. |

## Next

[`biomcp search gene`](/reference/commands/search-gene/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
