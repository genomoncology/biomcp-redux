---
title: biomcp get phenotype
description: "Reference for the BioMCP 1.0.0 command biomcp get phenotype: Read
  an exact HP term from HPO or Monarch, or page native source sections."
goal: Look up the arguments, sections and examples of biomcp get phenotype.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get phenotype
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get phenotype`.

Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

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

`--source` accepts these sources: [hpo](/reference/sources/hpo/), [monarch](/reference/sources/monarch/).

## Examples

```sh
biomcp get phenotype HP:0000001
biomcp get phenotype HP:0000001 --source monarch
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
