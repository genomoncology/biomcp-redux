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
verified: 2026-11-09
---

`biomcp article supplement` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one supplementary file of an article, or the rows of it that match a search, or saves the file. This page lists its arguments, examples and reply.

The program's help describes it:

> Read a research article supplementary file. Sources: pmc-oa, pubmed.

## Usage

```text
biomcp article supplement [OPTIONS] <input> <file>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--find` | string | no |  |  | Exact case-insensitive needle. Value form: string. Example: --find F1761I. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Matching-row or text-fragment offset. Value form: integer. Example: --offset 3. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
| `--replace` | boolean | no |  |  | Replace an existing destination only when explicitly requested with save. Value form: boolean. Example: --replace. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
| `--rows` | string | no |  |  | Inclusive original row range. Value form: string. Example: --rows 84-86. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
| `--save` | string | no |  |  | Save the whole original document to an explicit file or existing folder. Example: --save article-output. Value form: string. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
| `--sheet` | string | no |  |  | Sheet name for row selection. Value form: string. Example: --sheet ST2. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
| `--variant` | string | no |  |  | Explicit protein substitution. Value form: string. Example: --variant P245S. Command sources: pmc-oa, pubmed; source restrictions in the field description apply. |
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

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --find F1761I
```

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --find F1761I \
  --offset 3
```

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --sheet ST2 \
  --rows 84-86
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-supplement/7`](/reference/schemas/biomcp-article-supplement-7/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
