---
title: Reply of biomcp get gwas (biomcp.gwas-page/8)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get gwas: the fields
  of schema biomcp.gwas-page/8."
goal: "Parse the biomcp.gwas-page/8 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.gwas-page/8
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get gwas`](/reference/commands/get-gwas/), [`biomcp search gwas`](/reference/commands/search-gwas/), [`biomcp search`](/reference/commands/search/) and [`biomcp get`](/reference/commands/get/) reply with `schema` set to `biomcp.gwas-page/8`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.gwas-page/8` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_GwasPage](#reply-gwaspage). `query` holds [Reply_GwasPageQuery](#reply-gwaspagequery). `query` repeats the request as BioMCP read it.

### Reply_GwasPage

One native association page, also used for a study associations section.

| Field | Type | Meaning |
| --- | --- | --- |
| `continuation` | [Reply_GwasPageQuery](#reply-gwaspagequery) | Same-selector search continuation. Optional. |
| `items` | list of [Reply_GwasAssociation](/reference/schemas/types/#reply-gwasassociation) | Shared complete association records. Optional. |
| `page` | [Reply_PageInfo](/reference/schemas/types/#reply-pageinfo) | Native page scope and total. |
| `qualifying_total` | integer or null | Complete qualifying count, unknown until the threshold walk reaches its end. Optional. |

### Reply_GwasPageQuery

Search and study-association pages echo their original checked request.

One of these forms:

- `operation` is `"search"`: Native association search. Fields: `query` ([Reply_GwasSearch](/reference/schemas/types/#reply-gwassearch)).
- `operation` is `"get"`: Exact study associations relation. Fields: `query` ([Reply_GwasGet](/reference/schemas/types/#reply-gwasget)).

## Next

[`biomcp get gwas`](/reference/commands/get-gwas/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
