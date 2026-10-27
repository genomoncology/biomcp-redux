---
title: Reply of biomcp search author (biomcp.author_page/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search author: a
  real example and the fields of schema biomcp.author_page/2."
goal: "Parse the biomcp.author_page/2 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_page/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search author`](/reference/commands/search-author/) replies with `schema` set to `biomcp.author_page/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp search author 'Louis Williams' --limit 3
biomcp search author 'Louis Williams'
```

`biomcp schema biomcp.author_page/2` prints the full JSON Schema.

## Example reply

`biomcp search author 'Louis Williams' --limit 3` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.author_page/2",
  "status": "complete",
  "query": {
    "query": "Louis Williams",
    "source": "semantic-scholar",
    "limit": 3,
    "offset": 0
  },
  "result": {
    "items": [
      {
        "id": "semantic-scholar:2162869982",
        "name": "L. Williams",
        "affiliations": [],
        "counts": {
          "scope": "provider_record",
          "paper_count": 97,
          "citation_count": 269,
          "h_index": 7
        }
      },
      … 2 more
    ],
    "matching_records": 58
  },
  "page": {
    "offset": 0,
    "returned": 3,
    "total": 58,
    "pool": {
      "id": "1579697d96634dab0785dbe527d54ccb",
      "size": 3,
      "created": 1793140296,
      "expires": 1793226696
    }
  },
  "sources": [
    {
      "source": "semantic-scholar",
      "use": "noncommercial",
      "terms": "Semantic Scholar API License Agreement",
      "terms_url": "https://api.semanticscholar.org/license/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "disambiguate",
      "args": ["get", "author", "semantic-scholar:2162869982"]
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [AuthorPage](#authorpage). `query` holds [AuthorSearch](#authorsearch). `query` repeats the request as BioMCP read it.

### AuthorPage

Native author candidates in provider order; search total counts only this query.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [AuthorHeader](/reference/schemas/types/#authorheader) | yes | Source candidates; no deduplication by display name. |
| `matching_records` | integer or null | no | Native matching-record total, if supplied. |

### AuthorSearch

Literal author name query and native window; positional text maps to query.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `limit` | integer (uint32) | no | Bounded source page size. |
| `offset` | integer (uint32) | no | Native offset. |
| `pool` | [Text](/reference/schemas/types/#text) | no | Retained page identity. |
| `query` | string | yes | Literal query from the positional argument or --query. |
| `source` | [AuthorSearchSource](#authorsearchsource) | no | Native author source. |

### AuthorSearchSource

Author search supports one provider and its legacy spelling.

Values: `semantic-scholar`, `semanticscholar`.

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp search author`](/reference/commands/search-author/) gives the command's arguments.
