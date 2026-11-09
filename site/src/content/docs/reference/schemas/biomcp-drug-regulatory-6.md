---
title: Reply of biomcp get drug (biomcp.drug-regulatory/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-regulatory/6."
goal: "Parse the biomcp.drug-regulatory/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-regulatory/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-regulatory/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-regulatory/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugRegulatory](#reply-drugregulatory). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply-drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugRegulatory

Source-scoped regulator events preserving date roles and original status.

| Field | Type | Meaning |
| --- | --- | --- |
| `date_roles` | list of [Reply_DateRoleLabel](#reply-daterolelabel) | The source's own published description of each date role in these records. Optional. |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply-drugheader) | Selected native identity, checked independently of section facts. |
| `regulatory` | [Reply_Section_DrugRecords_AuthorizationEvent](#reply-section-drugrecords-authorizationevent) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### Other types in this reply

3 more types appear only in this reply: [Reply_DateRoleLabel](#reply-daterolelabel), [Reply_DrugRecords_AuthorizationEvent](#reply-drugrecords-authorizationevent), [Reply_Section_DrugRecords_AuthorizationEvent](#reply-section-drugrecords-authorizationevent). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_DateRoleLabel

One native date role with the source's own published description of it.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Reply_Text](/reference/schemas/types/#reply-text) or null | The source's published words for this date; omitted when the source publishes none. Optional. |
| `procedure` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native procedure reference the source shows beside this date; omitted when none. Optional. |
| `role` | [Reply_Text](/reference/schemas/types/#reply-text) | Native date role, as it appears in the records' dates. |

### Reply_DrugRecords_AuthorizationEvent

One bounded native row page, without fabricating an unknown total.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Source row offset. |
| `records` | list of [Reply_AuthorizationEvent](/reference/schemas/types/#reply-authorizationevent) | Native source rows in order. Optional. |
| `retrieved` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Actual retrieval date, never substituted for source revision. Optional. |
| `total` | integer or null | Native total only when supplied. Optional. |
| `truncated` | boolean or null | Native source truncation indicator. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Dataset or [bundle](/local-data/) revision. Optional. |

### Reply_Section_DrugRecords_AuthorizationEvent

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugRecords_AuthorizationEvent](#reply-drugrecords-authorizationevent)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
