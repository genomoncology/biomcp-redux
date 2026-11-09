---
title: Reply of biomcp help key (biomcp.key/5)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp help key: the fields
  of schema biomcp.key/5."
goal: "Parse the biomcp.key/5 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.key/5
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp help key`](/reference/commands/help-key/), [`biomcp help key list`](/reference/commands/help-key-list/), [`biomcp help key set`](/reference/commands/help-key-set/), [`biomcp help key remove`](/reference/commands/help-key-remove/), [`biomcp help key ncbi`](/reference/commands/help-key-ncbi/), [`biomcp help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/), [`biomcp help key orcid`](/reference/commands/help-key-orcid/), [`biomcp help key oncokb`](/reference/commands/help-key-oncokb/), [`biomcp help key alphagenome`](/reference/commands/help-key-alphagenome/), [`biomcp help key nci`](/reference/commands/help-key-nci/) and [`biomcp help key openfda`](/reference/commands/help-key-openfda/) reply with `schema` set to `biomcp.key/5`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.key/5` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_KeyHelp](#reply-keyhelp). `query` holds [Reply_EmptyQuery](/reference/schemas/types/#reply-emptyquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_KeyHelp

Configuration names for one keyed source; never includes credential values.

| Field | Type | Meaning |
| --- | --- | --- |
| `config` | [Reply_Text](/reference/schemas/types/#reply-text) | Configuration file key path. |
| `environment` | [Reply_Text](/reference/schemas/types/#reply-text) | Environment variable name. |
| `guidance` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Optional source-specific key guidance. Optional. |
| `source` | [Reply_Source](/reference/schemas/types/#reply-source) | Source name. |

## Next

[`biomcp help key`](/reference/commands/help-key/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
