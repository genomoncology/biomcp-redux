---
title: Reply of biomcp get drug (biomcp.drug-targets/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-targets/8."
goal: "Parse the biomcp.drug-targets/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-targets/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-targets/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-targets/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugTargets](#reply-drugtargets). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply-drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugTargets

Native target and mechanism relations, without a clinical indication inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply-drugheader) | Selected native identity, checked independently of section facts. |
| `targets` | [Reply_Section_DrugRecords_DrugTargetRelation](#reply-section-drugrecords-drugtargetrelation) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |

### Other types in this reply

4 more types appear only in this reply: [Reply_DrugNativeTarget](#reply-drugnativetarget), [Reply_DrugRecords_DrugTargetRelation](#reply-drugrecords-drugtargetrelation), [Reply_DrugTargetRelation](#reply-drugtargetrelation), [Reply_Section_DrugRecords_DrugTargetRelation](#reply-section-drugrecords-drugtargetrelation). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### Reply_DrugNativeTarget

One source-native target identity, without inventing a gene label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact published target identifier. |
| `label` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native target label, only when supplied. Optional. |
| `namespace` | [Reply_Text](/reference/schemas/types/#reply-text) | Source responsible for the native identifier. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native target link, only when supplied. Optional. |

### Reply_DrugRecords_DrugTargetRelation

One bounded native row page, without fabricating an unknown total.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Source row offset. |
| `records` | list of [Reply_DrugTargetRelation](#reply-drugtargetrelation) | Native source rows in order. Optional. |
| `retrieved` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Actual retrieval date, never substituted for source revision. Optional. |
| `total` | integer or null | Native total only when supplied. Optional. |
| `truncated` | boolean or null | Native source truncation indicator. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Dataset or [bundle](/local-data/) revision. Optional. |

### Reply_DrugTargetRelation

A native target or mechanism relation, without an indication inference.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply-attribution) | Native relation provenance. |
| `drug` | [Reply_DrugIdentity](/reference/schemas/types/#reply-drugidentity) | Source drug identity. |
| `mechanism` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native mechanism wording, only when supplied. Optional. |
| `native_target` | [Reply_DrugNativeTarget](#reply-drugnativetarget) | Native target identity independent of a gene reference. Optional. |
| `references` | list of [Reply_DrugNativeReference](/reference/schemas/types/#reply-drugnativereference) | Source-native supporting references. Optional. |
| `relation` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Native relationship label. Optional. |
| `target` | [Reply_GeneRef](/reference/schemas/types/#reply-generef) | Source-stated target gene, without using mechanism text as a gene label. Optional. |

### Reply_Section_DrugRecords_DrugTargetRelation

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugRecords_DrugTargetRelation](#reply-drugrecords-drugtargetrelation)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
