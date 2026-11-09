---
title: biomcp author
description: "The BioMCP 1.0.0 command biomcp author: Read papers attributed to
  a source author record."
goal: Look up the arguments, sections and examples of biomcp author.
reader: A BioMCP user or agent checking one command
sidebar:
  label: author
kind: reference
verified: 2026-11-09
---

`biomcp author` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It read papers attributed to a source author record. This page lists its examples and reply.

The program's help describes it:

> Read papers attributed to a source author record.

## Usage

```text
biomcp author
```

## Examples

The program's help gives this example.

```sh
biomcp author papers semantic-scholar:123
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.author-paper-details/6`](/reference/schemas/biomcp-author-paper-details-6/), [`biomcp.author-papers/6`](/reference/schemas/biomcp-author-papers-6/), [`biomcp.orcid-works/8`](/reference/schemas/biomcp-orcid-works-8/), whose page lists its fields.

## Next

The [author guide](/user-guide/author/) shows every author command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
