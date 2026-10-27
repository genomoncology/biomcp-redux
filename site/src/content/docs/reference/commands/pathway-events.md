---
title: biomcp pathway events
description: "Reference for the BioMCP 1.0.0 command biomcp pathway events: Read
  Reactome event objects and unresolved native integer references in source
  order."
goal: Look up the arguments, sections and examples of biomcp pathway events.
reader: A BioMCP user or agent checking one command
sidebar:
  label: pathway events
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp pathway events`.

Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp pathway events [OPTIONS] <id>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Five complete relation rows by default, at most fifty. |
| `--offset` | integer | no | min 0 |  | Offset within the retained selected relation. |
| `--pool` | string | no |  |  | Immutable relation snapshot. |
| `id` (position 1) | string | yes |  |  | Checked source-owned human pathway identity. |

## Examples

```sh
biomcp pathway events R-HSA-5673001
biomcp pathway events R-HSA-5673001 --limit 50
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
