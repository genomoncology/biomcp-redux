---
title: Reply of biomcp get adverse-event reactions section
  (biomcp.adverse_event_reactions/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event
  reactions: a real example and the fields of schema
  biomcp.adverse_event_reactions/1."
goal: "Parse the biomcp.adverse_event_reactions/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_reactions/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event reactions`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_reactions/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 26907716 reactions
```

`biomcp schema biomcp.adverse_event_reactions/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 26907716 reactions` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.adverse_event_reactions/1",
  "status": "complete",
  "query": {
    "id": "26907716",
    "source": "faers",
    "section": "reactions",
    "section_limit": 5,
    "section_offset": 0
  },
  "result": {
    "header": {
      "id": "26907716",
      "source": "faers",
      "version": "1",
      "serious": "1",
      "first_received": "20260622"
    },
    "snapshot": "04a589ed93a30065c4532d2eb2fa5f8083ca9716d4185b8d2e91711430d7…",
    "reactions": {
      "records": [
        {
          "index": 0,
          "term": {"label": "Rash"},
          "vocabulary_version": "29.0",
          "outcome": "6"
        },
        {
          "index": 1,
          "term": {"label": "Cardiomyopathy"},
          "vocabulary_version": "29.0",
          "outcome": "6"
        },
        … 3 more
      ],
      "offset": 0,
      "total": 5
    },
    "components": [
      {
        "component": "meddra",
        "terms": {
          "usage": "restricted",
          "name": "MedDRA statement on data sharing; third-party terminology",
          "url": "https://admin.meddra.org/sites/default/files/page/documents/…"
        }
      }
    ]
  },
  "sources": [
    {
      "source": "faers",
      "use": "open",
      "terms": "openFDA terms and license; third-party exceptions",
      "terms_url": "https://open.fda.gov/terms/",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [AdverseEventReactions](#adverseeventreactions). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventReactions

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | yes | Applicable component terms. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | yes | Exact source report identity and version. |
| `reactions` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventReaction](#adverseeventreaction) | yes | One page of complete native rows. |
| `snapshot` | [Text](/reference/schemas/types/#text) | yes | Opaque retained source snapshot. |

### AdverseEventReaction

Independent native reaction row without drug attribution.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `index` | integer (uint32) | yes | Absolute native reaction index. |
| `outcome` | string or null | no | Native outcome string; present empty values remain present. |
| `term` | [Concept](/reference/schemas/types/#concept) | yes | Source term label without invented CURIE. |
| `vocabulary_version` | string or null | no | Native vocabulary_version string; present empty values remain present. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments.
