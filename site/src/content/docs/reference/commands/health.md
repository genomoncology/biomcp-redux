---
title: biomcp health
description: "The BioMCP 1.0.0 command biomcp health: Lists every source with
  its terms and checks that each one answers."
goal: Look up the arguments, sections and examples of biomcp health.
reader: A BioMCP user or agent checking one command
sidebar:
  label: health
kind: reference
verified: 2026-10-27
---

`biomcp health` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists every source with its terms and checks that each one answers. This page lists its reply.

The program's help describes it:

> Report publisher terms and bounded source reachability; at most one page for each implemented source.

## Usage

```text
biomcp health
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Reply

The reply is one JSON object. Its schema is [`biomcp.health/2`](/reference/schemas/biomcp-health-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [commands index](/reference/commands/) lists every command.
