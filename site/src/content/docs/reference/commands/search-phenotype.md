---
title: biomcp search phenotype
description: "The BioMCP 1.0.0 command biomcp search phenotype: Finds Human
  Phenotype Ontology (HP) terms by text in HPO or Monarch."
goal: Look up the arguments, sections and examples of biomcp search phenotype.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search phenotype
kind: reference
verified: 2026-11-09
---

`biomcp search phenotype` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds Human Phenotype Ontology (HP) terms by text in HPO or Monarch. This page lists its arguments, examples and reply.

The program's help describes it:

> Search human phenotypes in HPO. Sources: hpo, monarch.

## Usage

```text
biomcp search phenotype [OPTIONS] <text>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum source headers, default five. Value form: integer. Example: --limit 1. Command sources: hpo, monarch; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source source row offset. Value form: integer. Example: --offset 0. Command sources: hpo, monarch; source restrictions in the field description apply. |
| `--source` | string | no |  | `hpo`, `monarch` | Monarch only; HPO search is unsupported; help search phenotype Monarch only; HPO search is unsupported. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source monarch. Command sources: hpo, monarch; source restrictions in the field description apply. |
| `text` (position 1) | string | yes |  |  | Unchanged exact term search text. |

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

```sh
biomcp search phenotype headache --source monarch
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.phenotype-search/5`](/reference/schemas/biomcp-phenotype-search-5/), whose page lists its fields.

## Next

The [phenotype guide](/user-guide/phenotype/) shows every phenotype command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
