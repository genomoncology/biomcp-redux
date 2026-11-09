---
title: Reply of biomcp get drug (biomcp.drug_who_card/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: a real
  example and the fields of schema biomcp.drug_who_card/1."
goal: "Parse the biomcp.drug_who_card/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug_who_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) replies with `schema` set to `biomcp.drug_who_card/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get drug 'EMA Art 58 H-W-2319-2' --source who-pq
```

`biomcp schema biomcp.drug_who_card/1` prints the full JSON Schema.

## Example reply

`biomcp get drug 'EMA Art 58 H-W-2319-2' --source who-pq` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.drug_who_card/1",
  "status": "failed",
  "query": {
    "input": "EMA Art 58 H-W-2319-2",
    "source": "who-pq"
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

`result` holds [DrugWhoCard](#drugwhocard). `query` holds [DrugWhoGet](#drugwhoget). `query` repeats the request as BioMCP read it.

### DrugWhoCard

Source-scoped native rows for one exact WHO reference.

| Field | Type | Meaning |
| --- | --- | --- |
| `native_rows` | list of [DrugWhoRow](/reference/schemas/types/#drugwhorow) | Every native row for that reference, without invented fields. |
| `reference` | [Text](/reference/schemas/types/#text) | Exact requested and source-matched reference. |

### DrugWhoGet

WHO exact-reference selector keeps the shared DrugId contract unchanged.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | [Text](/reference/schemas/types/#text) | Exact WHO reference number, including native spaces; no name or alias resolution. |
| `source` | [DrugSource](/reference/schemas/types/#drugsource) | Explicit source who-pq; prequalification is source-scoped. |

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
