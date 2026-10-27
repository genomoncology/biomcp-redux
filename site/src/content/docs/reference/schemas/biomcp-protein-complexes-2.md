---
title: Reply of biomcp protein complexes (biomcp.protein_complexes/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein complexes: a
  real example and the fields of schema biomcp.protein_complexes/2."
goal: "Parse the biomcp.protein_complexes/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_complexes/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein complexes`](/reference/commands/protein-complexes/) replies with `schema` set to `biomcp.protein_complexes/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp protein complexes P15056 --limit 2
biomcp protein complexes P00533
```

`biomcp schema biomcp.protein_complexes/2` prints the full JSON Schema.

## Example reply

`biomcp protein complexes P15056 --limit 2` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_complexes/2",
  "status": "complete",
  "query": {"accession": "P15056", "limit": 2, "offset": 0},
  "result": {
    "items": [
      {
        "id": "CPX-10863",
        "name": "LNP1:FOXO3:PFKFB2:SRGAP2:SASH1:BRAF:TFEB:PTPN3:CDC25B:HDAC4:…",
        "taxon": 9606,
        "predicted": true,
        "participant_total": 29,
        "matched_participant": "P15056",
        "url": "https://www.ebi.ac.uk/complexportal/complex/CPX-10863"
      },
      … 1 more
    ],
    "native_total": 18
  },
  "page": {
    "offset": 0,
    "returned": 2,
    "total": 12,
    "pool": {
      "id": "de6422a12e88e963a5bd5b11ce2c052a",
      "size": 12,
      "created": 1793140311,
      "expires": 1793226711
    }
  },
  "sources": [
    {
      "source": "complex-portal",
      "use": "open",
      "terms": "CC0",
      "terms_url": "https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "protein", "complex-members", "P15056", "--complex", "CPX-10863",
        "--pool", "05e5d93feb81dda9c8e73e637aa0e5f8"
      ]
    },
    … 2 more
  ]
}
```

## Fields

`result` holds [ProteinComplexes](/reference/schemas/types/#proteincomplexes), a shared type. `query` holds [ProteinRelationQuery](/reference/schemas/types/#proteinrelationquery), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp protein complexes`](/reference/commands/protein-complexes/) gives the command's arguments.
