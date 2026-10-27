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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein features`](/reference/commands/protein-features/) replies with `schema` set to `biomcp.protein_features/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp protein features P00533
biomcp protein features P00533 --limit 1
```

`biomcp schema biomcp.protein_features/1` prints the full JSON Schema.

## Example reply

`biomcp protein features P00533` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_features/1",
  "status": "complete",
  "query": {"accession": "P00533", "limit": 5, "offset": 0},
  "result": {
    "items": [
      {
        "feature": {
          "protein": "P00533",
          "span": {…},
          "kind": {"label": "Signal"},
          "evidence": [
            "ECO:0000269|PubMed|15340161", "ECO:0000269|PubMed|6328312",
            "ECO:0000269|Reference|Ref.16"
          ]
        },
        "attribution": {
          "source": "uniprot",
          "record": "P00533",
          "version": "301",
          "evaluated": "2026-09-02",
          "url": "https://www.uniprot.org/uniprotkb/P00533/entry"
        },
        "source_coordinates": "one_based_inclusive"
      },
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

`result` holds [ProteinFeatures](/reference/schemas/types/#proteinfeatures), a shared type. `query` holds [ProteinFeaturesQuery](#proteinfeaturesquery). `query` repeats the request as BioMCP read it.

### ProteinFeaturesQuery

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | [Text](/reference/schemas/types/#text) | no | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `position` | integer (uint32) | no | Native one-based residue; zero fails during deserialization. |
| `type` | [Text](/reference/schemas/types/#text) | no | Exact UniProt native label, without an alias map. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp protein features`](/reference/commands/protein-features/) gives the command's arguments.
