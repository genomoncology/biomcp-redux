---
title: biomcp cell-line assay-count
description: "Reference for the BioMCP 1.0.0 command biomcp cell-line
  assay-count: A separate assay-count request, never implicit in metadata or
  all."
goal: Look up the arguments, sections and examples of biomcp cell-line assay-count.
reader: A BioMCP user or agent checking one command
sidebar:
  label: cell-line assay-count
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp cell-line assay-count`.

A separate assay-count request, never implicit in metadata or all. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp cell-line assay-count <id>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `id` (position 1) | string | yes |  |  | Exact ChEMBL cell identity returned by metadata. |

## Examples

```sh
biomcp cell-line assay-count CHEMBL3706573
biomcp cell-line assay-count CHEMBL3706573 --fresh
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
