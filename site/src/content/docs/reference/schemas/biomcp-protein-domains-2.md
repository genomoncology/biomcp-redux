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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp protein domains`](/reference/commands/protein-domains/) replies with `schema` set to `biomcp.protein_domains/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp protein domains P15056
biomcp protein domains P15056 --limit 2
```

`biomcp schema biomcp.protein_domains/2` prints the full JSON Schema.

## Example reply

`biomcp protein domains P15056 --limit 2` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.protein_domains/2",
  "status": "complete",
  "query": {"accession": "P15056", "limit": 2, "offset": 0},
  "result": {
    "items": [
      {…},
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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ProteinDomains](/reference/schemas/types/#proteindomains), a shared type. `query` holds [ProteinDomainsQuery](#proteindomainsquery). `query` repeats the request as BioMCP read it.

### ProteinDomainsQuery

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

| Field | Type | Meaning |
| --- | --- | --- |
| `accession` | [UniProtIsoform](/reference/schemas/types/#uniprotisoform) | Exact source protein sequence identity. |
| `limit` | integer (uint32) | Five rows by default, at most 25. Optional. |
| `offset` | integer (uint32) | Native or retained page offset. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | The ID of a stored list, from an earlier reply, so later pages read the same data. Optional. |
| `type` | [Text](/reference/schemas/types/#text) | Exact native InterPro type, not a normalized domain label. Optional. |

## Next

[`biomcp protein domains`](/reference/commands/protein-domains/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
