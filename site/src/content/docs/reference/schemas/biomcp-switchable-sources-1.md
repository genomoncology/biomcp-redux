---
title: Reply of biomcp source (biomcp.switchable-sources/1)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp source: the fields
  of schema biomcp.switchable-sources/1."
goal: "Parse the biomcp.switchable-sources/1 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.switchable-sources/1
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp source`](/reference/commands/source/) and [`biomcp source list`](/reference/commands/source-list/) reply with `schema` set to `biomcp.switchable-sources/1`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.switchable-sources/1` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_SwitchableSources](#reply-switchablesources). `query` holds [Reply_EmptyQuery](/reference/schemas/types/#reply-emptyquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_SwitchableSources

Source names a hosted request may leave out, and the header that carries them.

| Field | Type | Meaning |
| --- | --- | --- |
| `entries` | list of [Reply_SwitchableSource](#reply-switchablesource) | Accepted source names in public order. Optional. |
| `header` | [Reply_Text](/reference/schemas/types/#reply-text) | Comma-separated hosted HTTP request header. |

### Other types in this reply

One more type appear only in this reply: [Reply_SwitchableSource](#reply-switchablesource). Open the list below to read their fields.

<details>
<summary>Fields of the 1 other type</summary>

### Reply_SwitchableSource

One source name a hosted request may leave out.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) | Publisher display name. |
| `source` | [Reply_Source](/reference/schemas/types/#reply-source) | Source name accepted by the hosted header. |
| `usage` | [Reply_Use](/reference/schemas/types/#reply-use) | Publisher use label for the whole source. |

</details>

## Next

[`biomcp source`](/reference/commands/source/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
