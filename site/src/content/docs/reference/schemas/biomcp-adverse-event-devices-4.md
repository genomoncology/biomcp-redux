---
title: Reply of biomcp get adverse-event (biomcp.adverse-event-devices/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event:
  the fields of schema biomcp.adverse-event-devices/4."
goal: "Parse the biomcp.adverse-event-devices/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-devices/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.adverse-event-devices/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-devices/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventDevices](#reply-adverseeventdevices). `query` holds [Reply_AdverseEventGet](/reference/schemas/types/#reply-adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventDevices

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel) | Applicable component terms. Optional. |
| `devices` | [Reply_AdverseEventRecords_AdverseEventDevice](#reply-adverseeventrecords-adverseeventdevice) | One page of complete native rows. |
| `header` | [Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader) | Exact source report identity and version. |
| `snapshot` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Opaque retained source snapshot. Optional. |

### Other types in this reply

2 more types appear only in this reply: [Reply_AdverseEventDevice](#reply-adverseeventdevice), [Reply_AdverseEventRecords_AdverseEventDevice](#reply-adverseeventrecords-adverseeventdevice). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_AdverseEventDevice

Native device fields; administrative addresses are excluded.

| Field | Type | Meaning |
| --- | --- | --- |
| `brand_name` | string or null | Native brand_name string; present empty values remain present. Optional. |
| `catalog` | string or null | Native catalog string; present empty values remain present. Optional. |
| `combination_product_flag` | string or null | Native combination_product_flag string; present empty values remain present. Optional. |
| `date_received` | string or null | Native date_received string; present empty values remain present. Optional. |
| `date_removed_flag` | string or null | Native date_removed_flag string; present empty values remain present. Optional. |
| `date_removed_year` | string or null | Native date_removed_year string; present empty values remain present. Optional. |
| `device_age_text` | string or null | Native device_age_text string; present empty values remain present. Optional. |
| `device_availability` | string or null | Native device_availability string; present empty values remain present. Optional. |
| `device_evaluated_by_manufacturer` | string or null | Native device_evaluated_by_manufacturer string; present empty values remain present. Optional. |
| `device_event_key` | string or null | Native device_event_key string; present empty values remain present. Optional. |
| `device_operator` | string or null | Native device_operator string; present empty values remain present. Optional. |
| `expiration_date_of_device` | string or null | Native expiration_date_of_device string; present empty values remain present. Optional. |
| `generic_name` | string or null | Native generic_name string; present empty values remain present. Optional. |
| `implant_date_year` | string or null | Native implant_date_year string; present empty values remain present. Optional. |
| `implant_flag` | string or null | Native implant_flag string; present empty values remain present. Optional. |
| `index` | integer (uint32) | Absolute native device index. |
| `lot_number` | string or null | Native lot_number string; present empty values remain present. Optional. |
| `manufacturer` | string or null | Native manufacturer string; present empty values remain present. Optional. |
| `model` | string or null | Native model string; present empty values remain present. Optional. |
| `other_id_number` | string or null | Native other_id_number string; present empty values remain present. Optional. |
| `product_code` | string or null | Native product_code string; present empty values remain present. Optional. |
| `sequence` | string or null | Native sequence string; present empty values remain present. Optional. |
| `serviced_by_3rd_party_flag` | string or null | Native serviced_by_3rd_party_flag string; present empty values remain present. Optional. |
| `udi_di` | string or null | Native udi_di string; present empty values remain present. Optional. |
| `udi_public` | string or null | Native udi_public string; present empty values remain present. Optional. |

### Reply_AdverseEventRecords_AdverseEventDevice

One complete-row page tied to the retained report snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Native section offset. |
| `records` | list of [Reply_AdverseEventDevice](#reply-adverseeventdevice) | Source ordered whole rows. Optional. |
| `total` | integer (uint64) | Complete selected section length, not report or patient count. |

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
