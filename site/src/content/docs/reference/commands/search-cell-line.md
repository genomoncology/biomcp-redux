---
title: biomcp search cell-line
description: "Reference for the BioMCP 1.0.0 command biomcp search cell-line:
  One literal cell line search, with native order and no total claim."
goal: Look up the arguments, sections and examples of biomcp search cell-line.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search cell-line
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search cell-line`.

One literal cell line search, with native order and no total claim. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

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

```sh
biomcp search cell-line MOLM13
biomcp search cell-line --query 'MV4;11'
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
