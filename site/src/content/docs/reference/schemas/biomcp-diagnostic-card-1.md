---
title: Reply of biomcp get diagnostic (biomcp.diagnostic_card/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic: a
  real example and the fields of schema biomcp.diagnostic_card/1."
goal: "Parse the biomcp.diagnostic_card/1 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) replies with `schema` set to `biomcp.diagnostic_card/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get diagnostic K193046 --source fda-510k
```

`biomcp schema biomcp.diagnostic_card/1` prints the full JSON Schema.

## Example reply

`biomcp get diagnostic K193046 --source fda-510k` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.diagnostic_card/1",
  "status": "complete",
  "query": {
    "id": "K193046",
    "source": "fda-510k",
    "section_offset": 0
  },
  "result": {
    "kind": "clearance",
    "k_number": "K193046",
    "device_name": "Straumann® Retentive System  Novaloc TiN Abutments",
    "applicant": "Institut Straumann AG",
    "decision_code": "SESE",
    "decision_description": "Substantially Equivalent",
    "decision_date": "2020-08-13",
    "date_received": "2019-11-01",
    "product_code": "NHA"
  },
  "sources": [
    {
      "source": "fda-510k",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "pivot",
      "args": [
        "search", "diagnostic", "--device-id", "K193046", "--source", "fda-cdx"
      ]
    },
    {
      "kind": "fresh",
      "args": [
        "get", "diagnostic", "K193046", "--source", "fda-510k", "--fresh"
      ]
    }
  ]
}
```

## Fields

`result` holds [DiagnosticCard](#diagnosticcard). `query` holds [DiagnosticGet](/reference/schemas/types/#diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticCard

Exactly one source-owned diagnostic card; conflicts have no card variant.

One of these forms:

- `kind` is `"gtr_test"`: Exact resolved GTR accession/version. Fields: `clia_number` (string), `header` ([GtrHeader](/reference/schemas/types/#gtrheader)), `lab_test_id` (string), `last_touch_date` ([Text](/reference/schemas/types/#text)), `manufacturer_test_name` (string), `native_status` ([GtrNativeStatus](#gtrnativestatus)), `sections` (list of [DiagnosticSection](/reference/schemas/types/#diagnosticsection)), `state_license_numbers` (string), `state_licenses` (string), `test_create_date` ([Text](/reference/schemas/types/#text)), `test_deletion_date` (string), `test_development` (string), `test_order_code` (string).
- `kind` is `"who_ivd_product"`: All native rows under one WHO Product ID. Fields: `id` ([WhoProductId](/reference/schemas/types/#whoproductid)), `native_rows` (list of [WhoIvdRow](#whoivdrow)).
- `kind` is `"pma_submission"`: Exact native FDA base or supplement, never an application-wide inferred claim. Fields: `header` ([DiagnosticPmaHeader](/reference/schemas/types/#diagnosticpmaheader)), `sections` (list of [DiagnosticSection](/reference/schemas/types/#diagnosticsection)).
- `kind` is `"clearance"`: Exact native 510(k) clearance, separate from PMA approval. Fields: `applicant` (string or null), `date_received` (string or null), `decision_code` (string or null), `decision_date` (string or null), `decision_description` (string or null), `device_name` (string or null), `k_number` ([DiagnosticDeviceId](/reference/schemas/types/#diagnosticdeviceid)), `product_code` (string or null).

### GtrNativeStatus

Native flags retain distinct source meanings and exact source codes.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `lab_currStat` | string | yes | Native laboratory current status. |
| `lab_pubStat` | string | yes | Native laboratory public status. |
| `now_current` | string | yes | Current version flag, not current clinical availability. |
| `test_currStat` | string | yes | Native test current status. |
| `test_pubStat` | string | yes | Native test public status. |

### WhoIvdRow

One original WHO CSV row; each repeated field retains its own row scope.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `assay_format` | string | yes | Complete native assay format cell; no interpretation or inferred date. |
| `manufacturer` | string | yes | Complete native manufacturer cell; no interpretation or inferred date. |
| `marker` | string | yes | Complete native marker cell; no interpretation or inferred date. |
| `prequalification_year` | string | yes | Complete native prequalification year cell; no interpretation or inferred date. |
| `product_code` | string | yes | Complete native product code cell; no interpretation or inferred date. |
| `product_name` | string | yes | Complete native product name cell; no interpretation or inferred date. |
| `regulatory_version` | string | yes | Complete native regulatory version cell; no interpretation or inferred date. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments.
