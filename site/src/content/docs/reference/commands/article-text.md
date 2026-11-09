---
title: biomcp article text
description: "The BioMCP 1.0.0 command biomcp article text: Reads one part of an
  article's full text, finds matching passages, or saves the text as Markdown."
goal: Look up the arguments, sections and examples of biomcp article text.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article text
kind: reference
verified: 2026-11-09
---

`biomcp article text` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one part of an article's full text, finds matching passages, or saves the text as Markdown. This page lists its arguments, examples and reply.

The program's help describes it:

> Get the abstract card by default, or explicitly read article body text. Sources: europepmc, pmc-oa, pubmed.

## Usage

```text
biomcp article text [OPTIONS] <input>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--find` | string | no |  |  | Case-insensitive exact text to find. Value form: string. Example: --find AXIN2. Command sources: europepmc, pmc-oa, pubmed; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Number of matching rows already returned. Value form: integer. Example: --offset 0. Command sources: europepmc, pmc-oa, pubmed; source restrictions in the field description apply. |
| `--part` | integer | no | min 1 |  | One-based outline part; omit reader selectors for the abstract card. Value form: integer. Example: --part 1. Command sources: europepmc, pmc-oa, pubmed; source restrictions in the field description apply. |
| `--replace` | boolean | no |  |  | Replace an existing destination only when explicitly requested with save. Value form: boolean. Example: --replace. Command sources: europepmc, pmc-oa, pubmed; source restrictions in the field description apply. |
| `--save` | string | no |  |  | Save the whole original document to an explicit file or existing folder. Example: --save article-output. Value form: string. Command sources: europepmc, pmc-oa, pubmed; source restrictions in the field description apply. |
| `--variant` | string | no |  |  | Explicit checked protein substitution. Example: --variant p.V600E. Value form: string. Command sources: europepmc, pmc-oa, pubmed; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | PubMed or PMC identity. DOI selectors are not accepted by the reader. |

## Examples

The program's help gives these examples.

```sh
biomcp article text PMC5606870 --find AXIN2
```

```sh
biomcp article text PMC5606870 --part 1
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.article-cards/9`](/reference/schemas/biomcp-article-cards-9/), [`biomcp.article-text/7`](/reference/schemas/biomcp-article-text-7/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
