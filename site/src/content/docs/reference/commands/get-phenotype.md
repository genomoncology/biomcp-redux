---
title: biomcp get phenotype
description: "The BioMCP 1.0.0 command biomcp get phenotype: Reads one HP term
  from HPO or Monarch, and optionally its synonyms, links, related diseases or
  genes."
goal: Look up the arguments, sections and examples of biomcp get phenotype.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get phenotype
kind: reference
verified: 2026-10-28
---

`biomcp get phenotype` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one HP term from HPO or Monarch, and optionally its synonyms, links, related diseases or genes. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

## Usage

```text
biomcp get phenotype [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 50 |  | Default five text/link rows or one complete association row. |
| `--section-offset` | integer | no | min 0 |  | Native section row offset. |
| `--snapshot` | string | no |  |  | Retained exact source reply for local text and link pages; unsupported for diseases and genes. |
| `--source` | string | no |  | `hpo`, `monarch` | HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch. |
| `id` (position 1) | string | yes |  |  | Exact HP identifier. |
| `section` (position 2) | string | no |  | `synonyms`, `xrefs`, `hierarchy`, `diseases`, `genes` | Absent means the source identity card. |

## Sections

Absent means the source identity card.

| Section | Description |
| --- | --- |
| `synonyms` | Native synonyms. |
| `xrefs` | Native cross references. |
| `hierarchy` | Native direct parent and child links. |
| `diseases` | Native disease association rows. |
| `genes` | Native human gene association rows. |

## Sources

`--source` picks the source to ask: [hpo](/sources/hpo/), [monarch](/sources/monarch-initiative/).

## Examples

The program's help gives these examples.

```sh
biomcp get phenotype HP:0000001
```

```sh
biomcp get phenotype HP:0000001 --source monarch
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.phenotype-card/1`](/reference/schemas/biomcp-phenotype-card-1/), whose page shows a real reply and its fields.

## Next

The [phenotype guide](/user-guide/phenotype/) shows every phenotype command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
