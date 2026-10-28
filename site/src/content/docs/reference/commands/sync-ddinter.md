---
title: biomcp sync ddinter
description: "The BioMCP 1.0.0 command biomcp sync ddinter: Downloads the
  DDInter drug interaction bundle."
goal: Look up the arguments, sections and examples of biomcp sync ddinter.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync ddinter
kind: reference
verified: 2026-10-28
---

`biomcp sync ddinter` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the DDInter drug interaction [bundle](/local-data/). This page lists its examples and reply.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync ddinter
```

## Source

The bundle it downloads is the local copy that [DDInter](/sources/ddinter/) reads. Search and get commands for that source answer only after this command has run once. [Local data](/local-data/#bundle-sizes) lists each bundle's size.

## Examples

The program's help gives these examples.

```sh
biomcp sync ddinter
```

```sh
biomcp sync ddinter --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/), whose page shows a real reply and its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
