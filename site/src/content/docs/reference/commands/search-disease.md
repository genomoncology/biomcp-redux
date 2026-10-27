---
title: biomcp search disease
description: "The BioMCP 1.0.0 command biomcp search disease: Finds disease
  concepts by text, or lists SEER cancer sites."
goal: Look up the arguments, sections and examples of biomcp search disease.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search disease
kind: reference
verified: 2026-10-27
---

`biomcp search disease` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds disease concepts by text, or lists SEER cancer sites. This page lists its arguments and examples.

The program's help describes it:

> Literal discovery text or an explicit SEER catalog page.

## Usage

```text
biomcp search disease [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum source rows on one page. |
| `--offset` | integer | no | min 0 |  | Native row offset. |
| `--ontology` | string | no |  | `mondo`, `doid`, `ncit` | Exactly one ontology, default MONDO. |
| `--source` | string | no |  | `ols`, `mydisease`, `monarch`, `seer` | One explicit source, default OLS. |
| `text` (position 1) | value | no |  |  | Literal source search text; absent for the SEER site catalog. |

## Sources

`--source` picks the source to ask: [ols](/sources/ols/), [mydisease](/sources/mydisease/), [monarch](/sources/monarch-initiative/), [seer](/sources/seer/).

## Examples

The program's help gives these examples.

```sh
biomcp search disease --source seer --limit 1
```

```sh
biomcp search disease --source seer --limit 1 --offset 1
```

## Next

The [disease guide](/user-guide/disease/) shows every disease command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
