---
title: biomcp search gene
description: "The BioMCP 1.0.0 command biomcp search gene: Finds human genes by
  text, by cytoband, or both."
goal: Look up the arguments, sections and examples of biomcp search gene.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search gene
kind: reference
verified: 2026-11-09
---

`biomcp search gene` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds human genes by text, by cytoband, or both. This page lists its arguments, examples and reply.

The program's help describes it:

> Search human genes by name or identifier. Sources: hgnc.

## Usage

```text
biomcp search gene [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--band` | string | no |  |  | A cytoband prefix. Value form: string. Example: --band 9p21.3. Command sources: hgnc; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Number of results per page. Value form: integer. Example: --limit 50. Command sources: hgnc; source restrictions in the field description apply. |
| `--locus-group` | string | no |  | `protein_coding`, `non_coding_rna`, `pseudogene`, `phenotype`, `other` | Optional HGNC locus group intersected with the selector; help search gene Optional HGNC locus group intersected with the selector. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --locus-group pseudogene. Command sources: hgnc; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Number of results to skip. Value form: integer. Example: --offset 24. Command sources: hgnc; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Words matched against symbols, previous symbols, aliases and names. |

## Examples

The program's help gives these examples.

```sh
biomcp search gene KRAS
```

```sh
biomcp search gene p16
```

```sh
biomcp search gene --band 9p21.3
```

```sh
biomcp search gene --band 9p21.3 --limit 50
```

```sh
biomcp search gene --band 9p21.3 --locus-group pseudogene --limit 2 \
  --offset 24
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.gene-page/8`](/reference/schemas/biomcp-gene-page-8/), whose page lists its fields.

## Next

The [gene guide](/user-guide/gene/) shows every gene command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
