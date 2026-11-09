---
title: biomcp variant recurrence
description: "The BioMCP 1.0.0 command biomcp variant recurrence: Counts the
  samples that carry one variant in one cBioPortal study."
goal: Look up the arguments, sections and examples of biomcp variant recurrence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant recurrence
kind: reference
verified: 2026-11-09
---

`biomcp variant recurrence` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It counts the samples that carry one variant in one cBioPortal study. This page lists its arguments, examples and reply.

The program's help describes it:

> Get genetic variant observations in a cohort. Sources: cbioportal.

## Usage

```text
biomcp variant recurrence [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Identity lookup assembly; omitted uses the shared; help variant recurrence Identity lookup assembly; omitted uses the shared GRCh38 default. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --build GRCh37. Command sources: cbioportal; source restrictions in the field description apply. |
| `--gene` | string | yes |  |  | HGNC symbol resolved to the source's Entrez gene identifier. Value form: string. Example: --gene BRAF. Command sources: cbioportal; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 5000 |  | Source mutation rows per page, including excluded alleles. Value form: integer. Example: --limit 3. Command sources: cbioportal; source restrictions in the field description apply. |
| `--page` | integer | no | min 0 |  | Zero-based source page. Value form: integer. Example: --page 0. Command sources: cbioportal; source restrictions in the field description apply. |
| `--profile` | string | yes |  |  | Explicit mutation molecular profile. Value form: string. Example: --profile msk_impact_2017_mutations. Command sources: cbioportal; source restrictions in the field description apply. |
| `--sample-set` | string | yes |  |  | Explicit sample list; selected size is never an allele denominator. Value form: string. Example: --sample-set msk_impact_2017_all. Command sources: cbioportal; source restrictions in the field description apply. |
| `--source` | string | yes |  | `cbioportal` | Explicit cohort source; help variant recurrence Explicit cohort source. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source cbioportal. Command sources: cbioportal; source restrictions in the field description apply. |
| `--study` | string | yes |  |  | Explicit study; there is no default cohort. Value form: string. Example: --study msk_impact_2017. Command sources: cbioportal; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | Variant identity resolving to one unique source-qualified genomic replacement with nonempty reference and alternate alleles (pure insertions and deletions are unavailable). |

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

```sh
biomcp variant recurrence rs113488022 --source cbioportal --gene BRAF \
  --study msk_impact_2017 --profile msk_impact_2017_mutations \
  --sample-set msk_impact_2017_all --limit 3 --build GRCh37
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant-recurrence/11`](/reference/schemas/biomcp-variant-recurrence-11/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
