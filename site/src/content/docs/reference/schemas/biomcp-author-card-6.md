---
title: Reply of biomcp get author (biomcp.author-card/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get author: the
  fields of schema biomcp.author-card/6."
goal: "Parse the biomcp.author-card/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author-card/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get author`](/reference/commands/get-author/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.author-card/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.author-card/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AuthorCards](#reply-authorcards). `query` holds [Reply_AuthorGet](#reply-authorget). `query` repeats the request as BioMCP read it.

### Reply_AuthorCards

Exact source cards in input order; outer source outcomes retain failures.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_AuthorCardItem](#reply-authorcarditem) | Exact returned source cards, with no cross-source links. Optional. |

### Reply_AuthorGet

Exact qualified records in caller order, with no name reconciliation.

| Field | Type | Meaning |
| --- | --- | --- |
| `ids` | list of [Reply_AuthorId](/reference/schemas/types/#reply-authorid) | One or more exact qualified IDs. |

### Other types in this reply

2 more types appear only in this reply: [Reply_AuthorCard](#reply-authorcard), [Reply_AuthorCardItem](#reply-authorcarditem). Open the list below to read their fields.

<details>
<summary>Fields of the 2 other types</summary>

### Reply_AuthorCard

Exact source author metadata without identity reconciliation.

| Field | Type | Meaning |
| --- | --- | --- |
| `header` | [Reply_AuthorHeader](/reference/schemas/types/#reply-authorheader) | Exact source candidate. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Provider record link. Optional. |

### Reply_AuthorCardItem

One exact input and its independent outcome.

| Field | Type | Meaning |
| --- | --- | --- |
| `card` | [Reply_AuthorCard](#reply-authorcard) or null | Exact checked source card when available. Optional. |
| `input` | [Reply_AuthorId](/reference/schemas/types/#reply-authorid) | Requested qualified provider identity. |
| `status` | [Reply_Status](/reference/schemas/types/#reply-status) | This exact input's outcome. |

</details>

## Next

[`biomcp get author`](/reference/commands/get-author/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
