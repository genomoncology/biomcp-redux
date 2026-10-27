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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic regulatory`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_regulatory/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic P110020 regulatory --source fda-pma
```

`biomcp schema biomcp.diagnostic_regulatory/1` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic P110020 regulatory --source fda-pma` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      {
        "submission": {
          "pma_number": "P110020",
          "supplement_number": "",
          "trade_name": "COBAS 4800 BRAF V600 MUTATION TEST",
          "applicant": "Roche Molecular Systems, Inc.",
          "decision_date": "2011-08-17",
          "decision_code": "APPR",
          "url": "https://www.accessdata.fda.gov/scripts/cdrh/cfdocs/cfPMA/pma…"
        },
        "metadata": {
          "date_received": "2011-04-25",
          "fed_reg_notice_date": "2011-08-19",
          "advisory_committee": "PA",
          "advisory_committee_description": "Pathology",
          "product_code": "OWD",
          "supplement_type": "",
          "supplement_reason": ""
        },
        "field": "ao_statement",
        "offset": 0,
        "total": 1,
        "text": "APPROVAL FOR THE COBAS 4800 BRAF V600 MUTATION TEST.  THIS DE…"
      }
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

`result` holds [DiagnosticRegulatory](#diagnosticregulatory). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticRegulatory

Exact selected submission regulatory text fragments.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | list of [DiagnosticRegulatoryFragment](#diagnosticregulatoryfragment) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

### DiagnosticPmaRegulatory

Dated metadata belonging only to the selected submission.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `advisory_committee` | string or null | no | Native advisory_committee when supplied, including empty. |
| `advisory_committee_description` | string or null | no | Native advisory_committee_description when supplied, including empty. |
| `date_received` | string or null | no | Native date_received when supplied, including empty. |
| `fed_reg_notice_date` | string or null | no | Native fed_reg_notice_date when supplied, including empty. |
| `product_code` | string or null | no | Native product_code when supplied, including empty. |
| `supplement_reason` | string or null | no | Native supplement_reason when supplied, including empty. |
| `supplement_type` | string or null | no | Native supplement_type when supplied, including empty. |

### DiagnosticRegulatoryField

Regulatory text available in this source tier.

Values: `ao_statement`.

### DiagnosticRegulatoryFragment

One bounded regulatory fragment from the exact selected native submission.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `field` | [DiagnosticRegulatoryField](#diagnosticregulatoryfield) | yes | Native text field being continued. |
| `metadata` | [DiagnosticPmaRegulatory](#diagnosticpmaregulatory) | yes | Native metadata for this exact record, without a duplicated whole statement. |
| `offset` | integer (uint32) | yes | Fragment offset, independent of characters and supplement rows. |
| `submission` | [DiagnosticPmaHeader](/reference/schemas/types/#diagnosticpmaheader) | yes | Exact selected native record and its dated regulatory metadata. |
| `text` | string | no | Complete unmodified text fragment; omitted when the source statement is absent. |
| `total` | integer (uint32) | yes | Total mechanical fragments. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments.
