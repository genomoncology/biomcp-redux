---
title: biomcp variant recurrence
description: "Reference for the BioMCP 1.0.0 command biomcp variant recurrence:
  Observe exact-allele samples within one caller-selected source page."
goal: Look up the arguments, sections and examples of biomcp variant recurrence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant recurrence
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp variant recurrence`.

Observe exact-allele samples within one caller-selected source page.

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

`--source` accepts these sources: [cbioportal](/sources/cbioportal/).

## Examples

```sh
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3 --page 0
```

## Reply schema

The help examples of this command return [`biomcp.variant_recurrence/4`](/reference/schemas/biomcp-variant-recurrence-4/). Print the full JSON schema with `biomcp schema biomcp.variant_recurrence/4`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [variant entity](/user-guide/variant/) page lists every variant command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
