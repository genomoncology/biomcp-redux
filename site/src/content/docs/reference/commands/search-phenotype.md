---
title: biomcp search phenotype
description: "The BioMCP 1.0.0 command biomcp search phenotype: Finds Human
  Phenotype Ontology (HP) terms by text in HPO or Monarch."
goal: Look up the arguments, sections and examples of biomcp search phenotype.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search phenotype
kind: reference
verified: 2026-10-27
---

`biomcp search phenotype` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds Human Phenotype Ontology (HP) terms by text in HPO or Monarch. This page lists its arguments and examples.

The program's help describes it:

> Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

## Usage

```text
biomcp search phenotype [OPTIONS] <text>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum native headers, default five. |
| `--offset` | integer | no | min 0 |  | Native source row offset. |
| `--source` | string | no |  | `hpo`, `monarch` | Monarch only; HPO search is unsupported. |
| `text` (position 1) | string | yes |  |  | Unchanged literal term search text. |

## Sources

`--source` picks the source to ask: [hpo](/sources/hpo/), [monarch](/sources/monarch-initiative/).

## Examples

The program's help gives these examples.

```sh
biomcp search phenotype synthetic
```

```sh
biomcp search phenotype synthetic --limit 1
```

## Next

The [phenotype guide](/user-guide/phenotype/) shows every phenotype command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
