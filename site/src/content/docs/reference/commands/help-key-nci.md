---
title: biomcp help key nci
description: "The BioMCP 1.0.0 command biomcp help key nci: Shows how to set the
  NCI key: the key name and its environment variable."
goal: Look up the arguments, sections and examples of biomcp help key nci.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key nci
kind: reference
verified: 2026-10-27
---

`biomcp help key nci` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to set the NCI key: the key name and its environment variable. This page lists its reply.

The program's help describes it:

> Show credential registration and environment fallback names.

## Usage

```text
biomcp help key nci
```

## The key

`biomcp help key nci` reports these settings for the nci key:

| Setting | Value |
| --- | --- |
| Set it with | `biomcp key set nci` |
| Source it unlocks | [nci](/sources/nci/) |
| Field in config.json | `keys.nci_api_key` |
| Environment variable | `NCI_API_KEY` |

[Keys and configuration](/getting-started/api-keys/) shows how to set it.

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/2`](/reference/schemas/biomcp-key-2/), whose page shows a real reply and its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
