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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic supplements`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_supplements/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic P110020 supplements --source fda-pma
```

`biomcp schema biomcp.diagnostic_supplements/1` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic P110020 supplements --source fda-pma` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "pma_number": "P110020",
        "supplement_number": "S003",
        "trade_name": "COBAS 4800 BRAF V600 MUTATION TEST",
        "applicant": "Roche Molecular Systems, Inc.",
        "decision_date": "2012-09-04",
        "decision_code": "APPR",
        "url": "https://www.accessdata.fda.gov/scripts/cdrh/cfdocs/cfPMA/pma.c…"
      },
      {
        "pma_number": "P110020",
        "supplement_number": "S025",
        "trade_name": "Roche cobas DNA Sample Preparation Kit, Roche cobas 48…",
        "applicant": "Roche Molecular Systems, Inc.",
        "decision_date": "2018-07-03",
        "decision_code": "OK30",
        "url": "https://www.accessdata.fda.gov/scripts/cdrh/cfdocs/cfPMA/pma.c…"
      },
      … 3 more
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
    {
      "kind": "disambiguate",
      "args": [
        "get", "diagnostic", "P110020", "--source", "fda-pma", "--supplement",
        "S025"
      ]
    },
    … 5 more
  ]
}
```

## Fields

`result` holds [DiagnosticSupplements](#diagnosticsupplements). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticSupplements

Native supplement records retaining exact identity and independent dated decisions.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | list of [DiagnosticPmaHeader](/reference/schemas/types/#diagnosticpmaheader) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments.
