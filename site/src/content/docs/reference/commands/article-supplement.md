---
title: biomcp article supplement
description: "The BioMCP 1.0.0 command biomcp article supplement: Reads one
  supplementary file of an article, or the rows of it that match a search, or
  saves the file."
goal: Look up the arguments, sections and examples of biomcp article supplement.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article supplement
kind: reference
verified: 2026-10-27
---

`biomcp article supplement` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one supplementary file of an article, or the rows of it that match a search, or saves the file. This page lists its arguments, examples and reply.

The program's help describes it:

> Read a checked supplement file or page its original rows.

## Usage

```text
biomcp article supplement [OPTIONS] <input> <file>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--find` | string | no |  |  | Text to find, matched as typed, ignoring case. |
| `--offset` | integer | no | min 0 |  | Matching-row or text-fragment offset. |
| `--replace` | boolean | no |  |  | Replace an existing destination only when explicitly requested with save. |
| `--rows` | string | no |  |  | Inclusive original row range. |
| `--save` | string | no |  |  | Save the whole original document to an explicit file or existing folder. |
| `--sheet` | string | no |  |  | Sheet name for row selection. |
| `--variant` | string | no |  |  | Explicit protein substitution. |
| `input` (position 1) | string | yes |  |  | PubMed or PMC identity. |
| `file` (position 2) | string | yes |  |  | Exact file name from the supplements section. |

## Examples

The program's help gives these examples.

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
```

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article_supplement/3`](/reference/schemas/biomcp-article-supplement-3/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [article guide](/user-guide/article/) shows every article command at work.
