---
title: Reply of biomcp search author (biomcp.author-page/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp search author: the
  fields of schema biomcp.author-page/5."
goal: "Parse the biomcp.author-page/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author-page/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp search author`](/reference/commands/search-author/) and [`biomcp search`](/reference/commands/search/) reply with `schema` set to `biomcp.author-page/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.author-page/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AuthorPage](#reply-authorpage). `query` holds [Reply_AuthorSearch](#reply-authorsearch). `query` repeats the request as BioMCP read it.

### Reply_AuthorPage

Native author candidates in provider order; search total counts only this query.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_AuthorHeader](/reference/schemas/types/#reply-authorheader) | Source candidates; no deduplication by display name. Optional. |
| `literature` | [Reply_AuthorLiterature](#reply-authorliterature) or null | Citation-scoped authors and papers; names never establish a person identity. Optional. |
| `matching_records` | integer or null | Native matching-record total, if supplied. Optional. |

### Reply_AuthorSearch

Literal author name query and native window; positional text maps to query.

| Field | Type | Meaning |
| --- | --- | --- |
| `limit` | integer (uint32) | Bounded source page size. Optional. |
| `offset` | integer (uint32) | Native offset. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) | Retained page identity. Optional. |
| `query` | string | Literal query from the positional argument. |
| `source` | [Reply_AuthorSearchSource](#reply-authorsearchsource) | Source; auto uses PubMed if Semantic Scholar is off. PubMed accepts names or topics and returns unresolved names on papers. Optional. |

### Other types in this reply

3 more types appear only in this reply: [Reply_AuthorLiterature](#reply-authorliterature), [Reply_AuthorPublication](#reply-authorpublication), [Reply_AuthorSearchSource](#reply-authorsearchsource). Open the list below to read their fields.

<details>
<summary>Fields of the 3 other types</summary>

### Reply_AuthorLiterature

Open citation evidence without inferred researcher identities or metrics.

| Field | Type | Meaning |
| --- | --- | --- |
| `identity_resolution` | [Reply_Text](/reference/schemas/types/#reply-text) | Names remain separate assertions on each paper, even when spelling matches. |
| `papers` | list of [Reply_AuthorPublication](#reply-authorpublication) | Observed citation records, in source order. |

### Reply_AuthorPublication

One paper and the author names asserted by its citation source.

| Field | Type | Meaning |
| --- | --- | --- |
| `authors` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Received author names in source order, bounded on long lists, without linking equal names. |
| `authors_omitted` | integer or null | Names left out of `authors`; absent when every received name is shown. Optional. |
| `paper` | [Reply_ArticleHeader](/reference/schemas/types/#reply-articleheader) | Citation identity and metadata. |

### Reply_AuthorSearchSource

Author identity search or open citation-author discovery.

Values: `semantic-scholar`, `pubmed`, `auto`.

</details>

## Next

[`biomcp search author`](/reference/commands/search-author/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
