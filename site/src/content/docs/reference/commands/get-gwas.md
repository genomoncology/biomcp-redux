---
title: biomcp get gwas
description: "The BioMCP 1.0.0 command biomcp get gwas: Reads one GWAS Catalog
  association or study, and optionally its loci, associations or ancestries."
goal: Look up the arguments, sections and examples of biomcp get gwas.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get gwas
kind: reference
verified: 2026-10-27
---

`biomcp get gwas` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one GWAS Catalog association or study, and optionally its loci, associations or ancestries. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections and examples.

The program's help describes it:

> Source association or study and native context. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

## Usage

```text
biomcp get gwas [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 50 |  | Maximum native relation rows. |
| `--section-offset` | integer | no | min 0 |  | Native relation row offset. |
| `--snapshot` | string | no |  |  | Retained ancestry reply for a local page. |
| `id` (position 1) | string | yes |  |  | Qualified association ID or exact GCST study accession. |
| `section` (position 2) | string | no |  | `loci`, `associations`, `ancestries` | Association loci or study associations and ancestries. |

## Sections

Association loci or study associations and ancestries.

| Section | Description |
| --- | --- |
| `loci` | Native association loci. |
| `associations` | Native study association pages. |
| `ancestries` | Native study ancestry rows. |

## Examples

The program's help gives these examples.

```sh
biomcp get gwas association:6114
```

```sh
biomcp get gwas GCST000035
```

## Next

The [gwas guide](/user-guide/gwas/) shows every gwas command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
