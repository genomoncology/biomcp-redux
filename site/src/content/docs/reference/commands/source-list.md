---
title: biomcp source list
description: "The BioMCP 1.0.0 command biomcp source list: List the source names
  a hosted request may leave out."
goal: Look up the arguments, sections and examples of biomcp source list.
reader: A BioMCP user or agent checking one command
sidebar:
  label: source list
kind: reference
verified: 2026-11-09
---

`biomcp source list` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It list the source names a hosted request may leave out. This page lists its examples and reply.

The program's help describes it:

> List the source names a hosted request may leave out.

## Usage

```text
biomcp source list
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
