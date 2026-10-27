---
title: Adverse event identifiers reply (biomcp.adverse_event_identifiers/1)
description: "The JSON reply BioMCP 1.0.0 prints for no command (adverse event
  identifiers): a real example and the fields of schema
  biomcp.adverse_event_identifiers/1."
goal: "Parse the biomcp.adverse_event_identifiers/1 reply: see a real example
  and what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse_event_identifiers/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. The reply schema `biomcp.adverse_event_identifiers/1` is in the program, but no command returns it as a whole reply. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

The identifier sections of `get adverse-event`, such as `unii` and `rxcui`, are listed in the help, but BioMCP refuses them with `usage.arguments`, so no command returns this schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AdverseEventIdentifiers](#adverseeventidentifiers). `query` holds [AdverseEventGet](/reference/schemas/types/#adverseeventget), a shared type. `query` repeats the request as BioMCP read it.

### AdverseEventIdentifiers

Exactly one closed named identifier page in its original report snapshot.

One of these forms:

- `section` is `"unii"`: A single original identifier array. Fields: `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `snapshot` ([Text](/reference/schemas/types/#text)), `unii` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)).
- `section` is `"rxcui"`: A single original identifier array. Fields: `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `rxcui` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)), `snapshot` ([Text](/reference/schemas/types/#text)).
- `section` is `"product-ndc"`: A single original identifier array. Fields: `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `product_ndc` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)), `snapshot` ([Text](/reference/schemas/types/#text)).
- `section` is `"package-ndc"`: A single original identifier array. Fields: `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `package_ndc` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)), `snapshot` ([Text](/reference/schemas/types/#text)).
- `section` is `"spl-id"`: A single original identifier array. Fields: `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `snapshot` ([Text](/reference/schemas/types/#text)), `spl_id` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)).
- `section` is `"spl-set-id"`: A single original identifier array. Fields: `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `snapshot` ([Text](/reference/schemas/types/#text)), `spl_set_id` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)).
- `section` is `"application-number"`: A single original identifier array. Fields: `application_number` ([AdverseEventIdentifierRecords](#adverseeventidentifierrecords)), `components` (list of [ComponentLabel](/reference/schemas/types/#componentlabel)), `header` ([AdverseEventHeader](/reference/schemas/types/#adverseeventheader)), `snapshot` ([Text](/reference/schemas/types/#text)).

### Other types in this reply

One more type appear only in this reply: [AdverseEventIdentifierRecords](#adverseeventidentifierrecords). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### AdverseEventIdentifierRecords

One selected original identifier array, owned by an absolute drug index.

| Field | Type | Meaning |
| --- | --- | --- |
| `drug_index` | integer (uint32) | Absolute native drug-array index within this report snapshot. |
| `offset` | integer (uint32) | Original array offset. |
| `records` | list of string | Published identifier strings without rewriting. |
| `total` | integer (uint64) | Complete selected array length. |

</details>

## Next

[Reply schemas](/reference/schemas/) lists every reply.
