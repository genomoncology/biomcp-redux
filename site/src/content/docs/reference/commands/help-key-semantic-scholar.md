---
title: biomcp help key semantic-scholar
description: "The BioMCP 1.0.0 command biomcp help key semantic-scholar: Shows
  how to set the Semantic Scholar key: the key name and its environment
  variable."
goal: Look up the arguments, sections and examples of biomcp help key
  semantic-scholar.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key semantic-scholar
kind: reference
verified: 2026-10-27
---

`biomcp help key semantic-scholar` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to set the Semantic Scholar key: the key name and its environment variable. This page lists its reply.

The program's help describes it:

> Show credential registration and environment fallback names.

## Usage

```text
biomcp help key semantic-scholar
```

## Arguments

The command takes no arguments beyond those in its usage line.

## The key

`biomcp help key semantic-scholar` reports these settings for the semantic-scholar key:

| Setting | Value |
| --- | --- |
| Set it with | `biomcp key set semantic-scholar; a free key raises the request limit` |
| Source it unlocks | [semantic-scholar](/sources/semantic-scholar/) |
| Field in config.json | `keys.semantic_scholar_api_key` |
| Environment variable | `S2_API_KEY` |

[Keys and configuration](/getting-started/api-keys/) shows how to set it.

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/2`](/reference/schemas/biomcp-key-2/), whose page shows a real reply and its fields.

## Next

The [commands index](/reference/commands/) lists every command.
