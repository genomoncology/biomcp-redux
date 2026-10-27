---
title: Reply of biomcp protein domains (biomcp.protein_domains/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp protein domains: a
  real example and the fields of schema biomcp.protein_domains/2."
goal: "Parse the biomcp.protein_domains/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.protein_domains/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein domains`](/reference/commands/protein-domains/) replies with `schema` set to `biomcp.protein_domains/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp protein domains P15056
biomcp protein domains P15056 --limit 2
```

`biomcp schema biomcp.protein_domains/2` prints the full JSON Schema.

## Example reply

`biomcp protein domains P15056 --limit 2` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.protein_domains/2",
  "status": "complete",
  "query": {"accession": "P15056", "limit": 2, "offset": 0},
  "result": {
    "items": [
      {
        "id": "IPR000719",
        "name": "Protein kinase domain",
        "entry_type": "domain",
        "protein": "P15056",
        "protein_length": 766,
        "taxon": 9606,
        "locations": […],
        "member_signatures": […],
        "attribution": {…}
      },
      … 1 more
    ],
    "native_total": 11
  },
  "page": {
    "offset": 0,
    "returned": 2,
    "total": 11,
    "pool": {
      "id": "bdf34f4289605339910e552fd2362611",
      "size": 11,
      "created": 1793140316,
      "expires": 1793226716
    }
  },
  "sources": [
    {
      "source": "interpro",
      "use": "open",
      "terms": "CC0",
      "terms_url": "https://interpro-documentation.readthedocs.io/en/latest/l…",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "protein", "domains", "P15056", "--limit", "2", "--offset", "2",
        "--pool",
        … 1 more
      ]
    }
  ]
}
```

## Fields

`result` holds [ProteinDomains](/reference/schemas/types/#proteindomains), a shared type. `query` holds [ProteinDomainsQuery](#proteindomainsquery). `query` repeats the request as BioMCP read it.

### ProteinDomainsQuery

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `accession` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | yes | Exact source protein sequence identity. |
| `limit` | integer (uint32) | no | Five rows by default, at most 25. |
| `offset` | integer (uint32) | no | Native or retained page offset. |
| `pool` | [Text](/reference/schemas/types/#text) | no | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `type` | [Text](/reference/schemas/types/#text) | no | Exact native InterPro type, not a normalized domain label. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp protein domains`](/reference/commands/protein-domains/) gives the command's arguments.
