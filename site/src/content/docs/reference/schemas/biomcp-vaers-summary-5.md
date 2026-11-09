---
title: Reply of biomcp search adverse-event (biomcp.vaers-summary/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search
  adverse-event: the fields of schema biomcp.vaers-summary/5."
goal: "Parse the biomcp.vaers-summary/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.vaers-summary/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.vaers-summary/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.vaers-summary/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_VaersSummary](#reply_vaerssummary). `query` holds [Reply_AdverseEventSearch](/reference/schemas/types/#reply_adverseeventsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_VaersSummary

Bounded page of one independent native vaccine summary table.

| Field | Type | Meaning |
| --- | --- | --- |
| `dataset` | map of string | Native dataset attributes including processed-through vintage. |
| `footer` | [Reply_VaersRow](#reply_vaersrow) | Whole native footer cells, never a sum calculated by this program. |
| `grouping` | map of string | Native grouping attributes. |
| `interpretation` | string | Reviewed interpretation boundary. |
| `measure_metadata` | list of map of string | Response-specific measure metadata without denominator inference. Optional. |
| `measures` | list of map of string | Source measure definitions, including original names and codes. Optional. |
| `notices` | list of [Reply_VaersNotice](#reply_vaersnotice) | Applied native messages, evaluated caveats and footnotes. Optional. |
| `report_total` | string | Native report-total message, separate from the footer aggregation. |
| `rows` | list of [Reply_VaersRow](#reply_vaersrow) | Native row order with local paging. Optional. |
| `scope` | [Reply_VaersScope](#reply_vaersscope) | Native fixed State / Territory selection. |
| `vaccine` | map of string | Native selected vaccine attributes. |

### Other types in this reply

3 more types appear only in this reply: [Reply_VaersNotice](#reply_vaersnotice), [Reply_VaersRow](#reply_vaersrow), [Reply_VaersScope](#reply_vaersscope). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_VaersNotice

Applied source text, retaining its original content and source role.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | string | Message, footnote or evaluated caveat. |
| `text` | string | Original source text, including embedded publisher markup. |

### Reply_VaersRow

One whole native row, never reconstructed from another grouping.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | map of string | Native label cell; no code is invented when absent. |
| `percent` | map of string | Native Percent cell, retaining values above one hundred percent. |
| `reports` | map of string | Native Events Reported count for this row. A footer counts occurrences, separate from the message report total; suppressed and displayed-total text stay unchanged. |

### Reply_VaersScope

Fixed native geographic selection, without a caller-selectable geography filter.

| Field | Type | Meaning |
| --- | --- | --- |
| `value` | map of string | Exact selected native value and its own catalog label. |
| `variable` | map of string | Native variable attributes, including source name and code. |

</details>

## Next

[`biomcp search adverse-event`](/reference/commands/search-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
