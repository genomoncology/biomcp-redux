---
title: biomcp search cell-line
description: "The BioMCP 1.0.0 command biomcp search cell-line: Finds cell lines
  by name in Cellosaurus."
goal: Look up the arguments, sections and examples of biomcp search cell-line.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search cell-line
kind: reference
verified: 2026-10-27
---

`biomcp search cell-line` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds cell lines by name in Cellosaurus. This page lists its arguments, examples and reply.

The program's help describes it:

> One literal cell line search, with native order and no total claim. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp search cell-line [OPTIONS] <query>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Native page size. |
| `--offset` | integer | no | min 0 |  | Native start offset. |
| `query` (position 1) | string | yes |  |  | Literal source query; positional spelling and --query use the same field. |

## Examples

The program's help gives these examples.

```sh
biomcp search cell-line MOLM13
```

```sh
biomcp search cell-line --query 'MV4;11'
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.cell_line_page/2`](/reference/schemas/biomcp-cell-line-page-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [cell-line guide](/user-guide/cell-line/) shows every cell-line command at work.
