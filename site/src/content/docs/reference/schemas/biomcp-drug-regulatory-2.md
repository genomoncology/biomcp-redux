---
title: Reply of biomcp get drug regulatory section (biomcp.drug_regulatory/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug regulatory:
  a real example and the fields of schema biomcp.drug_regulatory/2."
goal: "Parse the biomcp.drug_regulatory/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_regulatory/2
kind: reference
verified: 2026-10-28
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug regulatory`](/reference/commands/get-drug/) and [`biomcp get drug`](/reference/commands/get-drug/) reply with `schema` set to `biomcp.drug_regulatory/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug EMEA/H/C/002409 regulatory --source ema
```

`biomcp schema biomcp.drug_regulatory/2` prints the full JSON Schema.

## Example reply

`biomcp get drug EMEA/H/C/002409 regulatory --source ema` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_regulatory/2",
  "status": "complete",
  "query": {
    "input": "EMEA/H/C/002409",
    "source": "ema",
    "section": ["regulatory"]
  },
  "result": {
    "identity": {
      "id": "EMEA/H/C/002409",
      "source": "ema",
      "name": "Zelboraf",
      "identity_level": "unknown",
      "type": "Human",
      "url": "https://www.ema.europa.eu/en/medicines/human/EPAR/zelboraf"
    },
    "regulatory": {
      "state": "data",
      "source": "ema",
      "data": {
        "records": […],
        "offset": 0,
        "total": 1,
        "version": "2026-10-27T18:19:10Z"
      }
    }
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "ema",
      "use": "attribution",
      "terms": "EMA reproduction and attribution terms; third-party exceptions",
      "terms_url": "https://www.ema.europa.eu/en/about-us/about-website/legal…",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DrugRegulatory](#drugregulatory). `query` holds [DrugGet](/reference/schemas/types/#drugget), a shared type. `query` repeats the request as BioMCP read it.

### DrugRegulatory

Source-scoped regulator events preserving date roles and original status.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [DrugHeader](/reference/schemas/types/#drugheader) | Selected native identity, checked independently of section facts. |
| `regulatory` | [Section](/reference/schemas/types/#section) of [DrugRecords](/reference/schemas/types/#drugrecords) of [AuthorizationEvent](/reference/schemas/types/#authorizationevent) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
