---
title: Reply of biomcp author papers (biomcp.author-paper-details/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp author papers: the
  fields of schema biomcp.author-paper-details/6."
goal: "Parse the biomcp.author-paper-details/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author-paper-details/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) and [`biomcp author`](/reference/commands/author/) reply with `schema` set to `biomcp.author-paper-details/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.author-paper-details/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AuthorPaperDetails](#reply_authorpaperdetails). `query` holds [Reply_AuthorPapersQuery](/reference/schemas/types/#reply_authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AuthorPaperDetails

Detailed metadata for the same native indexed-paper window.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [Reply_AuthorId](/reference/schemas/types/#reply_authorid) | Requested provider record. |
| `items` | list of [Reply_AuthorPaperDetail](#reply_authorpaperdetail) | Original source order; no native corpus total is claimed. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_AuthorByline](#reply_authorbyline), [Reply_AuthorPaperAccess](#reply_authorpaperaccess), [Reply_AuthorPaperDetail](#reply_authorpaperdetail). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_AuthorByline

One source byline row; repeated display names remain separate.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | [Reply_SemanticScholarAuthorId](/reference/schemas/types/#reply_semanticscholarauthorid) or null | Source author ID when supplied. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native display name. Optional. |

### Reply_AuthorPaperAccess

Native access metadata; access does not establish reuse rights.

| Field | Type | Meaning |
| --- | --- | --- |
| `disclaimer` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native source disclaimer. Optional. |
| `license` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native license label. Optional. |
| `status` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Open native access status. Optional. |
| `url` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Native PDF URL. Optional. |

### Reply_AuthorPaperDetail

Detailed indexed paper metadata for the same native paper page.

| Field | Type | Meaning |
| --- | --- | --- |
| `abstract_text` | [Reply_Text](/reference/schemas/types/#reply_text) or null | Source abstract when available. Optional. |
| `authors` | list of [Reply_AuthorByline](#reply_authorbyline) or null | Native ordered byline; completeness is unknown. Optional. |
| `citation_count` | integer or null | Native citations to this paper. Optional. |
| `fields_of_study` | list of [Reply_Text](/reference/schemas/types/#reply_text) or null | Native study labels. Optional. |
| `header` | [Reply_AuthorPaperHeader](/reference/schemas/types/#reply_authorpaperheader) | Same compact paper identity. |
| `influential_citation_count` | integer or null | Native influential citations to this paper. Optional. |
| `is_open_access` | boolean or null | Native open-access flag. Optional. |
| `open_access_pdf` | [Reply_AuthorPaperAccess](#reply_authorpaperaccess) or null | Native access metadata. Optional. |
| `publication_date` | [Reply_PartialDate](/reference/schemas/types/#reply_partialdate) or null | Native publication date. Optional. |
| `publication_types` | list of [Reply_Text](/reference/schemas/types/#reply_text) or null | Native publication type labels. Optional. |
| `reference_count` | integer or null | Native references on this paper. Optional. |

</details>

## Next

[`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
