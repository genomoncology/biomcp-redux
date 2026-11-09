---
title: Reply of biomcp protein features (biomcp.protein_features/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein features: a
  real example and the fields of schema biomcp.protein_features/1."
goal: "Parse the biomcp.protein_features/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_features/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein features`](/reference/commands/protein-features/) replies with `schema` set to `biomcp.protein_features/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp protein features P00533
biomcp protein features P00533 --limit 1
```

`biomcp schema biomcp.protein_features/1` prints the full JSON Schema.

## Example reply

`biomcp protein features P00533` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.protein_features/1",
  "status": "complete",
  "query": {"accession": "P00533", "limit": 5, "offset": 0},
  "result": {
    "items": [
      {…},
      … 4 more
    ]
  },
  "page": {
    "offset": 0,
    "returned": 5,
    "total": 321,
    "pool": {
      "id": "cadd5dd73e3e202c6d62fdd714cafb6c",
      "size": 321,
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
        "protein", "features", "P00533", "--offset", "5", "--pool",
        "cadd5dd73e3e202c6d62fdd714cafb6c"
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ProteinFeatures](/reference/schemas/types/#proteinfeatures), a shared type. `query` holds [ProteinFeaturesQuery](#proteinfeaturesquery). `query` repeats the request as BioMCP read it.

### ProteinFeaturesQuery

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | Exact source protein sequence identity. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |
| `position` | integer (uint32) | Native one-based residue; zero fails during deserialization. Optional. |
| `type` | [Text](/reference/schemas/types/#text) | Exact UniProt native label, without an alias map. Optional. |

## Next

[`biomcp protein features`](/reference/commands/protein-features/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
