---
title: Reply of biomcp get drug (biomcp.drug-indications/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get drug: the fields
  of schema biomcp.drug-indications/8."
goal: "Parse the biomcp.drug-indications/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.drug-indications/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get drug`](/reference/commands/get-drug/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.drug-indications/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.drug-indications/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DrugIndications](#reply_drugindications). `query` holds [Reply_DrugGet](/reference/schemas/types/#reply_drugget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DrugIndications

Source-native indication wording, populations and combinations.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity` | [Reply_DrugHeader](/reference/schemas/types/#reply_drugheader) | Selected native identity, checked independently of section facts. |
| `indications` | [Reply_Section_DrugRecords_DrugIndicationClaim](#reply_section_drugrecords_drugindicationclaim) | Existing typed section state, preserving empty, unavailable and skipped outcomes. |
| `regulatory_context` | [Reply_DrugRegulatoryContext](#reply_drugregulatorycontext) or null | Independent FDA application events found by the source's drug-name lookup. Optional. |

### Other types in this reply

6 more types appear only in this reply: [Reply_DrugDiseaseReference](#reply_drugdiseasereference), [Reply_DrugIndicationClaim](#reply_drugindicationclaim), [Reply_DrugRecords_DrugIndicationClaim](#reply_drugrecords_drugindicationclaim), [Reply_DrugRegulatoryContext](#reply_drugregulatorycontext), [Reply_DrugStage](#reply_drugstage), [Reply_Section_DrugRecords_DrugIndicationClaim](#reply_section_drugrecords_drugindicationclaim). Open the list below to read their fields.

<details>
<summary>Fields of the 6 other types</summary>

### Reply_DrugDiseaseReference

One native disease identifier and its own source label.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact identifier spelling, without CURIE conversion. |
| `label` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Label attached to this identifier by the source. Optional. |
| `namespace` | [Reply_Text](/reference/schemas/types/#reply_text) | Source namespace spelling, such as EFO or MeSH. |

### Reply_DrugIndicationClaim

A source indication retaining its full population and combination wording.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | [Reply_Attribution](/reference/schemas/types/#reply_attribution) | Native claim provenance. |
| `combination` | list of [Reply_DrugIdentity](/reference/schemas/types/#reply_drugidentity) | Complete source-stated combination members, without local joins. Optional. |
| `disease_references` | list of [Reply_DrugDiseaseReference](#reply_drugdiseasereference) | Independently labeled native disease identifiers, without cross-source normalization. Optional. |
| `drug` | [Reply_DrugIdentity](/reference/schemas/types/#reply_drugidentity) | Source drug identity. |
| `indication` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Complete native indication wording. Optional. |
| `population` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Population wording explicitly supplied separately. Optional. |
| `references` | list of [Reply_DrugNativeReference](/reference/schemas/types/#reply_drugnativereference) | Source-native supporting references. Optional. |
| `stage` | [Reply_DrugStage](#reply_drugstage) or null | Source-stated development stage, separate from authorization events. Optional. |

### Reply_DrugRecords_DrugIndicationClaim

One bounded native row page, without fabricating an unknown total.

| Field | Type | Meaning |
| --- | --- | --- |
| `offset` | integer (uint32) | Source row offset. |
| `records` | list of [Reply_DrugIndicationClaim](#reply_drugindicationclaim) | Native source rows in order. Optional. |
| `retrieved` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | Actual retrieval date, never substituted for source revision. Optional. |
| `total` | integer or null | Native total only when supplied. Optional. |
| `truncated` | boolean or null | Native source truncation indicator. Optional. |
| `version` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Dataset or [bundle](/local-data/) revision. Optional. |

### Reply_DrugRegulatoryContext

Native FDA events beside phase facts, without an indication-level assertion.

| Field | Type | Meaning |
| --- | --- | --- |
| `applications` | list of [Reply_DrugApprovals](/reference/schemas/types/#reply_drugapprovals) | Source-owned application identities, products and dated submission events. |
| `indications_linked` | boolean | False: the indication claims and FDA events have not been linked. |
| `notice` | string | Plain statement of the name-only relationship and independent source facts. |
| `total_applications` | integer (uint64) | Native FDA application count for the drug-name lookup. |

### Reply_DrugStage

One source development-stage field and its unchanged value.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | string | Exact native field name. |
| `value` | string | Exact source string, without an authorization interpretation. |

### Reply_Section_DrugRecords_DrugIndicationClaim

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_DrugRecords_DrugIndicationClaim](#reply_drugrecords_drugindicationclaim)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp get drug`](/reference/commands/get-drug/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
