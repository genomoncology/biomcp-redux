---
title: biomcp variant normalize
description: "The BioMCP 1.0.0 command biomcp variant normalize: Sends an HGVS
  variant name to the ClinGen Allele Registry, Mutalyzer or VariantValidator to
  check and normalize it."
goal: Look up the arguments, sections and examples of biomcp variant normalize.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant normalize
kind: reference
verified: 2026-11-09
---

`biomcp variant normalize` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It sends an HGVS variant name to the ClinGen Allele Registry, Mutalyzer or VariantValidator to check and normalize it. This page lists its arguments, examples and reply.

The program's help describes it:

> Normalize reference-qualified genetic variant HGVS. Sources: car, mutalyzer, variantvalidator.

## Usage

```text
biomcp variant normalize [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Caller assembly; required when VariantValidator is; help variant normalize Caller assembly; required when VariantValidator is selected. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --build GRCh38. Command sources: car, mutalyzer, variantvalidator; source restrictions in the field description apply. |
| `--hgvs` | string | yes |  |  | Reference-qualified HGVS, or chromosome genomic HGVS with an explicit build for CAR. Checked for reference and syntax only; no full HGVS parser. Value form: string. Example: --hgvs 'NM_004333.4:c.1799T&gt;A'. Command sources: car, mutalyzer, variantvalidator; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Source mapping rows per service, with typed continuation for remaining rows. Value form: integer. Example: --limit 10. Command sources: car, mutalyzer, variantvalidator; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source mapping rows to skip per service. Value form: integer. Example: --offset 0. Command sources: car, mutalyzer, variantvalidator; source restrictions in the field description apply. |
| `--source` | string | yes |  | `car`, `mutalyzer`, `variantvalidator`, `all` | Explicit service; all preserves independent results; help variant normalize Explicit service; all preserves independent results without consensus. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source mutalyzer. Command sources: car, mutalyzer, variantvalidator; source restrictions in the field description apply. |

## Sources

`--source` picks the source to ask: [car](/sources/clingen-allele-registry/), [mutalyzer](/sources/mutalyzer/), [variantvalidator](/sources/variantvalidator/).

## Examples

The program's help gives these examples.

```sh
biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'
```

```sh
biomcp variant normalize --source variantvalidator \
  --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant-normalization/8`](/reference/schemas/biomcp-variant-normalization-8/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
