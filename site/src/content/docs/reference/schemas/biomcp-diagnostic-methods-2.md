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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic methods`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_methods/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic GTR000006692.3 methods --source gtr
```

`biomcp schema biomcp.diagnostic_methods/2` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic GTR000006692.3 methods --source gtr` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "test_id": "GTR000006692.3",
        "list": "method_categories",
        "index": 0,
        "value": "Deletion/duplication analysis"
      },
      {
        "test_id": "GTR000006692.3",
        "list": "method_categories",
        "index": 1,
        "value": "Sequence analysis of the entire coding region"
      },
      … 3 more
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

`result` holds [DiagnosticMethods](#diagnosticmethods). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticMethods

Independent native method entries without positional pairing.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | list of [GtrMethod](/reference/schemas/types/#gtrmethod) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments.
