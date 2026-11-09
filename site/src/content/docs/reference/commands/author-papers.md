---
title: biomcp author papers
description: "The BioMCP 1.0.0 command biomcp author papers: Lists the papers of
  one author record from Semantic Scholar or ORCID, a page at a time."
goal: Look up the arguments, sections and examples of biomcp author papers.
reader: A BioMCP user or agent checking one command
sidebar:
  label: author papers
kind: reference
verified: 2026-11-09
---

`biomcp author papers` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It lists the papers of one author record from Semantic Scholar or ORCID, a page at a time. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> List an author's research papers. Sources: orcid, semantic-scholar.

## Usage

```text
biomcp author papers [OPTIONS] <id> [section]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--full` | boolean | no |  |  | Legacy alias for details; normalized on construction. Value form: boolean. Example: --full. Command sources: orcid, semantic-scholar; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 25 |  | Bounded row count. Value form: integer. Example: --limit 5. Command sources: orcid, semantic-scholar; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Source or retained collection offset. Value form: integer. Example: --offset 0. Command sources: orcid, semantic-scholar; source restrictions in the field description apply. |
| `--pool` | string | no |  |  | Retained collection identity. Value form: string. Example: --pool &lt;pool&gt; (syntax; replace the placeholder with your retained value). Command sources: orcid, semantic-scholar; source restrictions in the field description apply. |
| `id` (position 1) | string | yes |  |  | Exact qualified source record. |
| `section` (position 2) | string | no |  | `details` | Detailed disclosure when requested; help author papers Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Detailed disclosure when requested.

| Section | Description |
| --- | --- |
| `details` | Source detailed metadata for the same paper window. |

## Examples

The program's help gives these examples.

```sh
biomcp author papers semantic-scholar:123
```

```sh
biomcp author papers semantic-scholar:123 details
```

```sh
biomcp author papers semantic-scholar:123 --full
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.author-paper-details/6`](/reference/schemas/biomcp-author-paper-details-6/), [`biomcp.author-papers/6`](/reference/schemas/biomcp-author-papers-6/), [`biomcp.orcid-works/8`](/reference/schemas/biomcp-orcid-works-8/), whose page lists its fields.

## Next

The [author guide](/user-guide/author/) shows every author command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
