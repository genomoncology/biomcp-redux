---
title: Reply of biomcp protein isoforms (biomcp.protein_isoforms/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein isoforms: a
  real example and the fields of schema biomcp.protein_isoforms/1."
goal: "Parse the biomcp.protein_isoforms/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_isoforms/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein isoforms`](/reference/commands/protein-isoforms/) replies with `schema` set to `biomcp.protein_isoforms/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp protein isoforms P00533
biomcp protein isoforms P00533 --limit 1
```

`biomcp schema biomcp.protein_isoforms/1` prints the full JSON Schema.

## Example reply

`biomcp protein isoforms P00533` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_isoforms/1",
  "status": "complete",
  "query": {"accession": "P00533", "limit": 5, "offset": 0},
  "result": {
    "items": [
      {
        "ids": ["P00533-1"],
        "name": "1",
        "synonyms": ["p170"],
        "sequence_status": "Displayed",
        "direct_sequence": "not_checked"
      },
      {
        "ids": ["P00533-2"],
        "name": "2",
        "synonyms": ["p60", "Truncated", "TEGFR"],
        "sequence_status": "Described",
        "sequence_ids": ["VSP_002887", "VSP_002888"],
        "direct_sequence": "not_checked"
      },
      … 2 more
    ]
  },
  "page": {
    "offset": 0,
    "returned": 4,
    "total": 4,
    "pool": {
      "id": "cf489a61b5b17ffbf1c84ac9d7357653",
      "size": 4,
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
  ]
}
```

## Fields

`result` holds [ProteinIsoforms](/reference/schemas/types/#proteinisoforms), a shared type. `query` holds [ProteinRelationQuery](/reference/schemas/types/#proteinrelationquery), a shared type. `query` repeats the request as BioMCP read it.

Every type this reply uses is shared with other replies, so its fields are on the shared types page.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp protein isoforms`](/reference/commands/protein-isoforms/) gives the command's arguments.
