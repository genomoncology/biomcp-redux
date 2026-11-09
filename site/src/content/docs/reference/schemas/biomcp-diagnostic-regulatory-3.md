---
title: Reply of biomcp get diagnostic (biomcp.diagnostic-regulatory/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get diagnostic: the
  fields of schema biomcp.diagnostic-regulatory/3."
goal: "Parse the biomcp.diagnostic-regulatory/3 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.diagnostic-regulatory/3
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get diagnostic`](/reference/commands/get-diagnostic/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.diagnostic-regulatory/3`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.diagnostic-regulatory/3` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_DiagnosticRegulatory](#reply-diagnosticregulatory). `query` holds [Reply_DiagnosticGet](/reference/schemas/types/#reply-diagnosticget), a shared type. `query` repeats the request as BioMCP read it.

### Reply_DiagnosticRegulatory

Exact selected submission regulatory text fragments.

| Field | Type | Meaning |
| --- | --- | --- |
| `count_unit` | [Reply_DiagnosticCountUnit](/reference/schemas/types/#reply-diagnosticcountunit) or null | Exact meaning of the envelope total; supplied by the owning source command. Optional. |
| `distinct_id_total` | integer or null | Distinct complete native IDs across the selected relation collection. Optional. |
| `items` | list of [Reply_DiagnosticRegulatoryFragment](#reply-diagnosticregulatoryfragment) | Complete source-owned rows in native order; envelope totals count native rows. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_DiagnosticPmaRegulatory](#reply-diagnosticpmaregulatory), [Reply_DiagnosticRegulatoryField](#reply-diagnosticregulatoryfield), [Reply_DiagnosticRegulatoryFragment](#reply-diagnosticregulatoryfragment). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_DiagnosticPmaRegulatory

Dated metadata belonging only to the selected submission.

| Field | Type | Meaning |
| --- | --- | --- |
| `advisory_committee` | string or null | Native advisory_committee when supplied, including empty. Optional. |
| `advisory_committee_description` | string or null | Native advisory_committee_description when supplied, including empty. Optional. |
| `date_received` | string or null | Native date_received when supplied, including empty. Optional. |
| `fed_reg_notice_date` | string or null | Native fed_reg_notice_date when supplied, including empty. Optional. |
| `product_code` | string or null | Native product_code when supplied, including empty. Optional. |
| `supplement_reason` | string or null | Native supplement_reason when supplied, including empty. Optional. |
| `supplement_type` | string or null | Native supplement_type when supplied, including empty. Optional. |

### Reply_DiagnosticRegulatoryField

Regulatory text available in this source tier.

Values: `ao_statement`.

### Reply_DiagnosticRegulatoryFragment

One bounded regulatory fragment from the exact selected native submission.

| Field | Type | Meaning |
| --- | --- | --- |
| `field` | [Reply_DiagnosticRegulatoryField](#reply-diagnosticregulatoryfield) | Native text field being continued. |
| `metadata` | [Reply_DiagnosticPmaRegulatory](#reply-diagnosticpmaregulatory) | Native metadata for this exact record, without a duplicated whole statement. |
| `offset` | integer (uint32) | Fragment offset, independent of characters and supplement rows. |
| `submission` | [Reply_DiagnosticPmaHeader](/reference/schemas/types/#reply-diagnosticpmaheader) | Exact selected native record and its dated regulatory metadata. |
| `text` | string | Complete unmodified text fragment; omitted when the source statement is absent. Optional. |
| `total` | integer (uint32) | Total mechanical fragments. |

</details>

## Next

[`biomcp get diagnostic`](/reference/commands/get-diagnostic/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
