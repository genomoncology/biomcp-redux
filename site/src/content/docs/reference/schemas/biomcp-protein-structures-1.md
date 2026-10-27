---
title: Reply of biomcp protein structures (biomcp.protein_structures/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein structures:
  a real example and the fields of schema biomcp.protein_structures/1."
goal: "Parse the biomcp.protein_structures/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_structures/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein structures`](/reference/commands/protein-structures/) replies with `schema` set to `biomcp.protein_structures/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp protein structures P00533
biomcp protein structures P00533 --limit 1
```

`biomcp schema biomcp.protein_structures/1` prints the full JSON Schema.

## Example reply

`biomcp protein structures P00533` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_structures/1",
  "status": "complete",
  "query": {"accession": "P00533", "limit": 5, "offset": 0},
  "result": {
    "items": [
      {
        "database": "PDB",
        "id": "1IVO",
        "url": "https://www.rcsb.org/structure/1IVO",
        "properties": [
          {"key": "Method", "value": "X-ray"},
          {"key": "Resolution", "value": "3.30 A"},
          … 1 more
        ]
      },
      {
        "database": "PDB",
        "id": "1M14",
        "url": "https://www.rcsb.org/structure/1M14",
        "properties": [
          {"key": "Method", "value": "X-ray"},
          {"key": "Resolution", "value": "2.60 A"},
          … 1 more
        ]
      },
      … 3 more
    ]
  },
  "page": {
    "offset": 0,
    "returned": 5,
    "total": 385,
    "pool": {
      "id": "f17150c02d86258ec215038a8900d043",
      "size": 385,
      "created": 1793140312,
      "expires": 1793226712
    }
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
      "kind": "page",
      "args": [
        "protein", "structures", "P00533", "--offset", "5", "--pool",
        "f17150c02d86258ec215038a8900d043"
      ]
    }
  ]
}
```

## Fields

`result` holds [ProteinStructures](/reference/schemas/types/#proteinstructures), a shared type. `query` holds [ProteinRelationQuery](/reference/schemas/types/#proteinrelationquery), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp protein structures`](/reference/commands/protein-structures/) gives the command's arguments.
