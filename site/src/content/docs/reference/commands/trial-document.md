---
title: biomcp trial document
description: "The BioMCP 1.0.0 command biomcp trial document: Downloads one
  document a trial posted on ClinicalTrials.gov, such as its protocol PDF, to a
  file you name."
goal: Look up the arguments, sections and examples of biomcp trial document.
reader: A BioMCP user or agent checking one command
sidebar:
  label: trial document
kind: reference
verified: 2026-10-28
---

`biomcp trial document` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads one document a trial posted on ClinicalTrials.gov, such as its protocol PDF, to a file you name. This page lists its arguments, examples and reply.

The program's help describes it:

> Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

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

The program's help gives these examples.

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
```

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save analysis.pdf --replace
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.trial_document/2`](/reference/schemas/biomcp-trial-document-2/), whose page shows a real reply and its fields.

## Next

The [trial guide](/user-guide/trial/) shows every trial command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
