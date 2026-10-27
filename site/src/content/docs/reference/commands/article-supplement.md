---
title: biomcp article supplement
description: "Reference for the BioMCP 1.0.0 command biomcp article supplement:
  Read a checked supplement file or page its original rows."
goal: Look up the arguments, sections and examples of biomcp article supplement.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article supplement
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp article supplement`.

Read a checked supplement file or page its original rows.

## Usage

```text
biomcp article supplement [OPTIONS] <input> <file>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--find` | string | no |  |  | Literal case-insensitive needle. |
| `--offset` | integer | no | min 0 |  | Matching-row or text-fragment offset. |
| `--replace` | boolean | no |  |  | Replace an existing destination only when explicitly requested with save. |
| `--rows` | string | no |  |  | Inclusive original row range. |
| `--save` | string | no |  |  | Save the whole original document to an explicit file or existing folder. |
| `--sheet` | string | no |  |  | Sheet name for row selection. |
| `--variant` | string | no |  |  | Explicit protein substitution. |
| `input` (position 1) | string | yes |  |  | PubMed or PMC identity. |
| `file` (position 2) | string | yes |  |  | Exact file name from the supplements section. |

## Examples

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
