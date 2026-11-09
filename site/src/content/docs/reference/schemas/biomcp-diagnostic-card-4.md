---
title: Reply of biomcp get diagnostic (biomcp.diagnostic-card/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic: the
  fields of schema biomcp.diagnostic-card/4."
goal: "Parse the biomcp.diagnostic-card/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-card/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.diagnostic-card/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-card/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticCard](#reply_diagnosticcard). `query` holds [Reply_DiagnosticGet](/reference/schemas/types/#reply_diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticCard

Exactly one source-owned diagnostic card; conflicts have no card variant.

One of these forms:

- `kind` is `"device-recall"`: Exact native medical device recall with an optional exact enforcement classification. Fields: `classification` ([Reply_Text](/reference/schemas/types/#reply_text) or null), `distribution_pattern` ([Reply_Text](/reference/schemas/types/#reply_text) or null), `header` ([Reply_DeviceRecallHeader](/reference/schemas/types/#reply_devicerecallheader)), `k_numbers` (list of [Reply_Text](/reference/schemas/types/#reply_text) or null), `pma_numbers` (list of [Reply_Text](/reference/schemas/types/#reply_text) or null), `product_description` ([Reply_Text](/reference/schemas/types/#reply_text) or null), `reason` ([Reply_Text](/reference/schemas/types/#reply_text) or null), `root_cause` ([Reply_Text](/reference/schemas/types/#reply_text) or null).
- `kind` is `"gtr-test"`: Exact resolved GTR accession/version. Fields: `clia_number` (string), `header` ([Reply_GtrHeader](/reference/schemas/types/#reply_gtrheader)), `lab_test_id` (string), `last_touch_date` ([Reply_Text](/reference/schemas/types/#reply_text) or null), `manufacturer_test_name` (string), `native_status` ([Reply_GtrNativeStatus](#reply_gtrnativestatus)), `sections` (list of [Reply_DiagnosticSection](/reference/schemas/types/#reply_diagnosticsection)), `state_license_numbers` (string), `state_licenses` (string), `test_create_date` ([Reply_Text](/reference/schemas/types/#reply_text) or null), `test_deletion_date` (string), `test_development` (string), `test_order_code` (string).
- `kind` is `"who-ivd-product"`: All native rows under one WHO Product ID. Fields: `id` ([Reply_WhoProductId](/reference/schemas/types/#reply_whoproductid)), `native_rows` (list of [Reply_WhoIvdRow](#reply_whoivdrow)).
- `kind` is `"pma-submission"`: Exact native FDA base or supplement, never an application-wide inferred claim. Fields: `header` ([Reply_DiagnosticPmaHeader](/reference/schemas/types/#reply_diagnosticpmaheader)), `sections` (list of [Reply_DiagnosticSection](/reference/schemas/types/#reply_diagnosticsection)).
- `kind` is `"clearance"`: Exact native 510(k) clearance, separate from PMA approval. Fields: `applicant` (string or null), `date_received` (string or null), `decision_code` (string or null), `decision_date` (string or null), `decision_description` (string or null), `device_name` (string or null), `k_number` ([Reply_DiagnosticDeviceId](/reference/schemas/types/#reply_diagnosticdeviceid)), `product_code` (string or null).

### Other types in this reply

2 more types appear only in this reply: [Reply_GtrNativeStatus](#reply_gtrnativestatus), [Reply_WhoIvdRow](#reply_whoivdrow). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_GtrNativeStatus

Native flags retain distinct source meanings and exact source codes.

| Field | Type | Meaning |
| --- | --- | --- |
| `lab_currStat` | string | Native laboratory current status. |
| `lab_pubStat` | string | Native laboratory public status. |
| `now_current` | string | Current version flag, not current clinical availability. |
| `test_currStat` | string | Native test current status. |
| `test_pubStat` | string | Native test public status. |

### Reply_WhoIvdRow

One original WHO CSV row; each repeated field retains its own row scope.

| Field | Type | Meaning |
| --- | --- | --- |
| `assay_format` | string | Complete native assay format cell; no interpretation or inferred date. |
| `manufacturer` | string | Complete native manufacturer cell; no interpretation or inferred date. |
| `marker` | string | Complete native marker cell; no interpretation or inferred date. |
| `prequalification_year` | string | Complete native prequalification year cell; no interpretation or inferred date. |
| `product_code` | string | Complete native product code cell; no interpretation or inferred date. |
| `product_name` | string | Complete native product name cell; no interpretation or inferred date. |
| `regulatory_version` | string | Complete native regulatory version cell; no interpretation or inferred date. |

</details>

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
