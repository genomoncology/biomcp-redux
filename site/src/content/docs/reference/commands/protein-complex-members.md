---
title: biomcp protein complex-members
description: "Reference for the BioMCP 1.0.0 command biomcp protein
  complex-members: Page five native participants of an exact Complex Portal
  complex containing the anchor."
goal: Look up the arguments, sections and examples of biomcp protein complex-members.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein complex-members
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp protein complex-members`.

Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein complex-members [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--complex` | string | yes |  |  | Exact complex whose native participants contain the anchor. |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | Retained collection identity. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

```sh
biomcp protein complex-members P00533 --complex CPX-715
biomcp protein complex-members P00533 --complex CPX-715 --limit 1
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
