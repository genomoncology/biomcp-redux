---
title: Reply of biomcp search biomarker (biomcp.biomarker-page/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search biomarker:
  the fields of schema biomcp.biomarker-page/4."
goal: "Parse the biomcp.biomarker-page/4 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.biomarker-page/4
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search biomarker`](/reference/commands/search-biomarker/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.biomarker-page/4`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.biomarker-page/4` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_BiomarkerPage](#reply-biomarkerpage). `query` holds [Reply_BiomarkerSearch](#reply-biomarkersearch). `query` repeats the request as BioMCP read it.

### Reply_BiomarkerPage

Local page of a complete bounded source observation.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_BiomarkerHeader](/reference/schemas/types/#reply-biomarkerheader) | Native biomarker headers in source order. Optional. |

### Reply_BiomarkerSearch

Find NCI biomarker concepts by name or synonym. The public tier returns codes, name, type, trial count, synonyms, parent_ids, ancestor_ids and eligibility_criterion. These concepts differ from diagnostic --biomarker filters and literal trial --variant text. Use search trial --source nci --biomarker-code CODE for trials that list an exact concept. Native criteria do not establish patient eligibility.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum headers per local page. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) | Native partial name or synonym search. Optional. |
| `offset` | integer (uint32) | Offset within the complete retained observation. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Snapshot identity retained by continuation commands. Optional. |
| `source` | [Reply_BiomarkerSource](/reference/schemas/types/#reply-biomarkersource) | Explicit keyed NCI source. |

## Next

[`biomcp search biomarker`](/reference/commands/search-biomarker/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
