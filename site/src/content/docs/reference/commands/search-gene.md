---
title: biomcp search gene
description: "The BioMCP 1.0.0 command biomcp search gene: Finds human genes by
  text, by cytoband, or both."
goal: Look up the arguments, sections and examples of biomcp search gene.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search gene
kind: reference
verified: 2026-10-27
---

`biomcp search gene` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds human genes by text, by cytoband, or both. This page lists its arguments, examples and reply.

The program's help describes it:

> Search genes by text, cytoband, or their intersection.

## Usage

```text
biomcp search gene [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--band` | string | no |  |  | A cytoband prefix. |
| `--limit` | integer | no | min 1, max 50 |  | Number of results per page. |
| `--locus-group` | string | no |  | `protein_coding`, `non_coding_rna`, `pseudogene`, `phenotype`, `other` | Optional HGNC locus group intersected with the selector. |
| `--offset` | integer | no | min 0 |  | Number of results to skip. |
| `query` (position 1) | string | no |  |  | Words matched against symbols, previous symbols, aliases and names. |

## Examples

The program's help gives these examples.

```sh
biomcp search gene KRAS
```

```sh
biomcp search gene p16
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.gene_page/3`](/reference/schemas/biomcp-gene-page-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [gene guide](/user-guide/gene/) shows every gene command at work.
