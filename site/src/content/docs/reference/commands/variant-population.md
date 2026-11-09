---
title: biomcp variant population
description: "The BioMCP 1.0.0 command biomcp variant population: Lists gnomAD
  population frequency details for one variant."
goal: Look up the arguments, sections and examples of biomcp variant population.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant population
kind: reference
verified: 2026-11-09
---

`biomcp variant population` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists gnomAD population frequency details for one variant. This page lists its arguments, examples and reply.

The program's help describes it:

> Get genetic variant population frequencies. Sources: car, gnomad, mutalyzer, myvariant, variantvalidator.

## Usage

```text
biomcp variant population [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Explicit source assembly when required by identity; help variant population Explicit source assembly when required by identity input. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --build GRCh38. Command sources: car, gnomad, mutalyzer, myvariant, variantvalidator; source restrictions in the field description apply. |
| `--detail` | boolean | no |  |  | Read source ancestry groups, preserving overlapping groups individually. Value form: boolean. Example: --detail. Command sources: car, gnomad, mutalyzer, myvariant, variantvalidator; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum rows per page. Value form: integer. Example: --limit 3. Command sources: car, gnomad, mutalyzer, myvariant, variantvalidator; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source rows to skip. Value form: integer. Example: --offset 10. Command sources: car, gnomad, mutalyzer, myvariant, variantvalidator; source restrictions in the field description apply. |
| `--resolver` | string | no |  | `car`, `mutalyzer`, `variantvalidator` | Resolver for reference-qualified input; help variant population Resolver for reference-qualified input. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --resolver variantvalidator. Command sources: car, gnomad, mutalyzer, myvariant, variantvalidator; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | Exact published allele identity; ambiguous candidates must first be selected. |

## Examples

The program's help gives these examples.

```sh
biomcp variant population CA123643 --detail
```

```sh
biomcp variant population CA123643 --detail --offset 10
```

```sh
biomcp variant population 1-11796321-G-A --detail --limit 3
```

```sh
biomcp variant population 7-140753336-A-T --resolver variantvalidator
```

```sh
biomcp variant population CA123643 --resolver variantvalidator --build GRCh38
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant-population/13`](/reference/schemas/biomcp-variant-population-13/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
