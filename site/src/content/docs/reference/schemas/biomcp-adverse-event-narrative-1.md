---
title: Reply of biomcp get adverse-event narrative section
  (biomcp.adverse_event_narrative/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event
  narrative: a real example and the fields of schema
  biomcp.adverse_event_narrative/1."
goal: "Parse the biomcp.adverse_event_narrative/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_narrative/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event narrative`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_narrative/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 50941416 narrative --source maude
```

`biomcp schema biomcp.adverse_event_narrative/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 50941416 narrative --source maude` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.adverse_event_narrative/1",
  "status": "complete",
  "query": {
    "id": "50941416",
    "source": "maude",
    "section": "narrative",
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
    "narrative": {"records": [], "offset": 0, "total": 0},
    "components": []
  },
  "page": {"offset": 0, "returned": 0, "total": 0},
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

`result` holds [AdverseEventNarrative](#adverseeventnarrative). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventNarrative

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Exact source report identity and version. |
| `narrative` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventNarrativeItem](#adverseeventnarrativeitem) | One page of complete native rows. |
| `snapshot` | [Text](/reference/schemas/types/#text) | Opaque retained source snapshot. |

### AdverseEventNarrativeItem

Whole unchanged narrative row; oversized rows require a too-large Problem.

| Field | Type | Meaning |
| --- | --- | --- |
| `index` | integer (uint32) | Absolute native narrative index. |
| `mdr_text_key` | string or null | Native mdr_text_key string; present empty values remain present. Optional. |
| `patient_sequence_number` | string or null | Native patient_sequence_number string; present empty values remain present. Optional. |
| `text` | string or null | Unchanged complete source text; never silently truncated. Optional. |
| `type` | string or null | Native narrative text type code. Optional. |

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
