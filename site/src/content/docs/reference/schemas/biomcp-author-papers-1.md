---
title: Reply of biomcp author papers (biomcp.author_papers/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp author papers: a
  real example and the fields of schema biomcp.author_papers/1."
goal: "Parse the biomcp.author_papers/1 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_papers/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) replies with `schema` set to `biomcp.author_papers/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp author papers semantic-scholar:123
```

`biomcp schema biomcp.author_papers/1` prints the full JSON Schema.

## Example reply

`biomcp author papers semantic-scholar:123` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.author_papers/1",
  "status": "complete",
  "query": {
    "id": "semantic-scholar:123",
    "limit": 5,
    "offset": 0
  },
  "result": {"author": "semantic-scholar:123", "items": []},
  "sources": [
    {
      "source": "semantic-scholar",
      "use": "noncommercial",
      "terms": "Semantic Scholar API License Agreement",
      "terms_url": "https://api.semanticscholar.org/license/",
      "state": "ok"
    }
  ]
}
```

## Fields

`result` holds [AuthorPapers](#authorpapers). `query` holds [AuthorPapersQuery](/reference/schemas/types/#authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### AuthorPapers

Compact native indexed-paper window.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `author` | [AuthorId](/reference/schemas/types/#authorid) | yes | Requested provider record. |
| `items` | list of [AuthorPaperHeader](/reference/schemas/types/#authorpaperheader) | yes | Original source order; no native corpus total is claimed. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments.
