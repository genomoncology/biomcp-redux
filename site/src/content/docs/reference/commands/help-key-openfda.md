---
title: biomcp help key openfda
description: "The BioMCP 1.0.0 command biomcp help key openfda: Shows how to set
  the openFDA key: the key name and its environment variable."
goal: Look up the arguments, sections and examples of biomcp help key openfda.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key openfda
kind: reference
verified: 2026-10-27
---

`biomcp help key openfda` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to set the openFDA key: the key name and its environment variable. This page lists its reply.

The program's help describes it:

> Show credential registration and environment fallback names.

## Usage

```text
biomcp help key openfda
```

## The key

`biomcp help key openfda` reports these settings for the openfda key:

| Setting | Value |
| --- | --- |
| Set it with | `biomcp key set openfda` |
| Source it unlocks | [openfda-label](/sources/openfda/) |
| Field in config.json | `keys.openfda_api_key` |
| Environment variable | `OPENFDA_API_KEY` |

[Keys and configuration](/getting-started/api-keys/) shows how to set it.

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/2`](/reference/schemas/biomcp-key-2/), whose page shows a real reply and its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
