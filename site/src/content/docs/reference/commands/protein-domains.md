---
title: biomcp protein domains
description: "Reference for the BioMCP 1.0.0 command biomcp protein domains:
  Page five native InterPro matches with original types and fragments."
goal: Look up the arguments, sections and examples of biomcp protein domains.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein domains
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp protein domains`.

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein domains [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | Retained collection identity. |
| `--type` | string | no |  |  | Exact native InterPro type, not a normalized domain label. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

```sh
biomcp protein domains P00533
biomcp protein domains P00533 --limit 1
```

## Reply schema

The help examples of this command return [`biomcp.protein_domains/2`](/reference/schemas/biomcp-protein-domains-2/). Print the full JSON schema with `biomcp schema biomcp.protein_domains/2`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [protein entity](/user-guide/protein/) page lists every protein command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
