---
title: biomcp article citations
description: "The BioMCP 1.0.0 command biomcp article citations: Lists the
  papers that cite one PubMed article, a page at a time."
goal: Look up the arguments, sections and examples of biomcp article citations.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article citations
kind: reference
verified: 2026-10-27
---

`biomcp article citations` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the papers that cite one PubMed article, a page at a time. This page lists its arguments and examples.

The program's help describes it:

> Page the citing or referenced PubMed papers of one checked PMID.

## Usage

```text
biomcp article citations [OPTIONS] <pmid>
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
biomcp article citations 17611497
```

```sh
biomcp article citations 23000897 --source semantic-scholar
```

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
