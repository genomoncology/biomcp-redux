---
title: biomcp help
description: "The BioMCP 1.0.0 command biomcp help: Prints the help for every
  command, or for one, as JSON."
goal: Look up the arguments, sections and examples of biomcp help.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help
kind: reference
verified: 2026-11-09
---

`biomcp help` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It prints the help for every command, or for one, as JSON. This page lists its arguments and reply.

The program's help describes it:

> Print generated command help.

## Usage

```text
biomcp help [command]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `command` (position 1) | string | no |  |  | Command path or discovery topic to describe. |

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/6`](/reference/schemas/biomcp-help-6/), whose page lists its fields.

## Next

The [commands index](/reference/commands/) lists every command.
