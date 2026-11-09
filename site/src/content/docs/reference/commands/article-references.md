---
title: biomcp article references
description: "The BioMCP 1.0.0 command biomcp article references: Lists the
  papers one PubMed article cites, a page at a time."
goal: Look up the arguments, sections and examples of biomcp article references.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article references
kind: reference
verified: 2026-11-09
---

`biomcp article references` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the papers one PubMed article cites, a page at a time. This page lists its arguments, examples and reply.

The program's help describes it:

> List research papers referenced by an article. Sources: icite, pubmed, semantic-scholar.

## Usage

```text
biomcp article references [OPTIONS] <pmid>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 50 |  | Maximum papers on one page. Value form: integer. Example: --limit 1. Command sources: icite, pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Number of pool items to skip. Value form: integer. Example: --offset 112. Command sources: icite, pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Fixed snapshot from an earlier page. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: icite, pubmed, semantic-scholar; source restrictions in the field description apply. |
| `--source` | string | no |  | `icite`, `semantic-scholar` | Graph source; iCite remains the default; help article citations Graph source; iCite remains the default. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source semantic-scholar. Command sources: icite, pubmed, semantic-scholar; source restrictions in the field description apply. |
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

```sh
biomcp article references 33123537 --source semantic-scholar --offset 112 \
  --limit 1
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.article-edges/10`](/reference/schemas/biomcp-article-edges-10/), whose page lists its fields.

## Next

The [article guide](/user-guide/article/) shows every article command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
