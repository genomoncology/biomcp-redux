---
title: biomcp article text
description: "The BioMCP 1.0.0 command biomcp article text: Reads one part of an
  article's full text, finds matching passages, or saves the text as Markdown."
goal: Look up the arguments, sections and examples of biomcp article text.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article text
kind: reference
verified: 2026-10-27
---

`biomcp article text` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one part of an article's full text, finds matching passages, or saves the text as Markdown. This page lists its arguments, examples and reply.

The program's help describes it:

> Read one bounded body part, find matching rows, or save the complete Markdown document.

## Usage

```text
biomcp article text [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--find` | string | no |  |  | Case-insensitive literal text to find. |
| `--offset` | integer | no | min 0 |  | Number of matching rows already returned. |
| `--part` | integer | no | min 1 |  | One-based outline part number. |
| `--replace` | boolean | no |  |  | Replace an existing destination only when explicitly requested with save. |
| `--save` | string | no |  |  | Save the whole original document to an explicit file or existing folder. |
| `--variant` | string | no |  |  | Explicit checked protein substitution. |
| `input` (position 1) | string | yes |  |  | PubMed or PMC identity. DOI selectors are not accepted by the reader. |

## Examples

The program's help gives these examples.

```sh
biomcp article text PMC5606870 --find AXIN2
```

```sh
biomcp article text PMC5606870 --variant Pro245Ser
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article_text/3`](/reference/schemas/biomcp-article-text-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [article guide](/user-guide/article/) shows every article command at work.
