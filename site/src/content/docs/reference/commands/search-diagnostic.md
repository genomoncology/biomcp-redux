---
title: biomcp search diagnostic
description: "Reference for the BioMCP 1.0.0 command biomcp search diagnostic:
  Search one diagnostic source."
goal: Look up the arguments, sections and examples of biomcp search diagnostic.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search diagnostic
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search diagnostic`.

Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

## Usage

```text
biomcp search diagnostic [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--biomarker` | value | no |  |  | Exact table-0 biomarker cell; unsupported in the group table. |
| `--bundle-revision` | value | no |  |  | Immutable stored bundle revision retained by a continuation. |
| `--condition-id` | value | no |  |  | Exact MedGen:CUI identifier only. |
| `--device-id` | value | no |  |  | Exact PMA application, application/supplement or native K identity. |
| `--drug-application` | value | no |  |  | Cell-owned typed NDA/BLA application. |
| `--gene` | value | no |  |  | Exact source gene_symbol field, without alias expansion. |
| `--gene-id` | value | no |  |  | Exact source GeneID, exclusive with gene symbol. |
| `--limit` | integer | no | min 1, max 50 |  | Five results by default; at most fifty source rows or product headers. |
| `--manufacturer` | value | no |  |  | Literal whole-word phrase in WHO's Manufacturer name cell. |
| `--marker` | value | no |  |  | Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. |
| `--offset` | integer | no | min 0 |  | Zero-based source result offset. |
| `--source` | string | yes |  | `gtr`, `fda-cdx`, `who-ivd` | Explicit native GTR, WHO IVD or FDA companion discovery. |
| `--type` | value | no |  |  | Exact native test_type value. |

## Sources

`--source` accepts these sources: [gtr](/sources/gtr/), [fda-cdx](/sources/fda-cdx/), [who-ivd](/sources/who-ivd/).

## Examples

```sh
biomcp search diagnostic --source gtr --gene BRCA1
biomcp search diagnostic --source gtr --condition-id MedGen:C2676676
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
