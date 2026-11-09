---
title: Reply of biomcp get diagnostic conditions section
  (biomcp.diagnostic_conditions/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic
  conditions: a real example and the fields of schema
  biomcp.diagnostic_conditions/2."
goal: "Parse the biomcp.diagnostic_conditions/2 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_conditions/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic conditions`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_conditions/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic GTR000006692.3 conditions --source gtr
```

`biomcp schema biomcp.diagnostic_conditions/2` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic GTR000006692.3 conditions --source gtr` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.diagnostic_conditions/2",
  "status": "complete",
  "query": {
    "id": "GTR000006692.3",
    "source": "gtr",
    "section": "conditions",
    "section_offset": 0
  },
  "result": {
    "items": [
      {
        "test_id": "GTR000006692.3",
        "test_type": "Clinical",
        "condition": {
          "id": "MedGen:C2676676",
          "label": "Breast-ovarian cancer, familial, susceptibility to, 1"
        },
        "mim_number": "604370",
        "snomed_ct_id": ""
      }
    ],
    "count_unit": "native_relation_rows",
    "distinct_id_total": 1
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "gtr",
      "use": "open",
      "terms": "NCBI usage policies; submitter and third-party rights retained",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiagnosticConditions](#diagnosticconditions). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticConditions

Native GTR condition rows without ontology or clinical conversion.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [GtrConditionRelation](/reference/schemas/types/#gtrconditionrelation) | Complete source-owned rows in native order; envelope totals count native rows. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
