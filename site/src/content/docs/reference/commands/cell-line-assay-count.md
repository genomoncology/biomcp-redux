---
title: biomcp cell-line assay-count
description: "The BioMCP 1.0.0 command biomcp cell-line assay-count: Counts the
  ChEMBL assays recorded for one cell line."
goal: Look up the arguments, sections and examples of biomcp cell-line assay-count.
reader: A BioMCP user or agent checking one command
sidebar:
  label: cell-line assay-count
kind: reference
verified: 2026-10-27
---

`biomcp cell-line assay-count` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It counts the ChEMBL assays recorded for one cell line. This page lists its arguments and examples.

The program's help describes it:

> A separate assay-count request, never implicit in metadata or all. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp cell-line assay-count <id>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `id` (position 1) | string | yes |  |  | Exact ChEMBL cell identity returned by metadata. |

## Examples

The program's help gives these examples.

```sh
biomcp cell-line assay-count CHEMBL3706573
```

```sh
biomcp cell-line assay-count CHEMBL3706573 --fresh
```

## Next

The [cell-line guide](/user-guide/cell-line/) shows every cell-line command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
