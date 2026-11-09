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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event reactions`](/reference/commands/get-adverse-event/) replies with `schema` set to `biomcp.adverse_event_reactions/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get adverse-event 26907716 reactions
```

`biomcp schema biomcp.adverse_event_reactions/1` prints the full JSON Schema.

## Example reply

`biomcp get adverse-event 26907716 reactions` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      "records": […],
      "offset": 0,
      "total": 5
    },
    "components": [
      {…}
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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventReactions](#adverseeventreactions). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventReactions

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [ComponentLabel](/reference/schemas/types/#componentlabel) | Applicable component terms. |
| `header` | [AdverseEventHeader](/reference/schemas/types/#adverseeventheader) | Exact source report identity and version. |
| `reactions` | [AdverseEventRecords](/reference/schemas/types/#adverseeventrecords) of [AdverseEventReaction](#adverseeventreaction) | One page of complete native rows. |
| `snapshot` | [Text](/reference/schemas/types/#text) | Opaque retained source snapshot. |

### Other types in this reply

One more type appear only in this reply: [AdverseEventReaction](#adverseeventreaction). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### AdverseEventReaction

Independent native reaction row without drug attribution.

| Field | Type | Meaning |
| --- | --- | --- |
| `index` | integer (uint32) | Absolute native reaction index. |
| `outcome` | string or null | Native outcome string; present empty values remain present. Optional. |
| `term` | [Concept](/reference/schemas/types/#concept) | Source term label without invented CURIE. |
| `vocabulary_version` | string or null | Native vocabulary_version string; present empty values remain present. Optional. |

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
