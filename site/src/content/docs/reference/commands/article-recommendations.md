---
title: biomcp article recommendations
description: "The BioMCP 1.0.0 command biomcp article recommendations: Lists
  papers that Semantic Scholar recommends from one or more seed papers."
goal: Look up the arguments, sections and examples of biomcp article recommendations.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article recommendations
kind: reference
verified: 2026-11-09
---

`biomcp article recommendations` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists papers that Semantic Scholar recommends from one or more seed papers. This page lists its arguments, examples and reply.

The program's help describes it:

> Get related research article recommendations. Sources: semantic-scholar.

## Usage

```text
biomcp article recommendations [OPTIONS] <seeds...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 100 |  | Maximum recommendations requested, in source order. Value form: integer. Example: --limit 5. Command sources: semantic-scholar; source restrictions in the field description apply. |
| `--negative` | array | no | max 10 |  | Up to ten negative paper identities. Value form: repeatable values (repeat the flag). Example: --negative 33433946. Command sources: semantic-scholar; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Number of stored recommendations to skip. Value form: integer. Example: --offset 1. Command sources: semantic-scholar; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Fixed recommendation pool. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: semantic-scholar; source restrictions in the field description apply. |
| `seeds` (position 1) | array | yes | min 1, max 10 |  | One to ten positive paper identities. |

## Examples

The program's help gives these examples.

```sh
biomcp article recommendations 34096690 --limit 5
```

```sh
biomcp article recommendations 34096690 32955176 --negative 33433946 --limit 5
```

```sh
biomcp article recommendations 34096690 --limit 5 --offset 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-recommendations/9`](/reference/schemas/biomcp-article-recommendations-9/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
