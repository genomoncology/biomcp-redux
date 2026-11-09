---
title: Reply of biomcp get drug (biomcp.drug-xrefs/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-xrefs/5."
goal: "Parse the biomcp.drug-xrefs/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-xrefs/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-xrefs/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-xrefs/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugXrefs](#reply-drugxrefs). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply-drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugXrefs

A mechanical page of the selected document's complete checked native links.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply-drugheader) | Exact source-selected identity, without choosing any linked product. |
| `xrefs` | [Reply_Section_DrugRecords_DrugIdentityRelation](#reply-section-drugrecords-drugidentityrelation) | Native links with an exact link count, original version and retrieval date. |

### Other types in this reply

2 more types appear only in this reply: [Reply_DrugRecords_DrugIdentityRelation](#reply-drugrecords-drugidentityrelation), [Reply_Section_DrugRecords_DrugIdentityRelation](#reply-section-drugrecords-drugidentityrelation). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_DrugRecords_DrugIdentityRelation

One bounded native row page, without fabricating an unknown total.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Source row offset. |
| `records` | list of [Reply_DrugIdentityRelation](/reference/schemas/types/#reply-drugidentityrelation) | Native source rows in order. Optional. |
| `retrieved` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Actual retrieval date, never substituted for source revision. Optional. |
| `total` | integer or null | Native total only when supplied. Optional. |
| `truncated` | boolean or null | Native source truncation indicator. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Dataset or [bundle](/local-data/) revision. Optional. |

### Reply_Section_DrugRecords_DrugIdentityRelation

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugRecords_DrugIdentityRelation](#reply-drugrecords-drugidentityrelation)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
