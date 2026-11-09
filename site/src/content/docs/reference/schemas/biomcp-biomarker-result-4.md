---
title: Reply of biomcp get biomarker (biomcp.biomarker-result/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get biomarker: the
  fields of schema biomcp.biomarker-result/4."
goal: "Parse the biomcp.biomarker-result/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.biomarker-result/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get biomarker`](/reference/commands/get-biomarker/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.biomarker-result/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.biomarker-result/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_BiomarkerResult](#reply-biomarkerresult). `query` holds [Reply_BiomarkerGet](#reply-biomarkerget). `query` repeats the request as BioMCP read it.

### Reply_BiomarkerResult

Exact code result; multiple rows are candidates, never a selected card.

| Field | Type | Meaning |
| --- | --- | --- |
| `candidates` | list of [Reply_BiomarkerCard](#reply-biomarkercard) | Every native candidate when several rows matched the code. Optional. |
| `card` | [Reply_BiomarkerCard](#reply-biomarkercard) or null | The sole native row when exactly one row matched. Optional. |

### Reply_BiomarkerGet

Search NCI by exact NCIt code. Several native rows return candidates without choosing one. The public tier returns codes, name, type, trial count, synonyms, parent_ids, ancestor_ids and eligibility_criterion. No clinical interpretation is performed.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_NcitId](/reference/schemas/types/#reply-ncitid) | Exact NCIt code, such as C96022. |
| `source` | [Reply_BiomarkerSource](/reference/schemas/types/#reply-biomarkersource) | NCI is the biomarker concept source. Optional. |

### Other types in this reply

One more type appear only in this reply: [Reply_BiomarkerCard](#reply-biomarkercard). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_BiomarkerCard

Native terminology card; source criteria remain uninterpreted.

| Field | Type | Meaning |
| --- | --- | --- |
| `ancestor_ids` | list of [Reply_NcitId](/reference/schemas/types/#reply-ncitid) or null | Native ancestor codes. Optional. |
| `eligibility_criterion` | list of [Reply_Text](/reference/schemas/types/#reply-text) or null | Native eligibility criterion values, exactly as published. Optional. |
| `header` | [Reply_BiomarkerHeader](/reference/schemas/types/#reply-biomarkerheader) | Native concept identity and count. |
| `parent_ids` | list of [Reply_NcitId](/reference/schemas/types/#reply-ncitid) or null | Native direct parent codes. Optional. |
| `synonyms` | list of [Reply_Text](/reference/schemas/types/#reply-text) or null | Native synonym values. Optional. |

</details>

## Next

[`biomcp get biomarker`](/reference/commands/get-biomarker/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
