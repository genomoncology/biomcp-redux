---
title: Reply of biomcp search trial (biomcp.trial-page/14)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search trial: the
  fields of schema biomcp.trial-page/14."
goal: "Parse the biomcp.trial-page/14 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.trial-page/14
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search trial`](/reference/commands/search-trial/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.trial-page/14`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.trial-page/14` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_TrialPage](#reply-trialpage). `query` holds [Reply_TrialSearch](/reference/schemas/types/#reply-trialsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_TrialPage

Registry page preserving source order and source processing time.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution. |
| `condition_resolution` | [Reply_TrialConditionResolution](/reference/schemas/types/#reply-trialconditionresolution) or null | Exact condition identity and source-provided mapping, absent for literal text. Optional. |
| `gene_resolution` | [Reply_GeneResolution](/reference/schemas/types/#reply-generesolution) or null | Unresolved gene identity, without a registry request. Optional. |
| `items` | list of [Reply_TrialHeader](#reply-trialheader) | Native study headers in registry order. Optional. |
| `source_updated` | [Reply_SourceUpdated](/reference/schemas/types/#reply-sourceupdated) or null | Registry's source processing timestamp, independently of retrieval time. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_TrialHeader](#reply-trialheader). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_TrialHeader

Small source-attributed trial discovery record.

| Field | Type | Meaning |
| --- | --- | --- |
| `conditions` | [Reply_Capped_Text](/reference/schemas/types/#reply-capped-text) | Source condition names. |
| `interventions` | [Reply_Capped_Text](/reference/schemas/types/#reply-capped-text) | Source intervention names. |
| `nct_id` | [Reply_NctId](/reference/schemas/types/#reply-nctid) | Exact registry identity. |
| `phases` | list of [Reply_TrialPhase](/reference/schemas/types/#reply-trialphase) | Source phases; absent phases remain absent. Optional. |
| `status` | [Reply_TrialStatus](/reference/schemas/types/#reply-trialstatus) or null | Registry overall status, absent when missing or rejected; independent of site status. Optional. |
| `title` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source brief title, absent when missing or rejected. Optional. |
| `updated` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Last posted registry update, absent when missing or rejected. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) | Publisher's canonical record URL. |

</details>

## Next

[`biomcp search trial`](/reference/commands/search-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
