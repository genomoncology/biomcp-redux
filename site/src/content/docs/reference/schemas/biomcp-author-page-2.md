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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search author`](/reference/commands/search-author/) replies with `schema` set to `biomcp.author_page/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp search author 'Louis Williams' --limit 3
biomcp search author 'Louis Williams'
```

`biomcp schema biomcp.author_page/2` prints the full JSON Schema.

## Example reply

`biomcp search author 'Louis Williams' --limit 3` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
      {…},
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
…
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AuthorPage](#authorpage). `query` holds [AuthorSearch](#authorsearch). `query` repeats the request as BioMCP read it.

### AuthorPage

Native author candidates in provider order; search total counts only this query.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [AuthorHeader](/reference/schemas/types/#authorheader) | Source candidates; no deduplication by display name. |
| `matching_records` | integer or null | Native matching-record total, if supplied. Optional. |

### AuthorSearch

Literal author name query and native window; positional text maps to query.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Bounded source page size. Optional. |
| `offset` | integer (uint32) | Native offset. Optional. |
| `pool` | [Text](/reference/schemas/types/#text) | Retained page identity. Optional. |
| `query` | string | Literal query from the positional argument or --query. |
| `source` | [AuthorSearchSource](#authorsearchsource) | Native author source. Optional. |

### AuthorSearchSource

Author search supports one provider and its legacy spelling.

Values: `semantic-scholar`, `semanticscholar`.

## Next

[`biomcp search author`](/reference/commands/search-author/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
