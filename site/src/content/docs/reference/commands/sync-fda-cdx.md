---
title: biomcp sync fda-cdx
description: "The BioMCP 1.0.0 command biomcp sync fda-cdx: Downloads the FDA
  companion diagnostics bundle."
goal: Look up the arguments, sections and examples of biomcp sync fda-cdx.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync fda-cdx
kind: reference
verified: 2026-10-27
---

`biomcp sync fda-cdx` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads the FDA companion diagnostics [bundle](/local-data/). This page lists its examples.

The program's help describes it:

> Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync fda-cdx
```

## Source

The bundle it downloads is the local copy that [FDA companion diagnostics list](/sources/fda-cdx/) reads. Search and get commands for that source answer only after this command has run once. [Local data](/local-data/#bundle-sizes) lists each bundle's size.

## Examples

The program's help gives these examples.

```sh
biomcp sync fda-cdx
```

```sh
biomcp sync fda-cdx --fresh
```

## Next

[Local data](/local-data/) downloads bundles and cleans the store. The [commands index](/reference/commands/) lists every command.
