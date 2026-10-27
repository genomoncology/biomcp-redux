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

`biomcp sync ema` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the European Medicines Agency [bundle](/local-data/) that EMA drug commands read. This page lists its examples.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync ema
```

## Source

The bundle it downloads is the local copy that [European Medicines Agency](/sources/ema/) reads. Search and get commands for that source answer only after this command has run once. [Local data](/local-data/#bundle-sizes) lists each bundle's size.

## Examples

The program's help gives these examples.

```sh
biomcp sync ema
```

```sh
biomcp sync ema --fresh
```

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
