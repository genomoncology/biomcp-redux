---
title: biomcp protein domains
description: "The BioMCP 1.0.0 command biomcp protein domains: Lists the
  InterPro domains of one protein, five at a time."
goal: Look up the arguments, sections and examples of biomcp protein domains.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein domains
kind: reference
verified: 2026-11-09
---

`biomcp protein domains` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the InterPro domains of one protein, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Get protein domains and families from InterPro. Sources: interpro.

## Usage

```text
biomcp protein domains [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. Value form: integer. Example: --limit 1. Command sources: interpro; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source or retained page offset. Value form: integer. Example: --offset 0. Command sources: interpro; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained collection identity. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: interpro; source restrictions in the field description apply. |
| `--type` | string | no |  |  | Exact source InterPro type, not a normalized domain label. Value form: string. Example: --type &lt;type&gt; (syntax; replace the placeholder with your retained value). Command sources: interpro; source restrictions in the field description apply. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein domains P00533
```

```sh
biomcp protein domains P00533 --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein-domains/7`](/reference/schemas/biomcp-protein-domains-7/), whose page lists its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
