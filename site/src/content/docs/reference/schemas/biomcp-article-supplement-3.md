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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article supplement`](/reference/commands/article-supplement/) replies with `schema` set to `biomcp.article_supplement/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
```

`biomcp schema biomcp.article_supplement/3` prints the full JSON Schema.

## Example reply

`biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
      "data": {
        "license": "cc_by",
        "sha256": "74f9b11bff7dfad961c938574f48c2efabaa93af0f428c52d477a0e55e…",
        "rows": [
          {
            "sheet": "ST2",
            "row": 85,
            "first_row": {…},
            "cells": [
              "AXIN2", "NM_004655.3", "17:63554006", "c.733C>T", "p.Pro245Ser",
              "0.16775", "0.386", "0.041",
              … 58 more
            ]
          }
        ],
        "forms": ["P245S", "Pro245Ser", "p.P245S", "p.Pro245Ser"]
      }
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

`result` holds [ArticleSupplementResult](#articlesupplementresult). `query` holds [ArticleSupplement](#articlesupplement). `query` repeats the request as BioMCP read it.

### ArticleSupplementResult

One supplement reader answer.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `content` | [Section](/reference/schemas/types/#section) of [SupplementContent](#supplementcontent) | yes | Independently licensed supplement content. |
| `pmcid` | [Pmcid](/reference/schemas/types/#pmcid) or null | no | Resolved PMC identity. |

### ArticleSupplement

Read a checked supplement file or page its original rows.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `file` | [Text](/reference/schemas/types/#text) | yes | Exact file name from the supplements section. |
| `find` | [Text](/reference/schemas/types/#text) | no | Text to find, matched as typed, ignoring case. |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | yes | PubMed or PMC identity. |
| `offset` | integer (uint32) | no | Matching-row or text-fragment offset. |
| `replace` | boolean | no | Replace an existing destination only when explicitly requested with save. |
| `rows` | [SheetRows](#sheetrows) | no | Inclusive original row range. |
| `save` | [Text](/reference/schemas/types/#text) | no | Save the whole original document to an explicit file or existing folder. |
| `sheet` | [Text](/reference/schemas/types/#text) | no | Sheet name for row selection. |
| `variant` | [Text](/reference/schemas/types/#text) | no | Explicit protein substitution. |

### CellFragment

A bounded portion of one original first-row or selected-row cell.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `column` | integer (uint32) | yes | Original one-based column. |
| `first_row` | boolean | yes | Whether the cell belongs to the first source row. |
| `offset` | integer (uint32) | yes | Unicode character offset within this cell. |
| `text` | string | yes | Original cell text at this offset. |

### FirstSheetRow

The first original row without interpreting its cells as column headings.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cells` | list of string | yes | Original cells, empty when represented by cell fragments. |
| `row` | integer (uint32) | yes | Original one-based row number. |

### SheetInfo

Source sheet with its maximum original row number.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `name` | [Text](/reference/schemas/types/#text) | yes | Sheet name. |
| `rows` | integer (uint32) | yes | Last original row number, preserving gaps. |

### SheetRow

One original sparse row with its numbered first source row.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cells` | list of string | yes | This row's cells, including empty cells before populated columns. |
| `first_row` | [FirstSheetRow](#firstsheetrow) | yes | First original row, including empty cells; never inferred column headings. |
| `fragment` | [CellFragment](#cellfragment) or null | no | Sparse source cell fragment when the complete row exceeds the output bound. |
| `row` | integer (uint32) | yes | Original one-based row number. |
| `sheet` | [Text](/reference/schemas/types/#text) | yes | Original sheet name. |

### SheetRows

Type: string.

### SupplementContent

Downloaded bounded file contents or an unsupported format's original URL.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `forms` | list of string or null | no | Explicit variant spellings searched. |
| `license` | [License](/reference/schemas/types/#license) | yes | Article license. |
| `reason` | [Reason](/reference/schemas/types/#reason) or null | no | Typed unsupported reason. |
| `rows` | list of [SheetRow](#sheetrow) or null | no | Selected original rows. |
| `saved` | [ArticleSaved](/reference/schemas/types/#articlesaved) or null | no | Explicit original-byte save receipt. |
| `search_message` | string or null | no | Explicit absence of matching text from the returned search page. |
| `sha256` | [Text](/reference/schemas/types/#text) or null | no | Hash of the original downloaded bytes; absent when no download occurred. |
| `sheets` | list of [SheetInfo](#sheetinfo) or null | no | Original sheet dimensions. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Unsupported file URL, returned without downloading it. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp article supplement`](/reference/commands/article-supplement/) gives the command's arguments.
