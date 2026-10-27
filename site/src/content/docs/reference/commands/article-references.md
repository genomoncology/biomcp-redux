---
title: biomcp article references
description: "The BioMCP 1.0.0 command biomcp article references: Lists the
  papers one PubMed article cites, a page at a time."
goal: Look up the arguments, sections and examples of biomcp article references.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article references
kind: reference
verified: 2026-10-27
---

`biomcp article references` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the papers one PubMed article cites, a page at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> Page the citing or referenced PubMed papers of one checked PMID.

## Usage

```text
biomcp article references [OPTIONS] <pmid>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum papers on one page. |
| `--offset` | integer | no | min 0 |  | Number of pool items to skip. |
| `--pool` | string | no |  |  | Fixed snapshot from an earlier page. |
| `--source` | string | no |  | `icite`, `semantic-scholar` | Graph provider; iCite remains the default. |
| `pmid` (position 1) | string | yes |  |  | PMID, PMC or DOI; iCite accepts only a PMID. |

## Sources

`--source` picks the source to ask: [icite](/sources/icite/), [semantic-scholar](/sources/semantic-scholar/).

## Examples

The program's help gives these examples.

```sh
biomcp article references 34096690
```

```sh
biomcp article references 33123537 --source semantic-scholar
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article_edges/4`](/reference/schemas/biomcp-article-edges-4/), whose page shows a real reply and its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
