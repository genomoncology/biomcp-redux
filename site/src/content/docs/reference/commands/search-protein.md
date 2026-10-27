---
title: biomcp search protein
description: "Reference for the BioMCP 1.0.0 command biomcp search protein:
  Search five reviewed human UniProt headers by literal text or exact gene."
goal: Look up the arguments, sections and examples of biomcp search protein.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search protein
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search protein`.

Search five reviewed human UniProt headers by literal text or exact gene.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

## Usage

```text
biomcp search protein [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--gene` | string | no |  |  | Exact native gene field, without identifier inference. |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default; no more than 25. |
| `--offset` | integer | no | min 0 |  | Source/retained result offset. |
| `--pool` | string | no |  |  | Exact retained collection identity. |
| `--reviewed` | boolean | no |  |  | Reviewed human scope stays enabled; unreviewed search is deferred. |
| `query` (position 1) | string | no |  |  | Literal text, exclusive with gene. |

## Examples

```sh
biomcp search protein --gene BRAF
biomcp search protein EGFR
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
