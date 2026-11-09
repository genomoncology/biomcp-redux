---
title: Reply of biomcp search adverse-event (biomcp.adverse-event-page/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search
  adverse-event: the fields of schema biomcp.adverse-event-page/6."
goal: "Parse the biomcp.adverse-event-page/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.adverse-event-page/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search adverse-event`](/reference/commands/search-adverse-event/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.adverse-event-page/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.adverse-event-page/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AdverseEventPage](#reply_adverseeventpage). `query` holds [Reply_AdverseEventSearch](/reference/schemas/types/#reply_adverseeventsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AdverseEventPage

Ordered report headers; totals count reports with no population denominator.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_ComponentLabel](/reference/schemas/types/#reply_componentlabel) | Applicable component terms. Optional. |
| `fetched` | integer or null | Original retrieval time in Unix seconds, absent for undated legacy recordings. Optional. |
| `items` | list of [Reply_AdverseEventHeader](/reference/schemas/types/#reply_adverseeventheader) | Native ordered report candidates. Optional. |
| `reaction_counts` | [Reply_AdverseEventReactionCounts](#reply_adverseeventreactioncounts) or null | Source reaction buckets when aggregation was requested; never report header rows. Optional. |
| `reaction_match` | [Reply_AdverseEventReactionMatch](#reply_adverseeventreactionmatch) or null | Explicit analyzed reaction selector semantics when a reaction was supplied. Optional. |
| `source_query` | string | Exact literal provider expression; reaction selectors use phrase match. |
| `source_updated` | [Reply_SourceUpdated](/reference/schemas/types/#reply_sourceupdated) or null | The publisher last_updated date, separate from retrieval. Optional. |
| `unit` | [Reply_ReportUnit](#reply_reportunit) | Explicit reports unit for envelope page totals. |

### Other types in this reply

4 more types appear only in this reply: [Reply_AdverseEventReactionCount](#reply_adverseeventreactioncount), [Reply_AdverseEventReactionCounts](#reply_adverseeventreactioncounts), [Reply_AdverseEventReactionMatch](#reply_adverseeventreactionmatch), [Reply_ReportUnit](#reply_reportunit). Open the list below to read their fields.

<details>
<summary>Fields of the 4 other types</summary>

### Reply_AdverseEventReactionCount

One exact reaction term and its openFDA report count.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer (uint64) | Native count of matching reports containing this term, not exposed people. |
| `term` | [Reply_Text](/reference/schemas/types/#reply_text) | Exact native MedDRA term, without normalization or inferred drug association. |

### Reply_AdverseEventReactionCounts

Bounded native count response; buckets may overlap and are never summed.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_AdverseEventReactionCount](#reply_adverseeventreactioncount) | Native reaction buckets in provider order; a report may contribute to several. |
| `limit_reached` | boolean | The provider returned as many terms as requested; more terms may exist. This does not establish a total or prove that terms were omitted. |

### Reply_AdverseEventReactionMatch

Source reaction selector matching semantics.

Values: `phrase-match`.

### Reply_ReportUnit

The count describes reports, never exposed people or causal events.

| Value | Meaning |
| --- | --- |
| `reports` | Native report count without a population denominator. |

</details>

## Next

[`biomcp search adverse-event`](/reference/commands/search-adverse-event/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
