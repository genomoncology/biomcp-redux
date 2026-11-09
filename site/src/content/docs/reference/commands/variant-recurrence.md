---
title: biomcp variant recurrence
description: "The BioMCP 1.0.0 command biomcp variant recurrence: Counts the
  samples that carry one variant in one cBioPortal study."
goal: Look up the arguments, sections and examples of biomcp variant recurrence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant recurrence
kind: reference
verified: 2026-10-28
---

`biomcp variant recurrence` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It counts the samples that carry one variant in one cBioPortal study. This page lists its arguments, examples and reply.

The program's help describes it:

> Observe exact-allele samples within one caller-selected source page.

## Usage

```text
biomcp variant recurrence [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--gene` | string | yes |  |  | HGNC symbol resolved to the source's Entrez gene identifier. |
| `--limit` | integer | no | min 1, max 100 |  | Source mutation rows per page, including excluded alleles. |
| `--page` | integer | no | min 0 |  | Zero-based native source page. |
| `--profile` | string | yes |  |  | Explicit mutation molecular profile. |
| `--sample-set` | string | yes |  |  | Explicit sample list; selected size is never an allele denominator. |
| `--source` | string | yes |  | `cbioportal` | Explicit cohort source. |
| `--study` | string | yes |  |  | Explicit study; there is no default cohort. |
| `input` (position 1) | string | yes |  |  | Published variant identity; ambiguous identities are never selected silently. |

## Sources

`--source` picks the source to ask: [cbioportal](/sources/cbioportal/).

## Examples

The program's help gives these examples.

```sh
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal \
  --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations \
  --sample-set msk_impact_2017_all --limit 3
```

```sh
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal \
  --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations \
  --sample-set msk_impact_2017_all --limit 3 --page 0
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant_recurrence/4`](/reference/schemas/biomcp-variant-recurrence-4/), whose page shows a real reply and its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
