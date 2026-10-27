---
title: biomcp sync
description: "The BioMCP 1.0.0 command biomcp sync: Downloads a source's bundle,
  a local copy that commands read."
goal: Look up the arguments, sections and examples of biomcp sync.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync
kind: reference
verified: 2026-10-27
---

`biomcp sync` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads a source's [bundle](/local-data/), a local copy that commands read. This page lists its reply.

The program's help describes it:

> Synchronize a local source bundle.

## Usage

```text
biomcp sync [hgnc|clingen]
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/), whose page shows a real reply and its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
