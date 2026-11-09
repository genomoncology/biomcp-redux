---
title: Reply of biomcp get adverse-event (biomcp.adverse-event-narrative/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event:
  the fields of schema biomcp.adverse-event-narrative/4."
goal: "Parse the biomcp.adverse-event-narrative/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-narrative/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.adverse-event-narrative/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-narrative/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventNarrative](#reply_adverseeventnarrative). `query` holds [Reply_AdverseEventGet](/reference/schemas/types/#reply_adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventNarrative

A bounded report section; all rows belong to the exact header and snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component terms. Optional. |
| `header` | [Reply_AdverseEventHeader](/reference/schemas/types/#reply_adverseeventheader) | Exact source report identity and version. |
| `narrative` | [Reply_AdverseEventRecords_AdverseEventNarrativeItem](#reply_adverseeventrecords_adverseeventnarrativeitem) | One page of complete native rows. |
| `snapshot` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Opaque retained source snapshot. Optional. |

### Other types in this reply

2 more types appear only in this reply: [Reply_AdverseEventNarrativeItem](#reply_adverseeventnarrativeitem), [Reply_AdverseEventRecords_AdverseEventNarrativeItem](#reply_adverseeventrecords_adverseeventnarrativeitem). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_AdverseEventNarrativeItem

Whole unchanged narrative row; oversized rows require a too-large Problem.

| Field | Type | Meaning |
| --- | --- | --- |
| `index` | integer (uint32) | Absolute native narrative index. |
| `mdr_text_key` | string or null | Native mdr_text_key string; present empty values remain present. Optional. |
| `patient_sequence_number` | string or null | Native patient_sequence_number string; present empty values remain present. Optional. |
| `text` | string or null | Unchanged complete source text; never silently truncated. Optional. |
| `type` | string or null | Native narrative text type code. Optional. |

### Reply_AdverseEventRecords_AdverseEventNarrativeItem

One complete-row page tied to the retained report snapshot.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Native section offset. |
| `records` | list of [Reply_AdverseEventNarrativeItem](#reply_adverseeventnarrativeitem) | Source ordered whole rows. Optional. |
| `total` | integer (uint64) | Complete selected section length, not report or patient count. |

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
