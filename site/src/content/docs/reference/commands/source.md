---
title: biomcp source
description: "The BioMCP 1.0.0 command biomcp source: List source names as JSON."
goal: Look up the arguments, sections and examples of biomcp source.
reader: A BioMCP user or agent checking one command
sidebar:
  label: source
kind: reference
verified: 2026-11-09
---

`biomcp source` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It list source names as JSON. This page lists its examples and reply.

The program's help describes it:

> List source names as JSON. No source requests or key reads.

## Usage

```text
biomcp source
```

## Examples

The program's help gives this example.

```sh
biomcp source list
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.switchable-sources/1`](/reference/schemas/biomcp-switchable-sources-1/), whose page lists its fields.

## Next

The [commands index](/reference/commands/) lists every command.
