---
title: Reply of biomcp get drug interactions section (biomcp.drug_interactions/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug
  interactions: a real example and the fields of schema
  biomcp.drug_interactions/3."
goal: "Parse the biomcp.drug_interactions/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_interactions/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug interactions`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_interactions/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug DDInter1925 interactions --source ddinter --section-limit 3
```

`biomcp schema biomcp.drug_interactions/3` prints the full JSON Schema.

## Example reply

`biomcp get drug DDInter1925 interactions --source ddinter --section-limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_interactions/3",
  "status": "complete",
  "query": {
    "input": "DDInter1925",
    "source": "ddinter",
    "section": ["interactions"],
    "section_limit": 3
  },
  "result": {
    "identity": {
      "id": "DDInter1925",
      "source": "ddinter",
      "name": "Vemurafenib",
      "identity_level": "unknown"
    },
    "interactions": {
      "state": "data",
      "source": "ddinter",
      "data": {…}
    }
  },
  "page": {"offset": 0, "returned": 3, "total": 536},
  "sources": [
    {
      "source": "ddinter",
      "use": "noncommercial",
      "terms": "DDInter CC BY-NC-SA 4.0",
      "terms_url": "https://ddinter.scbdd.com/terms/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "drug", "DDInter1925", "interactions", "--section-limit", "3",
        "--section-offset", "3",
        … 2 more
      ]
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugInteractions](#druginteractions). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugInteractions

Native DDInter partner rows and levels without invented management text.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |
| `interactions` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [DrugInteractionClaim](#druginteractionclaim) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### DrugInteractionClaim

One native interaction pair and level, without invented management advice.

| Field | Type | Meaning |
| --- | --- | --- |
| `additional_attributions` | list of [Attribution](/reference/schemas/types/#attribution) | Further distinct provenance, excluding the primary attribution. Optional. |
| `anchor` | [DrugIdentity](/reference/schemas/types/#drugidentity) | Selected native anchor ID and name. |
| `attribution` | [Attribution](/reference/schemas/types/#attribution) | Primary pair record and [bundle](/local-data/) provenance. |
| `level` | [Text](/reference/schemas/types/#text) | Original native level, without a computed severity. |
| `partner` | [DrugIdentity](/reference/schemas/types/#drugidentity) | Native partner ID and name. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
