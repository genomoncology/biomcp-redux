---
title: Reply of biomcp get diagnostic methods section (biomcp.diagnostic_methods/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic
  methods: a real example and the fields of schema biomcp.diagnostic_methods/2."
goal: "Parse the biomcp.diagnostic_methods/2 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_methods/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic methods`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_methods/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic GTR000006692.3 methods --source gtr
```

`biomcp schema biomcp.diagnostic_methods/2` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic GTR000006692.3 methods --source gtr` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.diagnostic_methods/2",
  "status": "complete",
  "query": {
    "id": "GTR000006692.3",
    "source": "gtr",
    "section": "methods",
    "section_offset": 0
  },
  "result": {
    "items": [
      {…},
      … 4 more
    ],
    "count_unit": "native_method_entries"
  },
  "page": {"offset": 0, "returned": 5, "total": 6},
  "sources": [
    {
      "source": "gtr",
      "use": "open",
      "terms": "NCBI usage policies; submitter and third-party rights retained",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "get", "diagnostic", "GTR000006692.3", "methods", "--bundle-revision",
        "sha256:421cbbba145a93789b4dd2020dd4f64a0e7fa4b51e2a3cf2c0c184d4bf55e…",
        "--section-offset", "5",
        … 2 more
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiagnosticMethods](#diagnosticmethods). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticMethods

Independent native method entries without positional pairing.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [GtrMethod](/reference/schemas/types/#gtrmethod) | Complete source-owned rows in native order; envelope totals count native rows. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
