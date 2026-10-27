---
title: biomcp search author
description: "The BioMCP 1.0.0 command biomcp search author: Finds author
  records by name in Semantic Scholar."
goal: Look up the arguments, sections and examples of biomcp search author.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search author
kind: reference
verified: 2026-10-27
---

`biomcp search author` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds author records by name in Semantic Scholar. This page lists its arguments, examples and reply.

The program's help describes it:

> Literal author name query and native window; positional text maps to query. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

## Usage

```text
biomcp search author [OPTIONS] <query>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Bounded source page size. |
| `--offset` | integer | no | min 0 |  | Native offset. |
| `--pool` | string | no |  |  | Retained page identity. |
| `--source` | string | no |  | `semantic-scholar`, `semanticscholar` | Native author source. |
| `query` (position 1) | string | yes |  |  | Literal query from the positional argument or --query. |

## Sources

`--source` picks the source to ask: [semantic-scholar](/sources/semantic-scholar/).

## Examples

The program's help gives these examples.

```sh
biomcp search author 'Louis Williams'
```

```sh
biomcp search author 'Louis Williams' --source semanticscholar
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.author_page/2`](/reference/schemas/biomcp-author-page-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [author guide](/user-guide/author/) shows every author command at work.
