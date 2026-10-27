---
title: biomcp protein sequence
description: "Reference for the BioMCP 1.0.0 command biomcp protein sequence:
  Read one complete native sequence for an exact source accession."
goal: Look up the arguments, sections and examples of biomcp protein sequence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein sequence
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp protein sequence`.

Read one complete native sequence for an exact source accession.
No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein sequence <isoform>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `isoform` (position 1) | string | yes |  |  | Exact isoform or base accession; no canonical suffix is synthesized. |

## Examples

```sh
biomcp protein sequence P00533
biomcp protein sequence P00533 --fresh
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
