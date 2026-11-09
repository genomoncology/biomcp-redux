---
title: Reply of biomcp pathway events (biomcp.pathway-events/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp pathway events: the
  fields of schema biomcp.pathway-events/6."
goal: "Parse the biomcp.pathway-events/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.pathway-events/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp pathway events`](/reference/commands/pathway-events/) and [`biomcp pathway`](/reference/commands/pathway/) reply with `schema` set to `biomcp.pathway-events/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.pathway-events/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_PathwayEvents](/reference/schemas/types/#reply-pathwayevents), a shared type. `query` holds [Reply_PathwayEventsQuery](#reply-pathwayeventsquery). `query` repeats the request as BioMCP read it.

### Reply_PathwayEventsQuery

Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | string | Checked source-owned human pathway identity. |
| `limit` | integer (uint32) | Five complete relation rows by default, at most fifty. Optional. |
| `offset` | integer (uint32) | Offset within the retained selected relation. Optional. |
| `pool` | [Reply_Text](/reference/schemas/types/#reply-text) | Immutable relation snapshot. Optional. |

## Next

[`biomcp pathway events`](/reference/commands/pathway-events/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
