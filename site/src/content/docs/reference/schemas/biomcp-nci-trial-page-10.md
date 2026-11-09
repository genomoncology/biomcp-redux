---
title: Reply of biomcp search trial (biomcp.nci-trial-page/10)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search trial: the
  fields of schema biomcp.nci-trial-page/10."
goal: "Parse the biomcp.nci-trial-page/10 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.nci-trial-page/10
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search trial`](/reference/commands/search-trial/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.nci-trial-page/10`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.nci-trial-page/10` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_NciTrialPage](#reply-ncitrialpage). `query` holds [Reply_TrialSearch](/reference/schemas/types/#reply-trialsearch), a shared type. `query` repeats the request as BioMCP read it.

### Reply_NciTrialPage

NCI search page in native order with native count.

| Field | Type | Meaning |
| --- | --- | --- |
| `attribution` | string | Publisher attribution. |
| `condition_resolution` | [Reply_TrialConditionResolution](/reference/schemas/types/#reply-trialconditionresolution) or null | Exact condition identity and source-provided mapping, absent for literal text. Optional. |
| `items` | list of [Reply_NciTrial](/reference/schemas/types/#reply-ncitrial) | Source rows, never merged with another provider. Optional. |
| `request` | list of [Reply_TrialParameter](#reply-trialparameter) | Exact source parameters, excluding credentials. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_TrialParameter](#reply-trialparameter). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_TrialParameter

One exact source request parameter, in canonical name order.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | string | Native parameter name. |
| `value` | string | Exact unencoded parameter value. |

</details>

## Next

[`biomcp search trial`](/reference/commands/search-trial/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
