---
title: biomcp get cell-line
description: "Reference for the BioMCP 1.0.0 command biomcp get cell-line: One
  exact identity and an optional bounded named section."
goal: Look up the arguments, sections and examples of biomcp get cell-line.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get cell-line
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp get cell-line`.

One exact identity and an optional bounded named section. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp get cell-line [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--section-limit` | integer | no | min 1, max 25 |  | Explicit row limit for each requested collection; native variation defaults to three. |
| `--section-offset` | integer | no | min 0 |  | Offset within each independently requested collection. |
| `id` (position 1) | string | yes |  |  | One exact primary accession or qualified source ID. |
| `section` (position 2) | string | no |  | `names`, `identity`, `xrefs`, `variants`, `chembl`, `all` | Omit for the default card; all returns bounded first pages without an assay count. |

## Sections

Omit for the default card; all returns bounded first pages without an assay count.

| Section | Description |
| --- | --- |
| `names` | Native display names. |
| `identity` | Independent species, diseases and qualified warnings. |
| `xrefs` | Complete source cross-reference rows. |
| `variants` | Complete native variation rows. |
| `chembl` | Exact accession-linked ChEMBL cell metadata. |
| `all` | Bounded first page of every named section. |

## Examples

```sh
biomcp get cell-line CVCL_2119
biomcp get cell-line CVCL_0004
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
