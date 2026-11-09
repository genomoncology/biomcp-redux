---
title: biomcp search author
description: "The BioMCP 1.0.0 command biomcp search author: Finds author
  records by name in Semantic Scholar."
goal: Look up the arguments, sections and examples of biomcp search author.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search author
kind: reference
verified: 2026-11-09
---

`biomcp search author` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds author records by name in Semantic Scholar. This page lists its arguments, examples and reply.

The program's help describes it:

> Search research paper authors. Sources: pubmed, semantic-scholar.

## Usage

```text
biomcp search author [OPTIONS] <query>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 25 |  | Bounded source page size. Value form: integer. Example: --limit 5. Command sources: pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source offset. Value form: integer. Example: --offset 0. Command sources: pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained page identity. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--source` | string | no |  | `semantic-scholar`, `pubmed`, `auto` | Source; auto uses PubMed if Semantic Scholar is off; help search author Source; auto uses PubMed if Semantic Scholar is off. PubMed accepts names or topics and returns unresolved names on papers. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source semanticscholar. Command sources: pubmed, semantic-scholar; source restrictions in the field description apply. |
| `query` (position 1) | string | yes |  |  | Exact query from the positional argument. |

## Sources

`--source` picks the source to ask: [semantic-scholar](/sources/semantic-scholar/), [pubmed](/sources/pubmed/).

## Examples

The program's help gives these examples.

```sh
biomcp search author 'Louis Williams'
```

```sh
biomcp search author 'Louis Williams' --source semanticscholar
```

```sh
biomcp search author 'Kai Muller' --disable-sources noncommercial,orcid
```

```sh
biomcp search author 'Morgan Lee' --disable-sources noncommercial,orcid \
  --format md
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.author-page/5`](/reference/schemas/biomcp-author-page-5/), whose page lists its fields.

## Next

The [author guide](/user-guide/author/) shows every author command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
