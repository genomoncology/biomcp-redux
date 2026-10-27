---
title: biomcp article text
description: "Reference for the BioMCP 1.0.0 command biomcp article text: Read
  one bounded body part, find matching rows, or save the complete Markdown
  document."
goal: Look up the arguments, sections and examples of biomcp article text.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article text
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp article text`.

Read one bounded body part, find matching rows, or save the complete Markdown document.

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

```sh
biomcp article text PMC5606870 --find AXIN2
biomcp article text PMC5606870 --variant Pro245Ser
```

## Reply schema

The help examples of this command return [`biomcp.article_text/3`](/reference/schemas/biomcp-article-text-3/). Print the full JSON schema with `biomcp schema biomcp.article_text/3`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [article entity](/user-guide/article/) page lists every article command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
