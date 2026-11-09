---
title: Reply of biomcp author papers (biomcp.author-papers/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp author papers: the
  fields of schema biomcp.author-papers/6."
goal: "Parse the biomcp.author-papers/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.author-papers/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp author papers`](/reference/commands/author-papers/) and [`biomcp author`](/reference/commands/author/) reply with `schema` set to `biomcp.author-papers/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.author-papers/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_AuthorPapers](#reply_authorpapers). `query` holds [Reply_AuthorPapersQuery](/reference/schemas/types/#reply_authorpapersquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_AuthorPapers

Compact native indexed-paper window.

| Field | Type | Meaning |
| --- | --- | --- |
| `author` | [Reply_AuthorId](/reference/schemas/types/#reply_authorid) | Requested provider record. |
| `items` | list of [Reply_AuthorPaperHeader](/reference/schemas/types/#reply_authorpaperheader) | Original source order; no native corpus total is claimed. Optional. |

## Next

[`biomcp author papers`](/reference/commands/author-papers/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
