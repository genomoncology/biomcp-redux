---
title: Reply of biomcp get adverse-event devices section
  (biomcp.adverse_event_devices/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event
  devices: a real example and the fields of schema
  biomcp.adverse_event_devices/1."
goal: "Parse the biomcp.adverse_event_devices/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_devices/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event devices`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_devices/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 50941416 devices --source maude
```

`biomcp schema biomcp.adverse_event_devices/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 50941416 devices --source maude` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.adverse_event_devices/1",
  "status": "complete",
  "query": {
    "id": "50941416",
    "source": "maude",
    "section": "devices",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "header": {
      "id": "50941416",
      "source": "maude",
      "event_type": "Malfunction",
      "first_received": "20260930"
    },
    "snapshot": "2bb8441c980469973b92b2ba332304633e4c026606abfbca200de5f72fe3…",
    "devices": {
      "records": […],
      "offset": 0,
      "total": 1
    },
    "components": [
      {…}
    ]
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "maude",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventDevices](#adverseeventdevices). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventDevices

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `devices` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventDevice](#adverseeventdevice) | One page of complete native rows. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Exact source report identity and version. |
| `snapshot` | [Text](/reference/schemas/types/#text) | Opaque retained source snapshot. |

### Other types in this reply

One more type appear only in this reply: [AdverseEventDevice](#adverseeventdevice). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### AdverseEventDevice

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

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
