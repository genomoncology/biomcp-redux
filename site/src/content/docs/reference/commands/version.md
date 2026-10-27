---
title: biomcp version
description: "The BioMCP 1.0.0 command biomcp version: Prints the version;
  --verbose adds the commit it was built from, the schema IDs and the paths
  BioMCP uses."
goal: Look up the arguments, sections and examples of biomcp version.
reader: A BioMCP user or agent checking one command
sidebar:
  label: version
kind: reference
verified: 2026-10-27
---

`biomcp version` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It prints the version; --verbose adds the commit it was built from, the schema IDs and the paths BioMCP uses. This page lists its arguments.

The program's help describes it:

> Print version; --verbose adds build, schema and path provenance. No source requests or key reads.

## Usage

```text
biomcp version [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--verbose` | boolean | no |  |  | Include commit, target, schema versions, config and cache paths. |

## Next

The [commands index](/reference/commands/) lists every command.
