---
title: biomcp search pgx
description: "The BioMCP 1.0.0 command biomcp search pgx: Finds CPIC gene-drug
  pairs for a gene."
goal: Look up the arguments, sections and examples of biomcp search pgx.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search pgx
kind: reference
verified: 2026-11-09
---

`biomcp search pgx` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds CPIC gene-drug pairs for a gene. This page lists its arguments, examples and reply.

The program's help describes it:

> Search pharmacogenomic gene and drug pairs. Sources: cpic.

## Usage

```text
biomcp search pgx [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--cpic-level` | string | no |  |  | Exact source level, including compound levels such as B/C. Value form: string. Example: --cpic-level B/C. Command sources: cpic; source restrictions in the field description apply. |
| `--gene` | string | yes |  |  | Explicit source gene symbol. Value form: string. Example: --gene DPYD. Command sources: cpic; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Rows per source page. Value form: integer. Example: --limit 1. Command sources: cpic; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source source offset. Value form: integer. Example: --offset 0. Command sources: cpic; source restrictions in the field description apply. |
| `--source` | string | yes |  | `cpic` | Required CPIC source; ClinPGx is not supported for search; help search pgx Required CPIC source; ClinPGx is not supported for search. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source cpic. Command sources: cpic; source restrictions in the field description apply. |

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

The reply is one JSON object. Its schema is [`biomcp.pgx-page/6`](/reference/schemas/biomcp-pgx-page-6/), whose page lists its fields.

## Next

The [pgx guide](/user-guide/pgx/) shows every pgx command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
