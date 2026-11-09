---
title: biomcp sync
description: "The BioMCP 1.0.0 command biomcp sync: Downloads a source's bundle,
  a local copy that commands read."
goal: Look up the arguments, sections and examples of biomcp sync.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync
kind: reference
verified: 2026-11-09
---

`biomcp sync` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads a source's [bundle](/local-data/), a local copy that commands read. This page lists its arguments, examples and reply.

The program's help describes it:

> Synchronize a local source bundle.

## Usage

```text
biomcp sync [bundle]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `bundle` (position 1) | string | no |  | `hgnc`, `clingen`, `gencc`, `wikipathways`, `cbioportal-studies`, `ema`, `ddinter`, `who-pq`, `gtr`, `who-ivd`, `fda-cdx` | Local source bundle to synchronize. Listed choices accept any letter case; replies use their listed spelling. |

## Examples

The program's help gives this example.

```sh
biomcp sync hgnc
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.sync/7`](/reference/schemas/biomcp-sync-7/), [`biomcp.wikipathways-sync/4`](/reference/schemas/biomcp-wikipathways-sync-4/), whose page lists its fields.

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
