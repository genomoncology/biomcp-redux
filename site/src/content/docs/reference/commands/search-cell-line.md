---
title: biomcp search cell-line
description: "The BioMCP 1.0.0 command biomcp search cell-line: Finds cell lines
  by name in Cellosaurus."
goal: Look up the arguments, sections and examples of biomcp search cell-line.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search cell-line
kind: reference
verified: 2026-11-09
---

`biomcp search cell-line` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds cell lines by name in Cellosaurus. This page lists its arguments, examples and reply.

The program's help describes it:

> Search cell lines by name or identifier. Sources: cellosaurus.

## Usage

```text
biomcp search cell-line [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--disease` | string | no |  |  | Cellosaurus disease: NCIt name or ID (for example Melanoma or C3224). Value form: string. Example: --disease Melanoma. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--exclude-derived` | boolean | no |  |  | Leave out lines derived from a parent line; needs another search term. Value form: boolean. Example: --exclude-derived. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Cellosaurus variation gene, as the source writes it (for example KRAS). Value form: string. Example: --gene KRAS. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 25 |  | Source page size. Value form: integer. Example: --limit 25. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--mutation` | string | no |  |  | Cellosaurus variation text, as the source writes it (for example p.Gly12Asp). Value form: string. Example: --mutation p.Gly12Asp. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source start offset. Value form: integer. Example: --offset 0. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--protein` | string | no |  |  | Protein change, sent in the source's spelling (G12D is sent as p.Gly12Asp). Value form: string. Example: --protein G12D. Command sources: cellosaurus; source restrictions in the field description apply. |
| `--species` | string | no |  |  | Cellosaurus species: NCBI Taxonomy name or ID (for example Homo sapiens or 9606). Value form: string. Example: --species 'Homo sapiens'. Command sources: cellosaurus; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Exact source query from the positional argument. |

## Examples

The program's help gives these examples.

```sh
biomcp search cell-line MOLM13
```

```sh
biomcp search cell-line 'MV4;11'
```

```sh
biomcp search cell-line MOLM13 -l 5 --offset 0
```

```sh
biomcp search cell-line --species 'Homo sapiens' --disease Melanoma
```

```sh
biomcp search cell-line --species 9606 --disease 'Desmoplastic melanoma' \
  --limit 25
```

```sh
biomcp search cell-line AsPC-1 --exclude-derived
```

```sh
biomcp search cell-line --gene KRAS --protein G12D --species 9606 \
  --exclude-derived
```

```sh
biomcp search cell-line --gene KRAS --mutation p.Gly12Asp
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.cell-line-filtered-page/1`](/reference/schemas/biomcp-cell-line-filtered-page-1/), [`biomcp.cell-line-page/7`](/reference/schemas/biomcp-cell-line-page-7/), whose page lists its fields.

## Next

The [cell-line guide](/user-guide/cell-line/) shows every cell-line command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
