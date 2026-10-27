---
title: biomcp help network proxy
description: "The BioMCP 1.0.0 command biomcp help network proxy: Shows how to
  make BioMCP reach the sources through a proxy."
goal: Look up the arguments, sections and examples of biomcp help network proxy.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help network proxy
kind: reference
verified: 2026-10-27
---

`biomcp help network proxy` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to make BioMCP reach the sources through a proxy. This page lists its reply.

The program's help describes it:

> Set proxy in config.json to an HTTP or HTTPS proxy URL. Otherwise HTTPS_PROXY and HTTP_PROXY select routes for their schemes; NO_PROXY bypasses exact hosts/IPs, .domain, *.domain, optional ports, or *. An explicit proxy overrides proxy environment settings.

## Usage

```text
biomcp help network proxy
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/2`](/reference/schemas/biomcp-help-2/), whose page shows a real reply and its fields.

## Next

The [commands index](/reference/commands/) lists every command.
