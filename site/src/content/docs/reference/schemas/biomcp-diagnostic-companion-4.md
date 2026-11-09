---
title: Reply of biomcp search diagnostic (biomcp.diagnostic-companion/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search diagnostic:
  the fields of schema biomcp.diagnostic-companion/4."
goal: "Parse the biomcp.diagnostic-companion/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-companion/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search diagnostic`](/reference/commands/search-diagnostic/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.diagnostic-companion/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-companion/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticCompanion](#reply-diagnosticcompanion). `query` holds [Reply_DiagnosticSearch](/reference/schemas/types/#reply-diagnosticsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticCompanion

Whole distinct physical companion-table rows with cell-owned identifiers.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [Reply_DiagnosticCountUnit](/reference/schemas/types/#reply-diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [Reply_CompanionDiagnosticRelation](#reply-companiondiagnosticrelation) | Complete source-owned rows in native order; envelope totals count native rows. Optional. |

### Other types in this reply

6 more types appear only in this reply: [Reply_CompanionDiagnosticRelation](#reply-companiondiagnosticrelation), [Reply_CompanionGroupCells](#reply-companiongroupcells), [Reply_CompanionIndividualCells](#reply-companionindividualcells), [Reply_DiagnosticCell](#reply-diagnosticcell), [Reply_DiagnosticCellLink](#reply-diagnosticcelllink), [Reply_DiagnosticDrugApplication](#reply-diagnosticdrugapplication). Open the list below to read their fields.

<details>
<summary>Fields of the 6 other types</summary>

### Reply_CompanionDiagnosticRelation

A physical companion-table row; the two table layouts cannot acquire each other’s cells.

One of these forms:

- `kind` is `"individual"`: One complete native individual table row. Fields: `cells` ([Reply_CompanionIndividualCells](#reply-companionindividualcells)), `device_ids` (list of [Reply_DiagnosticDeviceId](/reference/schemas/types/#reply-diagnosticdeviceid)), `drug_applications` (list of [Reply_DiagnosticDrugApplication](#reply-diagnosticdrugapplication)), `index` (integer (uint32)), `snapshot` (string).
- `kind` is `"group"`: One complete native group table row. Fields: `cells` ([Reply_CompanionGroupCells](#reply-companiongroupcells)), `device_ids` (list of [Reply_DiagnosticDeviceId](/reference/schemas/types/#reply-diagnosticdeviceid)), `drug_applications` (list of [Reply_DiagnosticDrugApplication](#reply-diagnosticdrugapplication)), `index` (integer (uint32)), `snapshot` (string).

### Reply_CompanionGroupCells

Known native group-table cells keyed by their exact source headers.

| Field | Type | Meaning |
| --- | --- | --- |
| `Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original drug_group cell. |
| `Diagnostic Name
(Manufacturer)` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original diagnostic cell. |
| `Indication(s) - Sample Type` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original indication cell. |
| `PMA (Approval Date)` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original device cell. |

### Reply_CompanionIndividualCells

Known native individual-table cells keyed by their exact source headers.

| Field | Type | Meaning |
| --- | --- | --- |
| `Biomarker(s)` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original biomarker cell. |
| `Biomarker(s) (Details)` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original details cell. |
| `Diagnostic Name (Manufacturer)` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original diagnostic cell. |
| `Drug Trade Name (Generic) NDA / BLA` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original drug cell. |
| `Indication - Sample Type` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original indication cell. |
| `PMA / 510(k) /
513(f)(2) / HDE (Approval / Clearance / Grant Date)` | [Reply_DiagnosticCell](#reply-diagnosticcell) | Complete original device cell. |

### Reply_DiagnosticCell

Whole physical source-table cell and its own links.

| Field | Type | Meaning |
| --- | --- | --- |
| `links` | list of [Reply_DiagnosticCellLink](#reply-diagnosticcelllink) | Only links physically owned by this cell, in native order. Optional. |
| `text` | string | Original complete text, including empty strings and line breaks. |

### Reply_DiagnosticCellLink

Source link without a name-based identity inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `href` | string (uri-reference) | Original source URL. |
| `label` | string or null | Native label when the capture supplies one. Optional. |

### Reply_DiagnosticDrugApplication

A typed drug application established by its own complete source cell and link.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_FdaDrugApplication](/reference/schemas/types/#reply-fdadrugapplication) | Published NDA/BLA prefix and six digits. |
| `url` | string (uri) | The same cell’s owned source URL. |

</details>

## Next

[`biomcp search diagnostic`](/reference/commands/search-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
