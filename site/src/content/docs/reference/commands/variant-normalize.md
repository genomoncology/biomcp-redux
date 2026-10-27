---
title: biomcp variant normalize
description: "Reference for the BioMCP 1.0.0 command biomcp variant normalize:
  Send unchanged reference-qualified notation to explicitly named services."
goal: Look up the arguments, sections and examples of biomcp variant normalize.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant normalize
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp variant normalize`.

Send unchanged reference-qualified notation to explicitly named services.

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

`--source` accepts these sources: [car](/reference/sources/car/), [mutalyzer](/reference/sources/mutalyzer/), [variantvalidator](/reference/sources/variantvalidator/).

## Examples

```sh
biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'
biomcp variant normalize --source variantvalidator --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
