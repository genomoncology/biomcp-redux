---
title: Reply of biomcp get diagnostic regulatory section
  (biomcp.diagnostic_regulatory/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic
  regulatory: a real example and the fields of schema
  biomcp.diagnostic_regulatory/1."
goal: "Parse the biomcp.diagnostic_regulatory/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_regulatory/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic regulatory`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_regulatory/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic P110020 regulatory --source fda-pma
```

`biomcp schema biomcp.diagnostic_regulatory/1` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic P110020 regulatory --source fda-pma` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.diagnostic_regulatory/1",
  "status": "complete",
  "query": {
    "id": "P110020",
    "source": "fda-pma",
    "section": "regulatory",
    "section_offset": 0
  },
  "result": {
    "items": [
      {…}
    ]
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
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
      "kind": "fresh",
      "args": [
        "get", "diagnostic", "P110020", "regulatory", "--source", "fda-pma",
        "--fresh"
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [DiagnosticRegulatory](#diagnosticregulatory). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticRegulatory

Exact selected submission regulatory text fragments.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [DiagnosticRegulatoryFragment](#diagnosticregulatoryfragment) | Complete source-owned rows in native order; envelope totals count native rows. |

### DiagnosticPmaRegulatory

Dated metadata belonging only to the selected submission.

| Field | Type | Meaning |
| --- | --- | --- |
| `advisory_committee` | string or null | Native advisory_committee when supplied, including empty. Optional. |
| `advisory_committee_description` | string or null | Native advisory_committee_description when supplied, including empty. Optional. |
| `date_received` | string or null | Native date_received when supplied, including empty. Optional. |
| `fed_reg_notice_date` | string or null | Native fed_reg_notice_date when supplied, including empty. Optional. |
| `product_code` | string or null | Native product_code when supplied, including empty. Optional. |
| `supplement_reason` | string or null | Native supplement_reason when supplied, including empty. Optional. |
| `supplement_type` | string or null | Native supplement_type when supplied, including empty. Optional. |

### DiagnosticRegulatoryField

Regulatory text available in this source tier.

Values: `ao_statement`.

### DiagnosticRegulatoryFragment

One bounded regulatory fragment from the exact selected native submission.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [DiagnosticRegulatoryField](#diagnosticregulatoryfield) | Native text field being continued. |
| `metadata` | [DiagnosticPmaRegulatory](#diagnosticpmaregulatory) | Native metadata for this exact record, without a duplicated whole statement. |
| `offset` | integer (uint32) | Fragment offset, independent of characters and supplement rows. |
| `submission` | [DiagnosticPmaHeader](/reference/schemas/types/#diagnosticpmaheader) | Exact selected native record and its dated regulatory metadata. |
| `text` | string | Complete unmodified text fragment; omitted when the source statement is absent. Optional. |
| `total` | integer (uint32) | Total mechanical fragments. |

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
