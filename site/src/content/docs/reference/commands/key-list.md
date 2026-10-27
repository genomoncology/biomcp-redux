---
title: biomcp key list
description: "The BioMCP 1.0.0 command biomcp key list: Lists the keys BioMCP
  supports and where each one is set, without showing values."
goal: Look up the arguments, sections and examples of biomcp key list.
reader: A BioMCP user or agent checking one command
sidebar:
  label: key list
kind: reference
verified: 2026-10-27
---

`biomcp key list` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the keys BioMCP supports and where each one is set, without showing values. This page lists its reply.

The program's help describes it:

> List supported credential names and their origin without values.

## Usage

```text
biomcp key list
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Reply

The reply is one JSON object. Its schema is [`biomcp.registered-keys/2`](/reference/schemas/biomcp-registered-keys-2/), whose page shows a real reply and its fields.

## Next

The [commands index](/reference/commands/) lists every command.
