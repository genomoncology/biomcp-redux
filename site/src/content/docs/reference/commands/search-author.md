---
title: biomcp search author
description: "Reference for the BioMCP 1.0.0 command biomcp search author:
  Literal author name query and native window; positional text maps to query."
goal: Look up the arguments, sections and examples of biomcp search author.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search author
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp search author`.

Literal author name query and native window; positional text maps to query. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

## Usage

```text
biomcp search author [OPTIONS] <query>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Bounded source page size. |
| `--offset` | integer | no | min 0 |  | Native offset. |
| `--pool` | string | no |  |  | Retained page identity. |
| `--source` | string | no |  | `semantic-scholar`, `semanticscholar` | Native author source. |
| `query` (position 1) | string | yes |  |  | Literal query from the positional argument or --query. |

## Sources

`--source` accepts these sources: [semantic-scholar](/sources/semantic-scholar/).

## Examples

```sh
biomcp search author 'Louis Williams'
biomcp search author 'Louis Williams' --source semanticscholar
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
