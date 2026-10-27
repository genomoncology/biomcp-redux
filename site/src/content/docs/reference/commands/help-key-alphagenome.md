---
title: biomcp help key alphagenome
description: "The BioMCP 1.0.0 command biomcp help key alphagenome: Shows how to
  set the AlphaGenome key: the key name and its environment variable."
goal: Look up the arguments, sections and examples of biomcp help key alphagenome.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key alphagenome
kind: reference
verified: 2026-10-27
---

`biomcp help key alphagenome` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to set the AlphaGenome key: the key name and its environment variable. This page lists its reply.

The program's help describes it:

> Show credential registration and environment fallback names.

## Usage

```text
biomcp help key alphagenome
```

## The key

`biomcp help key alphagenome` reports these settings for the alphagenome key:

| Setting | Value |
| --- | --- |
| Set it with | `biomcp key set alphagenome` |
| Source it unlocks | [alphagenome](/sources/alphagenome/) |
| Field in config.json | `keys.alphagenome_api_key` |
| Environment variable | `ALPHAGENOME_API_KEY` |

[Keys and configuration](/getting-started/api-keys/) shows how to set it.

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/2`](/reference/schemas/biomcp-key-2/), whose page shows a real reply and its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
