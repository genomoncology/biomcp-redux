---
title: Reply of biomcp help key oncokb (biomcp.key/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp help key oncokb: a
  real example and the fields of schema biomcp.key/2."
goal: "Parse the biomcp.key/2 reply: see a real example and what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.key/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp help key oncokb`](/reference/commands/help-key-oncokb/), [`biomcp help key ncbi`](/reference/commands/help-key-ncbi/), [`biomcp help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/), [`biomcp help key orcid`](/reference/commands/help-key-orcid/), [`biomcp help key alphagenome`](/reference/commands/help-key-alphagenome/), [`biomcp help key nci`](/reference/commands/help-key-nci/) and [`biomcp help key openfda`](/reference/commands/help-key-openfda/) reply with `schema` set to `biomcp.key/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp help key oncokb
```

`biomcp schema biomcp.key/2` prints the full JSON Schema.

## Example reply

`biomcp help key oncokb` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.key/2",
  "status": "complete",
  "query": null,
  "result": {
    "guidance": "biomcp key set oncokb",
    "source": "oncokb",
    "config": "keys.oncokb_api_key",
    "environment": "ONCOKB_TOKEN"
  }
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [KeyHelp](#keyhelp). `query` repeats the request as BioMCP read it.

### KeyHelp

Configuration names for one keyed source; never includes credential values.

| Field | Type | Meaning |
| --- | --- | --- |
| `config` | [Text](/reference/schemas/types/#text) | Configuration file key path. |
| `environment` | [Text](/reference/schemas/types/#text) | Environment variable name. |
| `guidance` | [Text](/reference/schemas/types/#text) or null | Optional source-specific key guidance. Optional. |
| `source` | [Source](/reference/schemas/types/#source) | Source name. |

## Next

[`biomcp help key oncokb`](/reference/commands/help-key-oncokb/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
