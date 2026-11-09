---
title: biomcp key
description: "The BioMCP 1.0.0 command biomcp key: Register and manage source
  credentials without showing values."
goal: Look up the arguments, sections and examples of biomcp key.
reader: A BioMCP user or agent checking one command
sidebar:
  label: key
kind: reference
verified: 2026-11-09
---

`biomcp key` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It register and manage source credentials without showing values. This page lists its examples and reply.

The program's help describes it:

> Register and manage source credentials without showing values.

## Usage

```text
biomcp key
```

## Examples

The program's help gives this example.

```sh
biomcp key list
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.registered-keys/5`](/reference/schemas/biomcp-registered-keys-5/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
