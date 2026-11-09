---
title: biomcp article
description: "The BioMCP 1.0.0 command biomcp article: Read article text,
  references, citations and files."
goal: Look up the arguments, sections and examples of biomcp article.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article
kind: reference
verified: 2026-11-09
---

`biomcp article` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read article text, references, citations and files. This page lists its examples and reply.

The program's help describes it:

> Read article text, references, citations and files.

## Usage

```text
biomcp article
```

## Examples

The program's help gives this example.

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.article-cards/9`](/reference/schemas/biomcp-article-cards-9/), [`biomcp.article-edges/10`](/reference/schemas/biomcp-article-edges-10/), [`biomcp.article-evidence/6`](/reference/schemas/biomcp-article-evidence-6/), [`biomcp.article-pdf/6`](/reference/schemas/biomcp-article-pdf-6/), [`biomcp.article-recommendations/9`](/reference/schemas/biomcp-article-recommendations-9/), [`biomcp.article-supplement/7`](/reference/schemas/biomcp-article-supplement-7/), [`biomcp.article-text/7`](/reference/schemas/biomcp-article-text-7/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
