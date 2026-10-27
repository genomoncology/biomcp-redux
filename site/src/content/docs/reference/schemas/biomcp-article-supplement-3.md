---
title: Reply of biomcp article supplement (biomcp.article_supplement/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article supplement:
  a real example and the fields of schema biomcp.article_supplement/3."
goal: "Parse the biomcp.article_supplement/3 reply: see a real example and what
  each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_supplement/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article supplement`](/reference/commands/article-supplement/) replies with `schema` set to `biomcp.article_supplement/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
```

`biomcp schema biomcp.article_supplement/3` prints the full JSON Schema.

## Example reply

`biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_supplement/3",
  "status": "complete",
  "query": {
    "input": "28944238",
    "file": "MGG3-5-553-s002.xlsx",
    "variant": "P245S",
    "offset": 0
  },
  "result": {
    "pmcid": "PMC5606870",
    "content": {
      "state": "data",
      "source": "pmc-oa",
      "data": {…}
    }
  },
  "page": {"offset": 0, "returned": 1, "total": 1},
  "sources": [
    {
      "source": "pubmed",
      "use": "restricted",
      "terms": "NCBI policies; publisher rights in abstracts",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    },
    … 1 more
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "article", "supplement", "28944238", "MGG3-5-553-s002.xlsx", "--rows",
        "1-20", "--sheet", "ST2"
      ]
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticleSupplementResult](#articlesupplementresult). `query` holds [ArticleSupplement](#articlesupplement). `query` repeats the request as BioMCP read it.

### ArticleSupplementResult

One supplement reader answer.

| Field | Type | Meaning |
| --- | --- | --- |
| `content` | [Section](/reference/schemas/types/#section) of [SupplementContent](#supplementcontent) | Independently licensed supplement content. |
| `pmcid` | [Pmcid](/reference/schemas/types/#pmcid) or null | Resolved PMC identity. Optional. |

### ArticleSupplement

Read a checked supplement file or page its original rows.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Text](/reference/schemas/types/#text) | Exact file name from the supplements section. |
| `find` | [Text](/reference/schemas/types/#text) | Text to find, matched as typed, ignoring case. Optional. |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | PubMed or PMC identity. |
| `offset` | integer (uint32) | Matching-row or text-fragment offset. Optional. |
| `replace` | boolean | Replace an existing destination only when explicitly requested with save. Optional. |
| `rows` | [SheetRows](#sheetrows) | Inclusive original row range. Optional. |
| `save` | [Text](/reference/schemas/types/#text) | Save the whole original document to an explicit file or existing folder. Optional. |
| `sheet` | [Text](/reference/schemas/types/#text) | Sheet name for row selection. Optional. |
| `variant` | [Text](/reference/schemas/types/#text) | Explicit protein substitution. Optional. |

### CellFragment

A bounded portion of one original first-row or selected-row cell.

| Field | Type | Meaning |
| --- | --- | --- |
| `column` | integer (uint32) | Original one-based column. |
| `first_row` | boolean | Whether the cell belongs to the first source row. |
| `offset` | integer (uint32) | Unicode character offset within this cell. |
| `text` | string | Original cell text at this offset. |

### FirstSheetRow

The first original row without interpreting its cells as column headings.

| Field | Type | Meaning |
| --- | --- | --- |
| `cells` | list of string | Original cells, empty when represented by cell fragments. |
| `row` | integer (uint32) | Original one-based row number. |

### SheetInfo

Source sheet with its maximum original row number.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Text](/reference/schemas/types/#text) | Sheet name. |
| `rows` | integer (uint32) | Last original row number, preserving gaps. |

### SheetRow

One original sparse row with its numbered first source row.

| Field | Type | Meaning |
| --- | --- | --- |
| `cells` | list of string | This row's cells, including empty cells before populated columns. |
| `first_row` | [FirstSheetRow](#firstsheetrow) | First original row, including empty cells; never inferred column headings. |
| `fragment` | [CellFragment](#cellfragment) or null | Sparse source cell fragment when the complete row exceeds the output bound. Optional. |
| `row` | integer (uint32) | Original one-based row number. |
| `sheet` | [Text](/reference/schemas/types/#text) | Original sheet name. |

### SheetRows

Type: string.

### SupplementContent

Downloaded bounded file contents or an unsupported format's original URL.

| Field | Type | Meaning |
| --- | --- | --- |
| `forms` | list of string or null | Explicit variant spellings searched. Optional. |
| `license` | [License](/reference/schemas/types/#license) | Article license. |
| `reason` | [Reason](/reference/schemas/types/#reason) or null | Typed unsupported reason. Optional. |
| `rows` | list of [SheetRow](#sheetrow) or null | Selected original rows. Optional. |
| `saved` | [ArticleSaved](/reference/schemas/types/#articlesaved) or null | Explicit original-byte save receipt. Optional. |
| `search_message` | string or null | Explicit absence of matching text from the returned search page. Optional. |
| `sha256` | [Text](/reference/schemas/types/#text) or null | Hash of the original downloaded bytes; absent when no download occurred. Optional. |
| `sheets` | list of [SheetInfo](#sheetinfo) or null | Original sheet dimensions. Optional. |
| `url` | [Text](/reference/schemas/types/#text) or null | Unsupported file URL, returned without downloading it. Optional. |

## Next

[`biomcp article supplement`](/reference/commands/article-supplement/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
