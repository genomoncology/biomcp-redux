---
title: Reply of biomcp search diagnostic (biomcp.diagnostic_companion/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search diagnostic: a
  real example and the fields of schema biomcp.diagnostic_companion/1."
goal: "Parse the biomcp.diagnostic_companion/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic_companion/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) replies with `schema` set to `biomcp.diagnostic_companion/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search diagnostic --source fda-cdx --biomarker BRAF --limit 3
biomcp search diagnostic --source fda-cdx --limit 1
```

`biomcp schema biomcp.diagnostic_companion/1` prints the full JSON Schema.

## Example reply

`biomcp search diagnostic --source fda-cdx --biomarker BRAF --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.diagnostic_companion/1",
  "status": "partial",
  "query": {
    "source": "fda-cdx",
    "biomarker": "BRAF",
    "limit": 3,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "kind": "individual",
        "snapshot": "sha256:1bc783e3bab36ce2c04d98b3a6f7900051aceae472490f68e…",
        "index": 6,
        "cells": {…},
        "device_ids": ["P200010/S026"],
        "drug_applications": […]
      },
      … 2 more
    ]
  },
  "page": {"offset": 0, "returned": 3, "total": 18},
  "sources": [
    {
      "source": "fda-cdx",
      "use": "open",
      "terms": "FDA website policies; third-party exceptions retained",
      "terms_url": "https://www.fda.gov/about-fda/about-website/website-polic…",
      "state": "ok",
      "reason": "unsupported"
    }
  ],
  "problems": [
    {
      "source": "fda-cdx",
      "field": "table_1.biomarker",
      "code": "value.unknown"
    }
  ],
  "next": [
    {
      "kind": "pivot",
      "args": [
        "get", "diagnostic", "P200010", "--source", "fda-pma", "--supplement",
        "S026"
      ]
    },
    … 7 more
  ]
}
```

## Fields

`result` holds [DiagnosticCompanion](#diagnosticcompanion). `query` holds [DiagnosticSearch](/reference/schemas/types/#diagnosticsearch), a shared type. `query` repeats the request as BioMCP read it.

### DiagnosticCompanion

Whole distinct physical companion-table rows with cell-owned identifiers.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count_unit` | [DiagnosticCountUnit](/reference/schemas/types/#diagnosticcountunit) or null | no | Exact meaning of the envelope total; supplied by the owning source command. |
| `distinct_id_total` | integer or null | no | Distinct complete native IDs across the selected relation collection. |
| `items` | list of [CompanionDiagnosticRelation](#companiondiagnosticrelation) | yes | Complete source-owned rows in native order; envelope totals count native rows. |

### CompanionDiagnosticRelation

A physical companion-table row; the two table layouts cannot acquire each other’s cells.

One of these forms:

- `kind` is `"individual"`: One complete native individual table row. Fields: `cells` ([CompanionIndividualCells](#companionindividualcells)), `device_ids` (list of [DiagnosticDeviceId](/reference/schemas/types/#diagnosticdeviceid)), `drug_applications` (list of [DiagnosticDrugApplication](#diagnosticdrugapplication)), `index` (integer (uint32)), `snapshot` (string).
- `kind` is `"group"`: One complete native group table row. Fields: `cells` ([CompanionGroupCells](#companiongroupcells)), `device_ids` (list of [DiagnosticDeviceId](/reference/schemas/types/#diagnosticdeviceid)), `drug_applications` (list of [DiagnosticDrugApplication](#diagnosticdrugapplication)), `index` (integer (uint32)), `snapshot` (string).

### CompanionGroupCells

Exact native CompanionGroupCells headers; any extra or changed header is refused.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA` | [DiagnosticCell](#diagnosticcell) | yes | Complete original drug_group cell. |
| `Diagnostic Name
(Manufacturer)` | [DiagnosticCell](#diagnosticcell) | yes | Complete original diagnostic cell. |
| `Indication(s) - Sample Type` | [DiagnosticCell](#diagnosticcell) | yes | Complete original indication cell. |
| `PMA (Approval Date)` | [DiagnosticCell](#diagnosticcell) | yes | Complete original device cell. |

### CompanionIndividualCells

Exact native CompanionIndividualCells headers; any extra or changed header is refused.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `Biomarker(s)` | [DiagnosticCell](#diagnosticcell) | yes | Complete original biomarker cell. |
| `Biomarker(s) (Details)` | [DiagnosticCell](#diagnosticcell) | yes | Complete original details cell. |
| `Diagnostic Name (Manufacturer)` | [DiagnosticCell](#diagnosticcell) | yes | Complete original diagnostic cell. |
| `Drug Trade Name (Generic) NDA / BLA` | [DiagnosticCell](#diagnosticcell) | yes | Complete original drug cell. |
| `Indication - Sample Type` | [DiagnosticCell](#diagnosticcell) | yes | Complete original indication cell. |
| `PMA / 510(k) /
513(f)(2) / HDE (Approval / Clearance / Grant Date)` | [DiagnosticCell](#diagnosticcell) | yes | Complete original device cell. |

### DiagnosticCell

Whole physical source-table cell and its own links.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `links` | list of [DiagnosticCellLink](#diagnosticcelllink) | yes | Only links physically owned by this cell, in native order. |
| `text` | string | yes | Original complete text, including empty strings and line breaks. |

### DiagnosticCellLink

Source link without a name-based identity inference.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `href` | string | yes | Original source URL. |
| `label` | string or null | no | Native label when the capture supplies one. |

### DiagnosticDrugApplication

A typed drug application established by its own complete source cell and link.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [FdaDrugApplication](/reference/schemas/types/#fdadrugapplication) | yes | Published NDA/BLA prefix and six digits. |
| `url` | string | yes | The same cell’s owned source URL. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) gives the command's arguments.
