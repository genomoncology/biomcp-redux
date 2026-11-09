---
title: Reply of biomcp get adverse-event (biomcp.adverse-event-identifiers/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get adverse-event:
  the fields of schema biomcp.adverse-event-identifiers/4."
goal: "Parse the biomcp.adverse-event-identifiers/4 reply: see what each field
  means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-identifiers/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get adverse-event`](/reference/commands/get-adverse-event/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.adverse-event-identifiers/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-identifiers/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventIdentifiers](#reply-adverseeventidentifiers). `query` holds [Reply_AdverseEventGet](/reference/schemas/types/#reply-adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventIdentifiers

Exactly one closed named identifier page in its original report snapshot.

One of these forms:

- `section` is `"unii"`: A single original identifier array. Fields: `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null), `unii` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)).
- `section` is `"rxcui"`: A single original identifier array. Fields: `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `rxcui` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null).
- `section` is `"product-ndc"`: A single original identifier array. Fields: `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `product_ndc` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null).
- `section` is `"package-ndc"`: A single original identifier array. Fields: `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `package_ndc` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null).
- `section` is `"spl-id"`: A single original identifier array. Fields: `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null), `spl_id` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)).
- `section` is `"spl-set-id"`: A single original identifier array. Fields: `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null), `spl_set_id` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)).
- `section` is `"application-number"`: A single original identifier array. Fields: `application_number` ([Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords)), `components` (list of [Reply_ComponentLabel](/reference/schemas/types/#reply-componentlabel)), `header` ([Reply_AdverseEventHeader](/reference/schemas/types/#reply-adverseeventheader)), `snapshot` ([Reply_Text](/reference/schemas/types/#reply-text) or null).

### Other types in this reply

One more type appear only in this reply: [Reply_AdverseEventIdentifierRecords](#reply-adverseeventidentifierrecords). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_AdverseEventIdentifierRecords

One selected original identifier array, owned by an absolute drug index.

| Field | Type | Meaning |
| --- | --- | --- |
| `drug_index` | integer (uint32) | Absolute native drug-array index within this report snapshot. |
| `offset` | integer (uint32) | Original array offset. |
| `records` | list of string | Published identifier strings without rewriting. Optional. |
| `total` | integer (uint64) | Complete selected array length. |

</details>

## Next

[`biomcp get adverse-event`](/reference/commands/get-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
