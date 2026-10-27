---
title: biomcp serve
description: "The BioMCP 1.0.0 command biomcp serve: Runs BioMCP as an MCP
  server over standard input and output, or over HTTP."
goal: Look up the arguments, sections and examples of biomcp serve.
reader: A BioMCP user or agent checking one command
sidebar:
  label: serve
kind: reference
verified: 2026-10-27
---

`biomcp serve` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It runs BioMCP as an MCP server over standard input and output, or over HTTP. This page lists its usage.

The program's help describes it:

> Serve MCP over stdio or --http [ADDR]. Non-loopback binds require --allow-host HOST or http.allowed_hosts; neither authenticates callers. HTTP refuses sync, store clean and --fresh. Stdio saves refuse --replace.

## Usage

```text
biomcp serve
```

## Arguments

The command takes no arguments beyond those in its usage line.

## Next

The [commands index](/reference/commands/) lists every command.
