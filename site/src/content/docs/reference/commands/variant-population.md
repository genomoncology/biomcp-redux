---
title: biomcp variant population
description: "Reference for the BioMCP 1.0.0 command biomcp variant population:
  Read a bounded ancestry-detail page from the exact source-given GRCh38
  allele."
goal: Look up the arguments, sections and examples of biomcp variant population.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant population
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp variant population`.

Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

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

```sh
biomcp variant population CA123643 --detail
biomcp variant population CA123643 --detail --offset 10
```

## Reply schema

The help examples of this command return [`biomcp.variant_population/4`](/reference/schemas/biomcp-variant-population-4/). Print the full JSON schema with `biomcp schema biomcp.variant_population/4`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [variant entity](/user-guide/variant/) page lists every variant command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
