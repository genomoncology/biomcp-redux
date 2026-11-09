---
title: biomcp get biomarker
description: "The BioMCP 1.0.0 command biomcp get biomarker: Get NCI biomarker
  concepts by code."
goal: Look up the arguments, sections and examples of biomcp get biomarker.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get biomarker
kind: reference
verified: 2026-11-09
---

`biomcp get biomarker` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It get NCI biomarker concepts by code. This page lists its arguments, examples and reply.

The program's help describes it:

> Get NCI biomarker concepts by code. Sources: nci.

## Usage

```text
biomcp get biomarker [OPTIONS] <id>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--source` | string | no |  | `nci` | NCI is the biomarker concept source; help get biomarker NCI is the biomarker concept source. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source nci. Command sources: nci; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact NCIt code, such as C96022. |

## Sources

`--source` picks the source to ask: [nci](/sources/nci/).

## Examples

The program's help gives these examples.

```sh
biomcp get biomarker C96022
```

```sh
biomcp get biomarker C96022 --source nci
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.biomarker-result/4`](/reference/schemas/biomcp-biomarker-result-4/), whose page lists its fields.

## Next

The [biomarker guide](/user-guide/biomarker/) shows every biomarker command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
