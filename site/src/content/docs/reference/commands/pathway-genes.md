---
title: biomcp pathway genes
description: "Reference for the BioMCP 1.0.0 command biomcp pathway genes: Read
  Reactome exact references or WikiPathways GeneProduct data nodes."
goal: Look up the arguments, sections and examples of biomcp pathway genes.
reader: A BioMCP user or agent checking one command
sidebar:
  label: pathway genes
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp pathway genes`.

Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp pathway genes [OPTIONS] <id> [details]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Five complete relation rows by default, at most fifty. |
| `--offset` | integer | no | min 0 |  | Offset within the retained selected relation. |
| `--pool` | string | no |  |  | Immutable relation snapshot. |
| `id` (position 1) | string | yes |  |  | Checked source-owned human pathway identity. |
| `details` (position 2) | string | no |  | `details` | Include every physical entity context for these references; never clips source text. |

## Examples

```sh
biomcp pathway genes R-HSA-5673001
biomcp pathway genes R-HSA-5673001 details
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
