---
title: Reply of biomcp get adverse-event (biomcp.adverse-event-maude-card/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event:
  the fields of schema biomcp.adverse-event-maude-card/4."
goal: "Parse the biomcp.adverse-event-maude-card/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-maude-card/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.adverse-event-maude-card/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-maude-card/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventMaudeCard](#reply_adverseeventmaudecard). `query` holds [Reply_AdverseEventGet](/reference/schemas/types/#reply_adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventMaudeCard

MAUDE card; native device/patient counts remain separate from array lengths.

| Field | Type | Meaning |
| --- | --- | --- |
| `available_sections` | list of [Reply_AdverseEventSection](/reference/schemas/types/#reply_adverseeventsection) | Named sections available in this report. Optional. |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component terms. Optional. |
| `date_report` | string or null | Native date_report string; present empty values remain present. Optional. |
| `date_report_to_fda` | string or null | Native date_report_to_fda string; present empty values remain present. Optional. |
| `device_count` | integer (uint32) | Actual device-array length. |
| `event_date` | string or null | Native event_date string; present empty values remain present. Optional. |
| `header` | [Reply_AdverseEventHeader](/reference/schemas/types/#reply_adverseeventheader) | Exact selected report identity. |
| `narrative_count` | integer (uint32) | Actual narrative-array length. |
| `number_devices_in_event` | string or null | Native number_devices_in_event string; present empty values remain present. Optional. |
| `number_patients_in_event` | string or null | Native number_patients_in_event string; present empty values remain present. Optional. |
| `received` | string or null | Native received string; present empty values remain present. Optional. |
| `report_date` | string or null | Native report_date string; present empty values remain present. Optional. |
| `report_number` | string or null | Native report_number string; present empty values remain present. Optional. |
| `report_url` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact successful source request URL, never guessed. |
| `snapshot` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Opaque checked retained-report snapshot identity. Optional. |
| `source_url` | [Reply_Text](/reference/schemas/types/#reply_text) | Publisher overview, never a next step. |

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
