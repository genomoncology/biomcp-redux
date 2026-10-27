---
title: biomcp protein sequence
description: "The BioMCP 1.0.0 command biomcp protein sequence: Prints the whole
  amino-acid sequence of one UniProt protein or isoform."
goal: Look up the arguments, sections and examples of biomcp protein sequence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein sequence
kind: reference
verified: 2026-10-27
---

`biomcp protein sequence` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It prints the whole amino-acid sequence of one UniProt protein or isoform. This page lists its arguments and examples.

The program's help describes it:

> Read one complete native sequence for an exact source accession.
> No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

## Usage

```text
biomcp protein sequence <isoform>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `isoform` (position 1) | string | yes |  |  | Exact isoform or base accession; no canonical suffix is synthesized. |

## Examples

The program's help gives these examples.

```sh
biomcp protein sequence P00533
```

```sh
biomcp protein sequence P00533 --fresh
```

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
