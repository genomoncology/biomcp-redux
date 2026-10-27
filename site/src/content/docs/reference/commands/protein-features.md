---
title: biomcp protein features
description: "The BioMCP 1.0.0 command biomcp protein features: Lists UniProt
  features of one protein, five at a time, filtered by type or residue."
goal: Look up the arguments, sections and examples of biomcp protein features.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein features
kind: reference
verified: 2026-10-27
---

`biomcp protein features` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists UniProt features of one protein, five at a time, filtered by type or residue. This page lists its arguments and examples.

The program's help describes it:

> Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein features [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. |
| `--offset` | integer | no | min 0 |  | Native or retained page offset. |
| `--pool` | string | no |  |  | The ID of a stored list, from an earlier reply, so later pages read the same data. |
| `--position` | integer | no | min 1, max 4294967295 |  | Native one-based residue; zero fails during deserialization. |
| `--type` | string | no |  |  | Exact UniProt native label, without an alias map. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein features P00533
```

```sh
biomcp protein features P00533 --limit 1
```

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
