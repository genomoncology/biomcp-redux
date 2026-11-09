---
title: Reply of biomcp search drug (biomcp.drug_page/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search drug: a real
  example and the fields of schema biomcp.drug_page/3."
goal: "Parse the biomcp.drug_page/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_page/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search drug`](/reference/commands/search-drug/) replies with `schema` set to `biomcp.drug_page/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search drug Zelboraf --source ema --limit 1
biomcp search drug daraxonrasib --limit 1
```

`biomcp schema biomcp.drug_page/3` prints the full JSON Schema.

## Example reply

`biomcp search drug Zelboraf --source ema --limit 1` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_page/3",
  "status": "complete",
  "query": {
    "query": "Zelboraf",
    "source": "ema",
    "limit": 1,
    "offset": 0
  },
  "result": {
    "items": [
      {…}
    ],
    "total": 1
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "ema",
      "use": "attribution",
      "terms": "EMA reproduction and attribution terms; third-party exceptions",
      "terms_url": "https://www.ema.europa.eu/en/about-us/about-website/legal…",
      "state": "ok",
      "count": 1,
      "fetched": 1
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "drug", "EMEA/H/C/002409", "--source", "ema"]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugPage](#drugpage). `query` holds [DrugSearch](/reference/schemas/types/#drugsearch), a shared type. `query` repeats the request as BioMCP read it.

### DrugPage

A single source candidate page in native order, without selecting the first hit.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [DrugHeader](/reference/schemas/types/#drugheader) | Native candidates, preserving their separate identity levels. Optional. |
| `total` | integer or null | Native total when supplied; absent remains unknown. Optional. |

## Next

[`biomcp search drug`](/reference/commands/search-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
