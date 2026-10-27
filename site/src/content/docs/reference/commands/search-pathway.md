---
title: biomcp search pathway
description: "Reference for the BioMCP 1.0.0 command biomcp search pathway:
  Search Reactome (default) or WikiPathways human pathways."
goal: Look up the arguments, sections and examples of biomcp search pathway.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search pathway
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search pathway`.

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

## Usage

```text
biomcp search pathway [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fresh` | boolean | no |  |  | Request a new retained observation. |
| `--gene` | string | no |  |  | Human gene input for exact resolution and UniProt mapping; Reactome only. |
| `--limit` | integer | no | min 1, max 50 |  | Five rows by default, one to fifty. Reactome rows equals this limit without refill. |
| `--offset` | integer | no | min 0 |  | Native Reactome Start row or retained WikiPathways catalog offset. |
| `--pool` | string | no |  |  | Retained snapshot identity for deterministic continuation. |
| `--source` | array | no | min 1, max 2 | `reactome`, `wikipathways` | Sources in caller order; repeated sources take turns without title joins. |
| `--top-level` | boolean | no |  |  | Native human Reactome top-level discovery, exclusive with text and gene. |
| `query` (position 1) | string | no |  |  | Literal source query; exclusive with gene and top-level discovery. |

## Sources

`--source` accepts these sources: [reactome](/reference/sources/reactome/), [wikipathways](/reference/sources/wikipathways/).

## Examples

```sh
biomcp search pathway 'Signaling by EGFR'
biomcp search pathway Hemostasis
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
