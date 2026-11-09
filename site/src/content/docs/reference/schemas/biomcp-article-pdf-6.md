---
title: Reply of biomcp article pdf (biomcp.article-pdf/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article pdf: the
  fields of schema biomcp.article-pdf/6."
goal: "Parse the biomcp.article-pdf/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-pdf/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article pdf`](/reference/commands/article-pdf/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-pdf/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-pdf/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticlePdfResult](#reply_articlepdfresult). `query` holds [Reply_ArticlePdf](#reply_articlepdf). `query` repeats the request as BioMCP read it.

### Reply_ArticlePdfResult

Result of an explicit publisher PDF download.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Reply_Section_PdfSaved](#reply_section_pdfsaved) | Saved receipt or the reason no file was saved. |

### Reply_ArticlePdf

Download one publisher PDF to an explicit file or existing folder.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | [Reply_ArticleInput](/reference/schemas/types/#reply_articleinput) | Article identity. |
| `replace` | boolean | Replace an existing file only when explicitly requested. Optional. |
| `save` | [Reply_Text](/reference/schemas/types/#reply_text) | Destination file or existing folder. |

### Other types in this reply

2 more types appear only in this reply: [Reply_PdfSaved](#reply_pdfsaved), [Reply_Section_PdfSaved](#reply_section_pdfsaved). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_PdfSaved

Receipt for one file explicitly saved by the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Number of saved bytes. |
| `license` | [Reply_License](/reference/schemas/types/#reply_license) | Normalized license; other grants no reuse rights. |
| `path` | [Reply_Text](/reference/schemas/types/#reply_text) | Absolute destination path. |
| `sha256` | [Reply_Text](/reference/schemas/types/#reply_text) | SHA-256 of the saved file. |
| `status` | [Reply_OpenAccessStatus](/reference/schemas/types/#reply_openaccessstatus) | Source access status. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) | Source publisher URL. |

### Reply_Section_PdfSaved

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_PdfSaved](#reply_pdfsaved)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply_sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply_reason)), `source` ([Reply_Source](/reference/schemas/types/#reply_source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

</details>

## Next

[`biomcp article pdf`](/reference/commands/article-pdf/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
