---
title: biomcp sync who-pq
description: "The BioMCP 1.0.0 command biomcp sync who-pq: Downloads the WHO
  prequalified medicines bundle."
goal: Look up the arguments, sections and examples of biomcp sync who-pq.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync who-pq
kind: reference
verified: 2026-10-27
---

`biomcp sync who-pq` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the WHO prequalified medicines [bundle](/local-data/). This page lists its examples and reply.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync who-pq
```

## Source

The bundle it downloads is the local copy that [WHO prequalified finished products](/sources/who-prequalification/) reads. Search and get commands for that source answer only after this command has run once. [Local data](/local-data/#bundle-sizes) lists each bundle's size.

## Examples

The program's help gives these examples.

```sh
biomcp sync who-pq
```

```sh
biomcp sync who-pq --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/), whose page shows a real reply and its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
