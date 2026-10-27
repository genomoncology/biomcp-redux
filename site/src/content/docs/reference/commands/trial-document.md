---
title: biomcp trial document
description: "Reference for the BioMCP 1.0.0 command biomcp trial document:
  Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path;
  never read its text."
goal: Look up the arguments, sections and examples of biomcp trial document.
reader: A BioMCP user or agent checking one command
sidebar:
  label: trial document
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp trial document`.

Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

## Usage

```text
biomcp trial document [OPTIONS] <input> <file>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--replace` | boolean | no |  |  | Replace an existing file only when explicitly requested. |
| `--save` | string | yes |  |  | Caller-owned file or existing folder. |
| `input` (position 1) | string | yes |  |  | Exact registry identity. |
| `file` (position 2) | string | yes |  |  | Exact filename from the registry manifest; never a URL or inferred path. |

## Examples

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
biomcp trial document NCT04303780 SAP_001.pdf --save analysis.pdf --replace
```

## Exit codes

See [exit codes](/reference/exit-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
