---
title: biomcp protein isoforms
description: "The BioMCP 1.0.0 command biomcp protein isoforms: Lists the
  isoforms of one protein, five at a time."
goal: Look up the arguments, sections and examples of biomcp protein isoforms.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein isoforms
kind: reference
verified: 2026-11-09
---

`biomcp protein isoforms` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the isoforms of one protein, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Get protein isoforms and alternative products. Sources: uniprot.

## Usage

```text
biomcp protein isoforms [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. Value form: integer. Example: --limit 1. Command sources: uniprot; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source or retained page offset. Value form: integer. Example: --offset 0. Command sources: uniprot; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained collection identity. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: uniprot; source restrictions in the field description apply. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein isoforms P00533
```

```sh
biomcp protein isoforms P00533 --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein-isoforms/4`](/reference/schemas/biomcp-protein-isoforms-4/), whose page lists its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
