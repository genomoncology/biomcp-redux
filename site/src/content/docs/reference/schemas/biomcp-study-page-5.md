---
title: Reply of biomcp search study (biomcp.study-page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search study: the
  fields of schema biomcp.study-page/5."
goal: "Parse the biomcp.study-page/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.study-page/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search study`](/reference/commands/search-study/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.study-page/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.study-page/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_StudyPage](#reply_studypage). `query` holds [Reply_StudySearch](#reply_studysearch). `query` repeats the request as BioMCP read it.

### Reply_StudyPage

Source-ordered checked study headers from one retained catalog.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_StudyHeader](/reference/schemas/types/#reply_studyheader) | Whole source headers, without expanded descriptions. Optional. |
| `page` | [Reply_PageInfo](/reference/schemas/types/#reply_pageinfo) | Snapshot-scoped matching counts and offset. |
| `snapshot` | [Reply_StudySnapshot](/reference/schemas/types/#reply_studysnapshot) | Catalog acquisition provenance. |

### Reply_StudySearch

Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

| Field | Type | Meaning |
| --- | --- | --- |
| `cancer_type` | [Reply_CbioportalCancerTypeId](/reference/schemas/types/#reply_cbioportalcancertypeid) | Exact cBioPortal cancer-type code; no OncoTree conversion. Optional. |
| `limit` | integer (uint32) | Maximum headers per page. Optional. |
| `offset` | integer (uint32) | Zero-based offset in the checked snapshot. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Opaque continuation retaining the catalog revision and selectors. Optional. |
| `text` | [Reply_Text](/reference/schemas/types/#reply_text) | Literal study ID, name or description text, without synonyms or typo repair. Optional. |

## Next

[`biomcp search study`](/reference/commands/search-study/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
