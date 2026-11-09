---
title: biomcp search biomarker
description: "The BioMCP 1.0.0 command biomcp search biomarker: Search NCI
  biomarker concepts by name or synonym."
goal: Look up the arguments, sections and examples of biomcp search biomarker.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search biomarker
kind: reference
verified: 2026-11-09
---

`biomcp search biomarker` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It search NCI biomarker concepts by name or synonym. This page lists its arguments, examples and reply.

The program's help describes it:

> Search NCI biomarker concepts by name or synonym. Sources: nci.

## Usage

```text
biomcp search biomarker [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum headers per local page. Value form: integer. Example: --limit 5. Command sources: nci; source restrictions in the field description apply. |
| `--name` | string | no |  |  | Source partial name or synonym search. Value form: string. Example: --name PD-L1. Command sources: nci; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Offset within the complete retained observation. Value form: integer. Example: --offset 0. Command sources: nci; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Snapshot identity retained by continuation commands. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: nci; source restrictions in the field description apply. |
| `--source` | string | yes |  | `nci` | Explicit keyed NCI source; help search biomarker Explicit keyed NCI source. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source nci. Command sources: nci; source restrictions in the field description apply. |

## Sources

`--source` picks the source to ask: [nci](/sources/nci/).

## Examples

The program's help gives this example.

```sh
biomcp search biomarker --source nci --name PD-L1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.biomarker-page/4`](/reference/schemas/biomcp-biomarker-page-4/), whose page lists its fields.

## Next

The [biomarker guide](/user-guide/biomarker/) shows every biomarker command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
