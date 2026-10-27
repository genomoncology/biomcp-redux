---
title: biomcp search gene
description: "Reference for the BioMCP 1.0.0 command biomcp search gene: Search
  genes by text, cytoband, or their intersection."
goal: Look up the arguments, sections and examples of biomcp search gene.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search gene
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search gene`.

Search genes by text, cytoband, or their intersection.

## Usage

```text
biomcp search gene [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--band` | string | no |  |  | A cytoband prefix. |
| `--limit` | integer | no | min 1, max 50 |  | Number of results per page. |
| `--locus-group` | string | no |  | `protein_coding`, `non_coding_rna`, `pseudogene`, `phenotype`, `other` | Optional HGNC locus group intersected with the selector. |
| `--offset` | integer | no | min 0 |  | Number of results to skip. |
| `query` (position 1) | string | no |  |  | Words matched against symbols, previous symbols, aliases and names. |

## Examples

```sh
biomcp search gene KRAS
biomcp search gene p16
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
