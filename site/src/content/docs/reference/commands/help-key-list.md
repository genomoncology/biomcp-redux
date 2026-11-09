---
title: biomcp help key list
description: "The BioMCP 1.0.0 command biomcp help key list: List supported
  discovery topics."
goal: Look up the arguments, sections and examples of biomcp help key list.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help key list
kind: reference
verified: 2026-11-09
---

`biomcp help key list` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It list supported discovery topics. This page lists its reply.

The program's help describes it:

> List supported discovery topics.

## Usage

```text
biomcp help key list
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.key/5`](/reference/schemas/biomcp-key-5/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
