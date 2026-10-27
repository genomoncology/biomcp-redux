---
title: Reply of biomcp author papers (biomcp.author_paper_details/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp author papers: a
  real example and the fields of schema biomcp.author_paper_details/1."
goal: "Parse the biomcp.author_paper_details/1 reply: see a real example and
  what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author_paper_details/1
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) replies with `schema` set to `biomcp.author_paper_details/1`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp author papers semantic-scholar:123 details
```

`biomcp schema biomcp.author_paper_details/1` prints the full JSON Schema.

## Example reply

`biomcp author papers semantic-scholar:123 details` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.author_paper_details/1",
  "status": "complete",
  "query": {
    "id": "semantic-scholar:123",
    "section": "details",
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

`result` holds [AuthorPaperDetails](#authorpaperdetails). `query` holds [AuthorPapersQuery](/reference/schemas/types/#authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### AuthorPaperDetails

Detailed metadata for the same native indexed-paper window.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `author` | [AuthorId](/reference/schemas/types/#authorid) | yes | Requested provider record. |
| `items` | list of [AuthorPaperDetail](#authorpaperdetail) | yes | Original source order; no native corpus total is claimed. |

### AuthorByline

One source byline row; repeated display names remain separate.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `id` | [SemanticScholarAuthorId](/reference/schemas/types/#semanticscholarauthorid) or null | no | Source author ID when supplied. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Native display name. |

### AuthorPaperAccess

Native access metadata; access does not establish reuse rights.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `disclaimer` | [Text](/reference/schemas/types/#text) or null | no | Native source disclaimer. |
| `license` | [Text](/reference/schemas/types/#text) or null | no | Native license label. |
| `status` | [Text](/reference/schemas/types/#text) or null | no | Open native access status. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Native PDF URL. |

### AuthorPaperDetail

Detailed indexed paper metadata for the same native paper page.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `abstract_text` | [Text](/reference/schemas/types/#text) or null | no | Source abstract when available. |
| `authors` | list of [AuthorByline](#authorbyline) or null | no | Native ordered byline; completeness is unknown. |
| `citation_count` | integer or null | no | Native citations to this paper. |
| `fields_of_study` | list of [Text](/reference/schemas/types/#text) or null | no | Native study labels. |
| `header` | [AuthorPaperHeader](/reference/schemas/types/#authorpaperheader) | yes | Same compact paper identity. |
| `influential_citation_count` | integer or null | no | Native influential citations to this paper. |
| `is_open_access` | boolean or null | no | Native open-access flag. |
| `open_access_pdf` | [AuthorPaperAccess](#authorpaperaccess) or null | no | Native access metadata. |
| `publication_date` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Native publication date. |
| `publication_types` | list of [Text](/reference/schemas/types/#text) or null | no | Native publication type labels. |
| `reference_count` | integer or null | no | Native references on this paper. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments.
