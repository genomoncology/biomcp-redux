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
verified: 2026-10-27
---

`biomcp variant normalize` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It sends an HGVS variant name to the ClinGen Allele Registry, Mutalyzer or VariantValidator to check and normalize it. This page lists its arguments, examples and reply.

The program's help describes it:

> Send unchanged reference-qualified notation to explicitly named services.

## Usage

```text
biomcp variant normalize [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | value | no |  |  | Caller assembly; required when VariantValidator is selected. |
| `--hgvs` | string | yes |  |  | Unchanged reference-qualified HGVS; no local molecular parse. |
| `--limit` | integer | no | min 1, max 50 |  | Source mapping rows per service, with typed continuation for remaining rows. |
| `--offset` | integer | no | min 0 |  | Source mapping rows to skip per service. |
| `--source` | string | yes |  | `car`, `mutalyzer`, `variantvalidator`, `all` | Explicit service; all preserves independent results without consensus. |

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

The reply is one JSON object. Its schema is [`biomcp.variant_normalization/3`](/reference/schemas/biomcp-variant-normalization-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [variant guide](/user-guide/variant/) shows every variant command at work.
