---
title: biomcp help
description: "The BioMCP 1.0.0 command biomcp help: Prints the help for every
  command, or for one, as JSON."
goal: Look up the arguments, sections and examples of biomcp help.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help
kind: reference
verified: 2026-10-27
---

`biomcp help` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It prints the help for every command, or for one, as JSON. This page lists its reply.

The program's help describes it:

> Print generated command help.

## Usage

```text
biomcp help [COMMAND...]
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/2`](/reference/schemas/biomcp-help-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [commands index](/reference/commands/) lists every command.
