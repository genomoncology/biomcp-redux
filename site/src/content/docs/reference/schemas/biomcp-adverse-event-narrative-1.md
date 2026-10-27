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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event narrative`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_narrative/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 50941416 narrative --source maude
```

`biomcp schema biomcp.adverse_event_narrative/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 50941416 narrative --source maude` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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

`result` holds [AdverseEventNarrative](#adverseeventnarrative). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventNarrative

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | yes | Exact source report identity and version. |
| `narrative` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventNarrativeItem](#adverseeventnarrativeitem) | yes | One page of complete native rows. |
| `snapshot` | [Text](/reference/schemas/types/#text) | yes | Opaque retained source snapshot. |

### AdverseEventNarrativeItem

Whole unchanged narrative row; oversized rows require a too-large Problem.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `index` | integer (uint32) | yes | Absolute native narrative index. |
| `mdr_text_key` | string or null | no | Native mdr_text_key string; present empty values remain present. |
| `patient_sequence_number` | string or null | no | Native patient_sequence_number string; present empty values remain present. |
| `text` | string or null | no | Unchanged complete source text; never silently truncated. |
| `type` | string or null | no | Native narrative text type code. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments.
