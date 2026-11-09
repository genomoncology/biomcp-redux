---
title: biomcp protein complexes
description: "The BioMCP 1.0.0 command biomcp protein complexes: Lists the
  Complex Portal complexes that hold one protein, five at a time."
goal: Look up the arguments, sections and examples of biomcp protein complexes.
reader: A BioMCP user or agent checking one command
sidebar:
  label: protein complexes
kind: reference
verified: 2026-11-09
---

`biomcp protein complexes` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the Complex Portal complexes that hold one protein, five at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> List protein complexes containing a protein. Sources: complex-portal.

## Usage

```text
biomcp protein complexes [OPTIONS] <accession>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Five rows by default, at most 25. Value form: integer. Example: --limit 1. Command sources: complex-portal; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source or retained page offset. Value form: integer. Example: --offset 0. Command sources: complex-portal; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained collection identity. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: complex-portal; source restrictions in the field description apply. |
| `accession` (position 1) | string | yes |  |  | Exact source protein sequence identity. |

## Examples

The program's help gives these examples.

```sh
biomcp protein complexes P00533
```

```sh
biomcp protein complexes P00533 --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.protein-complexes/5`](/reference/schemas/biomcp-protein-complexes-5/), whose page lists its fields.

## Next

The [protein guide](/user-guide/protein/) shows every protein command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
