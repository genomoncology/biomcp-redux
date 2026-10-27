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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article text`](/reference/commands/article-text/) replies with `schema` set to `biomcp.article_text/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp article text PMC3549296 --find V600E
```

`biomcp schema biomcp.article_text/3` prints the full JSON Schema.

## Example reply

`biomcp article text PMC3549296 --find V600E` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
          {
            "part": 4,
            "markdown": "Vemurafenib (PLX4032) is a potent inhibitor of mutat…",
            "heading_path": []
          },
          … 6 more
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

`result` holds [ArticleTextResult](#articletextresult). `query` holds [ArticleText](#articletext). `query` repeats the request as BioMCP read it.

### ArticleTextResult

Body text response; the envelope supplies advancing reader steps.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `content` | [Section](/reference/schemas/types/#section) of [TextContent](#textcontent) | yes | Explicit section outcome including not applicable and not open. |
| `pmcid` | [Pmcid](/reference/schemas/types/#pmcid) or null | no | Requested PMC record when known. |

### ArticleText

Read one bounded body part, find matching rows, or save the complete Markdown document.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `find` | [Text](/reference/schemas/types/#text) | no | Case-insensitive literal text to find. |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | yes | PubMed or PMC identity. DOI selectors are not accepted by the reader. |
| `offset` | integer (uint32) | no | Number of matching rows already returned. |
| `part` | integer (uint32) | no | One-based outline part number. |
| `replace` | boolean | no | Replace an existing destination only when explicitly requested with save. |
| `save` | [Text](/reference/schemas/types/#text) | no | Save the whole original document to an explicit file or existing folder. |
| `variant` | [Text](/reference/schemas/types/#text) | no | Explicit checked protein substitution. |

### TextContent

One bounded part or matching rows with the bucket license.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `forms` | list of string or null | no | Literal protein change spellings searched by an explicit variant selector. |
| `heading_path` | list of [Text](/reference/schemas/types/#text) or null | no | Section ancestry for a selected part. |
| `hits` | list of [TextHit](#texthit) or null | no | Matching source rows for a find selection. |
| `license` | [License](/reference/schemas/types/#license) | yes | Bucket license. |
| `manuscript` | boolean | no | Author manuscript flag. |
| `markdown` | [Markdown](/reference/schemas/types/#markdown) or null | no | Bounded body text for a direct part selection. |
| `part` | integer or null | no | Part number for a direct part selection. |
| `processing_version` | integer (uint32) | no | Selected processing version. |
| `saved` | [ArticleSaved](/reference/schemas/types/#articlesaved) or null | no | Explicit save receipt, absent for reader views. |
| `scope` | [Text](/reference/schemas/types/#text) | yes | Scope of returned source text. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Source URL for an explicit save, including refused destinations. |

### TextHit

One matching paragraph or table row, split when necessary to fit the text budget.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `heading_path` | list of [Text](/reference/schemas/types/#text) | yes | Section ancestry in source order. |
| `markdown` | [Markdown](/reference/schemas/types/#markdown) | yes | Whole bounded source row. |
| `part` | integer (uint32) | yes | Outline part containing this text. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp article text`](/reference/commands/article-text/) gives the command's arguments.
