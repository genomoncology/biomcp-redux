---
title: biomcp cell-line
description: "The BioMCP 1.0.0 command biomcp cell-line: Read source cell line
  assay counts."
goal: Look up the arguments, sections and examples of biomcp cell-line.
reader: A BioMCP user or agent checking one command
sidebar:
  label: cell-line
kind: reference
verified: 2026-11-09
---

`biomcp cell-line` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read source cell line assay counts. This page lists its examples and reply.

The program's help describes it:

> Read source cell line assay counts.

## Usage

```text
biomcp cell-line
```

## Examples

The program's help gives this example.

```sh
biomcp cell-line assay-count CHEMBL3706573
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.cell-line-assay-count/6`](/reference/schemas/biomcp-cell-line-assay-count-6/), whose page lists its fields.

## Next

The [cell-line guide](/user-guide/cell-line/) shows every cell-line command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
