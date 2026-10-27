---
title: biomcp get gene
description: "Reference for the BioMCP 1.0.0 command biomcp get gene: Get one to
  twenty human gene inputs in their original order."
goal: Look up the arguments, sections and examples of biomcp get gene.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get gene
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get gene`.

Get one to twenty human gene inputs in their original order.

## Usage

```text
biomcp get gene [OPTIONS] <input...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section` | array | no |  | `protein`, `civic`, `clingen`, `constraint`, `expression`, `pathways`, `oncokb`, `all` | Requested section words, preserving caller order and repetitions. Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs. |
| `input` (position 1) | array | yes | min 1, max 20 |  | Gene symbols or checked identifiers, in input order. |

## Sections

Requested section words, preserving caller order and repetitions.
Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs.

| Section | Description |
| --- | --- |
| `protein` | Reviewed human proteins. |
| `civic` | CIViC gene curation. |
| `clingen` | ClinGen validity and dosage. |
| `constraint` | Population constraint metrics. |
| `expression` | Human Protein Atlas expression. |
| `pathways` | Human Reactome pathways. |
| `oncokb` | Keyed OncoKB annotations. |
| `all` | Every landed section. |

## Examples

```sh
biomcp get gene KRAS
biomcp get gene BRAF
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
