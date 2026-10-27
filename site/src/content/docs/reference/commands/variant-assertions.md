---
title: biomcp variant assertions
description: "Reference for the BioMCP 1.0.0 command biomcp variant assertions:
  Page assertions from one explicitly selected evidence source."
goal: Look up the arguments, sections and examples of biomcp variant assertions.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant assertions
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp variant assertions`.

Page assertions from one explicitly selected evidence source.

## Usage

```text
biomcp variant assertions [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--assertion` | value | no |  |  | Native assertion accession or document ID. |
| `--limit` | integer | no | min 1, max 50 |  | Assertions per page; omitted uses the native source default. |
| `--offset` | integer | no | min 0 |  | Assertions to skip in source order. |
| `--source` | string | yes |  | `clinvar`, `erepo` | One native source; no fallback or text search. |
| `--version` | value | no |  |  | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. |
| `input` (position 1) | string | yes |  |  | Checked Variation ID for ClinVar, or CAid for ERepo. |

## Sources

`--source` accepts these sources: [clinvar](/reference/sources/clinvar/), [erepo](/reference/sources/erepo/).

## Examples

```sh
biomcp variant assertions clinvar:13961 --source clinvar
biomcp variant assertions CA000434 --source erepo
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
