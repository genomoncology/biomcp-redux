---
title: biomcp store
description: "The BioMCP 1.0.0 command biomcp store: Manage local cached replies
  and snapshots."
goal: Look up the arguments, sections and examples of biomcp store.
reader: A BioMCP user or agent checking one command
sidebar:
  label: store
kind: reference
verified: 2026-11-09
---

`biomcp store` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It manage local cached replies and snapshots. This page lists its examples and reply.

The program's help describes it:

> Manage local cached replies and snapshots.

## Usage

```text
biomcp store
```

## Examples

The program's help gives this example.

```sh
biomcp store clean
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.store-clean/4`](/reference/schemas/biomcp-store-clean-4/), whose page lists its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
