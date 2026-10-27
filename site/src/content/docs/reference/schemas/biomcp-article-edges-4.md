---
title: Reply of biomcp article references (biomcp.article_edges/4)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article references:
  a real example and the fields of schema biomcp.article_edges/4."
goal: "Parse the biomcp.article_edges/4 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_edges/4
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article references`](/reference/commands/article-references/) and [`biomcp article citations`](/reference/commands/article-citations/) reply with `schema` set to `biomcp.article_edges/4`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp article references 12068308 --limit 5
biomcp article citations 12068308 --limit 5
```

`biomcp schema biomcp.article_edges/4` prints the full JSON Schema.

## Example reply

`biomcp article references 12068308 --limit 5` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.article_edges/4",
  "status": "complete",
  "query": {
    "pmid": "12068308",
    "source": "icite",
    "limit": 5,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "key": {"namespace": "pubmed", "id": "2841597"},
        "pmid": "2841597",
        "cited_by": 6033,
        "header": {…}
      },
      … 4 more
    ]
  },
  "page": {
    "offset": 0,
    "returned": 5,
    "total": 24,
    "pool": {
      "id": "a7abd131ea51b2c15a40809a1b4f5f43",
      "size": 24,
      "created": 1793140290,
      "expires": 1793226690
    },
    "truncated": false
  },
  "sources": [
    {
      "source": "icite",
      "use": "open",
      "terms": "NIH public-domain data",
      "terms_url": "https://support.icite.nih.gov/hc/en-us/articles/906650175…",
      "state": "ok"
    },
    … 1 more
  ],
  "next": [
    {
      "kind": "page",
      "args": [
        "article", "references", "12068308", "--limit", "5", "--offset", "5",
        "--pool",
        … 1 more
      ]
    },
    … 1 more
  ]
}
```

## Fields

`result` holds [ArticleEdgePage](#articleedgepage). `query` holds [ArticleEdges](#articleedges). `query` repeats the request as BioMCP read it.

### ArticleEdgePage

A page of citing or referenced papers in citation order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ArticleItem](/reference/schemas/types/#articleitem) | yes | Papers in the fixed edge pool. |

### ArticleEdges

Page the citing or referenced PubMed papers of one checked PMID.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Maximum papers on one page. |
| `offset` | integer (uint32) | no | Number of pool items to skip. |
| `pmid` | [ArticleInput](/reference/schemas/types/#articleinput) | yes | PMID, PMC or DOI; iCite accepts only a PMID. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Fixed snapshot from an earlier page. |
| `source` | [EdgeSource](#edgesource) | no | Graph provider; iCite remains the default. |

### EdgeSource

Graph edge provider, independent of article search providers.

Values: `icite`, `semantic-scholar`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp article references`](/reference/commands/article-references/) gives the command's arguments.
