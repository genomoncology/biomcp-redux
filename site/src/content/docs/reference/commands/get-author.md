---
title: biomcp get author
description: "The BioMCP 1.0.0 command biomcp get author: Reads author records
  by their Semantic Scholar or ORCID ID, in the order you give."
goal: Look up the arguments, sections and examples of biomcp get author.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get author
kind: reference
verified: 2026-11-09
---

`biomcp get author` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads author records by their Semantic Scholar or ORCID ID, in the order you give. This page lists its arguments, examples and reply.

The program's help describes it:

> Get research author details. Sources: orcid, semantic-scholar.

## Usage

```text
biomcp get author <ids...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `ids` (position 1) | array | yes | min 1, max 20 |  | One or more exact qualified IDs. |

## Examples

The program's help gives these examples.

```sh
biomcp get author semantic-scholar:456 semantic-scholar:123 \
  semantic-scholar:999
```

```sh
biomcp get author semantic-scholar:123 semantic-scholar:123
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.author-card/6`](/reference/schemas/biomcp-author-card-6/), whose page lists its fields.

## Next

The [author guide](/user-guide/author/) shows every author command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
