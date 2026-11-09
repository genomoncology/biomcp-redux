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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_maude_card/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 50941416 --source maude
```

`biomcp schema biomcp.adverse_event_maude_card/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 50941416 --source maude` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventMaudeCard](#adverseeventmaudecard). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventMaudeCard

MAUDE card; native device/patient counts remain separate from array lengths.

| Field | Type | Meaning |
| --- | --- | --- |
| `available_sections` | list of [AdverseEventSection](/reference/schemas/types/#adverseeventsection) | Named sections available in this report. |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `date_report` | string or null | Native date_report string; present empty values remain present. Optional. |
| `date_report_to_fda` | string or null | Native date_report_to_fda string; present empty values remain present. Optional. |
| `device_count` | integer (uint32) | Actual device-array length. |
| `event_date` | string or null | Native event_date string; present empty values remain present. Optional. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Exact selected report identity. |
| `narrative_count` | integer (uint32) | Actual narrative-array length. |
| `number_devices_in_event` | string or null | Native number_devices_in_event string; present empty values remain present. Optional. |
| `number_patients_in_event` | string or null | Native number_patients_in_event string; present empty values remain present. Optional. |
| `received` | string or null | Native received string; present empty values remain present. Optional. |
| `report_date` | string or null | Native report_date string; present empty values remain present. Optional. |
| `report_number` | string or null | Native report_number string; present empty values remain present. Optional. |
| `report_url` | [Text](/reference/schemas/types/#text) | Exact successful source request URL, never guessed. |
| `snapshot` | [Text](/reference/schemas/types/#text) | Opaque checked retained-report snapshot identity. |
| `source_url` | [Text](/reference/schemas/types/#text) | Publisher overview, never a next step. |

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
