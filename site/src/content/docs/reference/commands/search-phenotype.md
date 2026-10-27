---
title: biomcp search phenotype
description: "Reference for the BioMCP 1.0.0 command biomcp search phenotype:
  Search literal human HP term candidates through Monarch with bounded native
  pages."
goal: Look up the arguments, sections and examples of biomcp search phenotype.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search phenotype
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search phenotype`.

Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

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

`--source` accepts these sources: [hpo](/sources/hpo/), [monarch](/sources/monarch-initiative/).

## Examples

```sh
biomcp search phenotype synthetic
biomcp search phenotype synthetic --limit 1
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
