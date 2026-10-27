---
title: Reply of biomcp get adverse-event (biomcp.adverse_event_maude_card/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event: a
  real example and the fields of schema biomcp.adverse_event_maude_card/1."
goal: "Parse the biomcp.adverse_event_maude_card/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_maude_card/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_maude_card/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 50941416 --source maude
```

`biomcp schema biomcp.adverse_event_maude_card/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 50941416 --source maude` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.adverse_event_maude_card/1",
  "status": "complete",
  "query": {
    "id": "50941416",
    "source": "maude",
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
    "report_number": "2017865-2026-20439",
    "event_date": "20260910",
    "received": "20260930",
    "date_report": "20260929",
    "number_devices_in_event": "",
    "number_patients_in_event": "",
    "device_count": 1,
    "narrative_count": 0,
    "source_url": "https://open.fda.gov/apis/device/event/",
    "report_url": "https://api.fda.gov/device/event.json?search=mdr_report_ke…",
    "available_sections": ["devices", "narrative"],
    "components": []
  },
  "sources": [
    {
      "source": "maude",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "get", "adverse-event", "50941416", "devices", "--snapshot",
        "2bb8441c980469973b92b2ba332304633e4c026606abfbca200de5f72fe332d2",
        "--source", "maude"
      ]
    },
    {
      "kind": "section",
      "args": [
        "get", "adverse-event", "50941416", "narrative", "--snapshot",
        "2bb8441c980469973b92b2ba332304633e4c026606abfbca200de5f72fe332d2",
        "--source", "maude"
      ]
    }
  ]
}
```

## Fields

`result` holds [AdverseEventMaudeCard](#adverseeventmaudecard). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventMaudeCard

MAUDE card; native device/patient counts remain separate from array lengths.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `available_sections` | list of [AdverseEventSection](/reference/schemas/types/#adverseeventsection) | yes | Named sections available in this report. |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `date_report` | string or null | no | Native date_report string; present empty values remain present. |
| `date_report_to_fda` | string or null | no | Native date_report_to_fda string; present empty values remain present. |
| `device_count` | integer (uint32) | yes | Actual device-array length. |
| `event_date` | string or null | no | Native event_date string; present empty values remain present. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | yes | Exact selected report identity. |
| `narrative_count` | integer (uint32) | yes | Actual narrative-array length. |
| `number_devices_in_event` | string or null | no | Native number_devices_in_event string; present empty values remain present. |
| `number_patients_in_event` | string or null | no | Native number_patients_in_event string; present empty values remain present. |
| `received` | string or null | no | Native received string; present empty values remain present. |
| `report_date` | string or null | no | Native report_date string; present empty values remain present. |
| `report_number` | string or null | no | Native report_number string; present empty values remain present. |
| `report_url` | [Text](/reference/schemas/types/#text) | yes | Exact successful source request URL, never guessed. |
| `snapshot` | [Text](/reference/schemas/types/#text) | yes | Opaque checked retained-report snapshot identity. |
| `source_url` | [Text](/reference/schemas/types/#text) | yes | Publisher overview, never a next step. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments.
