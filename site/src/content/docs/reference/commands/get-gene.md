---
title: biomcp get gene
description: "The BioMCP 1.0.0 command biomcp get gene: Reads one to twenty
  human genes, in the order you give, and optionally sections such as constraint
  or expression."
goal: Look up the arguments, sections and examples of biomcp get gene.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get gene
kind: reference
verified: 2026-10-27
---

`biomcp get gene` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one to twenty human genes, in the order you give, and optionally sections such as constraint or expression. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get one to twenty human gene inputs in their original order.

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

The program's help gives these examples.

```sh
biomcp get gene KRAS
```

```sh
biomcp get gene BRAF
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.gene_cards/4`](/reference/schemas/biomcp-gene-cards-4/), whose page shows a real reply and its fields.

## Next

The [gene guide](/user-guide/gene/) shows every gene command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
