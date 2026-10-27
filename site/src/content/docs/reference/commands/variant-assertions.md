---
title: biomcp variant assertions
description: "The BioMCP 1.0.0 command biomcp variant assertions: Lists the
  assertions one source, such as ClinVar, makes about a variant, a page at a
  time."
goal: Look up the arguments, sections and examples of biomcp variant assertions.
reader: A BioMCP user or agent checking one command
sidebar:
  label: variant assertions
kind: reference
verified: 2026-10-27
---

`biomcp variant assertions` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the assertions one source, such as ClinVar, makes about a variant, a page at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Page assertions from one explicitly selected evidence source.

## Usage

```text
biomcp variant assertions [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--assertion` | value | no |  |  | Native assertion accession or document ID. |
| `--limit` | integer | no | min 1, max 50 |  | Assertions per page; omitted uses the native source default. |
| `--offset` | integer | no | min 0 |  | Assertions to skip in source order. |
| `--source` | string | yes |  | `clinvar`, `erepo` | One native source; no fallback or text search. |
| `--version` | value | no |  |  | VCV archive or ERepo document version; RCV/SCV selectors match a returned native version. |
| `input` (position 1) | string | yes |  |  | Checked Variation ID for ClinVar, or CAid for ERepo. |

## Sources

`--source` picks the source to ask: [clinvar](/sources/clinvar/), [erepo](/sources/clingen-erepo/).

## Examples

The program's help gives these examples.

```sh
biomcp variant assertions clinvar:13961 --source clinvar
```

```sh
biomcp variant assertions CA000434 --source erepo
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant_assertions/3`](/reference/schemas/biomcp-variant-assertions-3/), whose page shows a real reply and its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
