---
title: biomcp protein features
description: "Reference for the BioMCP 1.0.0 command biomcp protein features:
  Page five native UniProt features by exact type and displayed residue."
goal: Look up the arguments, sections and examples of biomcp protein features.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein features
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp protein features`.

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein features [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | Retained collection identity. |
| `--position` | integer | no | min 1, max 4294967295 |  | Native one-based residue; zero fails during deserialization. |
| `--type` | string | no |  |  | Exact UniProt native label, without an alias map. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

```sh
biomcp protein features P00533
biomcp protein features P00533 --limit 1
```

## Reply schema

The help examples of this command return [`biomcp.protein_features/1`](/reference/schemas/biomcp-protein-features-1/). Print the full JSON schema with `biomcp schema biomcp.protein_features/1`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [protein entity](/user-guide/protein/) page lists every protein command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
