---
title: Reply of biomcp get drug (biomcp.drug-label/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-label/7."
goal: "Parse the biomcp.drug-label/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-label/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-label/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-label/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugLabel](#reply_druglabel). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply_drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugLabel

One explicit native label heading and text continuation.

| Field | Type | Meaning |
| --- | --- | --- |
| `hint` | [Reply_Text](/reference/schemas/types/#reply_text) or null | How to read one of those sections. Optional. |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply_drugheader) | Selected native identity, checked independently of section facts. |
| `label` | [Reply_Section_DrugLabelText](#reply_section_druglabeltext) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `sections` | list of [Reply_Text](/reference/schemas/types/#reply_text) | Label section names present for this record, listed when no label_section was given. Optional. |

### Other types in this reply

4 more types appear only in this reply: [Reply_DrugLabelFragment](#reply_druglabelfragment), [Reply_DrugLabelReference](#reply_druglabelreference), [Reply_DrugLabelText](#reply_druglabeltext), [Reply_Section_DrugLabelText](#reply_section_druglabeltext). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### Reply_DrugLabelFragment

One slice of a native label array element, partitioning the page's unchanged text.

| Field | Type | Meaning |
| --- | --- | --- |
| `characters` | integer (uint32) | Consecutive Unicode scalars consumed from the page text for this fragment. |
| `source_offset` | integer (uint32) | Unicode scalar offset inside that original array element. |
| `source_part` | integer (uint32) | Zero-based native array element; empty elements have a zero-character fragment. |

### Reply_DrugLabelReference

One independently identified label set and revision.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply_attribution) | Label provenance. |
| `id` | [Reply_DrugId](/reference/schemas/types/#reply_drugid) | Published SPL set or native label record ID. |
| `revised` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | Source-stated revision date. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native label URL. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native label revision. Optional. |

### Reply_DrugLabelText

A mechanical continuation of one native label heading.

| Field | Type | Meaning |
| --- | --- | --- |
| `fragments` | list of [Reply_DrugLabelFragment](#reply_druglabelfragment) | Native array boundaries; consume the flat text in this order without separators. Optional. |
| `heading` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact native label heading. |
| `label` | [Reply_DrugLabelReference](#reply_druglabelreference) | Exact source label set and revision. |
| `offset` | integer (uint32) | Zero-based bounded text fragment offset; section_limit is a maximum fragment count. |
| `retrieved` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | Original source reply retrieval date; absent for legacy captures without a timestamp. Optional. |
| `text` | string | Original text slice, preserving whitespace and wording. |
| `total` | integer or null | Total bounded text fragments, including fragments for empty native array elements. Optional. |

### Reply_Section_DrugLabelText

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugLabelText](#reply_druglabeltext)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
