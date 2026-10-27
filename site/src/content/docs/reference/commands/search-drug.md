---
title: biomcp search drug
description: "Reference for the BioMCP 1.0.0 command biomcp search drug: Literal
  single-source candidate discovery, without an implicit identity selection."
goal: Look up the arguments, sections and examples of biomcp search drug.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search drug
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search drug`.

Literal single-source candidate discovery, without an implicit identity selection.

## Usage

```text
biomcp search drug [OPTIONS] <query>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Native candidate rows per page. |
| `--name-selector` | string | no |  | `preferred`, `synonym` | Explicit ChEMBL native name selector; omitted means preferred name. |
| `--offset` | integer | no | min 0 |  | Native rows to skip. |
| `--source` | string | no |  | `who-pq`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter` | Exactly one source; default MyChem never fans out. |
| `query` (position 1) | string | yes |  |  | Literal source name selector, unchanged by the consumer. |

## Sources

`--source` accepts these sources: [who-pq](/sources/who-prequalification/), [mychem](/sources/mychem/), [chembl](/sources/chembl/), [opentargets](/sources/opentargets/), [civic](/sources/civic/), [openfda-label](/sources/openfda/), [drugsfda](/sources/drugsfda/), [ema](/sources/ema/), [ddinter](/sources/ddinter/).

## Examples

```sh
biomcp search drug Tibsovo --source ema --limit 1
biomcp search drug Tibsovo --source ema --limit 1 --offset 1 --fresh
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
