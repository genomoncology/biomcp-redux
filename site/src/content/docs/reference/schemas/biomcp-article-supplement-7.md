---
title: Reply of biomcp article supplement (biomcp.article-supplement/7)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article supplement:
  the fields of schema biomcp.article-supplement/7."
goal: "Parse the biomcp.article-supplement/7 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-supplement/7
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article supplement`](/reference/commands/article-supplement/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-supplement/7`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-supplement/7` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticleSupplementResult](#reply-articlesupplementresult). `query` holds [Reply_ArticleSupplement](#reply-articlesupplement). `query` repeats the request as BioMCP read it.

### Reply_ArticleSupplementResult

One supplement reader answer.

| Field | Type | Meaning |
| --- | --- | --- |
| `content` | [Reply_Section_SupplementContent](#reply-section-supplementcontent) | Independently licensed supplement content. |
| `pmcid` | [Reply_Pmcid](/reference/schemas/types/#reply-pmcid) or null | Resolved PMC identity. Optional. |

### Reply_ArticleSupplement

Read a checked supplement file or page its original rows.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact file name from the supplements section. |
| `find` | [Reply_Text](/reference/schemas/types/#reply-text) | Text to find, matched as typed, ignoring case. Optional. |
| `input` | [Reply_ArticleInput](/reference/schemas/types/#reply-articleinput) | PubMed or PMC identity. |
| `offset` | integer (uint32) | Matching-row or text-fragment offset. Optional. |
| `replace` | boolean | Replace an existing destination only when explicitly requested with save. Optional. |
| `rows` | [Reply_SheetRows](#reply-sheetrows) | Inclusive original row range. Optional. |
| `save` | [Reply_Text](/reference/schemas/types/#reply-text) | Save the whole original document to an explicit file or existing folder. Example: --save article-output. Optional. |
| `sheet` | [Reply_Text](/reference/schemas/types/#reply-text) | Sheet name for row selection. Optional. |
| `variant` | [Reply_Text](/reference/schemas/types/#reply-text) | Explicit protein substitution. Optional. |

### Other types in this reply

7 more types appear only in this reply: [Reply_CellFragment](#reply-cellfragment), [Reply_FirstSheetRow](#reply-firstsheetrow), [Reply_Section_SupplementContent](#reply-section-supplementcontent), [Reply_SheetInfo](#reply-sheetinfo), [Reply_SheetRow](#reply-sheetrow), [Reply_SheetRows](#reply-sheetrows), [Reply_SupplementContent](#reply-supplementcontent). Open the list below to read their fields.

<details>
<summary>Fields of the 7 other types</summary>

### Reply_CellFragment

A bounded portion of one original first-row or selected-row cell.

| Field | Type | Meaning |
| --- | --- | --- |
| `column` | integer (uint32) | Original one-based column. |
| `first_row` | boolean | Whether the cell belongs to the first source row. |
| `offset` | integer (uint32) | Unicode character offset within this cell. |
| `text` | string | Original cell text at this offset. |

### Reply_FirstSheetRow

The first original row without interpreting its cells as column headings.

| Field | Type | Meaning |
| --- | --- | --- |
| `cells` | list of string | Original cells, empty when represented by cell fragments. Optional. |
| `row` | integer (uint32) | Original one-based row number. |

### Reply_Section_SupplementContent

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_SupplementContent](#reply-supplementcontent)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_SheetInfo

Source sheet with its maximum original row number.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) | Sheet name. |
| `rows` | integer (uint32) | Last original row number, preserving gaps. |

### Reply_SheetRow

One original sparse row with its numbered first source row.

| Field | Type | Meaning |
| --- | --- | --- |
| `cells` | list of string | This row's cells, including empty cells before populated columns. Optional. |
| `first_row` | [Reply_FirstSheetRow](#reply-firstsheetrow) | First original row, including empty cells; never inferred column headings. |
| `fragment` | [Reply_CellFragment](#reply-cellfragment) or null | Sparse source cell fragment when the complete row exceeds the output bound. Optional. |
| `row` | integer (uint32) | Original one-based row number. |
| `sheet` | [Reply_Text](/reference/schemas/types/#reply-text) | Original sheet name. |

### Reply_SheetRows

Inclusive original one-based row range. The parser additionally requires the end to be at least the start and fewer than 200 rows after it.

Type: string.

### Reply_SupplementContent

Downloaded bounded file contents or an unsupported format's original URL.

| Field | Type | Meaning |
| --- | --- | --- |
| `forms` | list of string or null | Explicit variant spellings searched. Optional. |
| `license` | [Reply_License](/reference/schemas/types/#reply-license) | Article license. |
| `reason` | [Reply_Reason](/reference/schemas/types/#reply-reason) or null | Typed unsupported reason. Optional. |
| `rows` | list of [Reply_SheetRow](#reply-sheetrow) or null | Selected original rows. Optional. |
| `saved` | [Reply_ArticleSaved](/reference/schemas/types/#reply-articlesaved) or null | Explicit original-byte save receipt. Optional. |
| `search_message` | string or null | Explicit absence of matching text from the returned search page. Optional. |
| `sha256` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Hash of the original downloaded bytes; absent when no download occurred. Optional. |
| `sheets` | list of [Reply_SheetInfo](#reply-sheetinfo) or null | Original sheet dimensions. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Unsupported file URL, returned without downloading it. Optional. |

</details>

## Next

[`biomcp article supplement`](/reference/commands/article-supplement/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
