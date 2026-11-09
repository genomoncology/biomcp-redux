---
title: biomcp search study
description: "The BioMCP 1.0.0 command biomcp search study: Finds cBioPortal
  studies by text in a downloaded copy of the study list."
goal: Look up the arguments, sections and examples of biomcp search study.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search study
kind: reference
verified: 2026-11-09
---

`biomcp search study` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds cBioPortal studies by text in a downloaded copy of the study list. This page lists its arguments, examples and reply.

The program's help describes it:

> Search cancer genomics studies in cBioPortal. Sources: cbioportal.

## Usage

```text
biomcp search study [OPTIONS] [text]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--cancer-type` | string | no |  |  | Exact cBioPortal cancer-type code; no OncoTree conversion. Value form: string. Example: --cancer-type mixed. Command sources: cbioportal; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Maximum headers per page. Value form: integer. Example: --limit 1. Command sources: cbioportal; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Zero-based offset in the checked snapshot. Value form: integer. Example: --offset 0. Command sources: cbioportal; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Opaque continuation retaining the catalog revision and selectors. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: cbioportal; source restrictions in the field description apply. |
| `text` (position 1) | string | no |  |  | Exact study ID, name or description text, without synonyms or typo repair. |

## Examples

The program's help gives these examples.

```sh
biomcp search study Atlas
```

```sh
biomcp search study --cancer-type mixed --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.study-page/5`](/reference/schemas/biomcp-study-page-5/), whose page lists its fields.

## Next

The [study guide](/user-guide/study/) shows every study command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
