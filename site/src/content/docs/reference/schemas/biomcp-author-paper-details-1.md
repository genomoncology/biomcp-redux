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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) replies with `schema` set to `biomcp.author_paper_details/1`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp author papers semantic-scholar:123 details
```

`biomcp schema biomcp.author_paper_details/1` prints the full JSON Schema.

## Example reply

`biomcp author papers semantic-scholar:123 details` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [AuthorPaperDetails](#authorpaperdetails). `query` holds [AuthorPapersQuery](/reference/schemas/types/#authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### AuthorPaperDetails

Detailed metadata for the same native indexed-paper window.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [AuthorId](/reference/schemas/types/#authorid) | Requested provider record. |
| `items` | list of [AuthorPaperDetail](#authorpaperdetail) | Original source order; no native corpus total is claimed. |

### AuthorByline

One source byline row; repeated display names remain separate.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [SemanticScholarAuthorId](/reference/schemas/types/#semanticscholarauthorid) or null | Source author ID when supplied. Optional. |
| `name` | [Text](/reference/schemas/types/#text) | Native display name. |

### AuthorPaperAccess

Native access metadata; access does not establish reuse rights.

| Field | Type | Meaning |
| --- | --- | --- |
| `disclaimer` | [Text](/reference/schemas/types/#text) or null | Native source disclaimer. Optional. |
| `license` | [Text](/reference/schemas/types/#text) or null | Native license label. Optional. |
| `status` | [Text](/reference/schemas/types/#text) or null | Open native access status. Optional. |
| `url` | [Text](/reference/schemas/types/#text) or null | Native PDF URL. Optional. |

### AuthorPaperDetail

Detailed indexed paper metadata for the same native paper page.

| Field | Type | Meaning |
| --- | --- | --- |
| `abstract_text` | [Text](/reference/schemas/types/#text) or null | Source abstract when available. Optional. |
| `authors` | list of [AuthorByline](#authorbyline) or null | Native ordered byline; completeness is unknown. Optional. |
| `citation_count` | integer or null | Native citations to this paper. Optional. |
| `fields_of_study` | list of [Text](/reference/schemas/types/#text) or null | Native study labels. Optional. |
| `header` | [AuthorPaperHeader](/reference/schemas/types/#authorpaperheader) | Same compact paper identity. |
| `influential_citation_count` | integer or null | Native influential citations to this paper. Optional. |
| `is_open_access` | boolean or null | Native open-access flag. Optional. |
| `open_access_pdf` | [AuthorPaperAccess](#authorpaperaccess) or null | Native access metadata. Optional. |
| `publication_date` | [PartialDate](/reference/schemas/types/#partialdate) or null | Native publication date. Optional. |
| `publication_types` | list of [Text](/reference/schemas/types/#text) or null | Native publication type labels. Optional. |
| `reference_count` | integer or null | Native references on this paper. Optional. |

## Next

[`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
