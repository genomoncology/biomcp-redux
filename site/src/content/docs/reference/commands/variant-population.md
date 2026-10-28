---
title: biomcp variant population
description: "The BioMCP 1.0.0 command biomcp variant population: Lists gnomAD
  population frequency details for one variant."
goal: Look up the arguments, sections and examples of biomcp variant population.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant population
kind: reference
verified: 2026-10-28
---

`biomcp variant population` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists gnomAD population frequency details for one variant. This page lists its arguments, examples and reply.

The program's help describes it:

> Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

## Usage

```text
biomcp variant population [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | value | no |  |  | Explicit source assembly when required by identity input. |
| `--detail` | boolean | no |  |  | Read source ancestry groups, preserving overlapping groups individually. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum rows per page. |
| `--offset` | integer | no | min 0 |  | Native rows to skip. |
| `--resolver` | string | no |  | `car`, `mutalyzer`, `variantvalidator` | Resolver for reference-qualified input. |
| `input` (position 1) | string | yes |  |  | Exact published allele identity; ambiguous candidates must first be selected. |

## Examples

The program's help gives these examples.

```sh
biomcp variant population CA123643 --detail
```

```sh
biomcp variant population CA123643 --detail --offset 10
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant_population/4`](/reference/schemas/biomcp-variant-population-4/), whose page shows a real reply and its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
