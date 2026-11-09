---
title: biomcp help network
description: "The BioMCP 1.0.0 command biomcp help network: Describe help network."
goal: Look up the arguments, sections and examples of biomcp help network.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help network
kind: reference
verified: 2026-11-09
---

`biomcp help network` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It describe help network. This page lists its arguments and reply.

The program's help describes it:

> Describe help network.

## Usage

```text
biomcp help network [setting]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `setting` (position 1) | string | no |  | `ca_bundle`, `proxy` | Network setting to describe. Listed choices accept any letter case; replies use their listed spelling. |

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/6`](/reference/schemas/biomcp-help-6/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
