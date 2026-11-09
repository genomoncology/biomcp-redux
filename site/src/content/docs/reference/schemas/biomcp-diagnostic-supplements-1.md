---
title: Reply of biomcp get diagnostic supplements section
  (biomcp.diagnostic_supplements/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic
  supplements: a real example and the fields of schema
  biomcp.diagnostic_supplements/1."
goal: "Parse the biomcp.diagnostic_supplements/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_supplements/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic supplements`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_supplements/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic P110020 supplements --source fda-pma
```

`biomcp schema biomcp.diagnostic_supplements/1` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic P110020 supplements --source fda-pma` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.diagnostic_supplements/1",
  "status": "complete",
  "query": {
    "id": "P110020",
    "source": "fda-pma",
    "section": "supplements",
    "section_offset": 0
  },
  "result": {
    "items": [
      {…},
      … 4 more
    ]
  },
  "page": {"offset": 0, "returned": 5, "total": 44},
  "sources": [
    {
      "source": "fda-pma",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": [
        "get", "diagnostic", "P110020", "--source", "fda-pma", "--supplement",
        "S003"
      ]
    },
    … 6 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiagnosticSupplements](#diagnosticsupplements). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticSupplements

Native supplement records retaining exact identity and independent dated decisions.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [DiagnosticPmaHeader](/reference/schemas/types/#diagnosticpmaheader) | Complete source-owned rows in native order; envelope totals count native rows. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
