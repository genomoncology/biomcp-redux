---
title: Reply of biomcp article pdf (biomcp.article_pdf/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article pdf: a real
  example and the fields of schema biomcp.article_pdf/2."
goal: "Parse the biomcp.article_pdf/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_pdf/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article pdf`](/reference/commands/article-pdf/) replies with `schema` set to `biomcp.article_pdf/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp article pdf 41351423 --save second.pdf --replace
```

`biomcp schema biomcp.article_pdf/2` prints the full JSON Schema.

## Example reply

`biomcp article pdf 41351423 --save second.pdf --replace` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_pdf/2",
  "status": "failed",
  "query": {
    "input": "41351423",
    "save": "second.pdf",
    "replace": true
  },
  "result": {
    "file": {
      "state": "unavailable",
      "source": "semantic-scholar",
      "reason": "rate_limited"
    }
  },
  "sources": [
    {
      "source": "semantic-scholar",
      "use": "noncommercial",
      "terms": "Semantic Scholar API License Agreement",
      "terms_url": "https://api.semanticscholar.org/license/",
      "state": "failed",
      "reason": "rate_limited"
    }
  ]
}
```

The source did not answer that run, so `status` is `failed` and `result` is missing. A reply that succeeds fills `result` with the fields below.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticlePdfResult](#articlepdfresult). `query` holds [ArticlePdf](#articlepdf). `query` repeats the request as BioMCP read it.

### ArticlePdfResult

Result of an explicit publisher PDF download.

| Field | Type | Meaning |
| --- | --- | --- |
| `file` | [Section](/reference/schemas/types/#section) of [PdfSaved](#pdfsaved) | Saved receipt or the reason no file was saved. |

### ArticlePdf

Download one publisher PDF to an explicit file or existing folder.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | Article identity. |
| `replace` | boolean | Replace an existing file only when explicitly requested. Optional. |
| `save` | [Text](/reference/schemas/types/#text) | Destination file or existing folder. |

### Other types in this reply

One more type appear only in this reply: [PdfSaved](#pdfsaved). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### PdfSaved

Receipt for one file explicitly saved by the caller.

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer (uint64) | Number of saved bytes. |
| `license` | [License](/reference/schemas/types/#license) | Normalized license; other grants no reuse rights. |
| `path` | [Text](/reference/schemas/types/#text) | Absolute destination path. |
| `sha256` | [Text](/reference/schemas/types/#text) | SHA-256 of the saved file. |
| `status` | [OpenAccessStatus](/reference/schemas/types/#openaccessstatus) | Source access status. |
| `url` | [Text](/reference/schemas/types/#text) | Source publisher URL. |

</details>

## Next

[`biomcp article pdf`](/reference/commands/article-pdf/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
