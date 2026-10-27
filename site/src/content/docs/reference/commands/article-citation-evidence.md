---
title: biomcp article citation-evidence
description: "The BioMCP 1.0.0 command biomcp article citation-evidence: Shows
  where and how one article cites another, with the citing passage when the full
  text is open."
goal: Look up the arguments, sections and examples of biomcp article
  citation-evidence.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article citation-evidence
kind: reference
verified: 2026-10-27
---

`biomcp article citation-evidence` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows where and how one article cites another, with the citing passage when the full text is open. This page lists its arguments, examples and reply.

The program's help describes it:

> Read every source context and optional explicit full-text citation paragraph.

## Usage

```text
biomcp article citation-evidence [OPTIONS] <citing> <cited>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fulltext` | boolean | no |  |  | Include the citing paper's own explicit reference paragraphs. |
| `--offset` | integer | no | min 0 |  | Context fragment position returned by the advancing step. |
| `--paragraph-offset` | integer | no | min 0 |  | Full-text fragment position returned by the advancing step. |
| `--pool` | string | no |  |  | Stored checked source scan and original hit positions. |
| `citing` (position 1) | string | yes |  |  | Citing PMID, PMC identity or DOI. |
| `cited` (position 2) | string | yes |  |  | Cited PMID, PMC identity or DOI. |

## Examples

The program's help gives these examples.

```sh
biomcp article citation-evidence 33123537 17611497
```

```sh
biomcp article citation-evidence 33123537 17611497 --fulltext
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article_evidence/2`](/reference/schemas/biomcp-article-evidence-2/), whose page shows a real reply and its fields.

## Next

[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. The [article guide](/user-guide/article/) shows every article command at work.
