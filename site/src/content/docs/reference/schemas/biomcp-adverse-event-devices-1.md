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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event devices`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_devices/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 50941416 devices --source maude
```

`biomcp schema biomcp.adverse_event_devices/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 50941416 devices --source maude` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      "records": [
        {…}
      ],
      "offset": 0,
      "total": 1
    },
    "components": [
      {
        "component": "gmdn",
        "terms": {…}
      }
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

`result` holds [AdverseEventDevices](#adverseeventdevices). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventDevices

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `devices` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventDevice](#adverseeventdevice) | yes | One page of complete native rows. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | yes | Exact source report identity and version. |
| `snapshot` | [Text](/reference/schemas/types/#text) | yes | Opaque retained source snapshot. |

### AdverseEventDevice

Native device fields; administrative addresses are excluded.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `brand_name` | string or null | no | Native brand_name string; present empty values remain present. |
| `catalog` | string or null | no | Native catalog string; present empty values remain present. |
| `combination_product_flag` | string or null | no | Native combination_product_flag string; present empty values remain present. |
| `date_received` | string or null | no | Native date_received string; present empty values remain present. |
| `date_removed_flag` | string or null | no | Native date_removed_flag string; present empty values remain present. |
| `date_removed_year` | string or null | no | Native date_removed_year string; present empty values remain present. |
| `device_age_text` | string or null | no | Native device_age_text string; present empty values remain present. |
| `device_availability` | string or null | no | Native device_availability string; present empty values remain present. |
| `device_evaluated_by_manufacturer` | string or null | no | Native device_evaluated_by_manufacturer string; present empty values remain present. |
| `device_event_key` | string or null | no | Native device_event_key string; present empty values remain present. |
| `device_operator` | string or null | no | Native device_operator string; present empty values remain present. |
| `expiration_date_of_device` | string or null | no | Native expiration_date_of_device string; present empty values remain present. |
| `generic_name` | string or null | no | Native generic_name string; present empty values remain present. |
| `implant_date_year` | string or null | no | Native implant_date_year string; present empty values remain present. |
| `implant_flag` | string or null | no | Native implant_flag string; present empty values remain present. |
| `index` | integer (uint32) | yes | Absolute native device index. |
| `lot_number` | string or null | no | Native lot_number string; present empty values remain present. |
| `manufacturer` | string or null | no | Native manufacturer string; present empty values remain present. |
| `model` | string or null | no | Native model string; present empty values remain present. |
| `other_id_number` | string or null | no | Native other_id_number string; present empty values remain present. |
| `product_code` | string or null | no | Native product_code string; present empty values remain present. |
| `sequence` | string or null | no | Native sequence string; present empty values remain present. |
| `serviced_by_3rd_party_flag` | string or null | no | Native serviced_by_3rd_party_flag string; present empty values remain present. |
| `udi_di` | string or null | no | Native udi_di string; present empty values remain present. |
| `udi_public` | string or null | no | Native udi_public string; present empty values remain present. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments.
