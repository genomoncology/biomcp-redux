---
title: biomcp sync who-ivd
description: "The BioMCP 1.0.0 command biomcp sync who-ivd: Downloads the WHO
  prequalified diagnostics bundle."
goal: Look up the arguments, sections and examples of biomcp sync who-ivd.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync who-ivd
kind: reference
verified: 2026-10-27
---

`biomcp sync who-ivd` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the WHO prequalified diagnostics [bundle](/local-data/). This page lists its examples and reply.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync who-ivd
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Examples

The program's help gives these examples.

```sh
biomcp sync who-ivd
```

```sh
biomcp sync who-ivd --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/), whose page shows a real reply and its fields.

## Next

The [commands index](/reference/commands/) lists every command.
