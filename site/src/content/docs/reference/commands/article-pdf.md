---
title: biomcp article pdf
description: "The BioMCP 1.0.0 command biomcp article pdf: Downloads an
  article's PDF from its publisher to a file you name."
goal: Look up the arguments, sections and examples of biomcp article pdf.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article pdf
kind: reference
verified: 2026-10-27
---

`biomcp article pdf` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It downloads an article's PDF from its publisher to a file you name. This page lists its arguments, examples and reply.

The program's help describes it:

> Download one publisher PDF to an explicit file or existing folder.

## Usage

```text
biomcp article pdf [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--replace` | boolean | no |  |  | Replace an existing file only when explicitly requested. |
| `--save` | string | yes |  |  | Destination file or existing folder. |
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

The reply is one JSON object. Its schema is [`biomcp.article_pdf/2`](/reference/schemas/biomcp-article-pdf-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [article guide](/user-guide/article/) shows every article command at work.
