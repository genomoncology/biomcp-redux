---
title: biomcp article pdf
description: "Reference for the BioMCP 1.0.0 command biomcp article pdf:
  Download one publisher PDF to an explicit file or existing folder."
goal: Look up the arguments, sections and examples of biomcp article pdf.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article pdf
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp article pdf`.

Download one publisher PDF to an explicit file or existing folder.

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

```sh
biomcp article pdf 41351423 --save owned.pdf --replace
biomcp article pdf 41351423 --save second.pdf --replace
```

## Reply schema

The help examples of this command return [`biomcp.article_pdf/2`](/reference/schemas/biomcp-article-pdf-2/). Print the full JSON schema with `biomcp schema biomcp.article_pdf/2`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [article entity](/user-guide/article/) page lists every article command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
