---
title: biomcp search pgx
description: "The BioMCP 1.0.0 command biomcp search pgx: Finds CPIC gene-drug
  pairs for a gene."
goal: Look up the arguments, sections and examples of biomcp search pgx.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search pgx
kind: reference
verified: 2026-10-27
---

`biomcp search pgx` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds CPIC gene-drug pairs for a gene. This page lists its arguments, examples and reply.

The program's help describes it:

> Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

## Usage

```text
biomcp search pgx [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--cpic-level` | value | no |  |  | Literal native level, including compound levels such as B/C. |
| `--gene` | string | yes |  |  | Explicit source gene symbol. |
| `--limit` | integer | no | min 1, max 50 |  | Rows per native page. |
| `--offset` | integer | no | min 0 |  | Native source offset. |
| `--source` | string | yes |  | `cpic` | Required CPIC source; ClinPGx is not supported for search. |

## Sources

`--source` picks the source to ask: [cpic](/sources/cpic/).

## Examples

The program's help gives these examples.

```sh
biomcp search pgx --gene DPYD --source cpic
```

```sh
biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.pgx_page/2`](/reference/schemas/biomcp-pgx-page-2/), whose page shows a real reply and its fields.

## Next

The [pgx guide](/user-guide/pgx/) shows every pgx command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
