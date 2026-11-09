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
verified: 2026-11-09
---

`biomcp article citation-evidence` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows where and how one article cites another, with the citing passage when the full text is open. This page lists its arguments, examples and reply.

The program's help describes it:

> Get research article citation evidence. Sources: pmc-oa, pubmed, semantic-scholar.

## Usage

```text
biomcp article citation-evidence [OPTIONS] <citing> <cited>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--fulltext` | boolean | no |  |  | Include the citing paper's own explicit reference paragraphs. Value form: boolean. Example: --fulltext. Command sources: pmc-oa, pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Context fragment position returned by the advancing step. Value form: integer. Example: --offset 0. Command sources: pmc-oa, pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--paragraph-offset` | integer | no | min 0 |  | Full-text fragment position returned by the advancing step. Value form: integer. Example: --paragraph-offset 0. Command sources: pmc-oa, pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Stored checked source scan and original hit positions. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: pmc-oa, pubmed, semantic-scholar; source restrictions in the field description apply. |
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

```sh
biomcp article citation-evidence 33123537 17611497 --fulltext \
  --disable-sources semantic-scholar
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-evidence/6`](/reference/schemas/biomcp-article-evidence-6/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
