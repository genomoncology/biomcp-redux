---
title: biomcp sync ema
description: "The BioMCP 1.0.0 command biomcp sync ema: Downloads the European
  Medicines Agency bundle that EMA drug commands read."
goal: Look up the arguments, sections and examples of biomcp sync ema.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync ema
kind: reference
verified: 2026-10-27
---

`biomcp sync ema` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the European Medicines Agency [bundle](/local-data/) that EMA drug commands read. This page lists its examples and reply.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync ema
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Examples

The program's help gives these examples.

```sh
biomcp sync ema
```

```sh
biomcp sync ema --fresh
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [commands index](/reference/commands/) lists every command.
