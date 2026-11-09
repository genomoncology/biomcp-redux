---
title: biomcp sync gtr
description: "The BioMCP 1.0.0 command biomcp sync gtr: Downloads the Genetic
  Testing Registry bundle that diagnostic commands read."
goal: Look up the arguments, sections and examples of biomcp sync gtr.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync gtr
kind: reference
verified: 2026-10-28
---

`biomcp sync gtr` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the Genetic Testing Registry [bundle](/local-data/) that diagnostic commands read. This page lists its examples and reply.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync gtr
```

## Source

The bundle it downloads is the local copy that [NCBI Genetic Testing Registry](/sources/gtr/) reads. Search and get commands for that source answer only after this command has run once. [Local data](/local-data/#bundle-sizes) lists each bundle's size.

## Examples

The program's help gives these examples.

```sh
biomcp sync gtr
```

```sh
biomcp sync gtr --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/), whose page shows a real reply and its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
