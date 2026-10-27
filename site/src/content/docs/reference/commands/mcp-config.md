---
title: biomcp mcp-config
description: "Reference for the BioMCP 1.0.0 command biomcp mcp-config: Print
  MCP configuration with this executable’s absolute path."
goal: Look up the arguments, sections and examples of biomcp mcp-config.
reader: A BioMCP user or agent checking one command
sidebar:
  label: mcp-config
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp mcp-config`.

Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.

## Usage

```text
biomcp mcp-config [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--absolute-path` | boolean | no |  |  | Retained compatibility flag; executable paths are always absolute. |
| `--client` | string | no |  | `mcp-servers`, `json`, `claude-desktop`, `claude-code`, `cursor`, `cline`, `servers`, `vscode`, `codex` | Choose a JSON client or the codex shell registration command; default mcp-servers. |

## Examples

The help lists no examples for this command.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
