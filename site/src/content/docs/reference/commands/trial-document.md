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

## Reply schema

The help examples of this command return [`biomcp.trial_document/2`](/reference/schemas/biomcp-trial-document-2/). Print the full JSON schema with `biomcp schema biomcp.trial_document/2`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [trial entity](/user-guide/trial/) page lists every trial command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
