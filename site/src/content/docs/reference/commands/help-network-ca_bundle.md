---
title: biomcp help network ca_bundle
description: "The BioMCP 1.0.0 command biomcp help network ca_bundle: Shows how
  to make BioMCP trust a private certificate authority."
goal: Look up the arguments, sections and examples of biomcp help network ca_bundle.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help network ca_bundle
kind: reference
verified: 2026-10-27
---

`biomcp help network ca_bundle` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to make BioMCP trust a private certificate authority. This page lists its reply.

The program's help describes it:

> Set ca_bundle in config.json to a PEM certificate file. These roots add to platform trust for every source. Keep certificate verification enabled.

## Usage

```text
biomcp help network ca_bundle
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/2`](/reference/schemas/biomcp-help-2/), whose page shows a real reply and its fields.

## Next

The [commands index](/reference/commands/) lists every command.
