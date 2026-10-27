---
title: biomcp search disease
description: "Reference for the BioMCP 1.0.0 command biomcp search disease:
  Literal discovery text or an explicit SEER catalog page."
goal: Look up the arguments, sections and examples of biomcp search disease.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search disease
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search disease`.

Literal discovery text or an explicit SEER catalog page.

## Usage

```text
biomcp search disease [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum source rows on one page. |
| `--offset` | integer | no | min 0 |  | Native row offset. |
| `--ontology` | string | no |  | `mondo`, `doid`, `ncit` | Exactly one ontology, default MONDO. |
| `--source` | string | no |  | `ols`, `mydisease`, `monarch`, `seer` | One explicit source, default OLS. |
| `text` (position 1) | value | no |  |  | Literal source search text; absent for the SEER site catalog. |

## Sources

`--source` accepts these sources: [ols](/reference/sources/ols/), [mydisease](/reference/sources/mydisease/), [monarch](/reference/sources/monarch/), [seer](/reference/sources/seer/).

## Examples

```sh
biomcp search disease --source seer --limit 1
biomcp search disease --source seer --limit 1 --offset 1
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
