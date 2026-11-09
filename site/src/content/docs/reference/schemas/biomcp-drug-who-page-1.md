---
title: Reply of biomcp search drug (biomcp.drug_who_page/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: a real
  example and the fields of schema biomcp.drug_who_page/1."
goal: "Parse the biomcp.drug_who_page/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_who_page/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) replies with `schema` set to `biomcp.drug_who_page/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search drug artesunate --source who-pq --limit 2
```

`biomcp schema biomcp.drug_who_page/1` prints the full JSON Schema.

## Example reply

`biomcp search drug artesunate --source who-pq --limit 2` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_who_page/1",
  "status": "failed",
  "query": {
    "query": "artesunate",
    "source": "who-pq",
    "limit": 2,
    "offset": 0
  },
  "sources": [
    {
      "source": "who-pq",
      "use": "restricted",
      "terms": "WHO copyright policy; export-specific reuse grant unverified",
      "terms_url": "https://www.who.int/about/policies/publishing/copyright",
      "state": "syncing",
      "reason": "syncing"
    }
  ],
  "next": [{"kind": "sync", "args": ["sync", "who-pq"]}]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugWhoPage](#drugwhopage). `query` holds [DrugSearch](/reference/schemas/types/#drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### DrugWhoPage

Source-ordered WHO matches; prequalification does not establish US/EU authorization.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [DrugWhoRow](/reference/schemas/types/#drugwhorow) | Bounded native rows in export order. |
| `total` | integer (uint64) | Matching rows in the complete local snapshot. |

## Next

[`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
