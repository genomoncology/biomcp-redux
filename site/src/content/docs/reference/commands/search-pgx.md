---
title: biomcp search pgx
description: "Reference for the BioMCP 1.0.0 command biomcp search pgx: Checked
  gene-keyed CPIC pair search."
goal: Look up the arguments, sections and examples of biomcp search pgx.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search pgx
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search pgx`.

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

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

`--source` accepts these sources: [cpic](/reference/sources/cpic/).

## Examples

```sh
biomcp search pgx --gene DPYD --source cpic
biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
