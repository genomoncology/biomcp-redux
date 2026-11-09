---
title: Reply of biomcp article text (biomcp.article_text/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article text: a real
  example and the fields of schema biomcp.article_text/3."
goal: "Parse the biomcp.article_text/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_text/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article text`](/reference/commands/article-text/) replies with `schema` set to `biomcp.article_text/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp article text PMC3549296 --find V600E
```

`biomcp schema biomcp.article_text/3` prints the full JSON Schema.

## Example reply

`biomcp article text PMC3549296 --find V600E` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_text/3",
  "status": "complete",
  "query": {
    "input": "PMC3549296",
    "find": "V600E",
    "offset": 0
  },
  "result": {
    "pmcid": "PMC3549296",
    "content": {
      "state": "data",
      "source": "pmc-oa",
      "data": {
        "license": "tdm",
        "processing_version": 1,
        "manuscript": true,
        "scope": "PMC body sections and tables",
        "hits": [
          {
            "part": 3,
            "markdown": "Approximately 40 to 60% of cutaneous melanomas carry…",
            "heading_path": []
          },
          … 7 more
        ]
      }
    }
  },
  "page": {"offset": 0, "returned": 8, "total": 8},
  "sources": [
    {
      "source": "pmc-oa",
      "use": "restricted",
      "terms": "PMC copyright; article-specific licenses",
      "terms_url": "https://pmc.ncbi.nlm.nih.gov/about/copyright/",
      "state": "ok"
    }
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticleTextResult](#articletextresult). `query` holds [ArticleText](#articletext). `query` repeats the request as BioMCP read it.

### ArticleTextResult

Body text response; the envelope supplies advancing reader steps.

| Field | Type | Meaning |
| --- | --- | --- |
| `content` | [Section](/reference/schemas/types/#section) of [TextContent](#textcontent) | Explicit section outcome including not applicable and not open. |
| `pmcid` | [Pmcid](/reference/schemas/types/#pmcid) or null | Requested PMC record when known. Optional. |

### ArticleText

Read one bounded body part, find matching rows, or save the complete Markdown document.

| Field | Type | Meaning |
| --- | --- | --- |
| `find` | [Text](/reference/schemas/types/#text) | Case-insensitive literal text to find. Optional. |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | PubMed or PMC identity. DOI selectors are not accepted by the reader. |
| `offset` | integer (uint32) | Number of matching rows already returned. Optional. |
| `part` | integer (uint32) | One-based outline part number. Optional. |
| `replace` | boolean | Replace an existing destination only when explicitly requested with save. Optional. |
| `save` | [Text](/reference/schemas/types/#text) | Save the whole original document to an explicit file or existing folder. Optional. |
| `variant` | [Text](/reference/schemas/types/#text) | Explicit checked protein substitution. Optional. |

### Other types in this reply

2 more types appear only in this reply: [TextContent](#textcontent), [TextHit](#texthit). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### TextContent

One bounded part or matching rows with the bucket license.

| Field | Type | Meaning |
| --- | --- | --- |
| `forms` | list of string or null | Literal protein change spellings searched by an explicit variant selector. Optional. |
| `heading_path` | list of [Text](/reference/schemas/types/#text) or null | Section ancestry for a selected part. Optional. |
| `hits` | list of [TextHit](#texthit) or null | Matching source rows for a find selection. Optional. |
| `license` | [License](/reference/schemas/types/#license) | Bucket license. |
| `manuscript` | boolean | Author manuscript flag. Optional. |
| `markdown` | [Markdown](/reference/schemas/types/#markdown) or null | Bounded body text for a direct part selection. Optional. |
| `part` | integer or null | Part number for a direct part selection. Optional. |
| `processing_version` | integer (uint32) | Selected processing version. Optional. |
| `saved` | [ArticleSaved](/reference/schemas/types/#articlesaved) or null | Explicit save receipt, absent for reader views. Optional. |
| `scope` | [Text](/reference/schemas/types/#text) | Scope of returned source text. |
| `url` | [Text](/reference/schemas/types/#text) or null | Source URL for an explicit save, including refused destinations. Optional. |

### TextHit

One matching paragraph or table row, split when necessary to fit the text budget.

| Field | Type | Meaning |
| --- | --- | --- |
| `heading_path` | list of [Text](/reference/schemas/types/#text) | Section ancestry in source order. |
| `markdown` | [Markdown](/reference/schemas/types/#markdown) | Whole bounded source row. |
| `part` | integer (uint32) | Outline part containing this text. |

</details>

## Next

[`biomcp article text`](/reference/commands/article-text/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
