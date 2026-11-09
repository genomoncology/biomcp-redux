---
title: biomcp article pdf
description: "The BioMCP 1.0.0 command biomcp article pdf: Downloads an
  article's PDF from its publisher to a file you name."
goal: Look up the arguments, sections and examples of biomcp article pdf.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article pdf
kind: reference
verified: 2026-11-09
---

`biomcp article pdf` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads an article's PDF from its publisher to a file you name. This page lists its arguments, examples and reply.

The program's help describes it:

> Save a research article PDF. Sources: semantic-scholar.

## Usage

```text
biomcp article pdf [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--replace` | boolean | no |  |  | Replace an existing file only when explicitly requested. Value form: boolean. Example: --replace. Command sources: semantic-scholar; source restrictions in the field description apply. |
| `--save` | string | yes |  |  | Destination file or existing folder. Value form: string. Example: --save owned.pdf. Command sources: semantic-scholar; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | Article identity. |

## Examples

The program's help gives these examples.

```sh
biomcp article pdf 41351423 --save owned.pdf --replace
```

```sh
biomcp article pdf 41351423 --save second.pdf --replace
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-pdf/6`](/reference/schemas/biomcp-article-pdf-6/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
