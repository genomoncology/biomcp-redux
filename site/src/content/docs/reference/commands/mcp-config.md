---
title: biomcp mcp-config
description: "The BioMCP 1.0.0 command biomcp mcp-config: Prints the
  configuration an MCP client needs to start BioMCP."
goal: Look up the arguments, sections and examples of biomcp mcp-config.
reader: A BioMCP user or agent checking one command
sidebar:
  label: mcp-config
kind: reference
verified: 2026-10-27
---

`biomcp mcp-config` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It prints the configuration an MCP client needs to start BioMCP. This page lists its arguments.

The program's help describes it:

> Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.

## Usage

```text
biomcp mcp-config [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--absolute-path` | boolean | no |  |  | Retained compatibility flag; executable paths are always absolute. |
| `--client` | string | no |  | `mcp-servers`, `json`, `claude-desktop`, `claude-code`, `cursor`, `cline`, `servers`, `vscode`, `codex` | Choose a JSON client or the codex shell registration command; default mcp-servers. |

## Next

[Connect an agent](/getting-started/mcp-clients/) adds the server to each client. The [commands index](/reference/commands/) lists every command.
