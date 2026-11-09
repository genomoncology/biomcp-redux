---
title: Reply of biomcp article recommendations (biomcp.article_recommendations/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp article
  recommendations: a real example and the fields of schema
  biomcp.article_recommendations/3."
goal: "Parse the biomcp.article_recommendations/3 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_recommendations/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp article recommendations`](/reference/commands/article-recommendations/) replies with `schema` set to `biomcp.article_recommendations/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp article recommendations 33123537 --limit 2
biomcp article recommendations 34096690 32955176 --negative 33433946 --limit 5
```

`biomcp schema biomcp.article_recommendations/3` prints the full JSON Schema.

## Example reply

`biomcp article recommendations 33123537 --limit 2` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_recommendations/3",
  "status": "complete",
  "query": {
    "seeds": ["33123537"],
    "negative": [],
    "limit": 2,
    "offset": 0
  },
  "result": {
    "items": [
      {…},
      … 1 more
    ],
    "scope": "Semantic Scholar recommends from its pool of recent papers"
  },
  "page": {
    "offset": 0,
    "returned": 2,
    "total": 2,
    "pool": {
      "id": "25fae09494d7c0d5d317dfa9f0cfc002",
      "size": 2,
      "created": 1793140292,
      "expires": 1793226692
    },
    "truncated": false
  },
  "sources": [
    {
      "source": "semantic-scholar",
      "use": "noncommercial",
      "terms": "Semantic Scholar API License Agreement",
      "terms_url": "https://api.semanticscholar.org/license/",
      "state": "ok"
    },
    … 1 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticleRecommendations](#articlerecommendations). `query` holds [ArticleRecommend](#articlerecommend). `query` repeats the request as BioMCP read it.

### ArticleRecommendations

Source-native recommendation page.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ArticleItem](/reference/schemas/types/#articleitem) | Papers in provider order, without a local topic filter. |
| `notice` | [EvidenceNotice](/reference/schemas/types/#evidencenotice) or null | Successful zero-result explanation. Optional. |
| `scope` | string | Provider's default pool, stated without a clinical relevance claim. |

### ArticleRecommend

Recommendations in the provider's recent-paper pool order.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Maximum recommendations requested, in native order. Optional. |
| `negative` | list of [ArticleInput](/reference/schemas/types/#articleinput) | Up to ten negative paper identities. Optional. |
| `offset` | integer (uint32) | Number of stored recommendations to skip. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Fixed recommendation pool. Optional. |
| `seeds` | list of [ArticleInput](/reference/schemas/types/#articleinput) | One to ten positive paper identities. |

## Next

[`biomcp article recommendations`](/reference/commands/article-recommendations/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
