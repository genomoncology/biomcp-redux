---
title: biomcp search study
description: "Reference for the BioMCP 1.0.0 command biomcp search study:
  Literal stored metadata search; text is never parsed as a query language."
goal: Look up the arguments, sections and examples of biomcp search study.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search study
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search study`.

Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

## Usage

```text
biomcp search study [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--cancer-type` | string | no |  |  | Exact cBioPortal cancer-type code; no OncoTree conversion. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum headers per page. |
| `--offset` | integer | no | min 0 |  | Zero-based offset in the checked snapshot. |
| `--pool` | value | no |  |  | Opaque continuation retaining the catalog revision and selectors. |
| `text` (position 1) | string | no |  |  | Literal study ID, name or description text, without synonyms or typo repair. |

## Examples

```sh
biomcp search study Atlas
biomcp search study --cancer-type mixed --limit 1
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
