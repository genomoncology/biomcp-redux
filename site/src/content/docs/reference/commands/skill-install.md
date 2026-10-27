---
title: biomcp skill install
description: "The BioMCP 1.0.0 command biomcp skill install: Installs the agent
  guide into a folder."
goal: Look up the arguments, sections and examples of biomcp skill install.
reader: A BioMCP user or agent checking one command
sidebar:
  label: skill install
kind: reference
verified: 2026-10-27
---

`biomcp skill install` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It installs the agent guide into a folder. This page lists its arguments.

The program's help describes it:

> Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. Existing files remain unchanged without --force.

## Usage

```text
biomcp skill install [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--force` | boolean | no |  |  | Repair managed files; retain unrelated files. |
| `directory` | string | yes |  |  |  |

## Next

The [commands index](/reference/commands/) lists every command.
