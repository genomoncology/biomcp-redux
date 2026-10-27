---
title: biomcp get pgx
description: "Reference for the BioMCP 1.0.0 command biomcp get pgx: PGx card or section."
goal: Look up the arguments, sections and examples of biomcp get pgx.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get pgx
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get pgx`.

PGx card or section. No phenotype, phase or dose is calculated.

## Usage

```text
biomcp get pgx [OPTIONS] [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--annotation-id` | string | no |  |  | Checked ClinPGx accession, never a shared PaId. |
| `--diplotype` | value | no |  |  | Unchanged declared expression required for the diplotype section only. |
| `--drug-id` | value | no |  |  | Exact published CPIC drug key; recommendations only. |
| `--gene` | string | no |  |  | Explicit source gene symbol. |
| `--kind` | value | no |  |  | One ClinPGx collection; clinical is the default only on annotations. |
| `--section-limit` | integer | no | min 1, max 50 |  | Rows per page: three clinical, five elsewhere. |
| `--section-offset` | integer | no | min 0 |  | Offset within the selected section. |
| `--snapshot` | value | no |  |  | Retained collection for local continuation. |
| `--source` | string | yes |  | `cpic`, `clinpgx` | Required source, checked against the section. |
| `section` (position 1) | value | no |  |  | One named section; omitted means the CPIC gene card. |

## Sections

One named section; omitted means the CPIC gene card.

| Section | Description |
| --- | --- |
| `recommendations` | Complete native recommendation contexts. |
| `guidelines` | Native guideline references. |
| `diplotype` | Declared source diplotype lookup. |
| `annotations` | One ClinPGx annotation kind. |

## Sources

`--source` accepts these sources: [cpic](/reference/sources/cpic/), [clinpgx](/reference/sources/clinpgx/).

## Examples

```sh
biomcp get pgx --gene DPYD --source cpic
biomcp get pgx --gene DPYD --source cpic diplotype --diplotype '*1/*2A'
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
